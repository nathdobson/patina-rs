use crate::transvoxel::cube_tetr::{CubeTetr, CubeTetrMesh};
use crate::transvoxel::cube_vertex;
use crate::transvoxel::cube_vertex::{CubeVertex, CubeVertexSet};
use arrayvec::ArrayVec;
use itertools::Itertools;
use std::ops::{Index, IndexMut};

#[derive(Copy, Clone, Debug)]
pub struct CubeFace {
    axis: u8,
    side: bool,
}

#[derive(Copy, Clone, Eq, Ord, PartialOrd, PartialEq, Hash, Debug)]
pub struct CubeFaceSet([[bool; 2]; 3]);

impl CubeFace {
    pub fn new(axis: u8, side: bool) -> Self {
        CubeFace { axis, side }
    }
    pub fn all() -> [CubeFace; 6] {
        (0..3)
            .flat_map(|axis| [false, true].map(move |side| CubeFace { axis, side }))
            .collect_array()
            .unwrap()
    }
    pub fn axis(&self) -> u8 {
        self.axis
    }
    pub fn side(&self) -> bool {
        self.side
    }
    pub fn face_center(&self) -> CubeVertex {
        let mut result = CubeVertex::splat(1);
        result[self.axis as usize] = self.side as u8 * 2;
        result
    }
    pub fn midpoints(&self) -> [CubeVertex; 4] {
        let mut result = ArrayVec::new();
        for axis2 in 0..3 {
            if axis2 != self.axis {
                for side2 in 0..2 {
                    let mut p = CubeVertex::splat(1);
                    p[self.axis as usize] = self.side as u8 * 2;
                    p[axis2 as usize] = side2 as u8 * 2;
                    result.push(p);
                }
            }
        }
        result.into_inner().unwrap()
    }
    pub fn other_axes(&self) -> [u8; 2] {
        [(self.axis + 1) % 3, (self.axis + 2) % 3]
    }
    pub fn other_axes_orders(&self) -> [[u8; 2]; 2] {
        let [a1, a2] = self.other_axes();
        [[a1, a2], [a2, a1]]
    }
}

impl CubeFaceSet {
    pub fn new() -> Self {
        CubeFaceSet([[false; 2]; 3])
    }
    pub fn add_samples_to(&self, mask: &mut CubeVertexSet) {
        for face in CubeFace::all() {
            if self[face] {
                mask[face.face_center()] |= true;
                for midpoint in face.midpoints() {
                    mask[midpoint] |= true;
                }
            }
        }
    }
}

impl Index<CubeFace> for CubeFaceSet {
    type Output = bool;
    fn index(&self, index: CubeFace) -> &Self::Output {
        &self.0[index.axis as usize][index.side as usize]
    }
}

impl IndexMut<CubeFace> for CubeFaceSet {
    fn index_mut(&mut self, index: CubeFace) -> &mut Self::Output {
        &mut self.0[index.axis as usize][index.side as usize]
    }
}
