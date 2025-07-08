use arrayvec::ArrayVec;
use patina_geo::geo3::triangle3::Triangle3;
use patina_vec::vector3::Vector3;
use std::ops::{Index, IndexMut};
pub type CubeVertex = Vector3<u8>;

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct CubeVertexSet([[[bool; 3]; 3]; 3]);

impl CubeVertexSet {
    pub fn new() -> Self {
        CubeVertexSet([[[false; 3]; 3]; 3])
    }
    pub fn all() -> Self {
        CubeVertexSet([[[true; 3]; 3]; 3])
    }
    pub fn corners() -> Self {
        let mut result = Self::new();
        for corner in cube_corners() {
            result[corner] = true;
        }
        result
    }
}

impl Index<CubeVertex> for CubeVertexSet {
    type Output = bool;
    fn index(&self, index: CubeVertex) -> &Self::Output {
        &self.0[index.x() as usize][index.y() as usize][index.z() as usize]
    }
}

impl IndexMut<CubeVertex> for CubeVertexSet {
    fn index_mut(&mut self, index: CubeVertex) -> &mut Self::Output {
        &mut self.0[index.x() as usize][index.y() as usize][index.z() as usize]
    }
}

pub fn cube_center() -> CubeVertex {
    CubeVertex::splat(1)
}

pub fn cube_corners() -> [CubeVertex; 8] {
    let mut result = ArrayVec::new();
    for x in 0..2 {
        for y in 0..2 {
            for z in 0..2 {
                result.push(CubeVertex::new(x * 2, y * 2, z * 2));
            }
        }
    }
    result.into_inner().unwrap()
}

pub fn cube_points() -> impl Iterator<Item = CubeVertex> {
    (0..3).flat_map(|x| (0..3).flat_map(move |y| (0..3).map(move |z| CubeVertex::new(x, y, z))))
}

pub fn oriented(tri: [CubeVertex; 3], other: CubeVertex) -> bool {
    let plane = Triangle3::new(tri.map(|v| v.map(|v| v as f64))).plane();
    let other = other.map(|x| x as f64);
    plane.outside(other)
}
