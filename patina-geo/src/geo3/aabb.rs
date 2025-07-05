use patina_vec::vec3::Vec3;

#[derive(Copy, Clone, Debug)]
pub struct Aabb {
    min: Vec3,
    max: Vec3,
}

impl Aabb {
    pub fn new(min: Vec3, max: Vec3) -> Self {
        Aabb { min, max }
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
}
