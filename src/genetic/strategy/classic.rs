use std::{collections::BinaryHeap, fmt::Debug};

use crate::genetic::genome::{Fen, Genome};

use rand::prelude::*;

#[derive(Debug, PartialEq)]
pub enum Status {
    Running,
    Over,
}

// TODO: make a safe `new`
#[derive(Debug)]
pub struct Nature
    <const GEN_SIZE   : usize
    ,const ELITE_SIZE : usize
    ,const MAX_GENS   : u64
    ,Dna: Genome> {
        pub population: [Dna; GEN_SIZE],
        pub status: Status,
        pub current_gen: u64, // bigint??
}

impl<const GEN_SIZE   : usize
    ,const ELITE_SIZE : usize
    ,const MAX_GENS   : u64
    ,Dna: Genome + Clone + Debug + Eq> Nature
    <GEN_SIZE, ELITE_SIZE,MAX_GENS, Dna> {

    pub fn new() -> Self {
        let population: [Dna; GEN_SIZE] = 
            std::iter::repeat_with(|| Dna::new())
            .take(GEN_SIZE)
            .collect::<Vec<_>>()
            .try_into()
            .unwrap();
            
        Nature {
            population, 
            status: Status::Running, 
            current_gen: 0,
        }
    }

    pub fn run(&mut self) -> Dna {
        while self.current_gen < MAX_GENS
           && self.status != Status::Over {
            self.next_generation()
        }

        self.population[0].clone()
    }
    
    fn next_generation(&mut self) {

        let elite = Self::pick_elite(&self.population);

        println!("generation: {:?}", self.current_gen);
        // dbg!(&elite[0]);

        // abuse
        let fit_best = Dna::fit(&elite[0]);
        if Dna::done(fit_best) {
            self.status = Status::Over;
        }
          

        let diff    = GEN_SIZE - ELITE_SIZE;
        let middle  = ELITE_SIZE + diff/2;
        let mut rng = rand::rng();

        for i in (ELITE_SIZE..middle).step_by(2) {
            let jl = rng.random_range(0..ELITE_SIZE);
            let jr = rng.random_range(0..ELITE_SIZE);

            let mut vl = elite[jl].clone();
            let mut vr = elite[jr].clone();

            vl.cross(&mut vr);

            self.population[i]   = vl;
            self.population[i+1] = vr;
        }

        for i in middle..GEN_SIZE {
            // WARNING: itll be optimized to reuse the same space??
            let j = rng.random_range(0..ELITE_SIZE);
            let mut v = elite[j].clone();
            v.point();
            self.population[i] = v;
        }

        for (i, v) in elite.into_iter().enumerate() {
          self.population[i] = v;
        }

        self.current_gen += 1;
    }

    fn pick_elite (population: &[Dna; GEN_SIZE]) -> [Dna; ELITE_SIZE] {
        let fit = |g: &Dna| Fen { 
                codex: g.clone(),
                score: Dna::fit(g),
            };

        let init      = &population[..ELITE_SIZE];
        let init_fens = init.iter().map(fit);

        let mut heap = BinaryHeap::new();
        for fen in init_fens {
            heap.push(fen);
        }

        for fen in population[ELITE_SIZE..].iter().map(fit) {
            let worst = heap.peek().unwrap();
            if fen > *worst {
                heap.pop();
                heap.push(fen)
            }
        }

        let elite: [Dna; ELITE_SIZE] = 
            heap
            .into_iter() 
            .take(ELITE_SIZE) // TODO: cut it out?
            .map(|fen| fen.codex) 
            .collect::<Vec<_>>()
            .try_into()
            .unwrap();

        elite
    }



}


