use patina_geo::geo3::triangle3::Triangle3;
use patina_mesh::mesh_triangle::MeshTriangle;
use patina_vec::vec3::Vec3;
use std::collections::HashMap;
use std::mem;
use std::ops::{Index, IndexMut};
use std::sync::LazyLock;
// #[derive(Copy, Clone, Eq, Ord, PartialEq, PartialOrd, Debug, Hash)]
// pub struct CubeVertex([u8; 3]);

// pub type CubeVertex = Vector3<u8>;

// pub struct CubeMesh {
//     vertices: Vec<CubeVertex>,
//     triangles: Vec<MeshTriangle>,
// }
//
// const V000: CubeVertex = CubeVertex::new(0, 0, 0);
// const V002: CubeVertex = CubeVertex::new[0, 0, 2];
// const V020: CubeVertex = CubeVertex([0, 2, 0]);
// const V022: CubeVertex = CubeVertex([0, 2, 2]);
// const V200: CubeVertex = CubeVertex([2, 0, 0]);
// const V202: CubeVertex = CubeVertex([2, 0, 2]);
// const V220: CubeVertex = CubeVertex([2, 2, 0]);
// const V222: CubeVertex = CubeVertex([2, 2, 2]);
//
// const CUBE: [CubeVertex; 8] = [V000, V002, V020, V022, V200, V202, V220, V222];
//
// const TETRAHEDRA: [[CubeVertex; 4]; 6] = [
//     [V000, V002, V022, V222],
//     [V000, V002, V202, V222],
//     [V000, V020, V022, V222],
//     [V000, V020, V220, V222],
//     [V000, V200, V202, V222],
//     [V000, V200, V220, V222],
// ];
//
// // impl CubeVertex {
// //     pub fn x(self) -> u8 {
// //         self.0[0]
// //     }
// //     pub fn y(self) -> u8 {
// //         self.0[1]
// //     }
// //     pub fn z(self) -> u8 {
// //         self.0[2]
// //     }
// //     pub fn center(self, other: CubeVertex) -> Self {
// //         let mut result = [0u8; 3];
// //         for i in 0..3 {
// //             let sum = self.0[i] + other.0[i];
// //             assert_eq!(sum % 2, 0);
// //             result[i] = sum / 2;
// //         }
// //         CubeVertex(result)
// //     }
// //     pub fn vec3(self) -> Vec3 {
// //         Vec3::from(self.0.map(|x| x as f64))
// //     }
// //     pub fn into_inner(self) -> [u8; 3] {
// //         self.0
// //     }
// //     pub fn oriented(tri: [Self; 3], out: Self) -> bool {
// //         Triangle3::new(tri.map(|x| x.vec3()))
// //             .plane()
// //             .outside(out.vec3())
// //     }
// //     pub fn new(x: u8, y: u8, z: u8) -> Self {
// //         CubeVertex([x, y, z])
// //     }
// // }
//
// // impl Index<usize> for CubeVertex {
// //     type Output = u8;
// //     fn index(&self, index: usize) -> &Self::Output {
// //         self.0.index(index)
// //     }
// // }
// //
// // impl IndexMut<usize> for CubeVertex {
// //     fn index_mut(&mut self, index: usize) -> &mut Self::Output {
// //         self.0.index_mut(index)
// //     }
// // }
//
// impl CubeMesh {
//     pub fn new_tetr(inside: &[CubeVertex], outside: &[CubeVertex]) -> Self {
//         let mut vertices = vec![];
//         let mut triangles = vec![];
//         match (inside, outside) {
//             ([], [_, _, _, _]) => {}
//             (&[i1], &[mut o1, mut o2, o3]) => {
//                 if CubeVertex::oriented([o1, o2, o3], i1) {
//                     mem::swap(&mut o1, &mut o2);
//                 }
//                 vertices.push(i1.center(o1));
//                 vertices.push(i1.center(o2));
//                 vertices.push(i1.center(o3));
//                 triangles.push(MeshTriangle::new(0, 1, 2));
//             }
//             (&[i1, i2], &[mut o1, mut o2]) => {
//                 if CubeVertex::oriented([i1, i2, o2], o1) {
//                     mem::swap(&mut o1, &mut o2);
//                 }
//                 vertices.push(i1.center(o1));
//                 vertices.push(i1.center(o2));
//                 vertices.push(i2.center(o2));
//                 vertices.push(i2.center(o1));
//                 triangles.push(MeshTriangle::new(0, 1, 2));
//                 triangles.push(MeshTriangle::new(0, 2, 3));
//             }
//             (&[mut i1, mut i2, i3], &[o1]) => {
//                 if !CubeVertex::oriented([i1, i2, i3], o1) {
//                     mem::swap(&mut i1, &mut i2);
//                 }
//                 vertices.push(o1.center(i1));
//                 vertices.push(o1.center(i2));
//                 vertices.push(o1.center(i3));
//                 triangles.push(MeshTriangle::new(0, 1, 2));
//             }
//             ([_, _, _, _], []) => {}
//             _ => unreachable!(),
//         }
//         CubeMesh {
//             vertices,
//             triangles,
//         }
//     }
//     fn new_cube(inside: &[CubeVertex]) -> Self {
//         let mut tetrs = vec![];
//         for tetrahedron in [
//             TETRAHEDRA[0],
//             TETRAHEDRA[1],
//             TETRAHEDRA[2],
//             TETRAHEDRA[3],
//             TETRAHEDRA[4],
//             TETRAHEDRA[5],
//         ] {
//             let mut in_tetr = vec![];
//             let mut out_tetr = vec![];
//             for vertex in tetrahedron {
//                 if inside.contains(&vertex) {
//                     in_tetr.push(vertex);
//                 } else {
//                     out_tetr.push(vertex);
//                 }
//             }
//             tetrs.push(CubeMesh::new_tetr(&in_tetr, &out_tetr));
//         }
//         let mut vertices = vec![];
//         let mut vertex_table = HashMap::new();
//         let mut triangles = vec![];
//         for tetr in tetrs {
//             let mut tetr_vertices = vec![];
//             for vertex in tetr.vertices {
//                 let index = *vertex_table.entry(vertex).or_insert_with(|| {
//                     let index = vertices.len();
//                     vertices.push(vertex);
//                     index
//                 });
//                 tetr_vertices.push(index);
//             }
//             for tri in tetr.triangles {
//                 triangles.push(MeshTriangle::from(tri.vertices().map(|x| tetr_vertices[x])));
//             }
//         }
//         assert_eq!(vertices.is_empty(), triangles.is_empty());
//         CubeMesh {
//             vertices,
//             triangles,
//         }
//     }
//     pub fn vertices(&self) -> &[CubeVertex] {
//         &self.vertices
//     }
//     pub fn triangles(&self) -> &[MeshTriangle] {
//         &self.triangles
//     }
// }
//
// static MARCHING_CUBES: LazyLock<Vec<CubeMesh>> = LazyLock::new(|| {
//     let mut result = vec![];
//     for i in 0..256 {
//         let mut verts = vec![];
//         for v in 0..8 {
//             if (i >> v) & 1 == 1 {
//                 verts.push(CUBE[v]);
//             }
//         }
//         result.push(CubeMesh::new_cube(&verts));
//     }
//     result
// });
//
// pub fn marching_cube(verts: [bool; 8]) -> &'static CubeMesh {
//     let mut index = 0;
//     for i in 0..8 {
//         if verts[i] {
//             index |= 1 << i;
//         }
//     }
//     &MARCHING_CUBES[index]
// }
