use crate::geo3::aabb::AABB;
use crate::geo3::ray3::Ray3;
use crate::math::float_bool::{Epsilon, FloatBool};
use crate::math::interval::Interval;
use crate::math::vec3::Vec3;

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
    pub fn intersects_aabb(&self, aabb: &AABB, eps: Epsilon) -> (FloatBool, Interval) {
        let (truth, time) = self.as_ray().intersect_aabb(aabb, eps);
        (truth.and(eps.less(time.min(), 1.0)), time)
    }
    pub fn project(&self, v: Vec3) -> (f64, Vec3) {
        let delta = self.p2 - self.p1;
        let time = delta.dot(v - self.p1) / delta.length_squared();
        (time, self.p1 + time * delta)
    }
    pub fn intersects_vertex(&self, v: Vec3, eps: Epsilon) -> (FloatBool, f64, Vec3) {
        let (t, pv) = self.project(v);
        let d = (v - pv).length();
        let truth = eps
            .equals(d, 0.0)
            .and(eps.less(0.0, t))
            .and(eps.less(1.0, t));
        (truth, t, pv)
    }
    pub fn intersects_segment(
        &self,
        other: &Segment3,
        eps: Epsilon,
    ) -> (FloatBool, f64, f64, Vec3) {
        let (truth, t1, t2, p) = self.as_ray().intersects_line(&other.as_ray(), eps);
        if truth == FloatBool::from(false) {
            return (truth, t1, t2, p);
        }
        assert!(t1.is_finite());
        assert!(t2.is_finite());
        (
            truth
                .and(eps.less(0.0, t1))
                .and(eps.less(t1, 1.0))
                .and(eps.less(0.0, t2))
                .and(eps.less(t2, 1.0)),
            t1,
            t2,
            p,
        )
    }
}
