// use num_bigint::{BigInt, BigUint};

use std::{cmp::Reverse, collections::HashMap, sync::{LazyLock, OnceLock}};

use approx::relative_eq;
use rand::{distr::Uniform, prelude::*};
use crate::genetic::{genome::Genome, mutation::{slice::{single_point_change, zip, zip_alt}, vec::{single_point_delete, single_point_insert, single_point_insert_with_replaces}}};
use num_bigint::BigInt;
use num_traits::ToPrimitive;
use data::*;

pub mod data;
pub mod score;
use score::*;

// // x² + 3x - 10
// const ANSWER: LazyLock<Polynomial> = LazyLock::new(|| {
//     Polynomial(vec![
//         Op::Inp, Op::Inp, Op::Mul,           // x²
//         Op::Lit(3), Op::Inp, Op::Mul, // 3x
//         Op::Add,  Op::Lit(10), Op::Sub, // (_ + _ - 10
//     ]) 
// });

// 7x^4 + 5x^3 − 4x^2 − x
const ANSWER: LazyLock<Polynomial> = LazyLock::new(|| {
    Polynomial(vec![
        Op::Inp,
        Op::Lit(7),
        Op::Mul,
        Op::Lit(5),
        Op::Add,
        Op::Inp,
        Op::Mul,
        Op::Lit(4),
        Op::Sub,
        Op::Inp,
        Op::Mul,
        Op::Lit(1),
        Op::Sub,
        Op::Inp,
        Op::Mul,
    ])
});

static oracle: LazyLock<Oracle<N_INPUTS>> = LazyLock::new(|| {
    println!("initializing OnceLock");
    Oracle::new()
});

const N_INPUTS: usize = 1_000;
struct Oracle<const N_INPUTS: usize> {
    pub inputs: [i32; N_INPUTS],
    pub table: HashMap<i32, BigInt>,
}

impl<const N_INPUTS: usize> Oracle<N_INPUTS> {
    fn new() -> Self {
        // println!("initializing OnceLock");
        let inputs: [i32; N_INPUTS] = {
            dbg!("one time");
            let mut rng = rand::rng();
            let i =-10_000;
            let j = 10_000;

            let dist = Uniform::new_inclusive(i, j);

            (0..N_INPUTS).map(|_| dist.unwrap().sample(&mut rng))
                .collect::<Vec<_>>()
                .try_into()
                .unwrap()
        };

        let table: HashMap<i32, BigInt> = {
            let mut table = HashMap::new();
            for i in inputs {
                let score_i = Polynomial::eval(&ANSWER, i);
                table.insert(i, score_i);
            }
        
            table
        };

        Oracle {
            inputs,
            table
        }
    }
}

impl Genome for Polynomial {
    type Score = Reverse<OrderedFloat>;

    fn fit(&self) -> Self::Score {
        let mut diffs = 0.;
        for i in oracle.inputs {
            let oracle_val = oracle.table.get(&i).unwrap_or_else(|| {
                panic!("not found key: {} oracle size: {}, unique inputs: {}", 
                i, oracle.table.len(), oracle.inputs.iter().collect::<std::collections::HashSet<_>>().len());
});
            let delta = oracle.table.get(&i).unwrap() - self.eval(i);

            let delta_p = delta.pow(2);
            let delta_f = delta_p.to_f64().unwrap_or(f64::NAN);
            diffs += delta_f;
        }

        const N: f64 = (N_INPUTS - 1) as f64;
        let s = diffs/N;
        if s.is_nan() { return Reverse(OrderedFloat(f64::INFINITY)) }
        // dbg!(s);
        // dbg!(s.sqrt());
        Reverse(OrderedFloat(s.sqrt()))
    }

    fn new() -> Self {
        let mut rng = rand::rng();
        let answer_len = ANSWER.0.len();
        let n = rng.random_range(answer_len/2..answer_len*2);

        let mut codex = Vec::with_capacity(n);
        for _ in 0..n {
            codex.push(Op::rnew());
        }

        Polynomial(codex)
    }

    // slow..
    fn point(&mut self) {
        // let mut rng = rand::rng();
        match rand::random_range(0..4) {
            0 => single_point_change(&mut self.0, Op::rnew),
            // TODO: paia. fix it? Just move..?
            1 => self.0 = single_point_insert(self.0.clone(), Op::rnew, None),
            2 => self.0 = single_point_insert_with_replaces(self.0.clone(), Op::rnew, None),
            _ => self.0 = single_point_delete(self.0.clone()),

        }
    }

    fn cross(&mut self, other: &mut Self) {
        match rand::random_range(0..2) {
            0 => zip(&mut self.0, &mut other.0),
            _ => zip_alt(&mut self.0, &mut other.0),
        }
    }

    fn done(score: Self::Score) -> bool {
        score == Reverse(OrderedFloat(0.))
        // relative_eq!(score.0, 0., epsilon = 1e-12)
        // // score.0 == 0.
    }

}


