use crate::geo2::polygon2::Polygon2;
use crate::geo3::triangle3::Triangle3;
use patina_vec::vec3::Vec3;

pub struct Polygon3 {
    vertices: Vec<Vec3>,
}

impl Polygon3 {
    pub fn new(vertices: Vec<Vec3>) -> Self {
        Polygon3 { vertices }
    }
    pub fn vertices(&self) -> &[Vec3] {
        &self.vertices
    }
    pub fn project(&self, tri: &Triangle3) -> Polygon2 {
        Polygon2::new(self.vertices.iter().map(|&v| tri.project(v)).collect())
    }
}
