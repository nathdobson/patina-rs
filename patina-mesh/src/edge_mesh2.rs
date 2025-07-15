use crate::directed_mesh_edge::DirectedMeshEdge;
use itertools::Itertools;
use patina_geo::geo2::polygon2::Polygon2;
use patina_vec::vec2::Vec2;

#[derive(Clone, Default)]
pub struct EdgeMesh2 {
    vertices: Vec<Vec2>,
    edges: Vec<DirectedMeshEdge>,
}

impl EdgeMesh2 {
    pub fn new() -> EdgeMesh2 {
        EdgeMesh2 {
            vertices: vec![],
            edges: vec![],
        }
    }
    pub fn vertices(&self) -> &[Vec2] {
        &self.vertices
    }
    pub fn edges(&self) -> &[DirectedMeshEdge] {
        &self.edges
    }
    pub fn add_polygon(&mut self, poly: impl Iterator<Item = Vec2>) {
        let mut vns = vec![];
        for v in poly {
            vns.push(self.vertices.len());
            self.vertices.push(v);
        }
        for (v1, v2) in vns.into_iter().circular_tuple_windows() {
            self.edges.push(DirectedMeshEdge::new(v1, v2));
        }
    }
    pub fn add_mesh(&mut self, other: &Self, invert: bool) {
        let shift = self.vertices.len();
        self.vertices.extend(other.vertices.iter().cloned());
        for edge in other.edges.iter() {
            let edge = DirectedMeshEdge::new(edge.v1() + shift, edge.v2() + shift);
            if invert {
                self.edges.push(edge.inverted());
            } else {
                self.edges.push(edge);
            }
        }
    }
}
