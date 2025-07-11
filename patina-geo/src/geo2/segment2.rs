use crate::geo2::ray2::Ray2;
use patina_vec::vec2::Vec2;

#[derive(Debug, Clone)]
pub struct Segment2 {
    p1: Vec2,
    p2: Vec2,
}

impl Segment2 {
    pub fn new(p1: Vec2, p2: Vec2) -> Self {
        Self { p1, p2 }
    }
    pub fn p1(&self) -> Vec2 {
        self.p1
    }
    pub fn p2(&self) -> Vec2 {
        self.p2
    }
    pub fn as_ray(&self) -> Ray2 {
        Ray2::new(self.p1, self.p2 - self.p1)
    }
    pub fn intersects(&self, other: &Self) -> bool {
        let r1 = self.as_ray();
        let r2 = other.as_ray();
        let a11 = r1.above(other.p1);
        let a12 = r1.above(other.p2);
        let a21 = r2.above(self.p1);
        let a22 = r2.above(self.p2);
        a11 != a12 && a21 != a22
    }
}
