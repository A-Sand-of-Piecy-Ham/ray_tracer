use std::fs::File;
// use f32::INFINITY;
use std::io::{BufWriter, Write};
use std::iter::zip;

use image::{ImageEncoder, ImageResult};
use indicatif::ProgressBar;
use rand::Rng;
use rayon::iter::{IndexedParallelIterator, IntoParallelIterator, ParallelIterator};
use image::codecs::png::PngEncoder;
use rayon::slice::{ParallelSlice, ParallelSliceMut};

use crate::core::color::{F32ColorToU8, GammaCorrection};
use crate::core::material::ScatterContext;
use crate::core::util::degrees_to_radians;
use crate::core::Scene;

// use super::hittable::HitRecord;
use super::material::Material;
use super::{hittable_list::HittableList, Vec3};

use super::{write_color, Interval, Ray, Vec3 as Point3};

use super::Color;
use super::unit_vector;

// use rand::prelude::*;
// use super::random;



#[derive(Debug, Default, Clone)]
pub enum AntiAliasing {
    #[default]
    None,
    RandomSamples(u16)
}

impl AntiAliasing {
    pub fn sample_scene(&self, i: usize, j: usize, camera: &Camera, world: &Scene) -> Color {

        match self {
            Self::None => {
                
                let pixel_center = camera.pixel00_loc + (camera.pixel_delta_u * i as f32) + (camera.pixel_delta_v * j as f32);

                let ray_direction = pixel_center - camera.center;


                let ray = Ray{origin: camera.center, direction: ray_direction};

                let pixel_color = Camera::scene_ray_color(ray, camera.max_depth, &world);
                

                pixel_color
            }
            Self::RandomSamples(num_samples) => {

                let mut pixel_color = Color(0., 0., 0.);

                for _ in 0..*num_samples {
                    let ray = camera.get_ray_rand(i, j);
                    pixel_color += Camera::scene_ray_color(ray, camera.max_depth, world);
                }


                pixel_color / *num_samples as f32
            }

        }
    }

    pub fn sample(&self, i: usize, j: usize, camera: &Camera, world: &HittableList) -> Color {

        match self {
            Self::None => {
                
                let pixel_center = camera.pixel00_loc + (camera.pixel_delta_u * i as f32) + (camera.pixel_delta_v * j as f32);

                let ray_direction = pixel_center - camera.center;


                let ray = Ray{origin: camera.center, direction: ray_direction};

                let pixel_color = Camera::ray_color(ray, camera.max_depth, &world);
                

                pixel_color
            }
            Self::RandomSamples(num_samples) => {

                let mut pixel_color = Color(0., 0., 0.);

                for _ in 0..*num_samples {
                    let ray = camera.get_ray_rand(i, j);
                    pixel_color += Camera::ray_color(ray, camera.max_depth, world);
                }


                pixel_color / *num_samples as f32
            }

        }
    }

}

#[derive(Debug, Default, Clone)]
pub struct CameraBuilder {
    pub aspect_ratio: f64,
    pub image_width: usize,
    pub anti_aliasing: AntiAliasing,
    pub max_depth: u32,

    pub vfov: f32,
    pub lookfrom: Point3,
    pub lookat: Point3,
    pub vup: Vec3, // Camera-relative up direction
}

// impl Default for CameraBuilder {

// }

