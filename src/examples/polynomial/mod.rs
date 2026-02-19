

// use num_bigint::{BigInt, BigUint};

use crate::genetic::genome::Genome;

mod data;
use data::*;

mod score;
use score::*;

impl Genome for Polynomial {
    type Score = i32;

    fn fit(&self) -> Self::Score {
        todo!()
    }

    fn new() -> Self {
        todo!()
    }

    fn point(&mut self) {
        todo!()
    }

    fn cross(&mut self, other: &mut Self) {
        todo!()
    }

    fn done(score: Self::Score) -> bool {
        todo!()
    }
}


