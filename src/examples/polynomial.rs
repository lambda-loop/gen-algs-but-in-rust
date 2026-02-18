
use num_bigint::{BigInt, BigUint};

use crate::genetic::genome::Genome;

type Num = BigInt; //i128;
enum Op {
    Inp,
    Lit(Num),
    Add,
    Sub,
    Mul,
    // Pow,
}

pub struct Polynomial(Vec<Op>);

impl Polynomial {
    pub fn eval(&self, input: Num) -> Num {
        let mut stack = Vec::new();
        for op in &self.0 {
            match op {
                Op::Inp    => stack.push(input.clone()),
                Op::Lit(n) => stack.push(n.clone()),

                Op::Add => 
                match stack.len() {
                    0 => stack.push(BigInt::from(0)),
                    1 => (),
                    _ => { 
                        let op0 = stack.pop().unwrap(); 
                        let op1 = stack.pop().unwrap();
                        stack.push(op1 + op0);
                    }
                },

                Op::Sub =>
                match stack.len() {
                    0 => stack.push(BigInt::from(0)),
                    1 => {
                        let op0 = stack.pop().unwrap(); 
                        stack.push(-op0);
                    },
                    _ => { 
                        let op0 = stack.pop().unwrap(); 
                        let op1 = stack.pop().unwrap();
                        stack.push(op1 - op0);
                    }
                },

                Op::Mul => 
                match stack.len() {
                    0 => stack.push(BigInt::from(1)),
                    1 => (),
                    _ => { 
                        let op0 = stack.pop().unwrap(); 
                        let op1 = stack.pop().unwrap();
                        stack.push(op1 * op0);
                    }
                },
            }
        }

        if let Some(value) = stack.first() {
            value.clone()
        } else { BigInt::from(0)  }

    }

    fn compile (&self) -> fn(input: BigInt) -> BigInt {
        todo!()

    }
}


impl Genome for Polynomial {
    type Score = f64;

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

