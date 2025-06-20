use crate::geo3::segment3::Segment3;
use crate::geo3::triangle3::Triangle3;
use crate::math::vec3::Vec3;
use crate::meshes::mesh_triangle::MeshTriangle;
use crate::util::sorted_pair::SortedPair;

#[derive(Copy, Clone, Eq, Ord, PartialOrd, PartialEq, Hash, Debug)]
pub struct MeshEdge {
    vertices: [usize; 2],
}

impl MeshEdge {
    pub fn new(v1: usize, v2: usize) -> Self {
        MeshEdge { vertices: [v1, v2] }
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
    pub fn sorted(&self) -> SortedPair<usize> {
        SortedPair::new(self.vertices[0], self.vertices[1])
    }
}

impl From<[usize; 2]> for MeshEdge {
    fn from(vertices: [usize; 2]) -> Self {
        MeshEdge { vertices }
    }
}
