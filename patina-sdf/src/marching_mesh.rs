use itertools::Itertools;
// use patina_calc::{EvalVisitor, Expr, ExprProgramBuilder, Program, ProgramVisit, Solver};
use crate::octree::{Octree, OctreeBranch, OctreePath, OctreeView, OctreeViewMut};
use crate::sdf::Sdf;
use crate::transvoxel::cube_edge::{CubeEdge, CubeEdgeSet};
use crate::transvoxel::cube_face::{CubeFace, CubeFaceSet};
use crate::transvoxel::cube_input::CubeInput;
use crate::transvoxel::cube_triangle::CubeTriMesh;
use crate::transvoxel::cube_vertex;
use crate::transvoxel::cube_vertex::{CubeVertex, CubeVertexSet, cube_corners, cube_points};
use inari::DecInterval;
use ordered_float::NotNan;
use patina_geo::geo3::aabb::Aabb;
use patina_mesh::mesh::Mesh;
use patina_mesh::mesh_triangle::MeshTriangle;
use patina_scalar::deriv::Deriv;
use patina_scalar::newton::Newton;
use patina_vec::vec3::Vec3;
use patina_vec::vector3::Vector3;
use std::collections::{HashMap, HashSet};
use std::mem;
// #[derive(Debug)]
// struct OctreeCube {
//     min: [usize; 3],
//     dim: usize,
// }
//
// impl OctreeCube {
//     pub fn new(min: [usize; 3], dim: usize) -> OctreeCube {
//         assert!(dim.is_power_of_two());
//         OctreeCube { min, dim }
//     }
//     pub fn min(&self) -> [usize; 3] {
//         self.min
//     }
//     pub fn max(&self) -> [usize; 3] {
//         self.min.map(|x| x + self.dim)
//     }
//
//     pub fn dim(&self) -> usize {
//         self.dim
//     }
//     pub fn subcubes(&self) -> Option<([usize; 3], [Self; 8])> {
//         if self.dim == 2 {
//             return None;
//         }
//         let dim2 = self.dim / 2;
//         let mut subcubes = ArrayVec::new();
//         for x in 0..2 {
//             for y in 0..2 {
//                 for z in 0..2 {
//                     subcubes.push(OctreeCube::new(
//                         [
//                             self.min[0] + x * dim2,
//                             self.min[1] + y * dim2,
//                             self.min[2] + z * dim2,
//                         ],
//                         dim2,
//                     ));
//                 }
//             }
//         }
//         Some((
//             [self.min[0] + dim2, self.min[1] + dim2, self.min[2] + dim2],
//             subcubes.into_inner().ok().unwrap(),
//         ))
//     }
// }

#[derive(Debug)]
struct MarchingNode {}
pub struct MarchingMesh {
    min_render_depth: usize,
    max_render_depth: usize,
    subdiv_max_dot: f64,
    aabb: Aabb,
    vertices: Vec<Vec3>,
    vertex_table: HashMap<Vector3<NotNan<f64>>, usize>,
    triangles: Vec<MeshTriangle>,
    triangle_set: HashSet<[usize; 3]>,
}

impl Default for MarchingNode {
    fn default() -> Self {
        MarchingNode {}
    }
}

impl MarchingNode {}

