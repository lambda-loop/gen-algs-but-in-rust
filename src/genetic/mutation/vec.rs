
use rand::prelude::*;

// default very simple mutation for vec based genomes.

pub fn single_point_insert<A>
    (mut v: Vec<A>, point_gen: fn() -> A
    ,ranger: Option<fn() -> usize>) 
    -> Vec<A> {
    let mut rng = rand::rng();

    let v_len = v.len();
    let i = rng.random_range(0..=v_len);

    let r = if let Some(f) = ranger { f() } else { 
        let diff = v_len - i;
        rng.random_range(diff/2+1..=diff*2+1) 
    };

    let mut vi = Vec::with_capacity(r);
    for _ in 0..r {
        vi.push(point_gen());
    }

    v.splice(i..i, vi);
    v
}

pub fn single_point_insert_with_replaces<A>
    (mut v: Vec<A>, point_gen: fn() -> A
    ,ranger: Option<fn() -> usize>) 
    -> Vec<A> {
    let mut rng = rand::rng();

    let v_len = v.len();
    let i = rng.random_range(0..=v_len);
    let j = rng.random_range(i..=v_len);

    let r = if let Some(f) = ranger { f() } else { 
        let diff = v_len - i;
        rng.random_range(diff/2+1..=diff*2+1) 
    };

    let mut vi = Vec::with_capacity(r);
    for _ in 0..r {
        vi.push(point_gen());
    }

    v.splice(i..j, vi);
    v
}

pub fn single_point_delete<A>
    (mut v: Vec<A>) 
    -> Vec<A> {
    let mut rng = rand::rng();

    let v_len = v.len();
    let i = rng.random_range(0..=v_len);
    let j = rng.random_range(i..=v_len);

    v.drain(i..j);
    v
}

