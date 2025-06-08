
use std::sync::Arc;

use super::Interval;
use super::Point3;
use super::Vec3;
use super::Ray;

use super::material::Material;

// #[derive(Debug, Clone, Copy, Default)]
#[derive(Debug, Clone)]
/// t is the coefficient for the ray's direction vector, added with the origin, producing the hit point
pub struct HitRecord {
    pub point: Point3,
    pub normal: Vec3,
    pub t: f32,
    pub front_face: bool,
    pub material: Arc<Material>,
}

impl HitRecord {
    /// Returns front_face bool and corresponding normal vector
    /// *Note:* the parameter `outward_normal` is assumed to have unit length
    pub fn get_face_normal(ray: &Ray, outward_normal: &Vec3) -> (bool, Vec3) {
        let front_face = ray.direction.dot(outward_normal) < 0.0;
        let normal = match front_face {
            true => *outward_normal, 
            false => -*outward_normal
        };
        (front_face, normal)
    }

    // pub fn new() -> Self {
    //     todo!()
    //     // Self::default()
    // }
}

pub trait Hittable: Sync + Send {
    // fn hit<fT: From<f32> + From<f64> + ops::Mul<fT> + ops::Div<fT>>(&self, ray: &Ray, ray_tmin: fT, ray_tmax: fT, rec: &mut HitRecord<fT>) -> bool;
    fn hit(&self, ray: &Ray, ray_bounds: Interval) -> Option<HitRecord>;
}