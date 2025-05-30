
use std::ops;
use std::rc::Rc;

use super::interval;
use super::Interval;
use super::Point3;
use super::Vec3;
use super::Ray;

use super::material::Material;

// #[derive(Debug, Clone, Copy, Default)]
#[derive(Debug, Clone, Default)]
/// t is the coefficient for the ray's direction vector, added with the origin, producing the hit point
pub struct HitRecord {
    pub point: Point3,
    pub normal: Vec3,
    pub t: f32,
    pub front_face: bool,
    pub material: Rc<Material>,
}

impl HitRecord {
    /// Sets the hit record normal vector.
    /// *Note:* the parameter `outward_normal` is assumed to have unit length
    pub fn set_face_normal(&mut self, ray: &Ray, outward_normal: &Vec3) {
        self.front_face = ray.direction.dot(outward_normal) < 0.0;
        self.normal = match self.front_face {
            true => *outward_normal, 
            false => -*outward_normal
        };
    }
    pub fn new() -> Self {
        Self::default()
    }
}

pub trait Hittable {
    // fn hit<fT: From<f32> + From<f64> + ops::Mul<fT> + ops::Div<fT>>(&self, ray: &Ray, ray_tmin: fT, ray_tmax: fT, rec: &mut HitRecord<fT>) -> bool;
    fn hit(&self, ray: &Ray, ray_bounds: Interval, rec: &mut HitRecord) -> bool;
}