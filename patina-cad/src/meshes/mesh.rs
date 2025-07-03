use crate::geo3::ray3::Ray3;
use crate::math::float_bool::{Epsilon, FloatBool};
use crate::meshes::bimesh::Bimesh;
use crate::meshes::error::ManifoldError;
use crate::meshes::intersect_bvh_ray::MeshRayIntersection;
use crate::meshes::mesh_triangle::MeshTriangle;
use crate::meshes::remove_empty_triangles::RemoveEmptyTriangles;
use itertools::Itertools;
use patina_vec::vec3::Vec3;
use rand::{Rng, rng};
use std::collections::HashMap;

#[derive(Clone, Debug)]
pub struct Mesh {
    vertices: Vec<Vec3>,
    triangles: Vec<MeshTriangle>,
}

impl Mesh {
    pub fn triangles(&self) -> &[MeshTriangle] {
        &self.triangles
    }
    pub fn vertices(&self) -> &[Vec3] {
        &self.vertices
    }
    pub fn vertices_mut(&mut self) -> &mut [Vec3] {
        &mut self.vertices
    }
    pub fn check_manifold(&self, eps: Epsilon) -> Result<(), ManifoldError> {
        let mut edge_table = HashMap::<usize, HashMap<usize, Vec<usize>>>::new();
        for t in &self.triangles {
            if t[0] == t[1] || t[0] == t[2] || t[1] == t[2] {
                return Err(ManifoldError::DuplicateVertex);
            }
            for rot in 0..3 {
                let v1 = t[rot];
                let v2 = t[(rot + 1) % 3];
                let v3 = t[(rot + 2) % 3];
                edge_table
                    .entry(v1)
                    .or_default()
                    .entry(v2)
                    .or_default()
                    .push(v3);
            }
        }
        for v in 0..self.vertices.len() {
            let mut edges = edge_table.remove(&v).ok_or(ManifoldError::MissingVertex)?;
            let mut fan_count = 0;
            while !edges.is_empty() {
                fan_count += 1;
                let start = *edges.keys().next().unwrap();
                let mut walk = start;
                loop {
                    let next = edges
                        .remove(&walk)
                        .ok_or(ManifoldError::BrokenFan(v, walk))?;
                    walk = next
                        .into_iter()
                        .exactly_one()
                        .map_err(|e| ManifoldError::SplitFan(v))?;
                    if walk == start {
                        break;
                    }
                }
            }
            if fan_count != 1 {
                return Err(ManifoldError::DuplicateFan);
            }
        }
        if !edge_table.is_empty() {
            return Err(ManifoldError::BadVertex);
        }
        for t in &self.triangles {
            let t = t.for_vertices(&self.vertices);
            if t.area() <= eps.value() {
                return Err(ManifoldError::EmptyTriangle);
            }
        }
        Ok(())
    }
    pub fn new(vertices: Vec<Vec3>, triangles: Vec<MeshTriangle>) -> Self {
        for t in &triangles {
            for v in *t {
                assert!(v < vertices.len());
            }
        }
        Self {
            vertices,
            triangles,
        }
    }
    pub fn without_dead_vertices(&self) -> Mesh {
        let mut new_vertices = vec![];
        let mut vertex_map = HashMap::new();
        let mut new_triangles = vec![];
        for t1 in &self.triangles {
            new_triangles.push(MeshTriangle::from(t1.vertices().map(|v| {
                *vertex_map.entry(v).or_insert_with(|| {
                    new_vertices.push(self.vertices[v]);
                    new_vertices.len() - 1
                })
            })));
        }
        Mesh::new(new_vertices, new_triangles)
    }
    pub fn without_empty_triangles(&self, eps: Epsilon) -> Mesh {
        let mut result = self.clone();
        while let Some(next) = RemoveEmptyTriangles::new(eps, &result).build() {
            result = next;
        }
        result
    }
    pub fn union(&self, other: &Mesh, eps: Epsilon) -> Mesh {
        Bimesh::new(self, other, eps, &mut rng()).union()
    }
    pub fn intersect(&self, other: &Mesh, eps: Epsilon) -> Mesh {
        Bimesh::new(self, other, eps, &mut rng()).intersect()
    }
    pub fn difference(&self, other: &Mesh, eps: Epsilon) -> Mesh {
        Bimesh::new(self, other, eps, &mut rng()).forward_difference()
    }
    pub fn intersect_ray(&self, ray: &Ray3, eps: Epsilon) -> Vec<MeshRayIntersection> {
        let mut result = vec![];
        for (tri, mtri) in self.triangles.iter().enumerate() {
            let ptri = mtri.for_vertices(&self.vertices);
            let (truth, time) = ptri.intersect_ray(ray, eps);
            if truth.maybe() {
                result.push(MeshRayIntersection {
                    index: tri,
                    time,
                    truth,
                });
            }
        }
        result
    }
    pub fn intersects_point(&self, point: Vec3, eps: Epsilon, rng: &mut impl Rng) -> FloatBool {
        let ints = self.intersect_ray(&Ray3::new(point, Vec3::random_unit(rng)), eps);
        let mut result = FloatBool::from(false);
        for int in &ints {
            result = result.xor(int.truth);
        }
        result
    }
    pub fn perturb(&mut self, rng: &mut impl Rng, factor: f64) {
        for vertex in &mut self.vertices {
            *vertex += Vec3::new(
                rng.random::<f64>() * factor,
                rng.random::<f64>() * factor,
                rng.random::<f64>() * factor,
            );
        }
    }
    pub fn perturbed(&self, rng: &mut impl Rng, factor: f64) -> Mesh {
        let mut mesh = self.clone();
        mesh.perturb(rng, factor);
        mesh
    }
}

