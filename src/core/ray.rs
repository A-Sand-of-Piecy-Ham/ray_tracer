
use std::f32::INFINITY;

use super::hittable::{HitRecord, Hittable};
use super::hittable_list::HittableList;
use super::{dot, unit_vector, Interval, Vec3};
use super::Color;

type Point3 = Vec3;

pub struct Ray {
    pub origin: Point3, 
    pub direction: Vec3,
}


impl Ray {
    pub fn new(origin: Point3, direction: Vec3) -> Self {
        Self{origin, direction}
    }
    /// Follows ray down `dist` from origin and returns position
    pub fn at(&self, dist: f32) -> Point3 {
        self.origin + dist*self.direction
    }

}
