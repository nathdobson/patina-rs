use crate::mesh::Mesh;
use crate::mesh_triangle::MeshTriangle;
use ordered_float::NotNan;
use patina_geo::aabb::Aabb;
use patina_geo::geo3::cylinder::Cylinder;
use patina_vec::vec3::Vec3;
use std::f64;

impl Mesh {
    pub fn from_aabb(aabb: Aabb<3>) -> Mesh {
        Mesh::new(
            aabb.vertices().to_vec(),
            vec![
                MeshTriangle::new(0b000, 0b001, 0b011),
                MeshTriangle::new(0b000, 0b011, 0b010),
                MeshTriangle::new(0b100, 0b111, 0b101),
                MeshTriangle::new(0b100, 0b110, 0b111),
                //
                MeshTriangle::new(0b000, 0b101, 0b001),
                MeshTriangle::new(0b000, 0b100, 0b101),
                MeshTriangle::new(0b010, 0b011, 0b111),
                MeshTriangle::new(0b010, 0b111, 0b110),
                //
                MeshTriangle::new(0b000, 0b010, 0b110),
                MeshTriangle::new(0b000, 0b110, 0b100),
                MeshTriangle::new(0b001, 0b111, 0b011),
                MeshTriangle::new(0b001, 0b101, 0b111),
                //
            ],
        )
    }
    pub fn from_cylinder(cylinder: &Cylinder, detail: usize) -> Mesh {
        assert!(detail >= 3);
        let mut vertices = Vec::new();
        for i in 0..detail {
            let theta = i as f64 / detail as f64 * f64::consts::PI * 2.0;
            let x = theta.cos();
            let y = theta.sin();
            let helper_axis = Vec3::axes()
                .into_iter()
                .min_by_key(|axis| NotNan::new(axis.dot(cylinder.axis()).abs()).unwrap())
                .unwrap();
            let x_axis = cylinder.axis().cross(helper_axis).normalize() * cylinder.radius();
            assert!(x_axis.is_finite());
            let y_axis = cylinder.axis().cross(x_axis).normalize() * cylinder.radius();
            assert!(y_axis.is_finite());
            vertices.push(x_axis * x + y_axis * y + cylinder.origin());
            vertices.push(x_axis * x + y_axis * y + cylinder.origin() + cylinder.axis());
        }
        vertices.push(cylinder.origin());
        vertices.push(cylinder.origin() + cylinder.axis());
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
