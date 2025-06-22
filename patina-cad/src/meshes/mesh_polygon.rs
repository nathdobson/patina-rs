use crate::geo3::polygon3::Polygon3;
use crate::math::vec2::Vec2;
use crate::math::vec3::Vec3;

#[derive(Debug, Clone)]
pub struct MeshPolygon {
    vertices: Vec<usize>,
}

impl MeshPolygon {
    pub fn new(vertices: Vec<usize>) -> Self {
        MeshPolygon { vertices }
    }
    pub fn vertices(&self) -> &[usize] {
        &self.vertices
    }
    pub fn for_vertices(&self, vertices: &[Vec3]) -> Polygon3 {
        Polygon3::new(self.vertices.iter().map(|v| vertices[*v]).collect())
    }
}
