use crate::geo3::aabb::AABB;
use crate::geo3::plane::Plane;
use crate::geo3::ray3::Ray3;
use crate::math::disjoint_paths::DisjointPaths;
use crate::math::float_bool::Epsilon;
use crate::math::loop_builder::LoopBuilder;
use crate::math::vec2::Vec2;
use crate::math::vec3::Vec3;
use crate::meshes::bvh::Bvh;
use crate::meshes::intersect_bvh_bvh::{
    IntersectBvhBvh, MeshIntersect, MeshIntersectDescriptor, MeshIntersectVertex,
};
use crate::meshes::mesh::Mesh;
use crate::meshes::mesh_edge::MeshEdge;
use crate::meshes::mesh_polygon::MeshPolygon;
use crate::meshes::mesh_triangle::MeshTriangle;
use crate::meshes::ordered_mesh_edge::OrderedMeshEdge;
use crate::meshes::triangulation::Triangulation;
use crate::ser::stl::write_stl_file;
use arrayvec::ArrayVec;
use itertools::Itertools;
use ordered_float::NotNan;
use rand::prelude::SliceRandom;
use rand::{Rng, SeedableRng, rng};
use rand_xorshift::XorShiftRng;
use std::collections::{HashMap, HashSet};
use std::mem;
use std::path::PathBuf;

#[derive(Debug)]
struct NewFace {
    edge_vertices: [Vec<usize>; 3],
    internal_vertices: Vec<usize>,
    border_vertices: Vec<usize>,
    edges: HashSet<MeshEdge>,
}

#[derive(Debug)]
enum VertexOrigin {
    Mesh,
    Intersect { int: MeshIntersectVertex },
    Removed,
}

#[derive(Copy, Clone, Eq, Ord, PartialEq, PartialOrd, Hash, Debug)]
enum VertexPosition {
    Edge(MeshEdge),
    Tri(usize),
}

pub struct Bimesh<'a> {
    eps: Epsilon,
    meshes: [&'a Mesh; 2],
    bvhs: [Bvh; 2],
    vertices: Vec<Vec3>,
    vertex_origins: Vec<VertexOrigin>,
    vertex_positions: VertexPositionTable,
    input_tris: [Vec<MeshTriangle>; 2],
    input_wings: HashMap<MeshEdge, BimeshWings>,
    all_fans: [HashMap<usize, Vec<usize>>; 2],
    intersects: Vec<MeshIntersect>,
    new_vertices: HashMap<usize, MeshIntersectVertex>,
    new_vertices_by_tri_pair: HashMap<[usize; 2], ArrayVec<usize, 2>>,
    strong_edges: HashSet<OrderedMeshEdge>,
    weak_edges: Vec<OrderedMeshEdge>,
    loops: Vec<Vec<usize>>,
    forward_loop_adjacency: HashMap<usize, usize>,
    reverse_loop_adjacency: HashMap<usize, usize>,
    loop_ids: HashMap<usize, usize>,
    new_faces: [Vec<NewFace>; 2],
    new_tris: [Vec<MeshTriangle>; 2],
    new_tri_ccs: [Vec<Vec<MeshTriangle>>; 2],
    new_tri_cats: [[Vec<MeshTriangle>; 2]; 2],
}

#[derive(Debug)]
pub struct BimeshWings {
    mesh: usize,
    wing_tris: [usize; 2],
}

struct VertexPositionTable {
    vertex_positions: [HashMap<usize, VertexPosition>; 2],
}

impl VertexPositionTable {
    pub fn add_vertex_position(&mut self, mesh: usize, vertex: usize, new: VertexPosition) {
        if let Some(old) = self.vertex_positions[mesh].insert(vertex, new) {
            assert_eq!(old, new);
        }
    }
}

