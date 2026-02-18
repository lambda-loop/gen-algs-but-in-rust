use rand::prelude::*;
use std::{mem};

use crate::genetic::genome::Genome;

pub type Word<const N: usize> = [u8; N];
// pub const ANSWER: [u8; 100] = *b"Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore.";
pub const ANSWER: [u8; 12] = *b"Hello, Rust!";
// pub const ANSWER: [char; 100] = [
//     'L', 'o', 'r', 'e', 'm', ' ', 'i', 'p', 's', 'u', 'm', ' ', 'd', 'o', 'l', 'o', 'r', ' ', 's', 'i', 
//     't', ' ', 'a', 'm', 'e', 't', ',', ' ', 'c', 'o', 'n', 's', 'e', 'c', 't', 'e', 't', 'u', 'r', ' ', 
//     'a', 'd', 'i', 'p', 'i', 's', 'c', 'i', 'n', 'g', ' ', 'e', 'l', 'i', 't', ',', ' ', 's', 'e', 'd', 
//     ' ', 'd', 'o', ' ', 'e', 'i', 'u', 's', 'm', 'o', 'd', ' ', 't', 'e', 'm', 'p', 'o', 'r', ' ', 'i', 
//     'n', 'c', 'i', 'd', 'i', 'd', 'u', 'n', 't', ' ', 'u', 't', ' ', 'l', 'a', 'b', 'o', 'r', 'e', '.'
// ];
    // ['']
  // ['H', 'e', 'l', 'l','o', ','
  // , ' ', 'R', 'u', 's', 't'];


impl<const N: usize> Genome for Word<N>{
    type Score = usize;

    fn fit(&self) -> Self::Score {
        let mut count_equals = 0;

        for (ca, cs) 
        in ANSWER.iter().zip(self) {
            if ca == cs {
                count_equals += 1;
            }
        }

        count_equals
    }

    fn new() -> Self {
        let mut rng = rand::rng();
        std::iter::repeat_with(|| rng.random::<u8>())
            .take(N)
            .collect::<Vec<_>>()
            .try_into()
            .unwrap()
    }

    fn point(&mut self) {
        let mut rng = rand::rng();
        let i = rng.random_range(0..N) as usize;
        self[i] = rng.random::<u8>();
    }

    fn cross(&mut self, other: &mut Self) {
        let mut rng = rand::rng();

        // slow
        for i in 0..N { 
            if rng.random() {
                mem::swap(&mut self[i], &mut other[i]);
            }
        }
    }

    fn done(score: Self::Score) -> bool {
        score == ANSWER.len()
    }
}


