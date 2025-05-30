
use std::{mem::ManuallyDrop, rc::Rc};

use super::{hittable::{HitRecord, Hittable}, material::Material, Interval, Ray};

pub struct HittableList {
    objects: Vec<Rc<dyn Hittable>>,
}

impl HittableList {
    /// Behaves similarly to `std::Vec::from`
     pub fn new() -> Self {
        Self{objects: Vec::new()}
    }
     pub fn from(object: Rc<dyn Hittable>) -> Self {
        Self{objects: vec![object]}
    }
    pub fn clear(&mut self) {
        self.objects.clear();
    }
    pub fn add(&mut self, object: Rc<dyn Hittable>) {
        self.objects.push(object);
    }
    pub fn hit(&self, ray: &Ray, ray_bounds: Interval, rec: &mut HitRecord) -> bool {
        let mut temp_rec: HitRecord = HitRecord::default();
        
        let mut hit_anything = false;
        let mut material: Option<Material> = None;

        self.objects.iter().fold(ray_bounds.max, |closest_so_far, object| {
            if object.hit(ray, Interval::new(ray_bounds.min, closest_so_far), &mut temp_rec) {
                hit_anything = true;
                // FIX: BAD!! POSSIBLE PERFORMANCE COST
                *rec = temp_rec.clone();
                return temp_rec.t;   
            }
            closest_so_far
        });


        return hit_anything
    }
}