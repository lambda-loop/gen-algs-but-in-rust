
pub use num_bigint::{BigInt, BigUint};

pub type Num = i32; //BigInt; //i128;

#[derive(Clone, Copy, Eq, PartialEq, Debug)]
pub enum Op {
    Inp,
    Lit(i32),
    Add,
    Sub,
    Mul,
}

const NUM_LB: i32 =-100;
const NUM_UB: i32 = 100;

impl Op {
    pub fn rnew() -> Self {
        match rand::random_range(0..5) {
            0 => Op::Inp, // single_point_change(self, Self::new),
            1 => {
                let n = rand::random_range(
                    NUM_LB..NUM_UB
                );

                Op::Lit(n)
            } 

            2 => Op::Add,
            3 => Op::Sub,
            _ => Op::Mul,

        }
    }
}

#[derive(PartialEq, Eq, Debug, Clone)]
pub struct Polynomial(pub Vec<Op>);
impl Polynomial {
    pub fn eval(&self, input: i32) -> BigInt {
        if self.0.is_empty() { return 0.into() }

        let mut stack: Vec<BigInt> = Vec::new();
        for op in &self.0 {
            match op {
                Op::Inp    => stack.push(input.into()),
                Op::Lit(n) => stack.push(BigInt::from(*n)),

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

        stack.remove(0)
    }

    fn compile (&self) -> fn(input: BigInt) -> BigInt {
        todo!()

    }
}
