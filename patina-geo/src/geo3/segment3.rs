use crate::geo3::ray3::Ray3;
use patina_vec::vec3::Vec3;

#[derive(Debug, Copy, Clone)]
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
    #[must_use]
    pub fn reverse(&self) -> Segment3 {
        Segment3::new(self.p2, self.p1)
    }
    pub fn length(&self) -> f64 {
        (self.p2 - self.p1).length()
    }
    pub fn midpoint(&self) -> Vec3 {
        (self.p2 - self.p1) / 2.0 + self.p1
    }
    pub fn displacement(&self) -> Vec3 {
        self.p2 - self.p1
    }
}
