use crate::geo2::ray2::Ray2;
use crate::geo2::segment2::Segment2;
use crate::math::vec2::Vec2;
use itertools::Itertools;

pub struct Polygon2 {
    vertices: Vec<Vec2>,
}

impl Polygon2 {
    pub fn new(vertices: Vec<Vec2>) -> Self {
        Polygon2 { vertices }
    }
    pub fn vertices(&self) -> &Vec<Vec2> {
        &self.vertices
    }
    pub fn edges(&self) -> impl Iterator<Item = Segment2> {
        self.vertices
            .iter()
            .cloned()
            .circular_tuple_windows()
            .map(|(a, b)| Segment2::new(a, b))
    }
    pub fn intersects_point(&self, p: Vec2) -> bool {
        let ray2 = Ray2::new(p, Vec2::new(2.3, 4.5));
        for e in self.edges() {
            if ray2.intersect_segment(&e) {
                return true;
            }
        }
        return false;
    }
}
