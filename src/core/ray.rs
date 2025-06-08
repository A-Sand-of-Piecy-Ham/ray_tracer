

use super::Vec3;

type Point3 = Vec3;

#[derive(Debug, Clone, Copy)]
pub struct Ray {
    pub origin: Point3, 
    pub direction: Vec3,
}


impl Ray {
    pub fn new(origin: Point3, direction: Vec3) -> Self {
        Self{origin, direction}
    }
    /// Follows ray down `dist` from origin and returns position
    pub fn at(&self, dist: f32) -> Point3 {
        self.origin + dist*self.direction
    }

}
