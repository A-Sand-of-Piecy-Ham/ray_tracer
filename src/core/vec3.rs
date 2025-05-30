use std::ops;

use rand::{rngs::SmallRng, Rng};




/// TEST TEST
/// 
/// ```
/// // Egg coding
/// let egg = Vec3{1,1,1};
/// ```
/// 
/// 
#[derive(Default, Debug, Copy, Clone)]
pub struct Vec3(pub f32, pub f32, pub f32);

impl Vec3 {
    pub const fn div_scalar(self, rhs: f32) -> Self {
        Self(self.0/rhs, self.1/rhs, self.2/rhs)
    }
}

pub fn unit_vector(vec: Vec3) -> Vec3 {
    vec.normalize()
}

impl ops::Index<usize> for Vec3 {
    type Output = f32;

    fn index(&self, index: usize) -> &Self::Output {
        match index {
            0 => &self.0,
            1 => &self.1,
            2 => &self.2,
            x => panic!("Index {x} out of bounds!")
        }
    }
}

impl ops::Neg for Vec3 {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Vec3(-self.0, -self.1, -self.2)
    }
}
impl ops::Add for Vec3 {
    type Output = Self;

    fn add(self, rhs: Vec3) -> Self::Output {
        Vec3(self.0 + rhs.0, self.1 + rhs.1, self.2 + rhs.2)
    }
}
impl ops::AddAssign for Vec3 {
    fn add_assign(&mut self, rhs: Self) {
        // self.0 += rhs.0; 
        // self.1 += rhs.1;
        // self.2 += rhs.2;
        *self = *self + rhs
    }
}
impl ops::Sub for Vec3 {
    type Output = Self;

    fn sub(self, rhs: Vec3) -> Self::Output {
        Vec3(self.0 - rhs.0, self.1 - rhs.1, self.2 - rhs.2)
    }
}
impl ops::Mul for Vec3 {
    type Output = Self;

    fn mul(self, rhs: Vec3) -> Self::Output {
        Vec3(self.0 * rhs.0, self.1 * rhs.1, self.2 * rhs.2)
    }
}
impl<T> ops::Mul<T> for Vec3 
where f32: ops::Mul<T, Output = f32>, T: Copy {
    type Output = Self;

    fn mul(self, rhs: T) -> Self::Output {
        Vec3(self.0 * rhs, self.1 * rhs, self.2 * rhs)
    }
}
impl ops::Mul<Vec3> for f32
where f32: ops::Mul<Self, Output = f32>, Self: Copy {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Self::Output {
        Vec3(self * rhs.0, self * rhs.1, self * rhs.2)
    }
}
impl<T:std::ops::Mul<Output = f32>> ops::MulAssign<T> for Vec3
where Vec3: ops::Mul<T, Output = Vec3> {
    fn mul_assign(&mut self, rhs: T) {
        *self = std::mem::take(self) * rhs;
    }
}
impl<T> ops::Div<T> for Vec3 
where f32: ops::Div<T, Output = f32>, T: Copy {
    type Output = Self;

    fn div(self, rhs: T) -> Self::Output {
        Vec3(self.0 / rhs, self.1 / rhs, self.2 / rhs)
    }
}
impl<T, U> ops::DivAssign<T> for Vec3
where 
    u32: ops::Div<T, Output = U>, 
    Self: ops::MulAssign<U> 
{
    fn div_assign(&mut self, rhs: T) {
        *self *= 1/rhs
    }
}

pub fn dot(vec1: &Vec3, vec2: &Vec3) -> f32 {
    vec1.dot(vec2)
}

impl Vec3 {
    pub fn new(x:f32, y:f32, z:f32) -> Self {
        Self(x,y,z)
    }
    /// Creates a vec with random values between 0 and 1
    pub fn random<R: Rng>(rng: &mut R) -> Vec3 {
        Self(rng.random(), rng.random(), rng.random())
    }
    pub fn random_bounded<R: Rng>(rng: &mut R, min: f32, max:f32) -> Vec3 {
        Self(rng.random_range(min..max), rng.random_range(min..max), rng.random_range(min..max))
    }
    pub fn random_unit_vector<R: Rng>(rng: &mut R) -> Vec3 {
        loop {
            let p = Vec3::random_bounded(rng, -1.0, 1.0);
            let lensq = p.length_squared();
            if lensq <= 1.0 && lensq > 1e-160 { return p / lensq.sqrt(); }
        }
    }
    pub fn random_on_hemisphere<R: Rng>(normal: &Vec3, rng: &mut R) -> Vec3 {
        let on_unit_sphere = Self::random_unit_vector(rng);
        if dot(&on_unit_sphere, normal) > 0.0 {return on_unit_sphere}
        else {return -on_unit_sphere}
    }

    pub fn dot(&self, rhs: &Vec3) -> f32 {
        self.0 * rhs.0 + self.1 * rhs.1 + self.2 * rhs.2
    }

    pub fn length_squared(&self) -> f32 {
        self.0 * self.0 + self.1 * self.1 + self.2 * self.2
    }
    
    pub fn length(&self) -> f32 {
       self.length_squared().sqrt() 
    }

    pub fn normalize(&self) -> Vec3 {
        *self / self.length()
    }

    pub fn x(&self) -> &f32 {
        &self.0
    }
    pub fn y(&self) -> &f32 {
        &self.1
    }
    pub fn z(&self) -> &f32 {
        &self.2
    }
}