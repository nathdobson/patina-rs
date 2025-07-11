use patina_vec::vec2::Vec2;

#[derive(Debug)]
pub struct Triangle2([Vec2; 3]);

impl Triangle2 {
    pub fn new(points: [Vec2; 3]) -> Self {
        Triangle2(points)
    }
    pub fn points(&self) -> [Vec2; 3] {
        self.0
    }
    pub fn p1(&self) -> Vec2 {
        self.0[0]
    }
    pub fn p2(&self) -> Vec2 {
        self.0[1]
    }
    pub fn p3(&self) -> Vec2 {
        self.0[2]
    }
    pub fn signed_area(&self) -> f64 {
        (self.0[1] - self.0[0]).cross(self.0[2] - self.0[0]) / 2.0
    }
}
