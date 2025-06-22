use crate::geo2::ray2::Ray2;
use crate::geo2::segment2::Segment2;
use crate::math::vec2::Vec2;
use itertools::Itertools;

pub struct Triangle2([Vec2; 3]);

impl Triangle2 {
    pub fn new(ps: [Vec2; 3]) -> Triangle2 {
        Triangle2(ps)
    }
    pub fn vertices(&self) -> [Vec2; 3] {
        self.0
    }
    pub fn edges(&self) -> [Segment2; 3] {
        [
            Segment2::new(self.0[0], self.0[1]),
            Segment2::new(self.0[1], self.0[2]),
            Segment2::new(self.0[2], self.0[0]),
        ]
    }
    pub fn intersects_point(&self, p: Vec2) -> bool {
        self.edges()
            .map(|e| e.as_ray().is_left(p))
            .iter()
            .cloned()
            .all_equal()
    }
    pub fn area(&self) -> f64 {
        (self.vertices()[1] - self.vertices()[0]).cross(self.vertices()[2] - self.vertices()[0])
    }
}
