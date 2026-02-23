use crate::{examples::word::Word};


mod genetic;

mod examples;
use examples::polynomial::{self, data::Polynomial};
use genetic::strategy::classic::Nature;


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

