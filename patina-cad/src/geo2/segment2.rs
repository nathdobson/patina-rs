use crate::geo2::ray2::Ray2;
use crate::math::vec2::Vec2;

#[derive(Debug)]
pub struct Segment2 {
    p1: Vec2,
    p2: Vec2,
}

impl Segment2 {
    pub fn new(p1: Vec2, p2: Vec2) -> Self {
        Segment2 { p1, p2 }
    }
    pub fn p1(&self) -> Vec2 {
        self.p1
    }
    pub fn p2(&self) -> Vec2 {
        self.p2
    }
    pub fn at_time(&self, t: f64) -> Vec2 {
        self.p1 * (1.0 - t) + self.p2 * t
    }
    pub fn as_ray(&self) -> Ray2 {
        Ray2::new(self.p1, self.p2 - self.p1)
    }
    pub fn intersect_time(&self, other: Self) -> Option<Vec2> {
        let ts = self.as_ray().intersect_time(&other.as_ray())?;
        (ts.x() <= 1.0 && ts.y() <= 1.0).then_some(ts)
    }
}
