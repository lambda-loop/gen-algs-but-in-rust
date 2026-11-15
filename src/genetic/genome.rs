
use rand::{
    *, rngs::SmallRng,
};
use std::sync::{Arc, RwLock};

pub trait Genome: Clone + Send + Sync {
    type Score: Ord + Copy + Send + Sync;

    fn fit(&self) -> Self::Score; 
    fn new() -> Self;

    fn point (&mut self, rng: &mut SmallRng);
    fn cross_in (
        &self           ,
        other:     &Self,
        r#in : &mut Self,
        rng  : &mut SmallRng,
    ); 
}


#[derive(PartialEq, Eq)]
pub struct Fen<G> where G: Genome {
    pub codex: G,
    pub score: G::Score,
}

impl<G: Genome> PartialOrd for Fen<G>
where G: Eq {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.score.partial_cmp(&other.score)
    }
}

impl<G: Genome> Ord for Fen<G> where G: Eq {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.score.cmp(&other.score)
        
    }
}



