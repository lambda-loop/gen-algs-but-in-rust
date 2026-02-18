
use rand::prelude::*;

pub fn single_point_change<A>
    (v: &mut [A], point_gen: fn() -> A) {
    if let Some(a) = v.choose_mut(&mut rand::rng()) {
        *a = point_gen();
    }
}

pub fn zip<A>
    (vl: &mut [A], vr: &mut [A]) {
    let min_len = std::cmp::min(vl.len(), vr.len());

    let mut rng = rand::rng();
    for i in 0..min_len {
        if rng.random() {
            std::mem::swap(&mut vl[i], &mut vr[i]);
        }
    }

}

pub fn zip_alt<A>
    (vl: &mut [A], vr: &mut [A]) {
    let min_len = std::cmp::min(vl.len(), vr.len());

    for i in (0..min_len).step_by(2) {
        std::mem::swap(&mut vl[i], &mut vr[i]);
    }
}
