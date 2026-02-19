
use std::cmp::Ordering;
use approx::relative_eq;


pub struct OrderedFloat(pub f64);

impl PartialEq for OrderedFloat {
    fn eq(&self, other: &Self) -> bool {
        if self.0.is_nan() && other.0.is_nan() {
            return false;
        }
        if self.0.is_nan() || other.0.is_nan() {
            return false;
        }
        relative_eq!(self.0, other.0, epsilon = 1e-12)
    }
}

impl Eq for OrderedFloat {} 

impl PartialOrd for OrderedFloat {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other)) 
    } 
}

impl Ord for OrderedFloat {
    fn cmp(&self, other: &Self) -> Ordering {
        let a = self.0;
        let b = other.0;

        if a.is_nan() && b.is_nan() 
            { Ordering::Equal } 
        else if a.is_nan() 
            { Ordering::Greater }
        else if b.is_nan() 
            { Ordering::Less } 
        else if self == other 
            { Ordering::Equal }
        else { a.partial_cmp(&b).unwrap() } 
    }
}
