use crate::math::float_bool::{Epsilon, FloatBool};
use patina_vec::vec3::Vec3;
use crate::meshes::bvh::{Bvh, BvhNodeView, BvhTriangleView};
use crate::meshes::mesh::Mesh;
use crate::meshes::mesh_edge::MeshEdge;
use crate::sat::sat_intersects;
use arrayvec::ArrayVec;
use itertools::Itertools;
use std::fmt::{Debug, Formatter};

pub struct IntersectBvhBvh {
    eps: Epsilon,
    result: Vec<MeshIntersect>,
}

#[derive(Debug)]
#[non_exhaustive]
pub struct MeshIntersect {
    pub tris: [usize; 2],
    pub vertices: ArrayVec<MeshIntersectVertex, 2>,
}

#[non_exhaustive]
#[derive(Clone)]
pub struct MeshIntersectVertex {
    pub position: Vec3,
    pub descriptor: MeshIntersectDescriptor,
}

#[derive(Clone, Hash, Eq, PartialEq, PartialOrd, Ord)]
pub enum MeshIntersectDescriptor {
    VertexVertex {
        vertices: [usize; 2],
    },
    VertexEdge {
        vertex_mesh: usize,
        vertex: usize,
        edge_mesh: usize,
        edge: MeshEdge,
    },
    EdgeEdge {
        edges: [MeshEdge; 2],
    },
    VertexTriangle {
        vertex_mesh: usize,
        vertex: usize,
        tri_mesh: usize,
        tri: usize,
    },
    EdgeTriangle {
        edge_mesh: usize,
        edge: MeshEdge,
        tri_mesh: usize,
        tri: usize,
    },
}

impl IntersectBvhBvh {
    pub fn new(eps: Epsilon) -> Self {
        IntersectBvhBvh {
            eps,
            result: vec![],
        }
    }
    pub fn intersect_node_node(&mut self, node1: &BvhNodeView, node2: &BvhNodeView) {
        if !node1.aabb().intersects(&node2.aabb()) {
            return;
        }
        for child1 in node1.nodes() {
            self.intersect_node_node(&child1, node2);
        }
        for leaf1 in node1.leaves() {
            self.intersect_leaf_node(&leaf1, node2);
        }
    }

