use crate::bvh::{Bvh, BvhLeafBuilder, BvhNodeView};
use crate::edge_mesh2::EdgeMesh2;
use patina_geo::geo2::ray2::Ray2;
use patina_geo::geo2::segment2::Segment2;
use patina_vec::vec2::Vec2;
use rand::rng;
use std::rc::Rc;
use std::sync::Arc;

#[derive(Debug)]
#[non_exhaustive]
pub struct EdgeMeshSegmentIntersect {
    pub edge: usize,
    pub t1: f64,
    pub t2: f64,
    pub pos: Vec2,
}

#[derive(Debug)]
#[non_exhaustive]
pub struct EdgeMeshRayIntersect {
    pub edge: usize,
    pub t1: f64,
    pub t2: f64,
    pub pos: Vec2,
}

impl Bvh<2, EdgeMesh2, usize> {
    pub fn from_edge_mesh2(mesh: Arc<EdgeMesh2>) -> Self {
        Bvh::new(
            &mesh
                .edges()
                .iter()
                .enumerate()
                .map(|(index, edge)| {
                    BvhLeafBuilder::from_segment(index, edge.for_vertices(mesh.vertices()))
                })
                .collect::<Vec<_>>(),
            mesh,
        )
    }
    pub fn intersect_segment(&self, segment: &Segment2) -> Vec<EdgeMeshSegmentIntersect> {
        let mut result = vec![];
        self.root_view().intersect_segment(segment, &mut result);
        result
    }
    pub fn intersect_ray(&self, ray: &Ray2) -> Vec<EdgeMeshRayIntersect> {
        let mut result = vec![];
        self.root_view().intersect_ray(ray, &mut result);
        result
    }
    pub fn contains_point(&self, point: Vec2) -> bool {
        self.intersect_ray(&Ray2::new(point, Vec2::random_normal(&mut rng())))
            .len()
            % 2
            == 1
    }
}

impl<'a> BvhNodeView<'a, 2, EdgeMesh2, usize> {
    pub fn intersect_segment(
        &self,
        segment: &Segment2,
        result: &mut Vec<EdgeMeshSegmentIntersect>,
    ) {
        if !self.aabb().intersect_segment(segment).is_some() {
            return;
        }
        for leaf in self.leaves() {
            let seg1 = leaf.mesh.edges()[*leaf.leaf].for_vertices(leaf.mesh.vertices());
            if let Some((t1, t2, pos)) = seg1.intersect_segment(&segment) {
                result.push(EdgeMeshSegmentIntersect {
                    edge: *leaf.leaf(),
                    t1,
                    t2,
                    pos,
                });
            }
        }
        for node in self.nodes() {
            todo!();
        }
    }
    pub fn intersect_ray(&self, ray: &Ray2, result: &mut Vec<EdgeMeshRayIntersect>) {
        if !self.aabb().intersect_ray(ray).is_some() {
            return;
        }
        for leaf in self.leaves() {
            let seg1 = leaf.mesh.edges()[*leaf.leaf].for_vertices(leaf.mesh.vertices());
            if let Some((t1, t2, pos)) = ray.intersect_segment(&seg1) {
                result.push(EdgeMeshRayIntersect {
                    edge: *leaf.leaf(),
                    t1,
                    t2,
                    pos,
                });
            }
        }
        for node in self.nodes() {
            todo!();
        }
    }
}
