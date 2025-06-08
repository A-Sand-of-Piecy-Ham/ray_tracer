use core::f32;
// use std::rc::Rc;


// const INFINTIY: f32 = f32::INFINITY;
const PI: f32 = std::f32::consts::PI;

#[inline]
pub fn degrees_to_radians(degrees: f32) -> f32 {
    degrees * PI / 180.0
}


// pub use crate::core::color;
// pub use super::ray;
// pub use super::vec3;