impl CameraBuilder {
    pub fn build(self) -> Camera {
        let aspect_ratio = self.aspect_ratio;
        let image_width = self.image_width;
        let image_height: usize = match (image_width as f64 / aspect_ratio) as usize {
            x if x >= 1 => x,
            _ => 1
        };
        
        let center = self.lookfrom;

        // Determine viewport dimensions
        let focal_length = (self.lookfrom - self.lookat).length();
        let theta = degrees_to_radians(self.vfov);
        let h = (theta/2.0).tan();
        let viewport_height: f32 = 2.0 * h * focal_length;
        let viewport_width: f32 = viewport_height * (self.image_width as f32 / image_height as f32);

        let w = unit_vector(self.lookfrom - self.lookat);
        let u = unit_vector(Vec3::cross(self.vup, w));
        let v = Vec3::cross(w, u);
        
        
        // Calculate vectors across horizontal and down the vertial viewport edges
        let viewport_u: Vec3 = u * viewport_width;    //Vec3(viewport_width, 0., 0.);
        let viewport_v: Vec3 = -v * viewport_height;  //Vec3(0., -viewport_height, 0.);
 
        // Calculate the horizontal and vertical delta vectors from pixel to pixel
        let pixel_delta_u: Vec3 = viewport_u.div_scalar(self.image_width as f32);
        let pixel_delta_v: Vec3 = viewport_v.div_scalar(image_height as f32);
        
        // Calculate the location of the upper left pixel
        let viewport_upper_left = center - focal_length * w - viewport_u/2. - viewport_v/2.;
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
            max_depth,
            // vfov: self.vfov,
            // basis: Vec3Basis { u, v, w },
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
    // /// Vertial view angle
    // vfov: f32,
    // lookfrom: Point3,
    // lookat: Point3,

    // basis: Vec3Basis,
}

#[allow(unused)]
#[derive(Debug, Clone, Copy)]
struct Vec3Basis {
    u: Vec3,
    v: Vec3,
    w: Vec3,
}

impl Camera {
    #[allow(unused)]
    pub fn new_builder(aspect_ratio: f64, image_width: usize, max_depth: u32, anti_aliasing: AntiAliasing, vfov: f32, lookat: Point3, lookfrom: Point3, vup: Vec3) -> CameraBuilder {
        CameraBuilder{aspect_ratio, image_width, anti_aliasing, max_depth, vfov, lookfrom, lookat, vup}
    }
    pub fn render_scene(&self, world: &Scene) -> Result<(), Box<dyn std::error::Error>> {
        // let num_samples = match self.anti_aliasing {
        //     AntiAliasing::None => 1,
        //     AntiAliasing::RandomSamples(num, _) => num 
        // };
        // let image_buffer = image::ImageBuffer::new();
        // let file = 

        let bar = ProgressBar::new((self.image_height*self.image_width) as u64);

        // let mut out = BufWriter::new(std::io::stdout());
        let mut img = File::create("img.png")?;
        // Prints nothing to std::out and panics after generating
        // let encoder = PngEncoder::new(&mut out);
        let encoder = PngEncoder::new(&mut img);

        let mut image_buffer: Vec<Color> = vec![Color::default(); self.image_height * self.image_width];

        // println!("P3\n{self.image_width} {self.image_height}\n255\n");
        // writeln!(&mut out, "P3\n{} {}\n255\n", self.image_width, self.image_height).unwrap();

        // (0..self.image_height).into_par_iter().zip(image_buffer.par_chunks_exact(self.image_width).into_par_iter()).for_each(|j, chunk| {
        (image_buffer.par_chunks_exact_mut(self.image_width)).enumerate().for_each(|(j, chunk)| {
            // let row_buf = vec![Color::default(); self.image_width];

            bar.inc(self.image_width as u64);
            for i in 0..self.image_width {

                let pixel_color = self.anti_aliasing.sample_scene(i, j, self, world);

                chunk[i] = pixel_color;
                // write_color(&mut out, &pixel_color).unwrap();

            }
            // let offset = self.image_height * j;
            // image_buffer[offset..(offset+self.image_width)].copy_from_slice(&row_buf); 
            // chunk.copy_from_slice(&row_buf); 
        });

        // let mut img = ImageWriter::new():
        self.write_color_encoded(encoder, image_buffer)?;
        bar.finish();
        Ok(())
    }

    pub fn render(&self, world: &HittableList) -> Result<(), Box<dyn std::error::Error>> {
        // let num_samples = match self.anti_aliasing {
        //     AntiAliasing::None => 1,
        //     AntiAliasing::RandomSamples(num, _) => num 
        // };
        // let image_buffer = image::ImageBuffer::new();
        // let file = 

        let bar = ProgressBar::new((self.image_height*self.image_width) as u64);

        // let mut out = BufWriter::new(std::io::stdout());
        let mut img = File::create("img.png")?;
        // Prints nothing to std::out and panics after generating
        // let encoder = PngEncoder::new(&mut out);
        let encoder = PngEncoder::new(&mut img);

        let mut image_buffer: Vec<Color> = vec![Color::default(); self.image_height * self.image_width];

        // println!("P3\n{self.image_width} {self.image_height}\n255\n");
        // writeln!(&mut out, "P3\n{} {}\n255\n", self.image_width, self.image_height).unwrap();

        // (0..self.image_height).into_par_iter().zip(image_buffer.par_chunks_exact(self.image_width).into_par_iter()).for_each(|j, chunk| {
        (image_buffer.par_chunks_exact_mut(self.image_width)).enumerate().for_each(|(j, chunk)| {
            // let row_buf = vec![Color::default(); self.image_width];

            bar.inc(self.image_width as u64);
            for i in 0..self.image_width {

                let pixel_color = self.anti_aliasing.sample(i, j, self, world);

                chunk[i] = pixel_color;
                // write_color(&mut out, &pixel_color).unwrap();

            }
            // let offset = self.image_height * j;
            // image_buffer[offset..(offset+self.image_width)].copy_from_slice(&row_buf); 
            // chunk.copy_from_slice(&row_buf); 
        });

        // let mut img = ImageWriter::new():
        self.write_color_encoded(encoder, image_buffer)?;
        bar.finish();
        Ok(())
    }