impl Mesh {
    pub fn from_aabb(aabb: AABB) -> Mesh {
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
}

#[test]
fn test_check_manifold() {
    let eps: Epsilon = Epsilon::new(1e-10);
    assert_eq!(
        Err(ManifoldError::DuplicateVertex),
        Mesh::new(
            vec![Vec3::zero(), Vec3::zero()],
            vec![MeshTriangle::new(0, 0, 1)]
        )
        .check_manifold(eps)
    );
    assert_eq!(
        Err(ManifoldError::MissingVertex),
        Mesh::new(
            vec![Vec3::zero(); 5],
            vec![
                MeshTriangle::new(0, 1, 2),
                MeshTriangle::new(3, 2, 1),
                MeshTriangle::new(0, 2, 3),
                MeshTriangle::new(1, 0, 3)
            ]
        )
        .check_manifold(eps)
    );
    assert_eq!(
        Ok(()),
        Mesh::new(
            vec![Vec3::zero(); 4],
            vec![
                MeshTriangle::new(0, 1, 2),
                MeshTriangle::new(3, 2, 1),
                MeshTriangle::new(0, 2, 3),
                MeshTriangle::new(1, 0, 3)
            ]
        )
        .check_manifold(eps)
    );
    assert_eq!(
        Err(ManifoldError::BrokenFan(0, 3)),
        Mesh::new(
            vec![Vec3::zero(); 4],
            vec![
                MeshTriangle::new(0, 1, 2),
                MeshTriangle::new(3, 2, 1),
                MeshTriangle::new(0, 2, 3)
            ]
        )
        .check_manifold(eps)
    );
    assert_eq!(
        Err(ManifoldError::SplitFan(2)),
        Mesh::new(
            vec![Vec3::zero(); 6],
            vec![
                MeshTriangle::new(0, 1, 2),
                MeshTriangle::new(3, 2, 1),
                MeshTriangle::new(0, 2, 3),
                MeshTriangle::new(1, 0, 3),
                MeshTriangle::new(2, 3, 4),
                MeshTriangle::new(5, 4, 3),
                MeshTriangle::new(2, 4, 5),
                MeshTriangle::new(3, 2, 5)
            ]
        )
        .check_manifold(eps)
    );
    assert_eq!(
        Err(ManifoldError::DuplicateFan),
        Mesh::new(
            vec![Vec3::zero(); 7],
            vec![
                MeshTriangle::new(0, 1, 2),
                MeshTriangle::new(3, 2, 1),
                MeshTriangle::new(0, 2, 3),
                MeshTriangle::new(1, 0, 3),
                MeshTriangle::new(3, 4, 5),
                MeshTriangle::new(6, 5, 4),
                MeshTriangle::new(3, 5, 6),
                MeshTriangle::new(4, 3, 6)
            ]
        )
        .check_manifold(eps)
    );
}
