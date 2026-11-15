
use rand::{rngs::SmallRng, SeedableRng, Rng};

use super::super::genome::*;

use std::{
    collections::BinaryHeap,
    sync::{
        atomic::{AtomicU64, Ordering},
        mpsc::{sync_channel, Receiver, SyncSender},
        Mutex,
    },
    thread,
};

// --------------------------------------------------
// TABLE
// --------------------------------------------------

struct Table<'a, G: Genome> {
    elite: &'a Mutex<Box<[G]>>,
    queue: BinaryHeap<(G::Score, usize)>,
}

impl<'a, G: Genome> Table<'a, G> {
    fn new(elite: &'a Mutex<Box<[G]>>) -> Self {
        let pop = elite.lock().unwrap();

        let mut queue = BinaryHeap::with_capacity(pop.len());

        for (i, g) in pop.iter().enumerate() {
            queue.push((G::fit(g), i));
        }

        Self { elite, queue }
    }
}

// --------------------------------------------------
// BUDGET
// --------------------------------------------------

// Reserva exatamente uma avaliação.
// Retorna false quando o budget acabou.
fn reserve(used: &AtomicU64, budget: u64) -> bool {
    used.fetch_update(
        Ordering::Relaxed,
        Ordering::Relaxed,
        |n| {
            if n < budget {
                Some(n + 1)
            } else {
                None
            }
        },
    ).is_ok()
}

// --------------------------------------------------
// MAIN
// --------------------------------------------------

fn run_island<G: Genome>(
    budget: u64,
    island_pop: usize,
    num_islands: usize,
    bound: usize,
    num_workers: usize,
) {
    assert!(island_pop > 0);
    assert!(num_islands > 0);
    assert!(num_workers > 0);

    let used = AtomicU64::new(0);

    // Uma elite por ilha.
    let elites: Vec<Mutex<Box<[G]>>> =
        (0..num_islands)
            .map(|_| {
                let pop: Box<[G]> = (0..island_pop)
                    .map(|_| G::new())
                    .collect();

                Mutex::new(pop)
            })
            .collect();

    // Uma tabela por ilha.
    let tables: Vec<Table<'_, G>> =
        elites.iter()
            .map(Table::new)
            .collect();

    // Pior score de cada ilha.
    let g_wrost: Vec<Mutex<G::Score>> =
        tables.iter()
            .map(|t| {
                let worst = t.queue.peek().unwrap().0;
                Mutex::new(worst)
            })
            .collect();

    // Um canal por ilha.
    let (sxs, rxs): (Vec<_>, Vec<_>) =
        (0..num_islands)
            .map(|_| sync_channel::<(G, G::Score)>(bound))
            .unzip();

    thread::scope(|scope| {
        // Managers
        for ((table, worst), rx) in
            tables.into_iter()
                .zip(g_wrost.iter())
                .zip(rxs)
        {
            let l_wrost = *worst.lock().unwrap();

            start_manager(
                scope,
                table,
                worst,
                l_wrost,
                rx,
            );
        }

        // Workers
        for i in 0..num_islands {
            for _ in 0..num_workers {
                start_worker(
                    scope,
                    &elites[i],
                    island_pop,
                    &g_wrost[i],
                    sxs[i].clone(),
                    SmallRng::from_os_rng(),
                    &used,
                    budget,
                );
            }
        }

        // Traveler global.
        // Recebe seus próprios clones dos senders.
        start_traveler(
            scope,
            &elites,
            island_pop,
            &g_wrost,
            sxs.iter().cloned().collect(),
            SmallRng::from_os_rng(),
            &used,
            budget,
        );

        // Fundamental: fechar os senders originais.
        // Os clones serão destruídos quando
        // workers e traveler terminarem.
        drop(sxs);
    });

    // Todas as threads terminaram.
    println!(
        "Avaliações: {}",
        used.load(Ordering::Relaxed)
    );
}

// --------------------------------------------------
// MANAGER
// --------------------------------------------------

fn start_manager<'scope, 'env, G>(
    scope: &'scope thread::Scope<'scope, 'env>,
    table: Table<'env, G>,
    g_wrost: &'env Mutex<G::Score>,
    l_wrost: G::Score,
    rx: Receiver<(G, G::Score)>,
)
where
    G: Genome + 'scope,
    G::Score: 'scope,
    'env: 'scope,
{
    scope.spawn(move || {
        manager(table, g_wrost, l_wrost, rx);
    });
}