impl<'a> Bimesh<'a> {
    fn new_uninit(eps: Epsilon, mesh1: &'a Mesh, mesh2: &'a Mesh) -> Self {
        let meshes = [mesh1, mesh2];
        Bimesh {
            eps,
            bvhs: meshes.map(Bvh::from_mesh),
            vertices: vec![],
            vertex_origins: vec![],
            meshes,
            input_tris: [vec![], vec![]],
            input_wings: HashMap::new(),
            loops: vec![],
            intersects: vec![],
            new_vertices: HashMap::new(),
            vertex_positions: VertexPositionTable {
                vertex_positions: [HashMap::new(), HashMap::new()],
            },
            all_fans: [HashMap::new(), HashMap::new()],
            new_vertices_by_tri_pair: Default::default(),
            strong_edges: HashSet::new(),
            weak_edges: vec![],
            forward_loop_adjacency: HashMap::new(),
            reverse_loop_adjacency: HashMap::new(),
            loop_ids: HashMap::new(),
            new_faces: [const { vec![] }; 2],
            new_tris: [const { vec![] }; 2],
            new_tri_ccs: [const { vec![] }; 2],
            new_tri_cats: [const { [const { vec![] }; 2] }; 2],
        }
    }
    pub fn new(mesh1: &'a Mesh, mesh2: &'a Mesh, eps: Epsilon, rng: &mut impl Rng) -> Self {
        let mut this = Self::new_uninit(eps, mesh1, mesh2);
        this.build(rng);
        this
    }
    pub fn build_relabel(&mut self) {
        let mut offset = 0;
        for mesh in 0..2 {
            for (ti, tri) in self.meshes[mesh].triangles().iter().enumerate() {
                let new_tri = MeshTriangle::from(tri.vertices().map(|n| n + offset));
                self.input_tris[mesh].push(new_tri);
                // for v in tri.vertices() {
                //     self.input_vertex_to_tris[mesh]
                //         .entry(v)
                //         .or_default()
                //         .insert(ti);
                // }
            }
            for v in self.meshes[mesh].vertices() {
                self.vertices.push(*v);
                self.vertex_origins.push(VertexOrigin::Mesh);
                offset += 1;
            }
        }
    }
    pub fn check_degenerate(&self) {
        for mesh in 0..2 {
            for tri in &self.input_tris[mesh] {
                assert!(tri.for_vertices(&self.vertices).area() > self.eps.value());
            }
        }
    }
    pub fn build_input_wings(&mut self) {
        for mesh in 0..2 {
            let mut wing_builders = HashMap::new();
            for (ti, tri) in self.input_tris[mesh].iter().enumerate() {
                for e in tri.edges() {
                    wing_builders
                        .entry(e)
                        .or_insert_with(ArrayVec::<_, 2>::new)
                        .push(ti);
                }
            }
            for (edge, wing) in wing_builders {
                let wing = wing.into_inner().unwrap();
                self.input_wings.insert(
                    edge,
                    BimeshWings {
                        mesh,
                        wing_tris: wing,
                    },
                );
            }
        }
    }
    pub fn build_new_faces(&mut self) {
        for mesh in 0..2 {
            for tri in self.input_tris[mesh].iter() {
                self.new_faces[mesh].push(NewFace {
                    edge_vertices: [const { vec![] }; 3],
                    internal_vertices: vec![],
                    border_vertices: vec![],
                    edges: HashSet::new(),
                });
            }
        }
    }
    pub fn build_intersects(&mut self) {
        let mut intersect = IntersectBvhBvh::new(self.eps);
        intersect.intersect_node_node(
            &self.bvhs[0].root_view(&self.input_tris[0], &self.vertices),
            &self.bvhs[1].root_view(&self.input_tris[1], &self.vertices),
        );
        self.intersects = intersect.build();
    }
    pub fn build_new_vertices(&mut self) {
        let mut by_descriptor = HashMap::new();
        let mut merge = HashMap::new();
        for int in &self.intersects {
            let mut vs = ArrayVec::<_, 2>::new();
            for int_vertex in &int.vertices {
                match &int_vertex.descriptor {
                    MeshIntersectDescriptor::VertexVertex { vertices } => {
                        self.vertex_origins[vertices[1]] = VertexOrigin::Removed;
                        merge.insert(vertices[1], vertices[0]);
                        vs.push(vertices[0]);
                        continue;
                    }
                    MeshIntersectDescriptor::VertexEdge {
                        vertex_mesh,
                        vertex,
                        edge_mesh,
                        edge,
                    } => {
                        self.vertex_positions.add_vertex_position(
                            *edge_mesh,
                            *vertex,
                            VertexPosition::Edge(*edge),
                        );
                        vs.push(*vertex);
                        continue;
                    }
                    MeshIntersectDescriptor::EdgeEdge { .. } => {}
                    MeshIntersectDescriptor::VertexTriangle {
                        vertex_mesh,
                        vertex,
                        tri_mesh,
                        tri,
                    } => {
                        self.vertex_positions.add_vertex_position(
                            *tri_mesh,
                            *vertex,
                            VertexPosition::Tri(*tri),
                        );
                        vs.push(*vertex);
                        continue;
                    }
                    MeshIntersectDescriptor::EdgeTriangle { .. } => {}
                }
                vs.push(
                    *by_descriptor
                        .entry(int_vertex.descriptor.clone())
                        .or_insert_with(|| {
                            let vertex = self.vertices.len();
                            self.vertices.push(int_vertex.position);
                            self.vertex_origins.push(VertexOrigin::Intersect {
                                int: int_vertex.clone(),
                            });
                            self.new_vertices.insert(vertex, int_vertex.clone());
                            match &int_vertex.descriptor {
                                MeshIntersectDescriptor::VertexVertex { .. } => {}
                                MeshIntersectDescriptor::VertexEdge { .. } => {}
                                MeshIntersectDescriptor::EdgeEdge { edges } => {
                                    self.vertex_positions.add_vertex_position(
                                        0,
                                        vertex,
                                        VertexPosition::Edge(edges[0]),
                                    );
                                    self.vertex_positions.add_vertex_position(
                                        1,
                                        vertex,
                                        VertexPosition::Edge(edges[1]),
                                    );
                                }
                                MeshIntersectDescriptor::VertexTriangle { .. } => {}
                                MeshIntersectDescriptor::EdgeTriangle {
                                    edge_mesh,
                                    edge,
                                    tri_mesh,
                                    tri,
                                } => {
                                    self.vertex_positions.add_vertex_position(
                                        *edge_mesh,
                                        vertex,
                                        VertexPosition::Edge(*edge),
                                    );
                                    self.vertex_positions.add_vertex_position(
                                        *tri_mesh,
                                        vertex,
                                        VertexPosition::Tri(*tri),
                                    );
                                }
                            }
                            vertex
                        }),
                );
            }
            self.new_vertices_by_tri_pair.insert(int.tris, vs.clone());
        }
        for mesh in 0..2 {
            for tri in &mut self.input_tris[mesh] {
                for vertex in tri.vertices_mut() {
                    if let Some(merged) = merge.get(vertex) {
                        *vertex = *merged;
                    }
                }
            }
            for (vertex, pos) in self.vertex_positions.vertex_positions[mesh].iter_mut() {
                match pos {
                    VertexPosition::Edge(e) => {
                        let mut e2 = e.vertices();
                        for v in &mut e2 {
                            if let Some(v2) = merge.get(v) {
                                *v = *v2;
                            }
                        }
                        *e = MeshEdge::from(e2);
                    }
                    VertexPosition::Tri(t) => {}
                }
            }
        }
        for origin in &mut self.vertex_origins {
            match origin {
                VertexOrigin::Mesh => {}
                VertexOrigin::Intersect { int } => match &mut int.descriptor {
                    MeshIntersectDescriptor::VertexVertex { vertices } => {
                        for v in vertices {
                            if let Some(v2) = merge.get(v) {
                                *v = *v2;
                            }
                        }
                    }
                    MeshIntersectDescriptor::VertexEdge {
                        vertex_mesh,
                        vertex,
                        edge_mesh,
                        edge,
                    } => {}
                    MeshIntersectDescriptor::EdgeEdge { edges } => {
                        for edge in edges {
                            let mut e2 = edge.vertices();
                            for v in &mut e2 {
                                if let Some(v2) = merge.get(v) {
                                    *v = *v2;
                                }
                            }
                            *edge = MeshEdge::from(e2);
                        }
                    }
                    MeshIntersectDescriptor::VertexTriangle {
                        vertex_mesh,
                        vertex,
                        tri_mesh,
                        tri,
                    } => {}
                    MeshIntersectDescriptor::EdgeTriangle {
                        edge_mesh,
                        edge,
                        tri_mesh,
                        tri,
                    } => {
                        let mut e2 = edge.vertices();
                        for v in &mut e2 {
                            if let Some(v2) = merge.get(v) {
                                *v = *v2;
                            }
                        }
                        *edge = MeshEdge::from(e2);
                    }
                },
                VertexOrigin::Removed => {}
            }
        }
    }
    // fn intersect_sets(s1: &HashSet<usize>, s2: &HashSet<usize>) -> HashSet<usize> {
    //     let mut s3 = HashSet::new();
    //     for v1 in s1 {
    //         if s2.contains(&v1) {
    //             s3.insert(*v1);
    //         }
    //     }
    //     s3
    // }
    // fn input_tris_for_input_vertex(&self, mesh: usize, v: usize) -> HashSet<usize> {
    //     self.input_vertex_to_tris[mesh]
    //         .get(&v)
    //         .cloned()
    //         .unwrap_or_default()
    // }
    // fn input_tris_for_input_edge(&self, mesh: usize, edge: MeshEdge) -> HashSet<usize> {
    //     self.input_wings[&edge].wing_tris.into_iter().collect()
    // }
    // fn input_tris_for_vertex(&self, mesh: usize, v: usize) -> HashSet<usize> {
    //     match &self.vertex_origins[v] {
    //         VertexOrigin::Mesh => self.input_tris_for_input_vertex(mesh, v).clone(),
    //         VertexOrigin::Intersect { int } => match int.descriptor {
    //             MeshIntersectDescriptor::VertexVertex { vertices } => self
    //                 .input_tris_for_input_vertex(mesh, vertices[mesh])
    //                 .clone(),
    //             MeshIntersectDescriptor::VertexEdge {
    //                 vertex_mesh,
    //                 vertex,
    //                 edge_mesh,
    //                 edge,
    //             } => {
    //                 if mesh == vertex_mesh {
    //                     self.input_tris_for_input_vertex(mesh, vertex).clone()
    //                 } else {
    //                     self.input_tris_for_input_edge(mesh, edge)
    //                 }
    //             }
    //             MeshIntersectDescriptor::EdgeEdge { edges } => {
    //                 self.input_tris_for_input_edge(mesh, edges[mesh])
    //             }
    //             MeshIntersectDescriptor::VertexTriangle {
    //                 vertex_mesh,
    //                 vertex,
    //                 tri_mesh,
    //                 tri,
    //             } => {
    //                 if mesh == vertex_mesh {
    //                     self.input_tris_for_input_vertex(mesh, vertex).clone()
    //                 } else {
    //                     [tri].into_iter().collect()
    //                 }
    //             }
    //             MeshIntersectDescriptor::EdgeTriangle {
    //                 edge_mesh,
    //                 edge,
    //                 tri_mesh,
    //                 tri,
    //             } => {
    //                 if mesh == edge_mesh {
    //                     self.input_tris_for_input_edge(mesh, edge)
    //                 } else {
    //                     [tri].into_iter().collect()
    //                 }
    //             }
    //         },
    //         VertexOrigin::Removed => HashSet::new(),
    //     }
    // }
    pub fn build_all_fans(&mut self) {
        for mesh in 0..2 {
            for (tri, mtri) in self.input_tris[mesh].iter().enumerate() {
                for v in mtri.vertices() {
                    self.all_fans[mesh].entry(v).or_default().push(tri);
                }
            }
            for (v, p) in self.vertex_positions.vertex_positions[mesh].iter() {
                match p {
                    VertexPosition::Edge(e) => {
                        assert!(
                            self.all_fans[mesh]
                                .insert(
                                    *v,
                                    self.input_wings
                                        .get(e)
                                        .unwrap_or_else(|| panic!("Cannot find wing for {:?}", e))
                                        .wing_tris
                                        .to_vec()
                                )
                                .is_none()
                        );
                    }
                    VertexPosition::Tri(t) => {
                        assert!(self.all_fans[mesh].insert(*v, vec![*t]).is_none());
                    }
                }
            }
        }
    }
    fn input_tris_for_edge(&self, mesh: usize, edge: MeshEdge) -> ArrayVec<usize, 2> {
        let [set1, set2] = edge
            .vertices()
            .map(|v| self.all_fans[mesh].get(&v).cloned().unwrap_or_default());
        let set1 = set1.into_iter().collect::<HashSet<_>>();
        let mut set = HashSet::new();
        for tri in set2 {
            if set1.contains(&tri) {
                set.insert(tri);
            }
        }
        set.into_iter().collect::<ArrayVec<usize, 2>>()
    }
    fn tangents_for_edge(&self, mesh: usize, edge: OrderedMeshEdge) -> [Vec3; 2] {
        let segment = edge.for_vertices(&self.vertices);
        let center = segment.as_ray().dir();
        let tris = self.input_tris_for_edge(mesh, edge.edge());
        if tris.is_empty() {
            panic!("No tangents for {:?}", edge);
        } else if tris.len() == 1 {
            let normal = self.input_tris[mesh][tris[0]]
                .for_vertices(&self.vertices)
                .normal();
            let v1 = normal.cross(center);
            [v1, -v1]
        } else {
            let mtri1 = &self.input_tris[mesh][tris[0]];
            let mtri2 = &self.input_tris[mesh][tris[1]];
            let mut v1 = *mtri1
                .vertices()
                .iter()
                .find(|v| !mtri2.vertices().iter().contains(v))
                .unwrap();
            let mut v2 = *mtri2
                .vertices()
                .iter()
                .find(|v| !mtri1.vertices().iter().contains(v))
                .unwrap();
            let mut e = mtri1
                .ordered_edges()
                .iter()
                .cloned()
                .find(|e| !e.vertices().iter().contains(&v1))
                .unwrap();
            if e.for_vertices(&self.vertices)
                .as_ray()
                .dir()
                .dot(segment.as_ray().dir())
                < 0.0
            {
                mem::swap(&mut v1, &mut v2);
                e.invert();
            }
            [
                self.vertices[v1] - self.vertices[e.vertices()[0]],
                self.vertices[v2] - self.vertices[e.vertices()[0]],
            ]
        }
    }
    pub fn build_new_edges(&mut self, rng: &mut impl Rng) {
        for (tris, vs) in self.new_vertices_by_tri_pair.iter() {
            if let Ok([v1, v2]) = vs.clone().into_inner() {
                let mut edge = OrderedMeshEdge::new(v1, v2);
                let ev = self.vertices[v2] - self.vertices[v1];
                let b1 = ev.cross(Vec3::random_unit(rng)).normalize();
                let b2 = ev.cross(b1).normalize();
                assert!(ev.dot(b1).abs() < 1e-10);
                assert!(ev.dot(b2).abs() < 1e-10);
                assert!(b1.dot(b2).abs() < 1e-10);
                let mut angle_order = ArrayVec::<(usize, usize, f64), 4>::new();
                for mesh in 0..2 {
                    for (index, tangent) in self.tangents_for_edge(mesh, edge).iter().enumerate() {
                        angle_order.push((
                            mesh,
                            index,
                            Vec2::new(tangent.dot(b1), tangent.dot(b2)).angle()
                                + std::f64::consts::PI,
                        ));
                    }
                }
                let mut angle_order = angle_order.into_inner().unwrap();
                angle_order.sort_by_key(|&(mesh, side, angle)| NotNan::new(angle).unwrap());
                let definite = angle_order
                    .iter()
                    .circular_tuple_windows()
                    .all(|((m1, s1, a1), (m2, s2, a2))| self.eps.less(*a1, *a2).definite());
                let alternating = angle_order
                    .iter()
                    .circular_tuple_windows()
                    .all(|((m1, s1, a1), (m2, s2, a2))| m1 != m2);
                if definite {
                    if alternating {
                        let forward = angle_order.iter().circular_tuple_windows().any(
                            |((m1, s1, a1), (m2, s2, a2))| (m1, s1, m2, s2) == (&0, &0, &1, &0),
                        );
                        let reverse = angle_order.iter().circular_tuple_windows().any(
                            |((m1, s1, a1), (m2, s2, a2))| (m1, s1, m2, s2) == (&0, &0, &1, &1),
                        );
                        assert_ne!(forward, reverse);
                        if reverse {
                            edge.invert();
                        }
                        self.strong_edges.insert(edge);
                    }
                } else {
                    self.weak_edges.push(edge);
                }
            }
        }
    }
    pub fn build_loops(&mut self) {
        let mut loops = LoopBuilder::new();
        for v in 0..self.vertices.len() {
            loops.add_vertex(v);
        }
        for edge in &self.strong_edges {
            loops.add_strong(edge.vertices()[0], edge.vertices()[1]);
        }
        for edge in &self.weak_edges {
            loops.add_weak(edge.vertices()[0], edge.vertices()[1]);
        }
        self.loops = loops.solve();
    }

