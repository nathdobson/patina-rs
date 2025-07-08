use crate::directed_mesh_edge::DirectedMeshEdge;
use crate::mesh::Mesh;
use crate::mesh_edge::MeshEdge;
use itertools::Itertools;
use std::collections::HashMap;

pub struct EdgeTable {
    table: HashMap<DirectedMeshEdge, HalfWing>,
}

pub struct HalfWing {
    triangle: usize,
    vertex: usize,
}

impl HalfWing {
    pub fn triangle(&self) -> usize {
        self.triangle
    }
    pub fn vertex(&self) -> usize {
        self.vertex
    }
}

impl EdgeTable {
    pub fn new(mesh: &Mesh) -> Self {
        let mut table = HashMap::new();
        for (tri, mtri) in mesh.triangles().iter().enumerate() {
            for (&v1, &v2, &v3) in mtri.vertices().iter().circular_tuple_windows() {
                table.insert(
                    DirectedMeshEdge::new(v1, v2),
                    HalfWing {
                        triangle: tri,
                        vertex: v3,
                    },
                );
            }
        }
        EdgeTable { table }
    }
    pub fn half_wing(&self, edge: DirectedMeshEdge) -> &HalfWing {
        &self.table[&edge]
    }
    pub fn directed_wing(&self, edge: DirectedMeshEdge) -> [&HalfWing; 2] {
        let wing1 = self.half_wing(edge);
        let wing2 = self.half_wing(edge.inverted());
        [wing1, wing2]
    }
    pub fn wing(&self, edge: MeshEdge) -> [&HalfWing; 2] {
        self.directed_wing(DirectedMeshEdge::new(edge.v1(), edge.v2()))
    }
}
