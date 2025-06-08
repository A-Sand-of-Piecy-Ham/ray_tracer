// mod vec3;

use std::process::Output;
use std::{io::Write, io::BufWriter};
use std::io;
use image::ImageEncoder;

use super::Interval;
pub use super::Vec3 as Color;


pub fn write_color<W: Write>(out: &mut BufWriter<W>, pixel_color: &Color) -> io::Result<()> {
    let intensity = Interval::new(0.0, 0.999);
    // let intensity = Interval::new(0.0, 1.0-EPSILON);

    let gamma_corrected = pixel_color.linear_to_gamma();

    // Gamma correction
    // (r, g, b) = (linear_to_gamma(r), linear_to_gamma(g), linear_to_gamma(b));
    
    let [ir, ig, ib] = gamma_corrected.f32_color_to_u8(); 

    // let ir = (255.999 * intensity.clamp(r)) as u32;
    // let ig = (255.999 * intensity.clamp(g)) as u32;
    // let ib = (255.999 * intensity.clamp(b)) as u32;

    // out.write_fmt(format_args!("{} {} {}", ir, ig, ib));
    writeln!(out, "{} {} {}", ir, ig, ib)?;
    // out.write()

    Ok(())
}

pub trait F32ColorToU8 {
    type Output;
    // const INTENSITY: Interval = Interval::new(0.0, 0.999);
    const INTENSITY: std::ops::Range<f32> = 0.0..0.999;
    fn f32_color_to_u8(&self) -> Self::Output;
}

impl F32ColorToU8 for f32 {
    type Output = u8;
    #[inline]
    fn f32_color_to_u8(&self) -> Self::Output {
        // (255.999 * Self::INTENSITY.clamp(*self)) as u32
        (255.999 * self.clamp(Self::INTENSITY.start, Self::INTENSITY.end)) as u8
    }
}

impl F32ColorToU8 for Color {
    type Output = [u8;3];
    #[inline]
    fn f32_color_to_u8(&self) -> Self::Output {
        [self.0.f32_color_to_u8(), self.1.f32_color_to_u8(), self.2.f32_color_to_u8()]     
    }
}


pub trait GammaCorrection{
    // type Output;
    fn linear_to_gamma(&self) -> Self;
}

impl GammaCorrection for Color {
    // type Output = (u32, u32, u32);
    #[inline]
    fn linear_to_gamma(&self) -> Self {
        Color(self.0.linear_to_gamma(), self.1.linear_to_gamma(), self.2.linear_to_gamma())
    }
}

impl GammaCorrection for f32 {
    // type Output = u32;
    #[inline]
    fn linear_to_gamma(&self) -> Self {
        if *self > 0.0 {
            return self.sqrt();
        }
        0.0
    }
}

// pub fn write_color_buf<W: Write>(image_buffer: &mut Vec<Color>, to_write: &[Color], offset: usize, pixel_color: &Color) -> io::Result<()> {
//     let intensity = Interval::new(0.0, 0.999);
//     // let intensity = Interval::new(0.0, 1.0-EPSILON);

//     let Color(mut r, mut g, mut b) = *pixel_color;

//     // Gamma correction
//     (r, g, b) = (linear_to_gamma(r), linear_to_gamma(g), linear_to_gamma(b));

//     let ir = (255.999 * intensity.clamp(r)) as u32;
//     let ig = (255.999 * intensity.clamp(g)) as u32;
//     let ib = (255.999 * intensity.clamp(b)) as u32;

//     // out.write_fmt(format_args!("{} {} {}", ir, ig, ib));
//     image_buffer[offset..(offset+to_write.len())].copy_from_slice(to_write); 
//     // out.write()

//     Ok(())
// }

/// Convert linear color space to gamma corrected
/// 
/// TODO: Implement one to work on full colors?
// #[inline]
fn linear_to_gamma(linear_component: f32) -> f32 {
    if linear_component > 0. {
        return linear_component.sqrt();
    }
    0.0
}