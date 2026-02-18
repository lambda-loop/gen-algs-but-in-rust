use crate::{examples::word::Word, genetic::simple_evol::Nature};


mod genetic;

mod examples;


fn main() {
    const w_size: usize  = examples::word::ANSWER.len();
    let mut nature: Nature<10_000,100,10_000, Word<w_size>> = Nature::new();

    let out = nature.run();
    let text: String = out.iter().map(|b| *b as char).collect();

    dbg!(nature.population[0]);
    
    // dbg!(out);
    dbg!(text);
    dbg!(nature.current_gen);
    dbg!(nature.status);

}
