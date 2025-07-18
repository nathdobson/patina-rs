use crate::directed_mesh_edge::DirectedMeshEdge;
use crate::edge_mesh2::EdgeMesh2;
use crate::mesh::Mesh;
use itertools::Itertools;
use patina_vec::vec2::Vec2;
use std::cmp::Ordering;
use std::collections::HashMap;
use patina_geo::geo2::segment2::Segment2;

pub struct MeshCut<'mesh> {
    mesh: &'mesh EdgeMesh2,
    cut: Segment2,
    path: Vec<Vec2>,
    vertices: Vec<Vec2>,
    vertex_map: HashMap<usize, usize>,
    edges: Vec<DirectedMeshEdge>,
}

impl<'mesh> MeshCut<'mesh> {
    pub fn new(mesh: &'mesh EdgeMesh2, path: Vec<Vec2>) -> Self {
        MeshCut {
            mesh,
            cut: Segment2::new(*path.last().unwrap(), *path.first().unwrap()),
            path,
            vertices: vec![],
            vertex_map: HashMap::new(),
            edges: vec![],
        }
    }
    pub fn build(mut self) -> EdgeMesh2 {
        for (index, &vertex) in self.mesh.vertices().iter().enumerate() {
            if self.cut.as_ray().above(vertex) == Ordering::Less {
                self.vertex_map.insert(index, self.vertices.len());
                self.vertices.push(vertex);
            }
        }
        let begin = self.vertices.len();
        for v in self.path {
            self.vertices.push(v);
        }
        let end = self.vertices.len() - 1;
        for (v1, v2) in (begin..=end).tuple_windows() {
            self.edges.push(DirectedMeshEdge::new(v1, v2));
        }
        let mut nvs1: Vec<usize> = vec![];
        let mut nvs2: Vec<usize> = vec![];
        nvs1.push(end);
        for &edge in self.mesh.edges() {
            let v1 = self.vertex_map.get(&edge.v1()).cloned();
            let v2 = self.vertex_map.get(&edge.v2()).cloned();
            match (v1, v2) {
                (None, None) => {}
                (Some(v1), None) => {
                    let ray = edge.for_vertices(self.mesh.vertices()).as_ray();
                    let nv = ray.at_time(ray.intersect_line(&self.cut.as_ray()).unwrap().0);
                    let nvi = self.vertices.len();
                    nvs1.push(nvi);
                    self.edges.push(DirectedMeshEdge::new(v1, nvi));
                    self.vertices.push(nv);
                }
                (None, Some(v2)) => {
                    let ray = edge.for_vertices(self.mesh.vertices()).as_ray();
                    let nv = ray.at_time(ray.intersect_line(&self.cut.as_ray()).unwrap().0);
                    let nvi = self.vertices.len();
                    nvs2.push(nvi);
                    self.edges.push(DirectedMeshEdge::new(nvi, v2));
                    self.vertices.push(nv);
                }
                (Some(v1), Some(v2)) => {
                    self.edges.push(edge);
                }
            }
        }
        nvs2.push(begin);
        for (v1, v2) in nvs1.iter().zip_eq(nvs2.iter()) {
            self.edges.push(DirectedMeshEdge::new(*v1, *v2));
        }
        EdgeMesh2::from_vecs(self.vertices, self.edges)
    }
}
