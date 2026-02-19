// use num_bigint::{BigInt, BigUint};

use std::{collections::HashMap, sync::LazyLock};

use approx::relative_eq;
use rand::{distr::Uniform, prelude::*};
use crate::genetic::{genome::Genome, mutation::{slice::{single_point_change, zip, zip_alt}, vec::{single_point_delete, single_point_insert, single_point_insert_with_replaces}}};
use num_bigint::BigInt;
use num_traits::ToPrimitive;
use data::*;

pub mod data;
pub mod score;
use score::*;

// x² + 3x - 10
const ANSWER: LazyLock<Polynomial> = LazyLock::new(|| {
    Polynomial(vec![
        Op::Inp, Op::Inp, Op::Mul,           // x²
        Op::Lit(3), Op::Inp, Op::Mul, // 3x
        Op::Add,  Op::Lit(10), Op::Sub, // (_ + _ - 10
    ])
});

const N_INPUTS: usize = 1_000;
const INPUTS: LazyLock<[i32; N_INPUTS]> = LazyLock::new(|| {
    let mut rng = rand::rng();
    let i =-10_000;
    let j = 10_000;

    let dist = Uniform::new_inclusive(i, j);

    (0..N_INPUTS).map(|_| dist.unwrap().sample(&mut rng))
        .collect::<Vec<_>>()
        .try_into()
        .unwrap()
});

const ORACLE: LazyLock<HashMap<i32, BigInt>> = LazyLock::new(|| {
    let mut table = HashMap::new();
    for i in *INPUTS {
        let score_i = Polynomial::eval(&ANSWER, i);
        table.insert(i, score_i);
    }

    table
});

impl Genome for Polynomial {
    type Score = OrderedFloat;

    fn fit(&self) -> Self::Score {
        let mut diffs = 0.;
        for i in *INPUTS {
            let delta = ORACLE.get(&i).unwrap() - self.eval(i);
            let delta_p = delta.pow(2);
            let delta_f = delta_p.to_f64().unwrap_or(f64::NAN);
            diffs += delta_f;
        }

        const N: f64 = (N_INPUTS - 1) as f64;
        let s = diffs/N;
        OrderedFloat(s.sqrt())
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
        relative_eq!(score.0, 0., epsilon = 1e-12)
        // score.0 == 0.
    }

}


