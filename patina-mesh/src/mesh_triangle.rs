use crate::directed_mesh_edge::DirectedMeshEdge;
use crate::mesh_edge::MeshEdge;
use patina_geo::geo3::triangle3::Triangle3;
use patina_vec::vec3::Vec3;
use std::fmt::{Debug, Formatter};
use std::ops::Index;

#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct MeshTriangle {
    vertices: [usize; 3],
}

impl MeshTriangle {
    pub fn new(v1: usize, v2: usize, v3: usize) -> Self {
        let mut vertices = [v1, v2, v3];
        let max_index = vertices.iter().enumerate().min_by_key(|v| v.1).unwrap().0;
        vertices.rotate_left(max_index);
        MeshTriangle { vertices }
    }
    pub fn invert(&mut self) {
        self.vertices.swap(1, 2);
    }
    pub fn vertices(&self) -> [usize; 3] {
        self.vertices
    }
    pub fn vertices_mut(&mut self) -> &mut [usize; 3] {
        &mut self.vertices
    }
    pub fn ordered_edges(&self) -> [DirectedMeshEdge; 3] {
        [
            DirectedMeshEdge::new(self.vertices[0], self.vertices[1]),
            DirectedMeshEdge::new(self.vertices[1], self.vertices[2]),
            DirectedMeshEdge::new(self.vertices[2], self.vertices[0]),
        ]
    }
    pub fn edges(&self) -> [MeshEdge; 3] {
        [
            MeshEdge::new(self.vertices[0], self.vertices[1]),
            MeshEdge::new(self.vertices[1], self.vertices[2]),
            MeshEdge::new(self.vertices[2], self.vertices[0]),
        ]
    }
    pub fn for_vertices(&self, vs: &[Vec3]) -> Triangle3 {
        Triangle3::new(self.vertices.map(|v| {
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
