use crate::transvoxel;
use crate::transvoxel::cube_edge::CubeEdgeSet;
use crate::transvoxel::cube_face::{CubeFace, CubeFaceSet};
use crate::transvoxel::cube_triangle::{CubeTri, CubeTriMesh};
use crate::transvoxel::cube_vertex;
use crate::transvoxel::cube_vertex::{CubeVertex, CubeVertexSet, oriented};
use arrayvec::ArrayVec;
use itertools::Itertools;
use std::mem;

#[derive(Debug)]
pub struct CubeTetr([CubeVertex; 4]);

pub struct CubeTetrMesh(Vec<CubeTetr>);

impl CubeTetr {
    pub fn new(vs: [CubeVertex; 4]) -> Self {
        CubeTetr(vs)
    }
    pub fn vertices(&self) -> &[CubeVertex; 4] {
        &self.0
    }
    pub fn signed_volume6(&self) -> isize {
        let [v0, v1, v2, v3] = self.0.map(|v| v.map(|x| x as isize));
        (v1 - v0).cross(v2 - v0).dot(v3 - v0)
    }
}

impl CubeTetrMesh {
    pub fn divided_cube(faces: &CubeFaceSet, edges: &CubeEdgeSet) -> CubeTetrMesh {
        if *faces == CubeFaceSet::new() && *edges == CubeEdgeSet::new() {
            return CubeTetrMesh::basic_cube();
        }
        let mut tetrs = vec![];
        let mut mask = CubeVertexSet::new();
        faces.add_samples_to(&mut mask);
        edges.add_samples_to(&mut mask);
        let center = cube_vertex::cube_center();
        for face in CubeFace::all() {
            let face_center = face.face_center();
            if faces[face] {
                for [axis2, axis3] in face.other_axes_orders() {
                    for y0 in 0..2 {
                        for z0 in 0..2 {
                            let mut v1 = face_center;
                            v1[axis2 as usize] = y0;
                            v1[axis3 as usize] = z0;
                            let mut v2 = face_center;
                            v2[axis2 as usize] = y0 + 1;
                            v2[axis3 as usize] = z0;
                            let mut v3 = face_center;
                            v3[axis2 as usize] = y0 + 1;
                            v3[axis3 as usize] = z0 + 1;
                            tetrs.push(CubeTetr::new([center, v1, v2, v3]));
                        }
                    }
                }
            } else {
                for [axis2, axis3] in face.other_axes_orders() {
                    let mut corner1 = face_center;
                    corner1[axis2 as usize] = 0;
                    corner1[axis3 as usize] = 0;
                    let mut midpoint12 = face_center;
                    midpoint12[axis2 as usize] = 1;
                    midpoint12[axis3 as usize] = 0;
                    let mut corner2 = face_center;
                    corner2[axis2 as usize] = 2;
                    corner2[axis3 as usize] = 0;
                    let mut midpoint23 = face_center;
                    midpoint23[axis2 as usize] = 2;
                    midpoint23[axis3 as usize] = 1;
                    let mut corner3 = face_center;
                    corner3[axis2 as usize] = 2;
                    corner3[axis3 as usize] = 2;
                    match (mask[midpoint12], mask[midpoint23]) {
                        (false, false) => {
                            tetrs.push(CubeTetr::new([center, corner1, corner2, corner3]));
                        }
                        (true, false) => {
                            tetrs.push(CubeTetr::new([center, corner1, midpoint12, corner3]));
                            tetrs.push(CubeTetr::new([center, midpoint12, corner2, corner3]));
                        }
                        (false, true) => {
                            tetrs.push(CubeTetr::new([center, corner1, corner2, midpoint23]));
                            tetrs.push(CubeTetr::new([center, corner1, midpoint23, corner3]));
                        }
                        (true, true) => {
                            tetrs.push(CubeTetr::new([center, corner1, midpoint12, corner3]));
                            tetrs.push(CubeTetr::new([center, midpoint12, midpoint23, corner3]));
                            tetrs.push(CubeTetr::new([center, midpoint12, corner2, midpoint23]));
                        }
                    }
                }
            }
        }
        CubeTetrMesh::new(tetrs)
    }
    pub fn new(tetrs: Vec<CubeTetr>) -> Self {
        CubeTetrMesh(tetrs)
    }
    pub fn basic_cube() -> Self {
        let mut tetrs = vec![];
        for axes in [0usize, 1, 2].iter().permutations(3) {
            let mut tetr = ArrayVec::<_, 4>::new();
            let mut v = CubeVertex::new(0, 0, 0);
            tetr.push(v);
            for &axis in axes {
                v[axis] = 2;
                tetr.push(v);
            }
            tetrs.push(CubeTetr(tetr.into_inner().unwrap()));
        }
        CubeTetrMesh(tetrs)
    }
    pub fn volume6(&self) -> usize {
        self.0
            .iter()
            .map(|tetr| tetr.signed_volume6().abs() as usize)
            .sum()
    }
    pub fn as_debug_tri_mesh(&self) -> CubeTriMesh {
        let mut tris = vec![];
        for tetr in &self.0 {
            for vs in tetr.0.iter().combinations(3) {
                tris.push(CubeTri::new(
                    vs.into_iter().map(|v| (*v, *v)).collect_array().unwrap(),
                ));
            }
        }
        CubeTriMesh::new(tris)
    }
    pub fn tetrs(&self) -> &[CubeTetr] {
        &self.0
    }
}

impl CubeTetr {
    pub fn as_mesh(&self, inside: &[CubeVertex], outside: &[CubeVertex]) -> CubeTriMesh {
        let mut triangles = vec![];
        match (inside, outside) {
            ([], [_, _, _, _]) => {}
            (&[i1], &[mut o1, mut o2, o3]) => {
                if oriented([o1, o2, o3], i1) {
                    mem::swap(&mut o1, &mut o2);
                }
                triangles.push(CubeTri::new([(i1, o1), (i1, o2), (i1, o3)]));
            }
            (&[i1, i2], &[mut o1, mut o2]) => {
                if oriented([i1, i2, o2], o1) {
                    mem::swap(&mut o1, &mut o2);
                }
                triangles.push(CubeTri::new([(i1, o1), (i1, o2), (i2, o2)]));
                triangles.push(CubeTri::new([(i1, o1), (i2, o2), (i2, o1)]));
            }
            (&[mut i1, mut i2, i3], &[o1]) => {
                if !oriented([i1, i2, i3], o1) {
                    mem::swap(&mut i1, &mut i2);
                }
                triangles.push(CubeTri::new([(o1, i1), (o1, i2), (o1, i3)]));
            }
            ([_, _, _, _], []) => {}
            _ => unreachable!(),
        }
        CubeTriMesh::new(triangles)
    }
}
