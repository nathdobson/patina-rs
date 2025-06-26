use crate::geo2::segment2::Segment2;
use crate::geo2::triangle2::Triangle2;
use crate::math::float_bool::Epsilon;
use crate::math::vec2::Vec2;
use crate::meshes::mesh_edge::MeshEdge;
use crate::meshes::mesh_triangle::MeshTriangle;
use crate::meshes::ordered_mesh_edge::OrderedMeshEdge;
use ordered_float::NotNan;
use std::collections::hash_set::Entry;
use std::collections::{HashMap, HashSet};
use std::hash::RandomState;

#[derive(Debug)]
pub struct Triangulation {
    eps: Epsilon,
    vertices: HashMap<usize, Vec2>,
    edges: HashSet<MeshEdge>,
    exclude: HashSet<MeshEdge>,
    triangles: Vec<MeshTriangle>,
    boundaries: HashSet<OrderedMeshEdge>,
    // boundaries_forward: HashMap<usize, usize>,
}

impl Triangulation {
    pub fn new(eps: Epsilon) -> Self {
        Triangulation {
            eps,
            vertices: HashMap::new(),
            edges: HashSet::new(),
            exclude: HashSet::new(),
            triangles: vec![],
            boundaries: HashSet::new(),
        }
    }
    pub fn add_vertex(&mut self, v: usize, p: Vec2) {
        self.vertices.insert(v, p);
    }
    pub fn add_boundary(&mut self, v1: usize, v2: usize) {
        self.boundaries.insert(OrderedMeshEdge::new(v1, v2));
    }
    pub fn add_edge(&mut self, v1: usize, v2: usize) {
        self.edges.insert(MeshEdge::new(v1, v2));
    }
    pub fn exclude_edge(&mut self, v1: usize, v2: usize) {
        self.exclude.insert(MeshEdge::new(v1, v2));
    }
    fn is_ccw(&self, v1: usize, v2: usize, v3: usize) -> bool {
        let mut vs = [v1, v2, v3];
        while vs[0] > vs[1] || vs[0] > vs[2] {
            vs.rotate_right(1);
        }
        if vs[1] < vs[2] {
            (self.vertices[&vs[1]] - self.vertices[&vs[0]])
                .cross(self.vertices[&vs[2]] - self.vertices[&vs[0]])
                >= 0.0
        } else {
            (self.vertices[&vs[2]] - self.vertices[&vs[0]])
                .cross(self.vertices[&vs[1]] - self.vertices[&vs[0]])
                < 0.0
        }
    }
    fn intersects_edges(&self, e1: &MeshEdge, e2: &MeshEdge) -> bool {
        let [v1, v2] = e1.vertices();
        let [v3, v4] = e2.vertices();
        let cross = (self.vertices[&v2] - self.vertices[&v1])
            .cross(self.vertices[&v4] - self.vertices[&v3]);
        if cross.abs() < self.eps.value() {
            return false;
        }
        self.is_ccw(v1, v2, v3) != self.is_ccw(v1, v2, v4)
            && self.is_ccw(v3, v4, v1) != self.is_ccw(v3, v4, v2)
    }
    pub fn solve(mut self) -> Vec<MeshTriangle> {
        while let Some(&boundary) = self.boundaries.iter().min() {
            let [v1, v2] = boundary.vertices();
            let mut candidates = vec![];
            'vertex_search: for &v3 in self.vertices.keys() {
                if v1 == v3 || v2 == v3 {
                    continue;
                }
                if self.exclude.contains(&MeshEdge::new(v2, v3)) {
                    continue;
                }
                if self.exclude.contains(&MeshEdge::new(v1, v3)) {
                    continue;
                }
                if !self.is_ccw(v1, v2, v3) {
                    continue;
                }
                for &vo in self.vertices.keys() {
                    if vo == v1 || vo == v2 || vo == v3 {
                        continue;
                    }
                    if self.is_ccw(v1, v2, vo) && self.is_ccw(v2, v3, vo) && self.is_ccw(v3, v1, vo)
                    {
                        continue 'vertex_search;
                    }
                }
                for old_edge in &self.edges {
                    for new_edge in &[MeshEdge::new(v2, v3), MeshEdge::new(v3, v1)] {
                        if !old_edge.shares_vertex(new_edge)
                            && self.intersects_edges(old_edge, new_edge)
                        {
                            continue 'vertex_search;
                        }
                    }
                }
                candidates.push(v3);
            }
            let v3 = candidates.into_iter().min().expect("no viable vertices");
            self.triangles.push(MeshTriangle::new(v1, v2, v3));
            self.edges.insert(MeshEdge::new(v2, v3));
            self.edges.insert(MeshEdge::new(v3, v1));
            assert!(self.boundaries.remove(&OrderedMeshEdge::new(v1, v2)));
            match self.boundaries.entry(OrderedMeshEdge::new(v2, v3)) {
                Entry::Occupied(ent) => {
                    ent.remove();
                }
                Entry::Vacant(ent) => {
                    self.boundaries.insert(OrderedMeshEdge::new(v3, v2));
                }
            }
            match self.boundaries.entry(OrderedMeshEdge::new(v3, v1)) {
                Entry::Occupied(ent) => {
                    ent.remove();
                }
                Entry::Vacant(ent) => {
                    self.boundaries.insert(OrderedMeshEdge::new(v1, v3));
                }
            }
        }
        assert!(self.triangles.len() > 0);
        self.triangles
    }
}
