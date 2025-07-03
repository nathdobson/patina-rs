use crate::geo3::segment3::Segment3;
use crate::geo3::triangle3::Triangle3;
use patina_vec::vec3::Vec3;
use crate::meshes::mesh_edge::MeshEdge;
use crate::meshes::mesh_triangle::MeshTriangle;
use std::fmt::{Debug, Formatter};
use std::ops::Index;

#[derive(Copy, Clone, Eq, Ord, PartialOrd, PartialEq, Hash)]
pub struct OrderedMeshEdge {
    vertices: [usize; 2],
}

impl OrderedMeshEdge {
    pub fn new(v1: usize, v2: usize) -> Self {
        OrderedMeshEdge { vertices: [v1, v2] }
    }
    pub fn invert(&mut self) {
        self.vertices.swap(0, 1);
    }
    pub fn vertices(&self) -> [usize; 2] {
        self.vertices
    }
    pub fn for_vertices(&self, vs: &[Vec3]) -> Segment3 {
        Segment3::new(vs[self.vertices[0]], vs[self.vertices[1]])
    }
    pub fn edge(&self) -> MeshEdge {
        MeshEdge::new(self.vertices[0], self.vertices[1])
    }
}

impl From<[usize; 2]> for OrderedMeshEdge {
    fn from(vertices: [usize; 2]) -> Self {
        OrderedMeshEdge { vertices }
    }
}

impl Index<usize> for OrderedMeshEdge {
    type Output = usize;
    fn index(&self, index: usize) -> &Self::Output {
        &self.vertices[index]
    }
}

impl Debug for OrderedMeshEdge {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.vertices)
    }
}
