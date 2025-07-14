use crate::directed_mesh_edge::DirectedMeshEdge;
use itertools::Itertools;
use patina_geo::geo2::polygon2::Polygon2;
use patina_vec::vec2::Vec2;

#[derive(Clone)]
pub struct EdgeMesh2 {
    vertices: Vec<Vec2>,
    edges: Vec<DirectedMeshEdge>,
}

impl EdgeMesh2 {
    pub fn new(vertices: Vec<Vec2>, edges: Vec<DirectedMeshEdge>) -> EdgeMesh2 {
        EdgeMesh2 { vertices, edges }
    }
    pub fn vertices(&self) -> &[Vec2] {
        &self.vertices
    }
    pub fn edges(&self) -> &[DirectedMeshEdge] {
        &self.edges
    }
    pub fn add_polygon(&mut self, poly: &Polygon2) {
        let mut vns = vec![];
        for v in poly.points() {
            vns.push(self.vertices.len());
            self.vertices.push(*v);
        }
        for (v1, v2) in vns.into_iter().circular_tuple_windows() {
            self.edges.push(DirectedMeshEdge::new(v1, v2));
        }
    }
}