impl MarchingMesh {
    pub fn new(aabb: Aabb) -> Self {
        Self {
            min_render_depth: 2,
            max_render_depth: 10,
            subdiv_max_dot: 0.9,
            aabb,
            vertices: vec![],
            vertex_table: HashMap::new(),
            triangles: vec![],
            triangle_set: HashSet::new(),
        }
    }
    // fn position(&self, ints: [usize; 3]) -> Vec3 {
    //     let dimensions = self.aabb.dimensions();
    //     (0..3)
    //         .map(|axis| {
    //             (ints[axis] as f64) / (self.render_dim as f64) * dimensions[axis]
    //                 + self.aabb.min()[axis]
    //         })
    //         .collect()
    // }
    // fn aabb(&self, cube: &OctreeCube) -> Aabb {
    //     Aabb::new(self.position(cube.min()), self.position(cube.max()))
    // }
    // fn evaluate(&mut self, ints: [usize; 3], sdf: &Sdf) -> f64 {
    //     let position = self.position(ints);
    //     sdf.evaluate(position)
    // }
    // fn build_cube(&mut self, cube: OctreeCube, sdf: &Sdf) {
    //     let aabb = self.aabb(&cube);
    //     let aabb_intervals: Vector3<DecInterval> = (0..3)
    //         .map(|axis| DecInterval::try_from((aabb.min()[axis], aabb.max()[axis])).unwrap())
    //         .collect();
    //     let (sdf2, range) = sdf.evaluate_constrain(aabb_intervals);
    //     if !range.contains(0.0) {
    //         return;
    //     }
    //     let sdf = sdf2.unwrap_or(sdf.clone());
    //     if let Some((center, subcubes)) = cube.subcubes() {
    //         let radius = self.evaluate(center, &sdf).abs();
    //         let min_radius = aabb.dimensions().length() / 2.0 + 10e-5;
    //         if radius <= min_radius {
    //             for cube in subcubes {
    //                 self.build_cube(cube, &sdf);
    //             }
    //         }
    //     } else {
    //         let mut bits = ArrayVec::new();
    //         let [x, y, z] = cube.min();
    //         for dx in 0..2 {
    //             for dy in 0..2 {
    //                 for dz in 0..2 {
    //                     let ints = [x + dx * 2, y + dy * 2, z + dz * 2];
    //                     let d = self.evaluate(ints, &sdf);
    //                     bits.push(d >= 0.0);
    //                 }
    //             }
    //         }
    //         let mcube = marching_cube(bits.into_inner().unwrap());
    //         let mut cube_vertices = vec![];
    //         for vertex in mcube.vertices() {
    //             cube_vertices.push(self.add_vertex(
    //                 [
    //                     x + vertex.x() as usize,
    //                     y + vertex.y() as usize,
    //                     z + vertex.z() as usize,
    //                 ],
    //                 &sdf,
    //             ));
    //         }
    //         for triangle in mcube.triangles() {
    //             self.triangles.push(MeshTriangle::from(
    //                 triangle.vertices().map(|v| cube_vertices[v]),
    //             ));
    //         }
    //     }
    // }
    // fn add_vertex(&mut self, ints: [usize; 3], sdf: &Sdf) -> usize {
    //     let position = self.position(ints);
    //     *self.vertex_table.entry(ints).or_insert_with(|| {
    //         let position_deriv = |t| {
    //             let mut inputs = Vector3::splat(Deriv::nan());
    //             for axis in 0..3 {
    //                 if ints[axis] % 2 == 0 {
    //                     inputs[axis] = Deriv::constant(position[axis]);
    //                 } else {
    //                     inputs[axis] = Deriv::constant(position[axis])
    //                         + Deriv::variable(t, 0)
    //                             * Deriv::constant(
    //                                 self.aabb.dimensions()[axis] / (self.render_dim as f64),
    //                             );
    //                 }
    //             }
    //             inputs
    //         };
    //         let lsdf = |t| sdf.evaluate_deriv1(position_deriv(t));
    //         let t = Newton::new().solve(-1.0..1.0, lsdf);
    //         let t = if let Some(t) = t {
    //             t.into_inner()
    //         } else {
    //             println!("Cannot solve {:?}", ints);
    //             0.5
    //         };
    //         assert!(t >= -1.0 && t <= 1.0, "{:?}", t);
    //         let t = t.clamp(-0.99, 0.99);
    //         let position = position_deriv(t).map(|x| x.value());
    //         let index = self.vertices.len();
    //         self.vertices.push(position);
    //         index
    //     })
    // }
    // pub fn build(mut self, sdf: &Sdf) -> Mesh {
    //     self.build_cube(OctreeCube::new([0, 0, 0], self.render_dim), sdf);
    //     Mesh::new(self.vertices, self.triangles)
    // }
    fn find_marching_cube(&self, aabb: &Aabb, sdf: &Sdf) -> CubeTriMesh {
        let mut result = CubeVertexSet::new();
        for cv in cube_corners() {
            let p = (0..3)
                .map(|axis| {
                    if cv[axis] > 0 {
                        aabb.max()[axis]
                    } else {
                        aabb.min()[axis]
                    }
                })
                .collect();
            let d = sdf.evaluate(p);
            result[cv] = d >= 0.0;
        }
        CubeInput::new(CubeFaceSet::new(), CubeEdgeSet::new(), result).as_mesh()
    }
    fn is_divided(&self, octree: &Octree<MarchingNode>, path: &OctreePath) -> bool {
        if let Some((index, path)) = path.view() {
            match octree.view() {
                OctreeView::Leaf(leaf) => false,
                OctreeView::Branch(branch) => self.is_divided(&branch[index], &path),
            }
        } else {
            match octree.view() {
                OctreeView::Leaf(_) => false,
                OctreeView::Branch(_) => true,
            }
        }
    }
    fn add_marching_cube_sub(
        &mut self,
        root: &Octree<MarchingNode>,
        octree: &Octree<MarchingNode>,
        aabb: &Aabb,
        sdf: &Sdf,
    ) {
        let mut faces = CubeFaceSet::new();
        for face in CubeFace::all() {
            let mut is_divided;
            if let Some(neighbor) = octree.path().face_adjacent_for(face) {
                is_divided = self.is_divided(root, &neighbor);
            } else {
                is_divided = false;
            }
            faces[face] = is_divided;
        }
        let mut edges = CubeEdgeSet::new();
        for edge in CubeEdge::all() {
            let mut is_divided;
            if let Some(neighbor) = octree.path().edge_adjacent_for(edge) {
                is_divided = self.is_divided(root, &neighbor);
            } else {
                is_divided = false;
            }
            edges[edge] = is_divided;
        }

        let mut to_sample = CubeVertexSet::corners();
        faces.add_samples_to(&mut to_sample);
        edges.add_samples_to(&mut to_sample);
        if to_sample != CubeVertexSet::new() {
            to_sample[cube_vertex::cube_center()] |= true;
        }
        let mut vertices = CubeVertexSet::new();
        for cv in cube_points() {
            if to_sample[cv] {
                let v = self.path_position(octree.path(), cv);
                // let numer = octree.path().position() * 2 + cv.map(|x| x as usize);
                // let denom = (2 << octree.path().depth()) as f64;
                // let v = numer.map(|x| x as f64) / denom;
                // let v = self.aabb.min() + v.mul_elements(self.aabb.dimensions());
                // let v: Vec3 = (0..3)
                //     .map(|axis| {
                //
                //         let start = aabb.min()[axis];
                //         let end = aabb.max()[axis];
                //         match cv[axis] {
                //             0 => start,
                //             1 => (start + end) / 2.0,
                //             2 => end,
                //             _ => unreachable!(),
                //         }
                //     })
                //     .collect();
                let eval = sdf.evaluate(v);
                vertices[cv] = eval >= 0.0;
            }
        }
        let transvoxel = CubeInput::new(faces, edges, vertices);
        let mesh2 = transvoxel.as_mesh();

        for tri in mesh2.triangles() {
            let tri_verts = tri
                .vertices()
                .map(|(v1, v2)| self.add_vertex(octree.path(), v1, v2));
            let tri2 = MeshTriangle::from(tri_verts);
            let mut vs = tri2.vertices();
            vs.sort();
            assert!(self.triangle_set.insert(vs));
            self.triangles.push(tri2);
        }
    }
    fn path_position(&self, path: &OctreePath, cv: CubeVertex) -> Vec3 {
        let numer = path.position() * 2 + cv.map(|x| x as usize);
        let denom = (2 << path.depth()) as f64;
        let v = numer.map(|x| x as f64) / denom;
        let v = self.aabb.min() + v.mul_elements(self.aabb.dimensions());
        v
    }
    fn add_vertex(&mut self, path: &OctreePath, mut v1: CubeVertex, mut v2: CubeVertex) -> usize {
        if v2 < v1 {
            mem::swap(&mut v1, &mut v2);
        }
        let v1 = self.path_position(path, v1);
        let v2 = self.path_position(path, v2);
        let v = (v1 + v2) / 2.0;
        let vnn = v.map(|x| NotNan::new(x).unwrap());
        *self.vertex_table.entry(vnn).or_insert_with(|| {
            let index = self.vertices.len();
            self.vertices.push(v);
            index
        })
    }
    fn build_octree(&mut self, tree: &mut Octree<MarchingNode>, sdf: &Sdf) {
        let aabb = tree.path().aabb_inside(&self.aabb);
        let aabb_intervals: Vector3<DecInterval> = (0..3)
            .map(|axis| DecInterval::try_from((aabb.min()[axis], aabb.max()[axis])).unwrap())
            .collect();
        let (sdf2, range) = sdf.evaluate_constrain(aabb_intervals);
        let sdf = sdf2.unwrap_or(sdf.clone());
        if !range.contains(0.0) {
            return;
        }
        if tree.path().depth() < self.min_render_depth {
            self.build_branch(tree, &sdf);
            return;
        }
        if tree.path().depth() >= self.max_render_depth {
            return;
        }
        let mcube = self.find_marching_cube(&aabb, &sdf);
        let vertices = mcube
            .triangles()
            .iter()
            .flat_map(|t| t.vertices().into_iter().cloned())
            .collect::<HashSet<_>>();
        let mut normals = vec![];
        for (v1, v2) in vertices {
            let (_, normal) = self.find_vertex(&aabb, v1, v2, &sdf);
            normals.push(normal);
        }
        if normals
            .iter()
            .tuple_combinations()
            .any(|(n1, n2)| n1.dot(*n2) < self.subdiv_max_dot)
        {
            self.build_branch(tree, &sdf);
        }
    }
    fn position(&self, aabb: &Aabb, v: CubeVertex) -> Vec3 {
        (0..3)
            .map(|axis| match v[axis] {
                0 => aabb.min()[axis],
                1 => (aabb.min()[axis] + aabb.max()[axis]) / 2.0,
                2 => aabb.max()[axis],
                _ => unreachable!(),
            })
            .collect()
    }
    fn position_range(&self, aabb: &Aabb, v1: CubeVertex, v2: CubeVertex) -> (Vec3, Vec3) {
        (self.position(aabb, v1), self.position(aabb, v2))
        // (
        //     (0..3)
        //         .map(|axis| match vertex[axis] {
        //             0 => aabb.min()[axis],
        //             1 => aabb.min()[axis],
        //             2 => aabb.max()[axis],
        //             _ => unreachable!(),
        //         })
        //         .collect_array()
        //         .unwrap()
        //         .into(),
        //     (0..3)
        //         .map(|axis| match vertex[axis] {
        //             0 => aabb.min()[axis],
        //             1 => aabb.max()[axis],
        //             2 => aabb.max()[axis],
        //             _ => unreachable!(),
        //         })
        //         .collect_array()
        //         .unwrap()
        //         .into(),
        // )
    }
    fn find_vertex(
        &mut self,
        aabb: &Aabb,
        v1: CubeVertex,
        v2: CubeVertex,
        sdf: &Sdf,
    ) -> (Vec3, Vec3) {
        let (min, max) = self.position_range(aabb, v1, v2);
        let range = max - min;
        let lsdf = |t| {
            sdf.evaluate_deriv1(
                min.map(Deriv::constant) + range.map(Deriv::constant) * Deriv::variable(t, 0),
            )
        };
        let t = Newton::new().solve(0.0..1.0, lsdf).unwrap().into_inner();
        assert!(t >= -1.0 && t <= 1.0, "{:?}", t);
        let vertex_position = min + range * t.clamp(-0.99, 0.99);
        let eval_position = min + range * t;
        let normal: Vec3 = sdf.normal(eval_position);
        (vertex_position, normal)
    }
    // fn add_vertex(&mut self, aabb: &Aabb, vertex: CubeVertex, sdf: &Sdf) {
    //     let position_deriv = |t| {
    //         (0..3)
    //             .map(|axis| match vertex.into_inner()[axis] {
    //                 0 => todo!(),
    //                 1 => todo!(),
    //                 2 => todo!(),
    //                 _ => unreachable!(),
    //             })
    //             .collect_array()
    //             .unwrap()
    //             .into();
    //         let mut inputs = Vector3::splat(Deriv::nan());
    //         for axis in 0..3 {
    //             if ints[axis] % 2 == 0 {
    //                 inputs[axis] = Deriv::constant(position[axis]);
    //             } else {
    //                 inputs[axis] = Deriv::constant(position[axis])
    //                     + Deriv::variable(t, 0)
    //                         * Deriv::constant(
    //                             self.aabb.dimensions()[axis] / (self.render_dim as f64),
    //                         );
    //             }
    //         }
    //         inputs
    //     };
    //     let lsdf = |t| sdf.evaluate_deriv1(position_deriv(t));
    //     let t = Newton::new().solve(-1.0..1.0, lsdf);
    //     let t = if let Some(t) = t {
    //         t.into_inner()
    //     } else {
    //         println!("Cannot solve {:?}", ints);
    //         0.5
    //     };
    //     assert!(t >= -1.0 && t <= 1.0, "{:?}", t);
    //     let t = t.clamp(-0.99, 0.99);
    //     let position = position_deriv(t).map(|x| x.value());
    //     let index = self.vertices.len();
    //     self.vertices.push(position);
    //     index
    // }
    fn build_branch(&mut self, tree: &mut Octree<MarchingNode>, sdf: &Sdf) {
        match tree.view_mut() {
            OctreeViewMut::Leaf(_) => {
                tree.set_branch(Default::default());
            }
            OctreeViewMut::Branch(_) => {}
        }
        let branch = match tree.view_mut() {
            OctreeViewMut::Leaf(_) => unreachable!(),
            OctreeViewMut::Branch(branch) => branch,
        };
        for child in branch.children_flat_mut() {
            self.build_octree(child, sdf);
        }
    }

