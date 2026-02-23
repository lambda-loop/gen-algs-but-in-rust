

// because, sometimes, changing the fitness function is awesome, lets use this:

use std::{collections::{BTreeSet, BinaryHeap, HashSet}, hash::{self, Hash}};
use crate::genetic::genome::{Fen, Genome};

struct Indexed<A> (usize, A);
// TODO: instance the Ord



struct Table<Dna: Genome, const ELITE_SIZE: usize> {
    elite: [Dna; ELITE_SIZE],
    heap : BTreeSet<
        Indexed<
            Fen<Dna>
        >
    >,
}

impl<Dna: Genome, const ELITE_SIZE: usize> 
    Table<Dna, ELITE_SIZE> 
    where Dna: Eq + Clone {

    fn safe_insert(&mut self, fen: Fen<Dna>) {
        const PLACE_HOLDER: usize = 0;
        let ifen = Indexed(PLACE_HOLDER, fen);
        if self.heap.contains(&ifen) { return () } // safe part

        // WARNING: will maybe be the last..
        let Indexed(i, _) = self.heap.pop_first().unwrap();

        self.elite[i] = ifen.1.codex.clone();
        self.heap.insert(Indexed(i, ifen.1));
    }

}



impl<A> PartialEq for Indexed<A> where A: PartialEq {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0 && self.1 == other.1
    }
}
impl<A> Eq for Indexed<A> where A: PartialEq { }
impl<A> PartialOrd for Indexed<A> where A: PartialOrd {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match self.0.partial_cmp(&other.0) {
            Some(core::cmp::Ordering::Equal) => {}
            ord => return ord,
        }
        self.1.partial_cmp(&other.1)
    }
}

impl<A> Ord for Indexed<A> where A: Ord {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        todo!()
    }
}

// TODO: evolver
// TODO: agents

// static fitness: Arc<RwLock<fn()>>
//
//     fn fit(&self) -> Self::Score; 

// pub static current_fi

// pub trait GenomeFit {
//     type Score: Ord;
//
//     const fit: Arc<RwLock<fn(g: &Self) -> Self::Score>>;
//     // fn fit(&self) -> Self::Score; 
//
//     fn new() -> Self;
//     // TODO: taking the ownership could be great!
//     fn point(&mut self);
//     fn cross(&mut self, other: &mut Self); 
//
//     fn done(score: Self::Score) -> bool;
//
// }
