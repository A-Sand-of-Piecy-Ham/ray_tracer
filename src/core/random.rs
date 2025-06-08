#![allow(unused)]
use std::{ops::Range};

use rand::Rng;

pub fn random_float() -> f32 {
    rand::rng().random()
    
}
pub fn random_float_in_range(range: Range<f32>) -> f32 {
    range.start + (range.end - range.start) * rand::rng().random::<f32>()
}

