use crate::geo3::ray3::Ray3;
use crate::math::vec3::Vec3;

#[derive(Debug)]
pub struct Segment3 {
    p1: Vec3,
    p2: Vec3,
}

impl Segment3 {
    pub fn new(p1: Vec3, p2: Vec3) -> Self {
        Segment3 { p1, p2 }
    }
    pub fn p1(&self) -> Vec3 {
        self.p1
    }
    pub fn p2(&self) -> Vec3 {
        self.p2
    }
    pub fn at_time(&self, t: f64) -> Vec3 {
        self.p1 * (1.0 - t) + self.p2 * t
    }
    pub fn as_ray(&self) -> Ray3 {
        Ray3::new(self.p1, self.p2 - self.p1)
    }
}
