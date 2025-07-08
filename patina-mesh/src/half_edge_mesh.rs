use crate::VertexId;
use crate::directed_mesh_edge::DirectedMeshEdge;
use crate::mesh::Mesh;
use crate::mesh_edge::MeshEdge;
use itertools::Itertools;
use patina_vec::vec3::Vec3;
use std::collections::HashMap;
use std::ops::Index;

pub struct HalfEdge {
    tri: HalfEdgeId,
    twin: HalfEdgeId,
    vertex: VertexId,
}

#[derive(Copy, Clone, Eq, Ord, PartialOrd, PartialEq, Hash, Debug)]
pub struct HalfEdgeId(usize);

pub struct HalfEdgeMesh {
    positions: Vec<Vec3>,
    fans: Vec<HalfEdgeId>,
    tris: Vec<HalfEdgeId>,
    edges: Vec<HalfEdge>,
}

impl HalfEdgeMesh {
    pub fn new(mesh: &Mesh) -> Self {
        let positions = mesh.vertices().to_vec();
        let mut tris = Vec::with_capacity(mesh.triangles().len());
        let mut directed_edges: Vec<DirectedMeshEdge> =
            Vec::with_capacity(mesh.triangles().len() * 3);
        for tri in mesh.triangles() {
            tris.push(HalfEdgeId(directed_edges.len()));
            for edge in tri.ordered_edges() {
                directed_edges.push(edge);
            }
        }
        let mut fans: Vec<HalfEdgeId> = vec![HalfEdgeId(usize::MAX); directed_edges.len()];
        for (id, edge) in directed_edges.iter().enumerate() {
            fans[edge.vertices()[0]] = HalfEdgeId(id);
        }
        assert!(fans.iter().all(|x| *x != HalfEdgeId(usize::MAX)));
        let edge_ids: HashMap<DirectedMeshEdge, HalfEdgeId> = directed_edges
            .iter()
            .cloned()
            .enumerate()
            .map(|(i, e)| (e, HalfEdgeId(i)))
            .collect();
        let mut edges = vec![];
        for (tri, mtri) in mesh.triangles().iter().enumerate() {
            let tri_edge = HalfEdgeId(edges.len());
            for edge in mtri.ordered_edges() {
                edges.push(HalfEdge {
                    tri: HalfEdgeId(tri),
                    twin: *edge_ids.get(&edge.inverted()).unwrap(),
                    vertex: VertexId(edge.v1()),
                });
            }
        }
        HalfEdgeMesh {
            positions,
            fans,
            tris,
            edges,
        }
    }
}

impl Index<HalfEdgeId> for HalfEdgeMesh {
    type Output = HalfEdge;

    fn index(&self, index: HalfEdgeId) -> &Self::Output {
        &self.edges[index.0]
    }
}
