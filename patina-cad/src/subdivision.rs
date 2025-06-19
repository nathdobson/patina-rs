use crate::meshes::mesh::Mesh;
use std::collections::HashMap;
use crate::meshes::mesh_triangle::MeshTriangle;

pub fn subdivide(mesh: &Mesh) -> Mesh {
    let mut vertices = mesh.vertices().to_vec();
    let mut triangles = vec![];
    let mut edge_to_midpoint = HashMap::new();
    for t in mesh.triangles() {
        let mut midpoints = [0usize; 3];
        for i in 0..3 {
            let v1 = t[i];
            let v2 = t[(i + 1) % 3];
            let mut e = [v1, v2];
            e.sort();
            midpoints[i] = *edge_to_midpoint.entry(e).or_insert_with(|| {
                let m = vertices.len();
                vertices.push((vertices[v1] + vertices[v2]) / 2.0);
                m
            });
        }
        for i in 0..3 {
            triangles.push(MeshTriangle::new(
                t[i],
                midpoints[(i + 0) % 3],
                midpoints[(i + 2) % 3],
            ));
        }
        triangles.push(MeshTriangle::from(midpoints));
    }
    Mesh::new(vertices, triangles)
}
