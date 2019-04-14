use crate::geometry::*;
use crate::scene::*;

pub struct Sphere {
    pos: Point,
    radius: f64,
}

impl Sphere {
    pub fn new(pos: Point, radius: f64) -> Self {
        Self {
            pos, radius
        }
    }
}

impl Draw for Sphere {
    fn distance(&self, p: &Point) -> f64 {
        (self.pos - *p).norm() - self.radius
    }
}
