use crate::mesh_triangle::MeshTriangle;
use patina_vec::vec2::Vec2;

pub struct TriMesh2 {
    vertices: Vec<Vec2>,
    tris: Vec<MeshTriangle>,
}

impl TriMesh2 {
    pub fn new(vertices: Vec<Vec2>, tris: Vec<MeshTriangle>) -> Self {
        Self { vertices, tris }
    }
    pub fn vertices(&self) -> &[Vec2] {
        &self.vertices
    }
    pub fn tris(&self) -> &[MeshTriangle] {
        &self.tris
    }
}
