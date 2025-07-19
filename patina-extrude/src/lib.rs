#![allow(unused_variables)]
#![allow(unused_imports)]
#![deny(unused_must_use)]
#![allow(unused_mut)]
#![allow(dead_code)]

use itertools::Itertools;
use ordered_float::NotNan;
use patina_geo::geo2::polygon2::Polygon2;
use patina_mesh::edge_mesh2::EdgeMesh2;
use patina_mesh::mesh::Mesh;
use patina_mesh::mesh_triangle::MeshTriangle;
use patina_mesh::triangulation::Triangulation;
use patina_vec::vec2::Vec2;
use patina_vec::vec3::Vec3;
use std::collections::HashMap;
use std::ops::Range;
use std::thread;
use std::thread::Thread;

pub struct ExtrusionBuilder {
    vertices: Vec<Vec3>,
    triangles: Vec<MeshTriangle>,
    planes: Vec<ExtrusionPlane>,
}

struct ExtrusionPlane {
    z: f64,
    invert: bool,
    mesh: EdgeMesh2,
    vertices: Vec<usize>,
}

impl ExtrusionBuilder {
    pub fn new() -> Self {
        ExtrusionBuilder {
            vertices: vec![],
            triangles: vec![],
            planes: vec![],
        }
    }
    pub fn add_plane(&mut self, z: f64, invert: bool) -> usize {
        let index = self.planes.len();
        self.planes.push(ExtrusionPlane {
            z,
            invert,
            mesh: EdgeMesh2::new(),
            vertices: vec![],
        });
        index
    }
    fn add_poly(&mut self, poly: &EdgeMesh2, plane: usize, invert: bool) -> Range<usize> {
        let plane = &mut self.planes[plane];
        plane.mesh.add_mesh(poly, invert);
        let start = self.vertices.len();
        for v in poly.vertices() {
            plane.vertices.push(self.vertices.len());
            self.vertices.push(Vec3::new(v.x(), v.y(), plane.z));
        }
        start..self.vertices.len()
    }
    pub fn add_prism(
        &mut self,
        poly: &EdgeMesh2,
        (plane1, invert1): (usize, bool),
        (plane2, invert2): (usize, bool),
    ) {
        let vs1 = self.add_poly(poly, plane1, invert1);
        let vs2 = self.add_poly(poly, plane2, invert2);
        for edge in poly.edges() {
            let v11 = vs1.start + edge.v1();
            let v12 = vs1.start + edge.v2();
            let v21 = vs2.start + edge.v1();
            let v22 = vs2.start + edge.v2();
            self.triangles.push(MeshTriangle::new(v11, v12, v21));
            self.triangles.push(MeshTriangle::new(v21, v12, v22));
        }
    }
    pub fn build(mut self) -> Mesh {
        for plane in self.planes {
            let tris = Triangulation::new(&plane.mesh).build();
            for tri in tris {
                let mut tri = MeshTriangle::from(tri.vertices().map(|v| plane.vertices[v]));
                if plane.invert {
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
    let plane1 = cube.add_plane(0.1, true);
    let plane2 = cube.add_plane(0.5, false);
    let mut edge_mesh = EdgeMesh2::new();
    edge_mesh.add_polygon(
        vec![
            Vec2::new(0.5, 0.5),
            Vec2::new(1.0, 0.5),
            Vec2::new(1.0, 1.0),
            Vec2::new(0.5, 1.0),
        ]
        .into_iter(),
    );
    cube.add_prism(&edge_mesh, (plane1, false), (plane2, false));
    let cube = cube.build();
    println!("{:#?}", cube);
    cube.check_manifold()?;
    write_test_stl_file(&cube, "test.stl").await?;
    Ok(())
}
