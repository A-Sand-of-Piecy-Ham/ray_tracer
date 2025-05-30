// mod vec3;

use std::f32::EPSILON;
use std::{io::Write, io::BufWriter};
use std::io;
use super::Interval;
pub use super::Vec3 as Color;


pub fn write_color<W: Write>(out: &mut BufWriter<W>, pixel_color: &Color) -> io::Result<()> {
    let intensity = Interval::new(0.0, 0.999);
    // let intensity = Interval::new(0.0, 1.0-EPSILON);

    let Color(r, g, b) = *pixel_color;

    let ir = (255.999 * intensity.clamp(r)) as u32;
    let ig = (255.999 * intensity.clamp(g)) as u32;
    let ib = (255.999 * intensity.clamp(b)) as u32;

    // out.write_fmt(format_args!("{} {} {}", ir, ig, ib));
    writeln!(out, "{} {} {}", ir, ig, ib)?;
    // out.write()

    Ok(())
}