    pub fn build_loop_meta(&mut self) {
        let mut keep_vertices = HashSet::new();
        for (id, loop1) in self.loops.iter().enumerate() {
            for (&v1, &v2) in loop1.iter().circular_tuple_windows() {
                assert!(self.forward_loop_adjacency.insert(v1, v2).is_none());
                assert!(self.reverse_loop_adjacency.insert(v2, v1).is_none());
            }
            for &v in loop1.iter() {
                keep_vertices.insert(v);
                self.loop_ids.insert(v, id);
            }
        }
        for (v, vo) in self.vertex_origins.iter_mut().enumerate() {
            if let VertexOrigin::Intersect { .. } = vo {
                if !keep_vertices.contains(&v) {
                    *vo = VertexOrigin::Removed;
                }
            }
        }
        self.new_vertices.retain(|k, v| keep_vertices.contains(k));
    }
    pub fn collect_face_vertices(&mut self) {
        for (v, origin) in self.vertex_origins.iter().enumerate() {
            let VertexOrigin::Intersect { int } = origin else {
                continue;
            };
            let mut in_edges = ArrayVec::<(usize, MeshEdge), 2>::new();
            let mut in_tris = ArrayVec::<(usize, usize), 2>::new();
            match int.descriptor {
                MeshIntersectDescriptor::VertexVertex { .. } => {}
                MeshIntersectDescriptor::VertexEdge {
                    vertex_mesh,
                    vertex,
                    edge_mesh,
                    edge,
                } => {
                    in_edges.push((edge_mesh, edge));
                }
                MeshIntersectDescriptor::EdgeEdge { edges } => {
                    in_edges.push((0, edges[0]));
                    in_edges.push((1, edges[1]));
                }
                MeshIntersectDescriptor::VertexTriangle {
                    vertex_mesh,
                    vertex,
                    tri_mesh,
                    tri,
                } => {
                    in_tris.push((tri_mesh, tri));
                }
                MeshIntersectDescriptor::EdgeTriangle {
                    edge_mesh,
                    edge,
                    tri_mesh,
                    tri,
                } => {
                    in_edges.push((edge_mesh, edge));
                    in_tris.push((tri_mesh, tri));
                }
            }
            for (mesh, edge) in in_edges {
                let wing = &self.input_wings[&edge];
                assert_eq!(wing.mesh, mesh);
                for tri in wing.wing_tris {
                    let pos = self.input_tris[mesh][tri]
                        .edges()
                        .iter()
                        .position(|&e2| e2 == edge)
                        .unwrap();
                    self.new_faces[mesh][tri].edge_vertices[pos].push(v);
                }
            }
            for &(mesh, tri) in &in_tris {
                self.new_faces[mesh][tri].internal_vertices.push(v);
            }
        }
    }
    pub fn order_edge_vertices(&mut self) {
        for mesh in 0..2 {
            for tri in 0..self.new_faces[mesh].len() {
                let input_tri = &self.input_tris[mesh][tri];
                let new_face = &mut self.new_faces[mesh][tri];
                for ei in 0..3 {
                    let ordered_edge = input_tri.ordered_edges()[ei];
                    new_face.edge_vertices[ei].sort_by_cached_key(|&v| {
                        NotNan::new(
                            self.vertices[v].distance(self.vertices[input_tri.vertices()[ei]]),
                        )
                        .unwrap()
                    });
                }
            }
        }
    }
    pub fn build_polygon_edges(&mut self) {
        for mesh in 0..2 {
            for (tri, face) in &mut self.new_faces[mesh].iter_mut().enumerate() {
                for e in 0..3 {
                    face.border_vertices
                        .push(self.input_tris[mesh][tri].vertices()[e]);
                    for &v in &face.edge_vertices[e] {
                        face.border_vertices.push(v);
                    }
                }
                for (&v1, &v2) in face.border_vertices.iter().circular_tuple_windows() {
                    face.edges.insert(MeshEdge::new(v1, v2));
                }
                let vs = face
                    .edge_vertices
                    .iter()
                    .flatten()
                    .chain(face.internal_vertices.iter())
                    .cloned()
                    .collect::<HashSet<_>>();
                for &v1 in &vs {
                    let v2 = self.forward_loop_adjacency[&v1];
                    if vs.contains(&v2) {
                        face.edges.insert(MeshEdge::new(v1, v2));
                    }
                }
            }
        }
    }
    pub fn build_triangulation(&mut self) {
        for mesh in 0..2 {
            for (tri, face) in &mut self.new_faces[mesh].iter_mut().enumerate() {
                println!("{} {:#?}", tri, face);
                let ptri = self.input_tris[mesh][tri].for_vertices(&self.vertices);
                let mut triangulation = Triangulation::new(self.eps);
                println!("a");
                for v in self.input_tris[mesh][tri] {
                    println!("{}",self.vertices[v]);
                    let mut proj = ptri.project(self.vertices[v]);
                    triangulation.add_vertex(v, proj);
                }
                println!("b");
                for vs in &face.edge_vertices {
                    for &v in vs {
                        println!("{}",self.vertices[v]);
                        triangulation.add_vertex(v, ptri.project(self.vertices[v]));
                    }
                }
                for (&v1, &v2) in face.border_vertices.iter().circular_tuple_windows() {
                    triangulation.add_boundary(v1, v2);
                }
                println!("c");
                for &v in &face.internal_vertices {
                    println!("{}",self.vertices[v]);
                    let proj = ptri.project(self.vertices[v]);
                    let proj_mid = ptri.project(ptri.midpoint());
                    triangulation.add_vertex(
                        v,
                        proj * (1.0 - self.eps.value()) + proj_mid * self.eps.value(),
                    );
                }
                for e in &face.edges {
                    triangulation.add_edge(e[0], e[1]);
                }
                for i in 0..3 {
                    let mut evs = vec![];
                    evs.push(self.input_tris[mesh][tri].vertices()[i]);
                    evs.extend(face.edge_vertices[i].iter().cloned());
                    evs.push(self.input_tris[mesh][tri].vertices()[(i + 1) % 3]);
                    for i1 in 0..evs.len() {
                        for i2 in i1 + 2..evs.len() {
                            triangulation.exclude_edge(evs[i1], evs[i2]);
                        }
                    }
                }
                let triangulation = triangulation.solve();
                let expected_area = ptri.area();
                let mut actual_area = 0.0;
                for &tri in &triangulation {
                    let tri3 = tri.for_vertices(&self.vertices);
                    let area = tri3.area();
                    // assert!(area >= self.eps.value(), "{:?} {:?}", tri, area);
                    actual_area += area;
                }
                assert!(
                    (actual_area - expected_area).abs() < 1e-10,
                    "{:?} {:?}",
                    actual_area,
                    expected_area
                );
                let mut edge_table = HashMap::<MeshEdge, HashSet<OrderedMeshEdge>>::new();

                for (&v1, &v2) in face.border_vertices.iter().circular_tuple_windows() {
                    let e = OrderedMeshEdge::new(v2, v1);
                    assert!(edge_table.entry(e.edge()).or_default().insert(e), "{:?}", e);
                }
                for tri in &triangulation {
                    for e in tri.ordered_edges() {
                        assert!(edge_table.entry(e.edge()).or_default().insert(e), "{:?}", e);
                    }
                }
                assert!(edge_table.values().all(|x| x.len() == 2));
                for tri in triangulation {
                    self.new_tris[mesh].push(tri);
                }
            }
        }
    }
    pub fn build_connected_components(&mut self) {
        for mesh in 0..2 {
            let mut edge_to_tris = HashMap::<_, ArrayVec<_, 2>>::new();
            for (tri_index, tri) in self.new_tris[mesh].iter().enumerate() {
                for edge in tri.edges() {
                    if self.forward_loop_adjacency.get(&edge.vertices()[0])
                        == Some(&edge.vertices()[1])
                    {
                        continue;
                    }
                    if self.forward_loop_adjacency.get(&edge.vertices()[1])
                        == Some(&edge.vertices()[0])
                    {
                        continue;
                    }
                    edge_to_tris.entry(edge).or_default().push(tri_index);
                }
            }
            let mut tri_adj = HashMap::<_, Vec<_>>::new();
            for (e, tris) in edge_to_tris {
                let [t1, t2] = tris.into_inner().unwrap_or_else(|_| {
                    panic!("Missing triangle pair for {:?} in mesh {}", e, mesh)
                });
                tri_adj.entry(t1).or_default().push(t2);
                tri_adj.entry(t2).or_default().push(t1);
            }
            let mut visited = HashSet::new();
            for start in 0..self.new_tris[mesh].len() {
                if visited.contains(&start) {
                    continue;
                }
                let mut stack = vec![start];
                let mut component = vec![];
                while let Some(next) = stack.pop() {
                    if !visited.insert(next) {
                        continue;
                    }
                    component.push(self.new_tris[mesh][next]);
                    if let Some(adj) = tri_adj.get(&next) {
                        stack.extend(adj);
                    }
                }
                self.new_tri_ccs[mesh].push(component);
            }
        }
    }
    pub fn build_categories(&mut self, rng: &mut impl Rng) {
        for mesh in 0..2 {
            for cc in &self.new_tri_ccs[mesh] {
                let mut forward = false;
                let mut reverse = false;
                for &tri in cc {
                    for e in tri.ordered_edges() {
                        if self.forward_loop_adjacency.get(&e.vertices()[0])
                            == Some(&e.vertices()[1])
                        {
                            forward = true;
                        }
                        if self.forward_loop_adjacency.get(&e.vertices()[1])
                            == Some(&e.vertices()[0])
                        {
                            reverse = true;
                        }
                    }
                }
                if forward && reverse {
                    panic!("inconsistent loop ordering");
                }
                if !forward && !reverse {
                    println!("Testing inside");
                    if self.bvhs[1 - mesh]
                        .root_view(&self.input_tris[1 - mesh], &self.vertices)
                        .intersects_point(self.vertices[cc[0].vertices()[0]], self.eps, rng)
                        .round()
                    {
                        self.new_tri_cats[mesh][1].extend(cc);
                    } else {
                        self.new_tri_cats[mesh][0].extend(cc);
                    }
                } else if forward {
                    self.new_tri_cats[mesh][1 - mesh].extend(cc);
                } else if reverse {
                    self.new_tri_cats[mesh][mesh].extend(cc);
                }
            }
        }
    }
    pub fn build(&mut self, rng: &mut impl Rng) {
        self.build_relabel();
        self.check_degenerate();
        self.build_new_faces();
        self.build_intersects();
        self.build_new_vertices();
        self.build_input_wings();
        self.build_all_fans();
        self.build_new_edges(rng);
        self.build_loops();
        self.build_loop_meta();
        self.collect_face_vertices();
        self.order_edge_vertices();
        self.build_polygon_edges();
        self.build_triangulation();
        self.build_connected_components();
        self.build_categories(rng);
    }
    fn new_mesh(&self, mesh: usize, inside: bool) -> Mesh {
        Mesh::new(
            self.vertices.clone(),
            self.new_tri_cats[mesh][inside as usize].clone(),
        )
    }
    pub fn binop(&self, inside1: bool, inside2: bool, invert1: bool, invert2: bool) -> Mesh {
        let vertices = self.vertices.clone();
        let tris1 = self.new_tri_cats[0][inside1 as usize]
            .iter()
            .map(|&(mut tri)| {
                if invert1 {
                    tri.invert();
                }
                tri
            });
        let tris2 = self.new_tri_cats[1][inside2 as usize]
            .iter()
            .map(|&(mut tri)| {
                if invert2 {
                    tri.invert();
                }
                tri
            });
        let tris = tris1.chain(tris2).collect();
        let mesh = Mesh::new(vertices, tris);
        let mesh = mesh.without_dead_vertices();
        let mesh=mesh.without_empty_triangles(self.eps);
        mesh.check_manifold(self.eps).unwrap();
        mesh
    }
    pub fn union(&self) -> Mesh {
        self.binop(false, false, false, false)
    }
    pub fn intersect(&self) -> Mesh {
        self.binop(true, true, false, false)
    }
    pub fn forward_difference(&self) -> Mesh {
        self.binop(false, true, false, true)
    }
    pub fn reverse_difference(&self) -> Mesh {
        self.binop(true, false, true, false)
    }
}

