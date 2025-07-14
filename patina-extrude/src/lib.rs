#![allow(unused_variables)]
#![allow(unused_imports)]
#![deny(unused_must_use)]
#![allow(unused_mut)]

use itertools::Itertools;
use ordered_float::NotNan;
use patina_geo::geo2::polygon2::Polygon2;
use patina_mesh::edge_mesh2::EdgeMesh2;
use patina_mesh::mesh::Mesh;
use patina_mesh::mesh_triangle::MeshTriangle;
use patina_mesh::ser::stl::{write_stl_file, write_test_stl_file};
use patina_mesh::triangulation::Triangulation;
use patina_vec::vec2::Vec2;
use patina_vec::vec3::Vec3;
use std::collections::HashMap;
use std::ops::Range;
use std::thread;
use std::thread::Thread;

#[derive(Default)]
struct Layer {
    mesh: EdgeMesh2,
    vertices: Vec<usize>,
    inverted: bool,
}
pub struct ExtrusionBuilder {
    vertices: Vec<Vec3>,
    triangles: Vec<MeshTriangle>,
    polys: Vec<Polygon2>,
    layers: HashMap<NotNan<f64>, Layer>,
}

impl ExtrusionBuilder {
    pub fn new() -> Self {
        ExtrusionBuilder {
            vertices: vec![],
            triangles: vec![],
            polys: vec![],
            layers: Default::default(),
        }
    }
    fn add_prism_side(&mut self, poly: &Polygon2, z: f64, top: bool) -> Range<usize> {
        let layer = self.layers.entry(NotNan::new(z).unwrap()).or_default();
        layer.mesh.add_polygon(&poly);
        let range = self.vertices.len()..self.vertices.len() + poly.points().len();
        for v in poly.points() {
            layer.vertices.push(self.vertices.len());
            self.vertices.push(Vec3::new(v.x(), v.y(), z));
        }
        if poly.signed_area() > 0.0 && !top {
            layer.inverted = true;
        }
        range
    }
    pub fn add_prism(&mut self, poly: Polygon2, z: Range<f64>) {
        let vs1 = self.add_prism_side(&poly, z.start, false);
        let vs2 = self.add_prism_side(&poly, z.end, true);
        for ((v11, v12), (v21, v22)) in vs1.zip(vs2).into_iter().circular_tuple_windows() {
            self.triangles.push(MeshTriangle::new(v22, v12, v21));
            self.triangles.push(MeshTriangle::new(v21, v12, v11));
        }
        self.polys.push(poly);
    }
    pub fn build(mut self) -> Mesh {
        for layer in self.layers.values() {
            for tri in Triangulation::new(&layer.mesh).build() {
                let mut tri = MeshTriangle::from(tri.vertices().map(|v| layer.vertices[v]));
                if layer.inverted {
                    tri.invert();
                }
                self.triangles.push(tri);
            }
        }
        Mesh::new(self.vertices, self.triangles)
    }
}

#[cfg(test)]
#[tokio::test]
async fn test() -> anyhow::Result<()> {
    let mut cube = ExtrusionBuilder::new();
    cube.add_prism(
        Polygon2::new(vec![
            Vec2::new(0.5, 0.5),
            Vec2::new(1.0, 0.5),
            Vec2::new(1.0, 1.0),
            Vec2::new(0.5, 1.0),
        ]),
        0.5..1.0,
    );
    let cube = cube.build();
    println!("{:#?}", cube);
    cube.check_manifold()?;
    write_test_stl_file(&cube, "test.stl").await?;
    Ok(())
}
