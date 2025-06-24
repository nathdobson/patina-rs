use crate::geo3::aabb::AABB;
use crate::geo3::ray3::Ray3;
use crate::math::float_bool::{Epsilon, FloatBool};
use crate::math::interval::Interval;
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
    pub fn reverse(&self) -> Segment3 {
        Segment3::new(self.p2, self.p1)
    }
    pub fn length(&self) -> f64 {
        (self.p2 - self.p1).length()
    }
    pub fn intersects_aabb(&self, aabb: &AABB, eps: Epsilon) -> (FloatBool, Interval) {
        let (truth, time) = self.as_ray().intersect_aabb(aabb, eps);
        (truth.and(eps.less(time.min(), 1.0)), time)
    }
}
