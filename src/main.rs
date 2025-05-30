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
    let material = Rc::new(Material::RandomDiffuse(rng.clone()));
    let max_bounces = 10;

    world.add(Rc::new(
        Sphere::new(Point3(0.,0.,-1.0), 0.5, material.clone())
    ));
    world.add(Rc::new(
        Sphere::new(Point3(0.,-100.5,-1.0), 100., material.clone())
    ));

    let aspect_ratio:f64 = 16.0 / 9.0;
    let image_width:usize = 400;
    let pixel_samples = 100;


    let anti_aliasing = AntiAliasing::RandomSamples(pixel_samples, rng.clone());

    let camera = Camera::new_builder(aspect_ratio, image_width, max_bounces, anti_aliasing).build();

    camera.render(&world);

    // Render


    // CAMERA STUFF

    
}
