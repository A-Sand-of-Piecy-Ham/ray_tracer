use std::{default, f32::INFINITY};



pub struct Interval {
    pub min: f32,
    pub max: f32,
}

impl Default for Interval {
    fn default() -> Self {
        Self::empty()
    }
}

impl Interval {
    pub fn new(min: f32, max: f32) -> Self {
        Self{min, max}
    }
    pub fn size(&self) -> f32 {
        self.max - self.min
    }
    pub fn contains(&self, x: f32) -> bool {
        return self.min <= x && x <= self.max;
    }

    pub fn surrounds(&self, x: f32) -> bool {
        return self.min < x && x < self.max;
    }

    pub fn clamp(&self, x: f32) -> f32 {
        if x < self.min { return self.min; }
        if x > self.max { return self.max; }
        x
    }

    pub fn empty() -> Self {
        Self{min: INFINITY, max: -INFINITY}
    }

    pub fn universe() -> Self {
        Self{min: -INFINITY, max: INFINITY}
    }
}