    fn intersect_leaf_node(&mut self, tri1: &BvhTriangleView, node2: &BvhNodeView) {
        if !tri1
            .triangle()
            .for_vertices(tri1.vertices())
            .intersects_aabb(node2.aabb(), self.eps)
            .maybe()
        {
            return;
        }
        for child2 in node2.nodes() {
            self.intersect_leaf_node(tri1, &child2);
        }
        for leaf2 in node2.leaves() {
            self.intersect_leaf_leaf(tri1, &leaf2);
        }
    }
    pub fn intersect_leaf_leaf(&mut self, tri1: &BvhTriangleView, tri2: &BvhTriangleView) {
        let tris = [tri1, tri2];
        let non_coplanar = tri1
            .triangle3()
            .normal()
            .cross(tri2.triangle3().normal())
            .length();
        if non_coplanar < self.eps.value() {
            return;
        }
        let mut ints = ArrayVec::<_, 4>::new();
        for v1 in tri1.triangle().vertices() {
            for v2 in tri2.triangle().vertices() {
                if tri1.vertices()[v1].distance(tri2.vertices()[v2]) < self.eps.value() {
                    let pos = (tri1.vertices()[v1] + tri2.vertices()[v2]) / 2.0;
                    ints.push(MeshIntersectVertex {
                        position: pos,
                        descriptor: MeshIntersectDescriptor::VertexVertex { vertices: [v1, v2] },
                    });
                }
            }
        }
        for (mesh1, mesh2) in [(0, 1), (1, 0)] {
            for v1 in tris[mesh1].triangle().vertices() {
                for e2 in tris[mesh2].triangle().edges() {
                    let (truth, t, p) = e2
                        .for_vertices(tris[mesh2].vertices())
                        .intersects_vertex(tris[mesh1].vertices()[v1], self.eps);
                    if truth.maybe() {
                        if !ints.iter().any(|int| match int.descriptor {
                            MeshIntersectDescriptor::VertexVertex { vertices } => {
                                v1 == vertices[mesh1] && e2.vertices().contains(&vertices[mesh2])
                            }
                            _ => false,
                        }) {
                            ints.push(MeshIntersectVertex {
                                position: p,
                                descriptor: MeshIntersectDescriptor::VertexEdge {
                                    vertex_mesh: mesh1,
                                    vertex: v1,
                                    edge_mesh: mesh2,
                                    edge: e2,
                                },
                            })
                        }
                    }
                }
            }
        }
        for e1 in tri1.triangle().edges() {
            for e2 in tri2.triangle().edges() {
                let s1 = e1.for_vertices(tri1.vertices());
                let s2 = e2.for_vertices(tri2.vertices());
                let (truth, t1, t2, p) = s1.intersects_segment(&s2, self.eps);
                if truth.maybe() {
                    let edges = [e1, e2];
                    if !ints.iter().any(|int| match &int.descriptor {
                        MeshIntersectDescriptor::VertexVertex { vertices } => {
                            e1.vertices().contains(&vertices[0])
                                && e2.vertices().contains(&vertices[1])
                        }
                        MeshIntersectDescriptor::VertexEdge {
                            vertex_mesh,
                            vertex,
                            edge_mesh,
                            edge,
                        } => {
                            edges[*vertex_mesh].vertices().contains(vertex)
                                && edges[*edge_mesh] == *edge
                        }
                        _ => false,
                    }) {
                        ints.push(MeshIntersectVertex {
                            position: p,
                            descriptor: MeshIntersectDescriptor::EdgeEdge { edges },
                        });
                    }
                }
            }
        }
        for (mesh1, mesh2) in [(0, 1), (1, 0)] {
            let tri1 = tris[mesh1];
            let tri2 = tris[mesh2];
            for v1 in tris[mesh1].triangle().vertices() {
                if tri2
                    .triangle3()
                    .intersect_point(tris[mesh1].vertices()[v1], self.eps)
                    .maybe()
                {
                    if !ints.iter().any(|int| match &int.descriptor {
                        MeshIntersectDescriptor::VertexVertex { vertices } => vertices[mesh1] == v1,
                        MeshIntersectDescriptor::VertexEdge {
                            vertex_mesh,
                            vertex,
                            edge_mesh,
                            edge,
                        } => {
                            if (mesh1, mesh2) == (*vertex_mesh, *edge_mesh) {
                                v1 == *vertex
                            } else if (mesh2, mesh1) == (*vertex_mesh, *edge_mesh) {
                                false
                            } else {
                                unreachable!();
                            }
                        }
                        MeshIntersectDescriptor::EdgeEdge { edges } => {
                            //distinct
                            false
                        }
                        _ => false,
                    }) {
                        ints.push(MeshIntersectVertex {
                            position: tris[mesh1].vertices()[v1],
                            descriptor: MeshIntersectDescriptor::VertexTriangle {
                                vertex_mesh: mesh1,
                                vertex: v1,
                                tri_mesh: mesh2,
                                tri: tri2.index(),
                            },
                        });
                    }
                }
            }
        }
        for (mesh1, mesh2) in [(0, 1), (1, 0)] {
            let tri1 = tris[mesh1];
            let tri2 = tris[mesh2];
            for e1 in tris[mesh1].triangle().edges() {
                let s1 = e1.for_vertices(tris[mesh1].vertices());
                let (truth, time) = tri2.triangle3().intersect_segment(&s1, self.eps);
                if truth.maybe() {
                    let pos = s1.at_time(time);
                    if !ints.iter().any(|int| match &int.descriptor {
                        MeshIntersectDescriptor::VertexVertex { vertices } => {
                            e1.vertices().contains(&vertices[mesh1])
                                && tri2.triangle().vertices().contains(&vertices[mesh2])
                        }
                        MeshIntersectDescriptor::VertexEdge {
                            vertex_mesh,
                            vertex,
                            edge_mesh,
                            edge,
                        } => {
                            if (mesh1, mesh2) == (*vertex_mesh, *edge_mesh) {
                                e1.vertices().contains(vertex)
                            } else if (mesh2, mesh1) == (*vertex_mesh, *edge_mesh) {
                                e1 == *edge
                            } else {
                                unreachable!();
                            }
                        }
                        MeshIntersectDescriptor::EdgeEdge { edges } => {
                            //
                            edges[mesh1] == e1
                        }
                        MeshIntersectDescriptor::VertexTriangle {
                            vertex_mesh,
                            vertex,
                            tri_mesh,
                            tri,
                        } => {
                            if (mesh1, mesh2) == (*vertex_mesh, *tri_mesh) {
                                e1.vertices().contains(vertex)
                            } else if (mesh2, mesh1) == (*vertex_mesh, *tri_mesh) {
                                false
                            } else {
                                unreachable!();
                            }
                        }
                        _ => false,
                    }) {
                        ints.push(MeshIntersectVertex {
                            position: pos,
                            descriptor: MeshIntersectDescriptor::EdgeTriangle {
                                edge_mesh: mesh1,
                                edge: e1,
                                tri_mesh: mesh2,
                                tri: tri2.index(),
                            },
                        });
                    }
                }
            }
        }
        if ints.len() > 2 {
            panic!("{:#?}", ints);
        }
        self.result.push(MeshIntersect {
            tris: [tri1.index(), tri2.index()],
            vertices: ints.into_iter().collect(),
        });
    }
    pub fn build(self) -> Vec<MeshIntersect> {
        self.result
    }
}

impl Debug for MeshIntersectVertex {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}: {:?}", self.position, self.descriptor)
    }
}

impl Debug for MeshIntersectDescriptor {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let cross = " X ";
        let width = 10;
        match self {
            MeshIntersectDescriptor::VertexVertex { vertices } => {
                write!(f, "v{:<width$}{cross}v{:<width$}", vertices[0], vertices[1])
            }
            MeshIntersectDescriptor::VertexEdge {
                vertex_mesh,
                vertex,
                edge_mesh,
                edge,
            } => {
                if *vertex_mesh == 0 {
                    write!(f, "v{:<width$}{cross}e{:<width$}", vertex, edge)
                } else {
                    write!(f, "e{:<width$}{cross}v{:<width$}", edge, vertex)
                }
            }
            MeshIntersectDescriptor::EdgeEdge { edges } => {
                write!(f, "e{:<width$}{cross}e{:<width$}", edges[0], edges[1])
            }
            MeshIntersectDescriptor::VertexTriangle {
                vertex_mesh,
                vertex,
                tri_mesh,
                tri,
            } => {
                if *vertex_mesh == 0 {
                    write!(f, "v{:<width$}{cross}t{:<width$}", vertex, tri)
                } else {
                    write!(f, "t{:<width$}{cross}v{:<width$}", tri, vertex)
                }
            }
            MeshIntersectDescriptor::EdgeTriangle {
                edge_mesh,
                edge,
                tri_mesh,
                tri,
            } => {
                if *edge_mesh == 0 {
                    write!(f, "e{:<width$}{cross}t{:<width$}", edge, tri)
                } else {
                    write!(f, "t{:<width$}{cross}e{:<width$}", tri, edge)
                }
            }
        }
    }
}
