use crate::sdf::{Sdf, Sdf3};
use arrayvec::ArrayVec;
use itertools::Itertools;
use ordered_float::NotNan;
use patina_geo::geo3::ray3::Ray3;
use patina_mesh::directed_mesh_edge::DirectedMeshEdge;
use patina_mesh::edge_table::EdgeTable;
use patina_mesh::mesh::Mesh;
use patina_mesh::mesh_edge::MeshEdge;
use patina_mesh::mesh_triangle::MeshTriangle;
use patina_scalar::deriv::Deriv;
use patina_vec::vec3::Vec3;
use std::collections::{HashMap, HashSet};

pub struct Subdivide {
    min_dot: f64,
    marching_eps: f64,
}

impl Subdivide {
    pub fn new() -> Self {
        Subdivide {
            min_dot: 0.9,
            marching_eps: 10e-10,
        }
    }
    pub fn subdivide(&mut self, mesh: &Mesh, sdf: &Sdf3) -> Mesh {
        let mut vertex_normals: Vec<Vec3> = vec![];
        for vertex in mesh.vertices() {
            vertex_normals.push(sdf.normal(*vertex));
        }
        let mut divide_edges = vec![];
        for tri in mesh.triangles() {
            for edge in tri.ordered_edges() {
                if edge.v1() < edge.v2() {
                    let dot = vertex_normals[edge.v1()].dot(vertex_normals[edge.v2()]);
                    if dot < self.min_dot {
                        divide_edges.push(edge);
                    }
                }
            }
        }
        println!("{:?}", divide_edges);
        let edge_table = EdgeTable::new(mesh);
        let mut new_vertices = mesh.vertices().to_vec();
        let mut divide_table = HashMap::<MeshEdge, usize>::new();
        for edge in divide_edges {
            divide_table.insert(edge.edge(), new_vertices.len());
            let segment = edge.for_vertices(mesh.vertices());
            let mut p = segment.midpoint();
            let [w1, w2] = edge_table
                .directed_wing(edge)
                .map(|wing| mesh.vertices()[wing.vertex()]);
            let dir = segment.displacement().cross(w1 - w2).normalize();
            let mut success = false;
            let mut ds = vec![];
            for _ in 0..100 {
                let d = sdf.evaluate(p);
                ds.push(d);
                if d < 10e-10 {
                    success = true;
                    break;
                }
                p = p + dir * d;
            }
            if !success {
                println!("fail {:#?}", ds);
            }
            new_vertices.push(p);
        }
        let mut new_tris = vec![];
        for tri in mesh.triangles() {
            let divided: [Option<usize>; 3] = tri
                .edges()
                .iter()
                .map(|edge| divide_table.get(edge).cloned())
                .collect_array()
                .unwrap();
            match (tri.vertices(), divided) {
                ([v1, v2, v3], [None, None, None]) => {
                    new_tris.push(*tri);
                }
                ([v1, v2, v3], [Some(v1v2), None, None])
                | ([v3, v1, v2], [None, Some(v1v2), None])
                | ([v2, v3, v1], [None, None, Some(v1v2)]) => {
                    new_tris.push(MeshTriangle::new(v1, v1v2, v3));
                    new_tris.push(MeshTriangle::new(v1v2, v2, v3));
                }
                ([v1, v2, v3], [Some(v1v2), Some(v2v3), None])
                | ([v3, v1, v2], [None, Some(v1v2), Some(v2v3)])
                | ([v2, v3, v1], [Some(v2v3), None, Some(v1v2)]) => {
                    new_tris.push(MeshTriangle::new(v1v2, v2, v2v3));
                    new_tris.push(MeshTriangle::new(v1, v1v2, v2v3));
                    new_tris.push(MeshTriangle::new(v3, v1, v2v3));
                }
                ([v1, v2, v3], [Some(v1v2), Some(v2v3), Some(v3v1)]) => {
                    new_tris.push(MeshTriangle::new(v1v2, v2, v2v3));
                    new_tris.push(MeshTriangle::new(v2v3, v3, v3v1));
                    new_tris.push(MeshTriangle::new(v3v1, v1, v1v2));
                    new_tris.push(MeshTriangle::new(v1v2, v2v3, v3v1));
                }
            }
            // let count = divided.iter().filter(|x| x.is_some()).count();
            // match count {
            //     0 => {
            //
            //     }
            //     1 => {}
            //     2 => {}
            //     3 => {}
            //     _ => unreachable!(),
            // }
        }
        // for tri in mesh.triangles() {
        //     let tri3 = tri.for_vertices(mesh.vertices());
        //     let mut normals = ArrayVec::<_, 3>::new();
        //     for v in tri3.points() {
        //         let normal: Vec3 =
        //         normals.push(normal);
        //     }
        //     for edge in tri.
        //     let min_dot = normals
        //         .iter()
        //         .circular_tuple_windows()
        //         .map(|(n1, n2)| NotNan::new(n1.dot(*n2)).unwrap())
        //         .min()
        //         .unwrap()
        //         .into_inner();
        //     if min_dot < self.min_dot {
        //         for edge in tri.edges() {
        //             sub_edges.insert(edge);
        //         }
        //     }
        //
        //     // let dir = tri3.normal();
        //     // let mut p = tri3.midpoint();
        //     // loop {
        //     //     let d = sdf.evaluate(p);
        //     //     if d < 10e-10 {
        //     //         break;
        //     //     }
        //     //     p = p + dir * d;
        //     // }
        //     // let [v0, v1, v2] = tri.vertices();
        //     // let v3 = vertices.len();
        //     // vertices.push(p);
        //     // tris.push(MeshTriangle::new(v0, v1, v3));
        //     // tris.push(MeshTriangle::new(v1, v2, v3));
        //     // tris.push(MeshTriangle::new(v2, v0, v3));
        // }
        // let edge_table = EdgeTable::new(mesh);
        // let new_vertices = HashMap::new();
        // for edge in sub_edges {
        //     let wing = edge_table.wing(edge);
        //     let [n1, n2] = wing.map(|t| mesh.triangles()[t].for_vertices(mesh.vertices()).normal());
        //     let normal = (n1 + n2).normalize();
        //
        //     new_vertices.isnert(edge);
        // }

        Mesh::new(new_vertices, new_tris)
    }
}