fn manager<G: Genome>(
    mut table: Table<'_, G>,
    g_wrost: &Mutex<G::Score>,
    mut l_wrost: G::Score,
    rx: Receiver<(G, G::Score)>,
) {
    while let Ok((g, s)) = rx.recv() {
        if s < l_wrost {
            let i = {
                let mut p = table.queue.peek_mut().unwrap();
                p.0 = s;
                p.1
            };

            table.elite.lock().unwrap()[i] = g;

            let new_worst = table.queue.peek().unwrap().0;

            *g_wrost.lock().unwrap() = new_worst;
            l_wrost = new_worst;
        }
    }
}

// --------------------------------------------------
// WORKER
// --------------------------------------------------

fn start_worker<'scope, 'env, G>(
    scope: &'scope thread::Scope<'scope, 'env>,
    elite: &'env Mutex<Box<[G]>>,
    elite_len: usize,
    g_wrost: &'env Mutex<G::Score>,
    sx: SyncSender<(G, G::Score)>,
    rng: SmallRng,
    used: &'env AtomicU64,
    budget: u64,
)
where
    G: Genome + 'scope,
    G::Score: 'scope,
    'env: 'scope,
{
    scope.spawn(move || {
        worker(
            elite, elite_len, g_wrost,
            sx, rng, used, budget,
        );
    });
}

fn worker<G: Genome>(
    elite: &Mutex<Box<[G]>>,
    elite_len: usize,
    g_wrost: &Mutex<G::Score>,
    sx: SyncSender<(G, G::Score)>,
    mut rng: SmallRng,
    used: &AtomicU64,
    budget: u64,
) {
    let mut elite_c = Vec::with_capacity(elite_len);
    let mut buffer = G::new();

    loop {
        // Evita copiar a população se acabou.
        if used.load(Ordering::Relaxed) >= budget {
            return;
        }

        let mut l_worst = *g_wrost.lock().unwrap();

        {
            let elite = elite.lock().unwrap();
            elite_c.extend(elite.iter().cloned());
        }

        for l in &elite_c {
            for r in &elite_c {
                // Reserva antes da avaliação.
                if !reserve(used, budget) {
                    return;
                }

                l.cross_in(r, &mut buffer, &mut rng);
                buffer.point(&mut rng);

                let s = G::fit(&buffer);

                if s < l_worst {
                    l_worst = s;

                    if sx.send((buffer.clone(), s)).is_err() {
                        return;
                    }
                }
            }
        }

        elite_c.clear();
    }
}

// --------------------------------------------------
// TRAVELER
// --------------------------------------------------

fn start_traveler<'scope, 'env, G>(
    scope: &'scope thread::Scope<'scope, 'env>,
    islands: &'env [Mutex<Box<[G]>>],
    elite_len: usize,
    g_wrost: &'env [Mutex<G::Score>],
    sxs: Vec<SyncSender<(G, G::Score)>>,
    rng: SmallRng,
    used: &'env AtomicU64,
    budget: u64,
)
where
    G: Genome + 'scope,
    G::Score: 'scope,
    'env: 'scope,
{
    scope.spawn(move || {
        traveler(
            islands, elite_len, g_wrost,
            sxs, rng, used, budget,
        );
    });
}

fn traveler<G: Genome>(
    islands: &[Mutex<Box<[G]>>],
    elite_len: usize,
    g_wrost: &[Mutex<G::Score>],
    sxs: Vec<SyncSender<(G, G::Score)>>,
    mut rng: SmallRng,
    used: &AtomicU64,
    budget: u64,
) {
    let islands_len = islands.len();

    let mut elite_c_l = Vec::with_capacity(elite_len);
    let mut elite_c_r = Vec::with_capacity(elite_len);
    let mut buffer = G::new();

    loop {
        if used.load(Ordering::Relaxed) >= budget {
            return;
        }

        let l_isl = rng.random_range(0..islands_len);
        let r_isl = rng.random_range(0..islands_len);
        let t_isl = rng.random_range(0..islands_len);

        {
            let elite = islands[l_isl].lock().unwrap();
            elite_c_l.extend(elite.iter().cloned());
        }

        {
            let elite = islands[r_isl].lock().unwrap();
            elite_c_r.extend(elite.iter().cloned());
        }

        let mut l_worst = *g_wrost[t_isl].lock().unwrap();
        let sx = &sxs[t_isl];

        for l in &elite_c_l {
            for r in &elite_c_r {
                if !reserve(used, budget) {
                    return;
                }

                l.cross_in(r, &mut buffer, &mut rng);
                buffer.point(&mut rng);

                let s = G::fit(&buffer);

                if s < l_worst {
                    l_worst = s;

                    if sx.send((buffer.clone(), s)).is_err() {
                        return;
                    }
                }
            }
        }

        elite_c_l.clear();
        elite_c_r.clear();
    }
}

