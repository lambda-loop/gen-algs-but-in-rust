use crate::{examples::word::Word, genetic::simple_evol::Nature};


mod genetic;

mod examples;
use examples::polynomial::{self, data::Polynomial};


fn main() {
    // const w_size: usize  = examples::word::ANSWER.len();
    let mut nature: Nature<1_000,100,1_000, Polynomial> = Nature::new();

    let out = nature.run();
    // let text: String = out.iter().map(|b| *b as char).collect();

    dbg!(&nature.population[0]);
    let fen = genetic::genome::Genome::fit(&nature.population[0]);
    dbg!(fen);
    
    // dbg!(out);
    // dbg!(text);
    dbg!(nature.current_gen);
    dbg!(nature.status);

}
