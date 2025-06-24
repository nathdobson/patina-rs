use crate::geo3::ray3::Ray3;
use crate::math::float_bool::{Epsilon, FloatBool};
use crate::meshes::bvh::{Bvh, BvhNodeView, BvhTriangleView};
use crate::meshes::intersect_bvh_bvh::MeshMeshIntersection;

#[derive(Debug)]
#[non_exhaustive]
pub struct MeshRayIntersection {
    pub index: usize,
    pub time: f64,
    pub truth: FloatBool,
}

pub struct IntersectBvhRay {
    eps: Epsilon,
    result: Vec<MeshRayIntersection>,
}

impl IntersectBvhRay {
    pub fn new(eps: Epsilon) -> Self {
        IntersectBvhRay {
            eps,
            result: vec![],
        }
    }
    pub fn intersect_node_ray(&mut self, node: &BvhNodeView, ray: &Ray3) {
        if !ray.intersect_aabb(node.aabb(), self.eps).0.maybe() {
            return;
        }
        for child in node.nodes() {
            self.intersect_node_ray(&child, ray);
        }
        for leaf in node.leaves() {
            self.intersect_leaf_ray(&leaf, ray);
        }
    }
    fn intersect_leaf_ray(&mut self, node: &BvhTriangleView, ray: &Ray3) {
        let (truth, time) = node.triangle().intersect_ray(ray, self.eps);
        if truth.maybe() {
            self.result.push(MeshRayIntersection {
                index: node.index(),
                time,
                truth,
            });
        }
    }
    pub fn build(self) -> Vec<MeshRayIntersection> {
        self.result
    }
}
