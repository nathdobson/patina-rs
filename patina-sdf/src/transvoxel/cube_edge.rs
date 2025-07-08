use crate::transvoxel::cube_vertex::{CubeVertex, CubeVertexSet};
use itertools::Itertools;
use std::ops::{Index, IndexMut};

#[derive(Debug, Clone, Copy)]
pub struct CubeEdge {
    axis: u8,
    side1: bool,
    side2: bool,
}

#[derive(Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct CubeEdgeSet([[[bool; 2]; 2]; 3]);

impl CubeEdge {
    pub fn all() -> [CubeEdge; 12] {
        (0..3)
            .flat_map(|axis| {
                [false, true].into_iter().flat_map(move |side1| {
                    [false, true]
                        .into_iter()
                        .map(move |side2| CubeEdge { axis, side1, side2 })
                })
            })
            .collect_array()
            .unwrap()
    }
    pub fn axis(&self) -> u8 {
        self.axis
    }
    pub fn axis1(&self) -> u8 {
        (self.axis + 1) % 3
    }
    pub fn axis2(&self) -> u8 {
        (self.axis + 2) % 3
    }
    pub fn side1(&self) -> bool {
        self.side1
    }
    pub fn side2(&self) -> bool {
        self.side2
    }
    pub fn vertex(&self, t: u8) -> CubeVertex {
        let mut result = CubeVertex::splat(0);
        result[self.axis as usize] = t;
        result[self.axis1() as usize] = (self.side1 as u8) * 2;
        result[self.axis2() as usize] = (self.side2 as u8) * 2;
        result
    }
}

impl Index<CubeEdge> for CubeEdgeSet {
    type Output = bool;
    fn index(&self, index: CubeEdge) -> &Self::Output {
        &self.0[index.axis as usize][index.side1 as usize][index.side2 as usize]
    }
}

impl IndexMut<CubeEdge> for CubeEdgeSet {
    fn index_mut(&mut self, index: CubeEdge) -> &mut Self::Output {
        &mut self.0[index.axis as usize][index.side1 as usize][index.side2 as usize]
    }
}

impl CubeEdgeSet {
    pub fn new() -> Self {
        CubeEdgeSet([[[false; 2]; 2]; 3])
    }
    pub fn add_samples_to(&self, mask: &mut CubeVertexSet) {
        for edge in CubeEdge::all() {
            if self[edge] {
                mask[edge.vertex(1)] |= true;
            }
        }
    }
}
