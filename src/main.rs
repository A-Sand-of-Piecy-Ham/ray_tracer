//! Super cool liberary
//! 
//! 
//! 


mod node;
mod server;
mod renderer;
mod core;


use core::camera::{AntiAliasing, Camera};
use core::material::Material;
use core::Vec3;
use core::Vec3 as Point3;

use core::{Ray};
use core::Color;
use core::util::*;
use core::sphere::Sphere;
use core::hittable_list::HittableList;
use core::write_color;
use std::cell::RefCell;
use std::io::{BufWriter, Write};
use std::rc::Rc;

use indicatif::ProgressBar;
use rand::rngs::SmallRng;
use rand::SeedableRng;

use crate::core::camera::CameraBuilder;
// use node::*;


// Image

// const aspect_ratio: f64 = 16.0 / 9.0;
// // const aspect_ratio: f64 = 1.0;
// // const IMAGE_HEIGHT: u32 = 256;
// const IMAGE_WIDTH: usize = 400;
// const IMAGE_HEIGHT: usize = match (IMAGE_WIDTH as f64 / aspect_ratio) as usize {
//     v if v >= 1 => v,
//     _ => 1
// }; 


// Camera






fn main() {



    // World

    let mut world = HittableList::new();
    let rng = RefCell::new(SmallRng::from_rng(&mut rand::rng()));

    // let material = Rc::new(Material::Debug);
    // let material = Rc::new(Material::RandomDiffuse(rng.clone(), Color(0.5,0.5,0.5)));
    // let material = Rc::new(Material::LambertianDiffuseRandom {rng_cell: rng.clone(), albedo: Color(0.5,0.5,0.5)});
    let material_ground = Rc::new(Material::LambertianDiffuseRandom {
        rng_cell: rng.clone(), 
        albedo: Color(0.8,0.8,0.0)
    });
    let material_left = Rc::new(Material::Dielectric(1.50, rng.clone()));
    let material_bubble = Rc::new(Material::Dielectric(1.0/1.50, rng.clone()));
    let material_right = Rc::new(Material::MetalicFuzz(Color(0.8,0.6,0.2), 1.0, rng.clone()));
    let material_center = Rc::new(Material::LambertianDiffuseRandom {
        rng_cell: rng.clone(), 
        albedo: Color(0.1,0.2,0.5)
    });
    
    let max_bounces = 10;
    // let max_bounces = 2;

    world.add(Rc::new(
        Sphere::new(Point3(0.,-100.5,-1.0), 100., material_ground.clone())
    ));
    world.add(Rc::new(
        Sphere::new(Point3(0.,0.,-1.2), 0.5, material_center.clone())
    ));
    world.add(Rc::new(
        Sphere::new(Point3(-1.0, 0.0, -1.0), 0.5, material_left.clone())
    ));
    world.add(Rc::new(
        Sphere::new(Point3(-1.0, 0.0, -1.0), 0.45, material_bubble.clone())
    ));
    world.add(Rc::new(
        Sphere::new(Point3(1.0, 0.0, -1.0), 0.5, material_right.clone())
    ));


    let aspect_ratio:f64 = 16.0 / 9.0;
    // let image_width:usize = 400;
    // let image_width:usize = 400;
    let image_width:usize = 600;
    let pixel_samples = 50;


    let anti_aliasing = AntiAliasing::RandomSamples(pixel_samples, rng.clone());

    let fov = 90.0;
    let lookfrom = Point3(-2.0, 2.0, 1.0);
    let lookat = Point3(0.0, 0.0, -1.0); 
    let vup = Vec3(0.0,1.0,0.0);

    let mut cam_builder = CameraBuilder::default();

    cam_builder.aspect_ratio = aspect_ratio;
    cam_builder.image_width = image_width;
    cam_builder.max_depth = max_bounces + 1;
    cam_builder.anti_aliasing = anti_aliasing;
    cam_builder.vfov = fov;
    cam_builder.lookat = lookat;
    cam_builder.lookfrom = lookfrom;
    cam_builder.vup = vup;

    let camera = cam_builder.build();
    // let camera = Camera::new_builder(aspect_ratio, image_width, max_bounces + 1, anti_aliasing, 90.0, lookat, lookfrom).build();

    camera.render(&world);

    // Render


    // CAMERA STUFF

    
}
