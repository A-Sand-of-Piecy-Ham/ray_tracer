pub(crate) mod vec3;
pub(crate) mod color;
pub(crate) mod ray;
pub(crate) mod sphere;
pub(crate) mod hittable;
pub(crate) mod hittable_list;
pub(crate) mod interval;



// mod random;
pub mod material;
pub mod camera;

pub mod util;

pub use vec3::{Vec3, unit_vector, dot};
pub use color::{write_color, Color};
pub use ray::{Ray};
pub use interval::Interval;

pub use vec3::Vec3 as Point3;