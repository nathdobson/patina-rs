use patina_geo::geo3::segment3::Segment3;
use patina_vec::vec3::Vec3;
use std::fmt::{Debug, Display, Formatter};
use std::ops::Index;

#[derive(Copy, Clone, Eq, Ord, PartialOrd, PartialEq, Hash)]
pub struct MeshEdge {
    vertices: [usize; 2],
}

impl MeshEdge {
    pub fn new(v1: usize, v2: usize) -> Self {
        let mut vs = [v1, v2];
        vs.sort();
        MeshEdge { vertices: vs }
    }
    pub fn v1(&self) -> usize {
        self.vertices[0]
    }
    pub fn v2(&self) -> usize {
        self.vertices[1]
    }
    pub fn vertices(&self) -> [usize; 2] {
        self.vertices
    }
    pub fn for_vertices(&self, vs: &[Vec3]) -> Segment3 {
        Segment3::new(vs[self.vertices[0]], vs[self.vertices[1]])
    }
    pub fn shares_vertex(&self, other: &MeshEdge) -> bool {
        for v1 in self.vertices {
            for v2 in other.vertices {
                if v1 == v2 {
                    return true;
                }
            }
        }
        false
    }
}

impl From<[usize; 2]> for MeshEdge {
    fn from(vertices: [usize; 2]) -> Self {
        MeshEdge { vertices }
    }
}

impl Index<usize> for MeshEdge {
    type Output = usize;
    fn index(&self, index: usize) -> &Self::Output {
        &self.vertices[index]
    }
}

impl Debug for MeshEdge {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{}, {}]", self.vertices[0], self.vertices[1])
    }
}

impl Display for MeshEdge {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let temp = format!("{}_{}", self.vertices[0], self.vertices[1]);
        f.pad(&temp)?;
        Ok(())
    }
}
