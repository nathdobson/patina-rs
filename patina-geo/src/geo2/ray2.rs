use std::cmp::Ordering;
use patina_vec::vec2::Vec2;

pub struct Ray2 {
    origin: Vec2,
    dir: Vec2,
}
impl Ray2 {
    pub fn new(origin: Vec2, dir: Vec2) -> Self {
        Self { origin, dir }
    }
    pub fn origin(&self) -> Vec2 {
        self.origin
    }
    pub fn dir(&self) -> Vec2 {
        self.dir
    }
    pub fn above(&self, v: Vec2) -> Ordering {
        (v - self.origin).cross(self.dir).total_cmp(&0.0)
    }
}
