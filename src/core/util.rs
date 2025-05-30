use core::f32;
use std::rc::Rc;


const infinity: f32 = f32::INFINITY;
const pi: f32 = std::f32::consts::PI;

#[inline]
pub fn degrees_to_radians(degrees: f32) -> f32 {
    degrees * pi / 180.0
}


// pub use crate::core::color;
// pub use super::ray;
// pub use super::vec3;