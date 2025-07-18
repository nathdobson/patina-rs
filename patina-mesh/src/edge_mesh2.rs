use crate::directed_mesh_edge::DirectedMeshEdge;
use itertools::Itertools;
use patina_geo::geo2::polygon2::Polygon2;
use patina_vec::vec2::Vec2;
use std::collections::{HashMap, HashSet};

#[derive(Clone, Default, Debug)]
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
    pub fn from_vecs(vertices: Vec<Vec2>, edges: Vec<DirectedMeshEdge>) -> EdgeMesh2 {
        EdgeMesh2 { vertices, edges }
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
    pub fn without_dead_vertices(&self) -> Self {
        let mut map = vec![None; self.vertices.len()];
        let mut vertices = vec![];
        let mut edges = vec![];
        for edge in self.edges.iter() {
            edges.push(DirectedMeshEdge::from(edge.vertices().map(|v| {
                *map[v].get_or_insert_with(|| {
                    let v2 = vertices.len();
                    vertices.push(self.vertices[v]);
                    v2
                })
            })));
        }
        EdgeMesh2 { vertices, edges }
    }
    pub fn as_polygons(&self) -> Vec<Polygon2> {
        let mut forward = HashMap::new();
        for edge in self.edges.iter() {
            assert!(forward.insert(edge.v1(), edge.v2()).is_none());
        }
        let mut polys = vec![];
        let mut visited = HashSet::new();
        for mut v in 0..self.vertices.len() {
            let mut poly = vec![];
            loop {
                if !visited.insert(v) {
                    break;
                }
                poly.push(self.vertices[v]);
                v = *forward.get(&v).unwrap();
            }
            if !poly.is_empty() {
                polys.push(Polygon2::new(poly));
            }
        }
        polys
    }
}
