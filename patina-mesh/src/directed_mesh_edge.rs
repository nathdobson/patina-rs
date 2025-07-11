use crate::mesh_edge::MeshEdge;
use patina_geo::geo3::segment3::Segment3;
use patina_vec::vec3::Vec3;
use std::fmt::{Debug, Formatter};
use std::ops::Index;
use patina_geo::geo2::segment2::Segment2;
use patina_vec::vec2::Vec2;

#[derive(Copy, Clone, Eq, Ord, PartialOrd, PartialEq, Hash)]
pub struct DirectedMeshEdge {
    vertices: [usize; 2],
}

impl DirectedMeshEdge {
    pub fn new(v1: usize, v2: usize) -> Self {
        DirectedMeshEdge { vertices: [v1, v2] }
    }
    pub fn vertices(&self) -> [usize; 2] {
        self.vertices
    }
    pub fn v1(&self) -> usize {
        self.vertices[0]
    }
    pub fn v2(&self) -> usize {
        self.vertices[1]
    }
    #[must_use]
    pub fn inverted(self) -> DirectedMeshEdge {
        Self::new(self.vertices[1], self.vertices[0])
    }
    pub fn for_vertices(&self, vs: &[Vec3]) -> Segment3 {
        Segment3::new(vs[self.vertices[0]], vs[self.vertices[1]])
    }
    pub fn for_vertices2(&self, vs: &[Vec2]) -> Segment2 {
        Segment2::new(vs[self.vertices[0]], vs[self.vertices[1]])
    }
    pub fn edge(&self) -> MeshEdge {
        MeshEdge::new(self.vertices[0], self.vertices[1])
    }
}

impl From<[usize; 2]> for DirectedMeshEdge {
    fn from(vertices: [usize; 2]) -> Self {
        DirectedMeshEdge { vertices }
    }
}

impl Index<usize> for DirectedMeshEdge {
    type Output = usize;
    fn index(&self, index: usize) -> &Self::Output {
        &self.vertices[index]
    }
}

impl Debug for DirectedMeshEdge {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.vertices)
    }
}
