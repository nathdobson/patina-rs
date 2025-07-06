use crate::geo1::interval::Interval;
use patina_vec::vec3::Vec3;
use rand::Rng;
use rand::prelude::Distribution;

#[derive(Copy, Clone, Debug)]
pub struct Aabb {
    min: Vec3,
    max: Vec3,
}

impl Aabb {
    pub fn new(min: Vec3, max: Vec3) -> Self {
        Aabb { min, max }
    }
    pub fn from_intervals(x: Interval, y: Interval, z: Interval) -> Self {
        Self::new(
            Vec3::new(x.min(), y.min(), z.min()),
            Vec3::new(x.max(), y.max(), z.max()),
        )
    }
    pub fn min(&self) -> Vec3 {
        self.min
    }
    pub fn max(&self) -> Vec3 {
        self.max
    }
    pub fn center(&self) -> Vec3 {
        (self.min + self.max) / 2.0
    }
    pub fn from_point(p: Vec3) -> Self {
        Self::new(p, p)
    }
    pub fn empty() -> Self {
        Self::new(Vec3::splat(f64::INFINITY), Vec3::splat(-f64::INFINITY))
    }
    pub fn union(&self, other: &Self) -> Self {
        Self::new(self.min.min(other.min), self.max.max(other.max))
    }
    pub fn surface_area(&self) -> f64 {
        let d = self.max - self.min;
        let d = d.max(Vec3::splat(0.0));
        d.x() * d.y() + d.x() * d.z() + d.y() * d.z()
    }
    pub fn intersect(&self, other: &Self) -> Self {
        Self::new(self.min.max(other.min), self.max.min(other.max))
    }
    pub fn dimensions(&self) -> Vec3 {
        (self.max - self.min).max(Vec3::zero())
    }
    pub fn intersects(&self, other: &Self) -> bool {
        self.intersect(other)
            .dimensions()
            .into_iter()
            .all(|x| x >= 0.0)
    }
    pub fn vertices(&self) -> [Vec3; 8] {
        let min = self.min;
        let max = self.max;
        [
            Vec3::new(min.x(), min.y(), min.z()),
            Vec3::new(min.x(), min.y(), max.z()),
            Vec3::new(min.x(), max.y(), min.z()),
            Vec3::new(min.x(), max.y(), max.z()),
            Vec3::new(max.x(), min.y(), min.z()),
            Vec3::new(max.x(), min.y(), max.z()),
            Vec3::new(max.x(), max.y(), min.z()),
            Vec3::new(max.x(), max.y(), max.z()),
        ]
    }
    pub fn octants(&self) -> [Self; 8] {
        let [x1, y1, z1] = self.min.into();
        let [x2, y2, z2] = self.center().into();
        let [x3, y3, z3] = self.max.into();
        let x12 = Interval::new(x1, x2);
        let x23 = Interval::new(x2, x3);
        let y12 = Interval::new(y1, y2);
        let y23 = Interval::new(y2, y3);
        let z12 = Interval::new(z1, z2);
        let z23 = Interval::new(z2, z3);
        [
            Aabb::from_intervals(x12, y12, z12),
            Aabb::from_intervals(x12, y12, z23),
            Aabb::from_intervals(x12, y23, z12),
            Aabb::from_intervals(x12, y23, z23),
            Aabb::from_intervals(x23, y12, z12),
            Aabb::from_intervals(x23, y12, z23),
            Aabb::from_intervals(x23, y23, z12),
            Aabb::from_intervals(x23, y23, z23),
        ]
    }
}

impl Distribution<Vec3> for Aabb {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Vec3 {
        (0..3)
            .map(|axis| rng.random_range(self.min[axis]..self.max[axis]))
            .collect()
    }
}