    pub fn write_color_encoded<E: ImageEncoder>(&self, encoder: E, pixel_buffer: Vec<Color>) -> ImageResult<()> {
        
        let bytes: Box<[u8]> = pixel_buffer.into_iter().flat_map(|x| {x.linear_to_gamma().f32_color_to_u8().into_iter()}).collect();
        encoder.write_image(&bytes, self.image_width as u32, self.image_height as u32, image::ExtendedColorType::Rgb8)?;
        
        Ok(())
    }

    // fn initialize() {
    //     todo!()
    // }

    // fn get_ray(&self, i: usize, j: usize) -> Ray {
    //     todo!()
    // }

    fn get_ray_rand(&self, i: usize, j: usize) -> Ray {

        let offset = Self::sample_square_rand();
        let pixel_sample = self.pixel00_loc 
            + ((i as f32 + offset.x()) * self.pixel_delta_u)
            + ((j as f32 + offset.y()) * self.pixel_delta_v);

        let ray_origin = self.center;
        let ray_direction = pixel_sample - ray_origin;
        
        Ray{origin: ray_origin, direction: ray_direction}
    }

    /// Returns vector to random point in the [-.5, -.5]-[+.5,+.5] unit square
    fn sample_square_rand() -> Vec3 {
        let mut rng = rand::rng();
        Vec3(rng.random::<f32>() - 0.5, rng.random::<f32>() - 0.5, 0.0)
    }

    fn scene_ray_color(ray: Ray, depth: u32, world: &Scene) -> Color {
        if depth == 0 {return Color(0.,0.,0.)}
        
        // Low bound to fix shadow acne
        if let Some(rec) = world.hit(ray, Interval::new(0.001, f32::INFINITY)) {
            if let Material::Debug(reflectance) = rec.material.as_ref() {
                
                return *reflectance * (rec.normal + Color(1., 1., 1.));
            }

            if let Some(ScatterContext{scattered, attenuation}) = rec.material.scatter(ray, &rec) {
                return attenuation * Self::scene_ray_color(scattered, depth-1, world);
            }
            return Color(0.,0.,0.);
        
        }

        
        // Color the sky
        let unit_direction: Vec3 = unit_vector(ray.direction);
        let a = 0.5 * (unit_direction.y() + 1.0);
        Color(1.0, 1.0, 1.0) * (1.0 - a) + a*Color(0.5, 0.7, 1.0)
    }
    fn ray_color(ray: Ray, depth: u32, world: &HittableList) -> Color {
        if depth == 0 {return Color(0.,0.,0.)}
        
        // Low bound to fix shadow acne
        if let Some(rec) = world.hit(ray, Interval::new(0.001, f32::INFINITY)) {
            if let Material::Debug(reflectance) = rec.material.as_ref() {
                
                return *reflectance * (rec.normal + Color(1., 1., 1.));
            }

            if let Some(ScatterContext{scattered, attenuation}) = rec.material.scatter(ray, &rec) {
                return attenuation * Self::ray_color(scattered, depth-1, world);
            }
            return Color(0.,0.,0.);
        
        }

        
        // Color the sky
        let unit_direction: Vec3 = unit_vector(ray.direction);
        let a = 0.5 * (unit_direction.y() + 1.0);
        Color(1.0, 1.0, 1.0) * (1.0 - a) + a*Color(0.5, 0.7, 1.0)
    }
}