
use std::{sync::Arc};

use super::{hittable::{HitRecord, Hittable}, Interval, Ray};

// #[derive(Clone)]
pub struct HittableList {
    objects: Vec<Arc<dyn Hittable>>,
}

impl HittableList {
    /// Behaves similarly to `std::Vec::from`
     pub fn new() -> Self {
        Self{objects: Vec::new()}
    }
    #[allow(dead_code)]
     pub fn from(object: Arc<dyn Hittable>) -> Self {
        Self{objects: vec![object]}
    }
    #[allow(dead_code)]
    pub fn clear(&mut self) {
        self.objects.clear();
    }
    pub fn add(&mut self, object: Arc<dyn Hittable>) {
        self.objects.push(object);
    }
    pub fn hit(&self, ray: Ray, ray_bounds: Interval) -> Option<HitRecord> {
        let mut rec: Option<HitRecord> = None; // = HitRecord::default();
        

        self.objects.iter().fold(ray_bounds.max, |closest_so_far, object| {
            if let Some(hit_rec) = object.hit(ray, Interval::new(ray_bounds.min, closest_so_far)) {
                // FIX: BAD!! POSSIBLE PERFORMANCE COST
                let t = hit_rec.t;
                rec = Some(hit_rec);
                return t;   
            }
            closest_so_far
        });

        return rec;
    }
}