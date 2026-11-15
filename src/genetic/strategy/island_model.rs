
use rand::{
    *, rngs::SmallRng,
};

use super::super::genome::*;
use std::{
    thread,
    sync::{
        atomic::{AtomicU64, Ordering},
        Arc, 
        Mutex, 
        mpsc::{
            sync_channel,
            SyncSender, Receiver,
        }
    },
    collections::{
        BinaryHeap,
    },
};

struct Table<'a, G: Genome> {
    elite: &'a Mutex<Box<[G]>>,
    queue: BinaryHeap<(G::Score, usize)>,
}

impl<'a, G: Genome> Table<'a, G> {
    fn new(elite: &'a Mutex<Box<[G]>>) -> Self {
        let guard = elite.lock().unwrap();

        let mut queue = BinaryHeap::with_capacity(guard.len());

        for (i, g) in guard.iter().enumerate() {
            queue.push((G::fit(g), i));
        }

        Self { elite, queue }
    }
}

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

    let used = Arc::new(AtomicU64::new(0));

    // Elites compartilhadas
    let elites: Vec<Mutex<Box<[G]>>> = (0..num_islands)
        .map(|_| {
            let pop: Box<[G]> = (0..island_pop)
                .map(|_| G::new())
                .collect();

            Mutex::new(pop)
        })
        .collect();

    // Uma fila de prioridades por ilha
    let tables: Vec<Table<'_, G>> = elites
        .iter()
        .map(Table::new)
        .collect();

    // Pior score de cada ilha
    let g_wrost: Vec<Mutex<G::Score>> = tables
        .iter()
        .map(|t| {
            Mutex::new(t.queue.peek().unwrap().0)
        })
        .collect();

    // Um canal por ilha
    let (sxs, rxs): (Vec<_>, Vec<_>) = (0..num_islands)
        .map(|_| sync_channel::<(G, G::Score)>(bound))
        .unzip();

    thread::scope(|scope| {
        for ((table, worst), rx) in tables
            .into_iter()
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

        for i in 0..num_islands {
            for _ in 0..num_workers {
                start_worker(
                    scope,
                    &elites[i],
                    island_pop,
                    &g_wrost[i],
                    sxs[i].clone(),
                    SmallRng::from_os_rng(),
                    Arc::clone(&used),
                    budget,
                );
            }
        }

        start_traveler(
            scope,
            &elites,
            island_pop,
            &g_wrost,
            &sxs,
            SmallRng::from_os_rng(),
            Arc::clone(&used),
            budget,
        );

        drop(sxs);

    });
}

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

fn start_worker<'scope, 'env, G: Genome> (
    scope: &'scope std::thread::Scope<'scope, '_>,
    elite      : &'env Mutex<Box<[G]>>           ,
    elite_len  : usize                           , 
    g_wrost    : &'env Mutex<G::Score>           , // global wrost
    sx         : SyncSender<(G, G::Score)>       ,
    rng        : SmallRng                        ,
    used       : Arc<AtomicU64>                  ,
) where 'env: 'scope, {
    scope.spawn(move || {
        worker(elite, elite_len, g_wrost, sx, rng, used, budget);
    });
}

fn worker<G: Genome> (
    elite      : &Mutex<Box<[G]>>         ,
    elite_len  : usize                    , 
    g_wrost    : &Mutex<G::Score>         , // global wrost
    sx         : SyncSender<(G, G::Score)>,
    mut rng    : SmallRng                 ,
    used       : Arc<AtomicU64>           ,
) {
    let mut elite_c  = Vec::with_capacity(elite_len);
    let mut buffer   = G::new();

    loop {
        let mut l_worst = *g_wrost.lock().unwrap();
        let elite = elite.lock().unwrap();
        for i in 0..elite_len {
            elite_c.push(elite[i].clone());
        }
        drop(elite);

        for l in &elite_c {
            for r in &elite_c {
                l.cross_in(r, &mut buffer, &mut rng);
                buffer.point(&mut rng);

                if used.fetch_update(
                    Ordering::Relaxed,
                    Ordering::Relaxed,
                    |n| (n < budget).then_some(n + 1),
                ).is_err() {
                    return;
                }
                
                let s = G::fit(&buffer);
                if s < l_worst {
                    l_worst = s;
                    sx.send((buffer.clone(), s)).unwrap();
                }
            }
        }

        elite_c.clear();
    }
}


fn start_traveler<'scope, 'env, G: Genome> (
    scope      : &'scope std::thread::Scope<'scope, '_>,
    islands    : &'env[Mutex<Box<[G]>>]                ,
    elite_len  : usize                                 , 
    g_wrost    : &'env[Mutex<G::Score>]                , // global wrost
    sxs        : &'env[SyncSender<(G, G::Score)>]      ,
    rng    : SmallRng                                  ,
    used       : Arc<AtomicU64>                        ,
) where 'env: 'scope, {
    scope.spawn(move || {
        traveler(islands, elite_len, g_wrost, sxs, rng, used);
    });
}


fn traveler<G: Genome> (
    islands    : &[Mutex<Box<[G]>>]          ,
    elite_len  : usize                       , 
    g_wrost    : &[Mutex<G::Score>]          , // global wrost
    sxs        : &[SyncSender<(G, G::Score)>],
    mut rng    : SmallRng                    ,
    used       : Arc<AtomicU64>              ,
) {
    let islands_len = islands.len();

    let mut elite_c_l  = Vec::with_capacity(elite_len);
    let mut elite_c_r  = Vec::with_capacity(elite_len);
    let mut buffer   = G::new();

    loop {
        let l_isl = rng.random_range(0..islands_len);
        let r_isl = rng.random_range(0..islands_len);
        let t_isl = rng.random_range(0..islands_len); // target

        let elite = islands[l_isl].lock().unwrap();
        for i in 0..elite_len {
            elite_c_l.push(elite[i].clone());
        }
        drop(elite);

        let elite = islands[r_isl].lock().unwrap();
        for i in 0..elite_len {
            elite_c_r.push(elite[i].clone());
        }
        drop(elite);

        let mut l_worst = *g_wrost[t_isl].lock().unwrap();
        let sx = &sxs[t_isl];
        for l in &elite_c_l {
            for r in &elite_c_r {
                l.cross_in(r, &mut buffer, &mut rng);
                buffer.point(&mut rng);

                let s = G::fit(&buffer);
                used.fetch_add(1, Ordering::Relaxed);

                if s < l_worst {
                    l_worst = s;
                    sx.send((buffer.clone(), s)).unwrap();
                }
            }
        }

        elite_c_l.clear();
        elite_c_r.clear();
    }
}

fn server() {}
