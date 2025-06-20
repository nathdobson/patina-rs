use crate::math::vec3::Vec3;
use crate::meshes::mesh::Mesh;
use crate::meshes::mesh_triangle::MeshTriangle;
use ordered_float::NotNan;
use std::f64::consts::PI;

pub struct Cylinder {
    origin: Vec3,
    axis: Vec3,
    radius: f64,
}

impl Cylinder {
    pub fn new(origin: Vec3, axis: Vec3, radius: f64) -> Self {
        Cylinder {
            origin,
            axis,
            radius,
        }
    }
    pub fn origin(&self) -> Vec3 {
        self.origin
    }
    pub fn axis(&self) -> Vec3 {
        self.axis
    }
    pub fn radius(&self) -> f64 {
        self.radius
    }
    pub fn as_mesh(&self, detail: usize) -> Mesh {
        let mut vertices = Vec::new();
        for i in 0..detail {
            let theta = i as f64 / detail as f64 * PI * 2.0;
            let x = theta.cos();
            let y = theta.sin();
            let helper_axis = Vec3::axes()
                .into_iter()
                .min_by_key(|axis| NotNan::new(axis.dot(self.axis)).unwrap())
                .unwrap();
            let x_axis = self.axis.cross(helper_axis).normalize() * self.radius;
            let y_axis = self.axis.cross(x_axis).normalize() * self.radius;
            vertices.push(x * x_axis + y * y_axis + self.origin);
            vertices.push(x * x_axis + y * y_axis + self.origin + self.axis);
        }
        vertices.push(self.origin);
        vertices.push(self.origin + self.axis);
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
}
