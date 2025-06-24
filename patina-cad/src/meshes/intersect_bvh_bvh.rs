use crate::math::float_bool::{Epsilon, FloatBool};
use crate::math::vec3::Vec3;
use crate::meshes::bvh::{Bvh, BvhNodeView, BvhTriangleView};
use crate::meshes::mesh::Mesh;
use crate::meshes::mesh_edge::MeshEdge;
use crate::sat::sat_intersects;

pub struct IntersectBvhBvh {
    eps: Epsilon,
    result: Vec<MeshMeshIntersection>,
}

#[derive(Debug)]
#[non_exhaustive]
pub struct MeshMeshIntersection {
    pub edge_mesh: usize,
    pub edge: MeshEdge,
    pub edge_tri: usize,
    pub plane_mesh: usize,
    pub plane_tri: usize,
    pub time: f64,
    pub position: Vec3,
    pub truth: FloatBool,
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
        for (mesh1, mesh2) in [(0, 1), (1, 0)] {
            for (e1, s1) in tris[mesh1]
                .mesh_triangle()
                .ordered_edges()
                .iter()
                .zip(tris[mesh1].triangle().edges().iter())
            {
                if e1.vertices()[0] < e1.vertices()[1] {
                    let (truth, time) = tris[mesh2].triangle().intersect_segment(&s1, self.eps);
                    if truth.maybe() {
                        self.result.push(MeshMeshIntersection {
                            edge_mesh: mesh1,
                            edge: e1.edge(),
                            edge_tri: tris[mesh1].index(),
                            plane_mesh: mesh2,
                            plane_tri: tris[mesh2].index(),
                            time,
                            position: s1.at_time(time),
                            truth,
                        });
                    }
                }
            }
        }
    }
    pub fn build(self) -> Vec<MeshMeshIntersection> {
        self.result
    }
}
