use crate::geo3::aabb::AABB;
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
    pub fn intersect_aabb(&self, aabb: &AABB) -> Option<Interval> {
        let mut interval = Interval::full();
        for axis in 0..3 {
            let m = self.dir[axis];
            let b = self.origin[axis];
            let min = aabb.min()[axis];
            let max = aabb.max()[axis];
            let part = Interval::new((min - b) / m, (max - b) / m);
            interval = interval.intersect(part);
        }
        (!interval.is_empty()).then_some(interval)
    }
    pub fn project(&self, p: Vec3) -> f64 {
        (p - self.origin).dot(self.dir)
    }
    pub fn at_time(&self, t: f64) -> Vec3 {
        self.origin + self.dir * t
    }
}
