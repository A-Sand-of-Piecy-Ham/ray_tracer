// mod vec3;

use std::{io::Write, io::BufWriter};
use std::io;
use super::Interval;
pub use super::Vec3 as Color;


pub fn write_color<W: Write>(out: &mut BufWriter<W>, pixel_color: &Color) -> io::Result<()> {
    let intensity = Interval::new(0.0, 0.999);
    // let intensity = Interval::new(0.0, 1.0-EPSILON);

    let Color(mut r, mut g, mut b) = *pixel_color;

    // Gamma correction
    (r, g, b) = (linear_to_gamma(r), linear_to_gamma(g), linear_to_gamma(b));

    let ir = (255.999 * intensity.clamp(r)) as u32;
    let ig = (255.999 * intensity.clamp(g)) as u32;
    let ib = (255.999 * intensity.clamp(b)) as u32;

    // out.write_fmt(format_args!("{} {} {}", ir, ig, ib));
    writeln!(out, "{} {} {}", ir, ig, ib)?;
    // out.write()

    Ok(())
}

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