    fn build_mesh(&mut self, root: &Octree<MarchingNode>, tree: &Octree<MarchingNode>, sdf: &Sdf) {
        match tree.view() {
            OctreeView::Leaf(_) => {
                self.build_mesh_leaf(root, tree, sdf);
            }
            OctreeView::Branch(branch) => {
                self.build_mesh_branch(root, branch, sdf);
            }
        }
    }

    fn build_mesh_branch(
        &mut self,
        root: &Octree<MarchingNode>,
        branch: &OctreeBranch<MarchingNode>,
        sdf: &Sdf,
    ) {
        for child in branch.children_flat() {
            self.build_mesh(root, child, sdf);
        }
    }

    fn build_mesh_leaf(
        &mut self,
        root: &Octree<MarchingNode>,
        tree: &Octree<MarchingNode>,
        sdf: &Sdf,
    ) {
        let aabb = tree.path().aabb_inside(&self.aabb);
        self.add_marching_cube_sub(root, tree, &aabb, sdf);
        // let mut vertex_ids = vec![];
        // for vertex in mcube.vertices() {
        //     let (vertex_position, _) = self.find_vertex(&aabb, *vertex, sdf);
        //     vertex_ids.push(self.vertices.len());
        //     self.vertices.push(vertex_position);
        // }
        // for tri in mcube.triangles() {
        //     self.triangles
        //         .push(MeshTriangle::from(tri.vertices().map(|v| vertex_ids[v])));
        // }
    }