#[cfg(test)]
#[tokio::test]
async fn test_tetrahedron() -> anyhow::Result<()> {
    let eps = Epsilon::new(1e-10);
    let mut mesh1 = Mesh::new(
        vec![Vec3::zero(), Vec3::axis_x(), Vec3::axis_y(), Vec3::axis_z()],
        vec![
            MeshTriangle::new(0, 2, 1),
            MeshTriangle::new(0, 3, 2),
            MeshTriangle::new(0, 1, 3),
            MeshTriangle::new(1, 2, 3),
        ],
    );
    let mut mesh2 = mesh1.clone();
    for v in mesh2.vertices_mut() {
        *v += Vec3::new(0.25, 0.25, 0.25);
    }
    let mut bimesh = Bimesh::new(&mesh1, &mesh2, eps, &mut rng());
    // bimesh.build();
    let dir = PathBuf::from("../")
        .join("target")
        .join("test_outputs")
        .join("test_tetrahedron2");
    tokio::fs::create_dir_all(&dir).await?;
    write_stl_file(&bimesh.new_mesh(0, false), &dir.join("mesh1_outside.stl")).await?;
    write_stl_file(&bimesh.new_mesh(0, true), &dir.join("mesh1_inside.stl")).await?;
    write_stl_file(&bimesh.new_mesh(1, false), &dir.join("mesh2_outside.stl")).await?;
    write_stl_file(&bimesh.new_mesh(1, true), &dir.join("mesh2_inside.stl")).await?;

    write_stl_file(&bimesh.union(), &dir.join("union.stl")).await?;
    write_stl_file(&bimesh.intersect(), &dir.join("intersect.stl")).await?;
    write_stl_file(
        &bimesh.forward_difference(),
        &dir.join("forward_difference.stl"),
    )
    .await?;
    write_stl_file(
        &bimesh.reverse_difference(),
        &dir.join("reverse_difference.stl"),
    )
    .await?;
    Ok(())
}

