use crate::directed_mesh_edge::DirectedMeshEdge;
use crate::edge_mesh2::EdgeMesh2;
use crate::mesh_triangle::MeshTriangle;
use crate::monotone_decomp::{MonoSide, MonoVertex, MonotoneDecomp};
use arrayvec::ArrayVec;
use itertools::Itertools;
use ordered_float::NotNan;
use patina_geo::geo2::polygon2::Polygon2;
use patina_geo::geo2::ray2::Ray2;
use patina_geo::geo2::segment2::Segment2;
use patina_geo::geo2::triangle2::Triangle2;
use patina_vec::vec2::Vec2;
use rand::{Rng, SeedableRng};
use rand_xorshift::XorShiftRng;
use std::cmp::Ordering;
use std::collections::HashMap;
use std::mem;

pub struct Triangulation<'mesh> {
    mesh: &'mesh EdgeMesh2,
}

impl<'mesh> Triangulation<'mesh> {
    pub fn new(mesh: &'mesh EdgeMesh2) -> Self {
        Triangulation { mesh }
    }
    fn diagonalize_monotonic(&self, mono: &[MonoVertex], tris: &mut Vec<MeshTriangle>) {
        let mut stack = vec![];
        stack.push(mono[0].clone());
        stack.push(mono[1].clone());
        for i in 2..mono.len() - 1 {
            if mono[i].side() != stack.last().unwrap().side() {
                let top = *stack.last().unwrap();
                while stack.len() >= 2 {
                    let p1 = stack.pop().unwrap();
                    let p2 = stack.last().unwrap();
                    let mut tri = MeshTriangle::new(mono[i].vertex(), p1.vertex(), p2.vertex());
                    if p1.side() == MonoSide::Up {
                        tri.invert();
                    }
                    tris.push(tri);
                }
                stack.pop();
                stack.push(top);
                stack.push(mono[i]);
            } else {
                while stack.len() >= 2 {
                    let p1 = stack.pop().unwrap();
                    let p2 = stack.last().unwrap();

                    let mut tri = MeshTriangle::new(mono[i].vertex(), p1.vertex(), p2.vertex());
                    if mono[i].side() == MonoSide::Up {
                        tri.invert();
                    }
                    if tri.for_vertices2(self.mesh.vertices()).signed_area() >= 0.0 {
                        tris.push(tri);
                    } else {
                        stack.push(p1);
                        break;
                    }
                }
                stack.push(mono[i]);
            }
        }
        let v3 = mono.last().unwrap();
        for (v1, v2) in stack.iter().tuple_windows() {
            let mut tri = MeshTriangle::new(v3.vertex(), v2.vertex(), v1.vertex());
            if v2.side() == MonoSide::Up {
                tri.invert();
            }
            tris.push(tri);
        }
    }
    pub fn build(self) -> Vec<Triangle2> {
        let mut monos = MonotoneDecomp::new(self.mesh).build();
        println!("{:?}", monos);
        let mut tris = vec![];
        for mono in &monos {
            self.diagonalize_monotonic(&mono, &mut tris);
        }
        println!("{:?}", tris);
        tris.into_iter()
            .map(|x| x.for_vertices2(&self.mesh.vertices()))
            .collect()
    }
}

#[test]
fn test_traps() {
    let mut mesh = EdgeMesh2::new(vec![], vec![]);
    mesh.add_polygon(&Polygon2::new(vec![
        Vec2::new(0.0, 0.0),
        Vec2::new(2.0, 1.0),
        Vec2::new(1.0, 2.0),
    ]));
    let mut tris = Triangulation::new(&mesh).build();
    assert_eq!(tris.len(), 1)
}

#[test]
fn test_triangulation() {
    for poly in Polygon2::test_cases() {
        println!("{}", poly);
        let mut mesh = EdgeMesh2::new(vec![], vec![]);
        mesh.add_polygon(&poly);
        let mut tris = Triangulation::new(&mesh).build();
        let mut total = 0.0;
        for tri in tris {
            let area = tri.signed_area();
            assert!(area >= 0.0);
            total += area;
        }
        let expected = poly.signed_area();
        assert!(
            (total - expected).abs() < 10e-10,
            "{:?} ~= {:?}",
            total,
            expected
        );
    }
}
