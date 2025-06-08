use std::cell::RefCell;

use rand::{rngs::SmallRng, Rng};

use crate::core::hittable::HitRecord;
use crate::core::unit_vector;
use crate::core::Ray;

use super::Color;
use super::Vec3;

#[derive(Debug, Clone)]
pub enum Material {
    /// Colors based off shape normals
    Debug(Color),
    #[allow(dead_code)]
    RandomDiffuse(RefCell<SmallRng>, Color),
    LambertianDiffuseRandom {rng_cell: RefCell<SmallRng>, albedo: Color },
    #[allow(dead_code)]
    Metalic(Color),
    // Albedo, fuzziness, rng_cell
    MetalicFuzz(Color, f32, RefCell<SmallRng>),
    // Refraction index    
    // Refractive index in vacuum or air, or the ratio of the material's refractive index over
    // the refractive index of the enclosing media
    Dielectric(f32, RefCell<SmallRng>)
}

impl Default for Material {
    fn default() -> Self {
        Self::Debug(Color(1.0,1.0,1.0))
    }
}

#[derive(Debug, Clone, Copy)]
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
                return Some(ScatterContext{scattered, attenuation})
            }
            Self::Dielectric(refraction_index, rng_cell) => {
                let attenuation = Color(1.0, 1.0, 1.0);
                let ri = if rec.front_face  {1.0/refraction_index} else {*refraction_index};


                let unit_direction = unit_vector(ray_in.direction);
                let cos_theta = (-unit_direction).dot(&rec.normal).min(1.0); // This is recomputed in the refraction code, is that bad?
                let sin_theta = (1.0 - cos_theta*cos_theta).sqrt();

                let cannot_refract: bool = ri * sin_theta > 1.0;

                let random_float:f32 = {rng_cell.borrow_mut().random()};

                let direction = if cannot_refract || reflectance(cos_theta, ri) > random_float {
                        Vec3::reflect(&unit_direction, &rec.normal)                   
                    }
                    else {
                        Vec3::refract(&unit_direction, &rec.normal, ri)
                    };

                let scattered = Ray::new(rec.point, direction);

                return Some(ScatterContext { attenuation, scattered})
            }
        };
    }
}

/// Schlick's approximation for reflectance of dielectric
fn reflectance(cosine: f32, refraction_index: f32) -> f32 {
   let r0 = (1.0 - refraction_index) / (1.0 + refraction_index);
   let r1 = r0*r0;
   r1 + (1.0-r1) * (1.0 - cosine).powi(5)
}