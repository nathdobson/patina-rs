use crate::geo3::aabb::AABB;
use crate::math::float_bool::{Epsilon, FloatBool};
use crate::math::interval::Interval;
use crate::math::vec3::Vec3;

#[derive(Debug)]
pub struct Ray3 {
    origin: Vec3,
    dir: Vec3,
}

impl Ray3 {
    pub fn new(start: Vec3, dir: Vec3) -> Self {
        Ray3 { origin: start, dir }
    }
    pub fn origin(&self) -> Vec3 {
        self.origin
    }
    pub fn dir(&self) -> Vec3 {
        self.dir
    }
    pub fn intersect_aabb(&self, aabb: &AABB, eps: Epsilon) -> (FloatBool, Interval) {
        let mut interval = Interval::new(0.0, f64::INFINITY);
        for axis in 0..3 {
            let m = self.dir[axis];
            let b = self.origin[axis];
            let min = aabb.min()[axis];
            let max = aabb.max()[axis];
            let part = Interval::new((min - b) / m, (max - b) / m);
            interval = interval.intersect(part);
        }
        (interval.is_empty(eps).not(), interval)
    }
    pub fn project(&self, p: Vec3) -> f64 {
        (p - self.origin).dot(self.dir)
    }
    pub fn at_time(&self, t: f64) -> Vec3 {
        self.origin + self.dir * t
    }
    pub fn intersects_line(&self, other: &Ray3, eps: Epsilon) -> (FloatBool, f64, f64, Vec3) {
        let e1 = self.origin;
        let d1 = self.dir;
        let e2 = other.origin;
        let d2 = other.dir;
        let n = d1.cross(d2).normalize();
        if !n.is_finite() {
            return (FloatBool::from(false), f64::NAN, f64::NAN, Vec3::nan());
        }
        let distance = n.dot(e2 - e1).abs();
        let t1 = d2.cross(n).dot(e2 - e1);
        let t2 = d1.cross(n).dot(e2 - e1);
        let p1 = self.at_time(t1);
        let p2 = other.at_time(t2);
        let p = (p1 + p2) / 2.0;
        assert!(t1.is_finite());
        assert!(t2.is_finite());
        (eps.equals(distance, 0.0), t1, t2, p)
    }
}
