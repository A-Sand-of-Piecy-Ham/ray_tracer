use std::cell::RefCell;
use std::f32::INFINITY;
use std::io::{BufWriter, Write};

use indicatif::ProgressBar;
use rand::rngs::SmallRng;
use rand::Rng;

use crate::core::material::ScatterContext;

use super::hittable::HitRecord;
use super::material::Material;
use super::{hittable_list::HittableList, Vec3};

use super::{write_color, Interval, Ray, Vec3 as Point3};

use super::Color;
use super::unit_vector;

// use rand::prelude::*;
// use super::random;



#[derive(Debug, Clone)]
pub enum AntiAliasing {
    None,
    RandomSamples(u16, RefCell<SmallRng>)
}

impl AntiAliasing {
    pub fn sample(&self, i: usize, j: usize, camera: &Camera, world: &HittableList) -> Color {

        match self {
            Self::None => {
                
                let pixel_center = camera.pixel00_loc + (camera.pixel_delta_u * i as f32) + (camera.pixel_delta_v * j as f32);

                let ray_direction = pixel_center - camera.center;


                let ray = Ray{origin: camera.center, direction: ray_direction};

                let pixel_color = Camera::ray_color(&ray, camera.max_depth, &world);
                

                pixel_color
            }
            Self::RandomSamples(num_samples, rng) => {
                let rng = &mut rng.borrow_mut();

                let mut pixel_color = Color(0., 0., 0.);

                for _ in 0..*num_samples {
                    let ray = camera.get_ray_rand(i, j, rng);
                    pixel_color += Camera::ray_color(&ray, camera.max_depth, world);
                }


                pixel_color / *num_samples as f32
            }

        }
    }

}


pub struct CameraBuilder {
    aspect_ratio: f64,
    image_width: usize,
    anti_aliasing: AntiAliasing,
    max_depth: u32,
}

impl CameraBuilder {
    pub fn build(self) -> Camera {
        let aspect_ratio = self.aspect_ratio;
        let image_width = self.image_width;
        let image_height: usize = match (image_width as f64 / aspect_ratio) as usize {
            v if v >= 1 => v,
            _ => 1
        };
        let center = Point3(0., 0., 0.);

        let focal_length = 1.0;
        let viewport_height: f32 = 2.0;
        let viewport_width: f32 = viewport_height * (self.image_width as f32 / image_height as f32);

        // Calculate vectors across horizontal and down the vertial viewport edges
        let viewport_u: Vec3 = Vec3(viewport_width, 0., 0.);
        let viewport_v: Vec3 = Vec3(0., -viewport_height, 0.);
 
        // Calculate the horizontal and vertical delta vectors from pixel to pixel
        let pixel_delta_u: Vec3 = viewport_u.div_scalar(self.image_width as f32);
        let pixel_delta_v: Vec3 = viewport_v.div_scalar(image_height as f32);
        
        // Calculate the location of the upper left pixel
        let viewport_upper_left = center - Vec3(0.,0., focal_length as f32) - viewport_u/2. - viewport_v/2.;
        let pixel00_loc = viewport_upper_left + 0.5 * (pixel_delta_u + pixel_delta_v);

        // Error: Cannot move out of builder
        let anti_aliasing = self.anti_aliasing;
        let max_depth = self.max_depth;

        Camera{
            aspect_ratio,
            image_width,
            image_height,
            center,
            pixel00_loc,
            pixel_delta_u,
            pixel_delta_v,
            anti_aliasing,
            max_depth
        }
    }
}

pub struct Camera {
    // pub position:
    // pub direction: 
    #[allow(unused)]
    pub aspect_ratio: f64, // = 16.0 / 9.0;
    pub image_width: usize, // = 400;
    pub anti_aliasing: AntiAliasing,
    image_height: usize,
    center: Point3,
    pixel00_loc: Point3,
    pixel_delta_u: Vec3,
    pixel_delta_v: Vec3,
    max_depth: u32,
}

impl Camera {
    pub fn new_builder(aspect_ratio: f64, image_width: usize, max_depth: u32, anti_aliasing: AntiAliasing) -> CameraBuilder {
        CameraBuilder{aspect_ratio, image_width, anti_aliasing, max_depth}
    }

    pub fn render(&self, world: &HittableList) {
        let num_samples = match self.anti_aliasing {
            AntiAliasing::None => 1,
            AntiAliasing::RandomSamples(num, _) => num 
        };
        let bar = ProgressBar::new((self.image_height*self.image_width) as u64);

        let mut out = BufWriter::new(std::io::stdout());

        // println!("P3\n{self.image_width} {self.image_height}\n255\n");
        writeln!(&mut out, "P3\n{} {}\n255\n", self.image_width, self.image_height).unwrap();

        for j in 0..self.image_height {
            bar.inc(self.image_width as u64);
            for i in 0..self.image_width {

                let pixel_color = self.anti_aliasing.sample(i, j, self, world);

                write_color(&mut out, &pixel_color).unwrap();

            }
        }
        bar.finish();
    }


    fn initialize() {

        todo!()
    }

    fn get_ray(&self, i: usize, j: usize) -> Ray {
        todo!()
    }

    fn get_ray_rand(&self, i: usize, j: usize, rng:  &mut SmallRng) -> Ray {

        let offset = Self::sample_square_rand(rng);
        let pixel_sample = self.pixel00_loc 
            + ((i as f32 + offset.x()) * self.pixel_delta_u)
            + ((j as f32 + offset.y()) * self.pixel_delta_v);

        let ray_origin = self.center;
        let ray_direction = pixel_sample - ray_origin;
        
        Ray{origin: ray_origin, direction: ray_direction}
    }

    /// Returns vector to random point in the [-.5, -.5]-[+.5,+.5] unit square
    fn sample_square_rand(rng: &mut SmallRng) -> Vec3 {
        Vec3(rng.random::<f32>() - 0.5, rng.random::<f32>() - 0.5, 0.0)
    }

    fn ray_color(ray: &Ray, depth: u32, world: &HittableList) -> Color {
        if depth <= 0 {return Color(0.,0.,0.)}
        
        // Low bound to fix shadow acne
        if let Some(rec) = world.hit(ray, Interval::new(0.001, INFINITY)) {
            if let Material::Debug(reflectance) = rec.material.as_ref() {
                
                return *reflectance * (rec.normal + Color(1., 1., 1.));
            }

            if let Some(ScatterContext{scattered, attenuation}) = rec.material.scatter(ray, &rec) {
                return attenuation * Self::ray_color(&scattered, depth-1, world);
            }
            return Color(0.,0.,0.);
        
        }

        
        // Color the sky
        let unit_direction: Vec3 = unit_vector(ray.direction);
        let a = 0.5 * (unit_direction.y() + 1.0);
        Color(1.0, 1.0, 1.0) * (1.0 - a) + a*Color(0.5, 0.7, 1.0)
    }
}