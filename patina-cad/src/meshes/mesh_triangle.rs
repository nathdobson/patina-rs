use crate::geo3::triangle::Triangle;
use crate::math::vec3::Vec3;
use crate::meshes::mesh_edge::MeshEdge;
use std::fmt::{Debug, Formatter};
use std::ops::Index;

#[derive(Copy, Clone)]
pub struct MeshTriangle {
    vertices: [usize; 3],
}

impl MeshTriangle {
    pub fn new(v1: usize, v2: usize, v3: usize) -> Self {
        MeshTriangle {
            vertices: [v1, v2, v3],
        }
    }
    pub fn invert(&mut self) {
        self.vertices.swap(1, 2);
    }
    pub fn vertices(&self) -> [usize; 3] {
        self.vertices
    }
    pub fn edges(&self) -> [MeshEdge; 3] {
        [
            MeshEdge::new(self.vertices[0], self.vertices[1]),
            MeshEdge::new(self.vertices[1], self.vertices[2]),
            MeshEdge::new(self.vertices[2], self.vertices[0]),
        ]
    }
    pub fn for_vertices(&self, vs: &[Vec3]) -> Triangle {
        Triangle::new(self.vertices.map(|v| {
            *vs.get(v)
                .unwrap_or_else(|| panic!("Vertex count is {} but the vertex is {}", vs.len(), v))
        }))
    }
}

impl From<[usize; 3]> for MeshTriangle {
    fn from(vertices: [usize; 3]) -> Self {
        MeshTriangle { vertices }
    }
}

impl Index<usize> for MeshTriangle {
    type Output = usize;
    fn index(&self, index: usize) -> &Self::Output {
        &self.vertices[index]
    }
}

impl IntoIterator for MeshTriangle {
    type Item = usize;
    type IntoIter = <[usize; 3] as IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        self.vertices.into_iter()
    }
}

impl Debug for MeshTriangle {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.vertices)
    }
}
