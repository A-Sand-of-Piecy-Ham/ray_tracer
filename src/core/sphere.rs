
use std::ops;
use std::rc::Rc;

use super::hittable::Hittable;
use super::ray::Ray;
use super::vec3::Vec3;
use super::{material, Point3};
use super::hittable::HitRecord;
use super::Interval;
use super::material::Material;

type fPE = f32;

pub struct Sphere {
    pub center: Point3,
    pub radius: fPE,
    pub material: Rc<Material>,
    _lock: ()
}

impl Sphere {
    pub fn new(center: Vec3, radius: f32, material: Rc<Material>) -> Self {
        Self{center: center, radius: radius.max(0.0), material, _lock:()}
    }
}

impl Hittable for Sphere { 
    // fn hit<fT: From<f32> + From<f64> + ops::Mul<fT> + ops::Div<fT>>(&self, ray: &Ray, ray_tmin: fT, ray_tmax: fT, rec: &mut HitRecord<fT>) -> bool {
    fn hit(&self, ray: &Ray, ray_bounds: Interval) -> Option<HitRecord> {

        let oc: Vec3 = self.center - ray.origin;
        let a: f32 = ray.direction.length_squared();
        let h: f32 = ray.direction.dot(&oc);
        // let b = -2.0 * dot(&ray.direction, &oc);
        let c: f32 = oc.length_squared() - (self.radius*self.radius);

        let discriminant: f32 = h*h - a*c;
        if discriminant < 0.0 {return None}
        
        let sqrtd: f32 = discriminant.sqrt();
        
        let mut root: f32 = (h - sqrtd) / a;
        if !ray_bounds.surrounds(root) {
            root = (h + sqrtd) / a;
            if !ray_bounds.surrounds(root) {
                return None;
            }
        }

        let t = root;
        let point = ray.at(t);

        let outward_normal = (point - self.center) / self.radius;
        let (front_face, normal) = HitRecord::get_face_normal(ray, &outward_normal);

        // FIX: Bad performance?
        let material = self.material.clone();

        return Some(HitRecord {
            point,
            normal,
            t,
            front_face,
            material
        });
    }
}