    fn get_neighbors(
        &mut self,
        octree: &Octree<MarchingNode>,
        depth: usize,
        neighbors: &mut HashSet<OctreePath>,
    ) {
        match octree.view() {
            OctreeView::Leaf(_) => {
                if octree.path().depth() == depth {
                    for neighbor in octree.path().face_adjacent() {
                        neighbors.insert(neighbor);
                    }
                    for neighbor in octree.path().edge_adjacent() {
                        neighbors.insert(neighbor);
                    }
                }
            }
            OctreeView::Branch(branch) => {
                for child in branch.children_flat() {
                    self.get_neighbors(child, depth, neighbors);
                }
            }
        }
    }
    fn refine_path(&mut self, octree: &mut Octree<MarchingNode>, path: OctreePath) {
        if path.depth() == 1 {
            return;
        }
        if let Some((index, path)) = path.view() {
            match octree.view_mut() {
                OctreeViewMut::Leaf(_) => octree.set_branch(Default::default()),
                OctreeViewMut::Branch(branch) => {}
            }
            match octree.view_mut() {
                OctreeViewMut::Leaf(_) => unreachable!(),
                OctreeViewMut::Branch(branch) => self.refine_path(branch.child_mut(index), path),
            }
        }
    }

    fn refine_neighbors(&mut self, octree: &mut Octree<MarchingNode>) {
        for depth in (0..=self.max_render_depth).rev() {
            let mut to_refine = HashSet::new();
            self.get_neighbors(octree, depth, &mut to_refine);
            for path in to_refine {
                self.refine_path(octree, path);
            }
        }
    }

    pub fn build(mut self, sdf: &Sdf) -> Mesh {
        let mut octree = Octree::<MarchingNode>::new_root(MarchingNode::default());
        self.build_octree(&mut octree, sdf);
        self.refine_neighbors(&mut octree);
        self.build_mesh(&octree, &octree, sdf);
        Mesh::new(self.vertices, self.triangles)
    }
}
