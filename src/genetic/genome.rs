

pub trait Genome {
    type Score: Ord;

    fn fit(&self) -> Self::Score; 

    fn new() -> Self;
    fn point(&mut self);
    fn cross(&mut self, other: &mut Self); 

    fn done(score: Self::Score) -> bool;

}

#[derive(PartialEq, Eq)]
pub struct Fen<A> where A: Genome {
    pub codex: A,
    pub score: A::Score,
}

impl<A: Genome> PartialOrd for Fen<A> where A: Eq {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.score.partial_cmp(&other.score)
    }
}

impl<A: Genome> Ord for Fen<A> where A: Eq {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.score.cmp(&other.score)
        
    }
}



