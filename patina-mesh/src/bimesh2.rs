use crate::bvh::Bvh;
use crate::directed_mesh_edge::DirectedMeshEdge;
use crate::edge_mesh2::EdgeMesh2;
use itertools::Itertools;
use ordered_float::NotNan;
use patina_geo::geo2::polygon2::Polygon2;
use patina_vec::vec2::Vec2;
use patina_vec::vec3::Vec3;
use rand::rng;
use std::collections::{BTreeMap, HashMap};
use std::rc::Rc;
use std::sync::Arc;

pub struct Bimesh2 {
    meshes: [Arc<EdgeMesh2>; 2],
    vertices: Vec<Vec2>,
    vertex_kinds: Vec<VertexKind>,
    edges: [Vec<DirectedMeshEdge>; 2],
}

#[derive(Clone, Debug)]
struct EdgeBuilder {
    vertices: BTreeMap<NotNan<f64>, usize>,
}

impl EdgeBuilder {
    pub fn new(v1: usize, v2: usize) -> Self {
        let mut vertices = BTreeMap::new();
        vertices.insert(NotNan::new(f64::NEG_INFINITY).unwrap(), v1);
        vertices.insert(NotNan::new(f64::INFINITY).unwrap(), v2);
        EdgeBuilder { vertices }
    }
}

enum VertexKind {
    Mesh { mesh: usize, interior: bool },
    Intersect { cross: bool },
}

impl Bimesh2 {
    pub fn new(mesh1: Arc<EdgeMesh2>, mesh2: Arc<EdgeMesh2>) -> Self {
        let meshes = [mesh1, mesh2];
        let mut vertices = vec![];
        let mut vertex_remap = [vec![], vec![]];
        let mut vertex_kinds = vec![];
        let bvhs = meshes.clone().map(|mesh| Bvh::from_edge_mesh2(mesh));
        for mesh in 0..2 {
            for &v in meshes[mesh].vertices() {
                let i = vertices.len();
                vertices.push(v);
                vertex_remap[mesh].push(i);
                vertex_kinds.push(VertexKind::Mesh {
                    mesh,
                    interior: bvhs[1 - mesh].contains_point(v),
                });
            }
        }
        let mut ebs = [vec![], vec![]];
        for mesh in 0..2 {
            for (i, e) in meshes[mesh].edges().iter().enumerate() {
                ebs[mesh].push(EdgeBuilder::new(
                    vertex_remap[mesh][e.v1()],
                    vertex_remap[mesh][e.v2()],
                ));
            }
        }
        for (i2, e2) in meshes[1].edges().iter().enumerate() {
            let s2 = e2.for_vertices(meshes[1].vertices());
            let ints = bvhs[0].intersect_segment(&s2);
            for int in ints {
                let i1 = int.edge;
                let e1 = meshes[0].edges()[i1];
                let s1 = e1.for_vertices(meshes[0].vertices());
                let v = vertices.len();
                vertices.push(int.pos);
                vertex_kinds.push(VertexKind::Intersect {
                    cross: s1.as_ray().dir().cross(s2.as_ray().dir()) > 0.0,
                });
                assert!(
                    ebs[0][i1]
                        .vertices
                        .insert(NotNan::new(int.t1).unwrap(), v)
                        .is_none()
                );
                assert!(
                    ebs[1][i2]
                        .vertices
                        .insert(NotNan::new(int.t2).unwrap(), v)
                        .is_none()
                );
            }
        }
        let mut edges = [vec![], vec![]];
        for mesh in 0..2 {
            for e in &ebs[mesh] {
                for (v1, v2) in e.vertices.values().cloned().tuple_windows() {
                    edges[mesh].push(DirectedMeshEdge::new(v1, v2));
                }
            }
        }
        Bimesh2 {
            meshes,
            vertices,
            vertex_kinds,
            edges,
        }
    }
    pub fn as_mesh(
        &self,
        interior1: bool,
        interior2: bool,
        invert1: bool,
        invert2: bool,
    ) -> EdgeMesh2 {
        let mut vertices = self.vertices.clone();
        let mut edges = vec![];
        for mesh in 0..2 {
            let want_interior = [interior1, interior2][mesh];
            for &e in &self.edges[mesh] {
                let keep = match (&self.vertex_kinds[e.v1()], &self.vertex_kinds[e.v2()]) {
                    (VertexKind::Mesh { interior, .. }, _) => *interior == want_interior,
                    (_, VertexKind::Mesh { interior, .. }) => *interior == want_interior,
                    (
                        VertexKind::Intersect { cross: cross1 },
                        VertexKind::Intersect { cross: cross2 },
                    ) => want_interior != [*cross1, *cross2][mesh],
                };
                if keep {
                    if [invert1, invert2][mesh] {
                        edges.push(e.inverted());
                    } else {
                        edges.push(e);
                    }
                }
            }
        }
        EdgeMesh2::from_vecs(vertices, edges).without_dead_vertices()
    }
    pub fn union(&self) -> EdgeMesh2 {
        self.as_mesh(false, false, false, false)
    }
    pub fn difference(&self) -> EdgeMesh2 {
        self.as_mesh(false, true, false, true)
    }
    pub fn intersection(&self) -> EdgeMesh2 {
        self.as_mesh(true, true, false, false)
    }
}

#[test]
fn test() {
    let poly1 = Polygon2::new(vec![
        Vec2::new(0.0, 0.0),
        Vec2::new(1.0, 0.0),
        Vec2::new(0.0, 1.0),
    ]);
    let poly2 = Polygon2::new(vec![
        Vec2::new(0.25, 0.25),
        Vec2::new(1.25, 0.25),
        Vec2::new(0.25, 1.25),
    ]);
    let mut mesh1 = EdgeMesh2::new();
    mesh1.add_polygon(poly1.points().iter().cloned());
    let mut mesh2 = EdgeMesh2::new();
    mesh2.add_polygon(poly2.points().iter().cloned());
    let bimesh = Bimesh2::new(Arc::new(mesh1), Arc::new(mesh2));
    println!();
    for poly in bimesh.union().as_polygons() {
        println!("{}", poly);
    }
    println!();
    for poly in bimesh.intersection().as_polygons() {
        println!("{}", poly);
    }
    println!();
    for poly in bimesh.difference().as_polygons() {
        println!("{}", poly);
    }
}