fn rand_tetr(rng: &mut XorShiftRng, steps: usize) -> Mesh {
    loop {
        let mut vs = (0..4)
            .map(|_| {
                Vec3::new(
                    rng.random_range(0..steps) as f64 / (steps as f64),
                    rng.random_range(0..steps) as f64 / (steps as f64),
                    rng.random_range(0..steps) as f64 / (steps as f64),
                )
            })
            .collect::<Vec<Vec3>>();
        let volume = (vs[1] - vs[0]).cross(vs[2] - vs[0]).dot(vs[3] - vs[0]);
        if volume < -1e-10 {
            vs.swap(2, 3);
        } else if volume < 1e-10 {
            continue;
        }
        let mut ts = vec![
            MeshTriangle::new(0, 2, 1),
            MeshTriangle::new(0, 3, 2),
            MeshTriangle::new(0, 1, 3),
            MeshTriangle::new(1, 2, 3),
        ];
        ts.shuffle(rng);
        return Mesh::new(vs, ts);
    }
}

#[cfg(test)]
#[tokio::test]
async fn test_random_tetrahedron() -> anyhow::Result<()> {
    let eps = Epsilon::new(1e-10);
    let dir = PathBuf::from("../")
        .join("target")
        .join("test_outputs")
        .join("test_random_tetrahedron");
    tokio::fs::create_dir_all(&dir).await?;

    for steps in 99..100 {
        println!("steps={:?}", steps);
        for seed in 2..1000 {
            println!("seed={:?}", seed);
            let mut rng = XorShiftRng::seed_from_u64(seed);
            let m1 = rand_tetr(&mut rng, steps);
            let m2 = rand_tetr(&mut rng, steps);
            // if m1
            //     .vertices()
            //     .iter()
            //     .any(|p| m2.vertices().iter().contains(p))
            // {
            //     //TODO: handle shared vertices by ordering loops with fans
            //     continue;
            // }
            println!("{:#?}\n{:#?}", m1, m2);
            write_stl_file(&m1, &dir.join("mesh1.stl")).await?;
            write_stl_file(&m2, &dir.join("mesh2.stl")).await?;
            let bm = Bimesh::new(&m1, &m2, eps, &mut rng);
            write_stl_file(&bm.new_mesh(0, false), &dir.join("mesh1_outside.stl")).await?;
            write_stl_file(&bm.new_mesh(0, true), &dir.join("mesh1_inside.stl")).await?;
            write_stl_file(&bm.new_mesh(1, false), &dir.join("mesh2_outside.stl")).await?;
            write_stl_file(&bm.new_mesh(1, true), &dir.join("mesh2_inside.stl")).await?;
            let union = bm.union();
            let inter = bm.intersect();
            write_stl_file(&union, &dir.join("union.stl")).await?;
            write_stl_file(&inter, &dir.join("inter.stl")).await?;
            for i in 2..100 {
                let p: Vec3 = rng.random();
                let in_m1 = m1.intersects_point(p, eps, &mut rng);
                let in_m2 = m2.intersects_point(p, eps, &mut rng);
                let in_union = union.intersects_point(p, eps, &mut rng);
                let in_inter = inter.intersects_point(p, eps, &mut rng);
                assert!(in_union.matches(in_m1.or(in_m2)));
                assert!(in_inter.matches(in_m1.and(in_m2)));
            }
        }
    }
    Ok(())
}

