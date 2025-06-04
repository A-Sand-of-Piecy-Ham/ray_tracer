use std::cell::RefCell;

use rand::{rngs::SmallRng, Rng};

use crate::core::hittable::HitRecord;
use crate::core::Ray;

use super::Color;
use super::Vec3;

#[derive(Debug, Clone)]
pub enum Material {
    /// Colors based off shape normals
    Debug(Color),
    RandomDiffuse(RefCell<SmallRng>, Color),
    LambertianDiffuseRandom {rng_cell: RefCell<SmallRng>, albedo: Color },
    Metalic(Color),
    MetalicFuzz(Color, f32, RefCell<SmallRng>),
}

impl Default for Material {
    fn default() -> Self {
        Self::Debug(Color(1.0,1.0,1.0))
    }
}

pub struct ScatterContext {
    pub attenuation: Color,
    pub scattered: Ray
}

impl Material {
    pub fn scatter(&self, ray_in: &Ray, rec: &HitRecord) -> Option<ScatterContext> {
        match self {
            Self::Debug(_albedo) => unimplemented!(),
            
            Self::RandomDiffuse(rng_cell, albedo) => {
                let mut direction = {
                    let rng = &mut rng_cell.borrow_mut();
                    Vec3::random_on_hemisphere(&rec.normal, rng)
                };
                if direction.near_zero() {
                    direction = rec.normal;
                }

                let scattered = Ray::new(rec.point, direction);
                let attenuation = *albedo;
                return Some(ScatterContext{scattered, attenuation});
            },

            Self::LambertianDiffuseRandom {rng_cell, albedo} => {
                let mut scatter_direction = {
                    let rng = &mut rng_cell.borrow_mut();
                    rec.normal + Vec3::random_unit_vector(rng)
                };

                if scatter_direction.near_zero() {
                    scatter_direction = rec.normal;
                }

                let scattered = Ray::new(rec.point, scatter_direction);
                let attenuation = *albedo;
                return Some(ScatterContext{scattered, attenuation});
            }
            Self::Metalic(albedo) => {
                let reflected = ray_in.direction.reflect(&rec.normal);
                let scattered = Ray::new(rec.point, reflected);
                let attenuation = *albedo;
                return Some(ScatterContext{scattered, attenuation});
            }
            Self::MetalicFuzz(albedo, fuzz, rng_cell) => {
                if *fuzz > 1.0 {panic!("Fuzz value cannot exceed 1")}

                let reflected = {
                    let rng = &mut rng_cell.borrow_mut();
                    ray_in.direction.reflect(&rec.normal).normalize() 
                        + (*fuzz * Vec3::random_unit_vector(rng))
                };
                let scattered = Ray::new(rec.point, reflected);
                let attenuation = *albedo;
                return Some(ScatterContext{scattered, attenuation});
            }
        }
    }

}