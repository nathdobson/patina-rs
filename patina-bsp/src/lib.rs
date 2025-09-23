#![allow(unused_imports)]
#![deny(unused_must_use)]
#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unused_assignments)]

use patina_geo::geo2::triangle2::Triangle2;
use patina_mesh::mesh::Mesh;
use patina_mesh::mesh_triangle::MeshTriangle;
use patina_vec::vec::Vector;
use patina_vec::vec2::Vec2;
use patina_vec::vec3::Vec3;
use rand::{Rng, RngCore, rng};
use std::collections::HashMap;

pub struct Bsp<const M: usize> {
    root: BspNode<M>,
    vertices: Vec<Vec3>,
}

pub enum BspNode<const M: usize> {
    Leaf(BspLeaf<M>),
    Branch(BspBranch<M>),
}

pub struct BspBranch<const M: usize> {
    normal: Vec3,
    tan1: Vec3,
    tan2: Vec3,
    offset: f64,
    children: [Box<Bsp<M>>; 2],
    triangles: [Vec<MeshTriangle>; M],
    vertices: HashMap<usize, Vec2>,
}

pub struct BspLeaf<const M: usize> {
    inside: [bool; M],
}

pub struct BspBuilder<'a> {
    rng: &'a mut dyn RngCore,
    eps: f64,
    vertices: Vec<Vec3>,
    max_unbalanced: usize,
}

impl Bsp<1> {
    pub fn from_mesh(mesh: &Mesh) -> Self {
        BspBuilder {
            rng: &mut rng(),
            eps: 10e-10,
            vertices: mesh.vertices().to_vec(),
            max_unbalanced: 10,
        }
        .build(mesh.triangles())
    }
}

impl BspBuilder<'_> {
    pub fn build(self, triangles: &[MeshTriangle]) -> Bsp<1> {
        let plane;
        if triangles.len() < self.max_unbalanced {
            let t =
                triangles[self.rng.random_range(0..triangles.len())].for_vertices(&self.vertices);
            plane = t.basis_plane();
        } else {
            let center = triangles
                .iter()
                .map(|t| t.for_vertices(&self.vertices).midpoint())
                .sum::<Vec3>()
                / (triangles.len() as f64);
        }
        todo!();
    }
}

#[test]
fn test() {}
