use crate::math::vec3::Vec3;
use crate::meshes::mesh::Mesh;
use std::f64::consts::PI;
use crate::meshes::mesh_triangle::MeshTriangle;

pub fn cylinder(detail: usize) -> Mesh {
    let mut vertices = Vec::new();
    for i in 0..detail {
        let theta = i as f64 / detail as f64 * PI * 2.0;
        let x = theta.cos();
        let y = theta.sin();
        vertices.push(Vec3::new(x, y, 0.0));
        vertices.push(Vec3::new(x, y, 1.0));
    }
    vertices.push(Vec3::new(0.0, 0.0, 0.0));
    vertices.push(Vec3::new(0.0, 0.0, 1.0));
    let mut triangles = Vec::new();
    for i in 0..detail {
        let l1 = i * 2;
        let l2 = ((i + 1) % detail) * 2;
        let lc = detail * 2;
        let u1 = i * 2 + 1;
        let u2 = ((i + 1) % detail) * 2 + 1;
        let uc = detail * 2 + 1;
        triangles.push(MeshTriangle::new(l2, l1, lc));
        triangles.push(MeshTriangle::new(u1, u2, uc));
        triangles.push(MeshTriangle::new(u1, l1, u2));
        triangles.push(MeshTriangle::new(l1, l2, u2));
    }
    Mesh::new(vertices, triangles)
}
