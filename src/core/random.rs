#![allow(unused)]
use std::{ops::Range};

use rand::Rng;

pub fn random_double() -> f32 {
    rand::rng().random()
    
}
pub fn random_double_range(range: Range<f32>) -> f32 {
    range.start + (range.end - range.start) * rand::rng().random::<f32>()
}