pub fn rand_prism(rng: &mut impl Rng) -> Mesh {
    let v1: Vec3 = rng.random();
    let v2: Vec3 = rng.random();
    AABB::new(v1.min(v2), v1.max(v2)).as_mesh()
}

#[cfg(test)]
#[tokio::test]
async fn test_random_prism() -> anyhow::Result<()> {
    let eps = Epsilon::new(1e-10);
    let dir = PathBuf::from("../")
        .join("target")
        .join("test_outputs")
        .join("test_random_prism");
    tokio::fs::create_dir_all(&dir).await?;

    for seed in 172..1000 {
        println!("seed={:?}", seed);
        let mut rng = XorShiftRng::seed_from_u64(seed);
        let m1 = rand_prism(&mut rng);
        let m2 = rand_prism(&mut rng);
        println!("{:#?}\n{:#?}", m1, m2);
        write_stl_file(&m1, &dir.join("mesh1.stl")).await?;
        write_stl_file(&m2, &dir.join("mesh2.stl")).await?;
        let bm = Bimesh::new(&m1, &m2, eps, &mut rng);
        write_stl_file(&bm.new_mesh(0, false), &dir.join("mesh1_outside.stl")).await?;
        write_stl_file(&bm.new_mesh(0, true), &dir.join("mesh1_inside.stl")).await?;
        write_stl_file(&bm.new_mesh(1, false), &dir.join("mesh2_outside.stl")).await?;
        write_stl_file(&bm.new_mesh(1, true), &dir.join("mesh2_inside.stl")).await?;
        let union = bm.union();
        let inter = bm.intersect();
        write_stl_file(&union, &dir.join("union.stl")).await?;
        write_stl_file(&inter, &dir.join("inter.stl")).await?;
        for i in 2..100 {
            let p: Vec3 = rng.random();
            let in_m1 = m1.intersects_point(p, eps, &mut rng);
            let in_m2 = m2.intersects_point(p, eps, &mut rng);
            let in_union = union.intersects_point(p, eps, &mut rng);
            let in_inter = inter.intersects_point(p, eps, &mut rng);
            assert!(in_union.matches(in_m1.or(in_m2)));
            assert!(in_inter.matches(in_m1.and(in_m2)));
        }
    }
    Ok(())
}
