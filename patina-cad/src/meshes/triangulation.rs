use crate::geo2::segment2::Segment2;
use crate::geo2::triangle2::Triangle2;
use crate::math::vec2::Vec2;
use crate::meshes::mesh_edge::MeshEdge;
use crate::meshes::mesh_triangle::MeshTriangle;
use ordered_float::NotNan;
use std::collections::hash_map::Entry;
use std::collections::{HashMap, HashSet};

#[derive(Debug)]
pub struct Triangulation {
    vertices: HashMap<usize, Vec2>,
    adjacency: HashMap<usize, HashSet<usize>>,
    exclude: HashSet<MeshEdge>,
    triangles: Vec<MeshTriangle>,
    boundaries_forward: HashMap<usize, usize>,
}

impl Triangulation {
    pub fn new() -> Self {
        Triangulation {
            vertices: HashMap::new(),
            adjacency: HashMap::new(),
            exclude: HashSet::new(),
            triangles: vec![],
            boundaries_forward: HashMap::new(),
        }
    }
    pub fn add_vertex(&mut self, v: usize, p: Vec2) {
        self.vertices.insert(v, p);
    }
    pub fn add_boundary(&mut self, v1: usize, v2: usize) {
        self.boundaries_forward.insert(v1, v2);
    }
    pub fn add_edge(&mut self, v1: usize, v2: usize) {
        self.adjacency.entry(v1).or_default().insert(v2);
        self.adjacency.entry(v2).or_default().insert(v1);
    }
    pub fn exclude_edge(&mut self, v1: usize, v2: usize) {
        self.exclude.insert(MeshEdge::new(v1, v2));
    }
    pub fn solve(mut self) -> Vec<MeshTriangle> {
        while let Some((&v1, &v2)) = self.boundaries_forward.iter().next() {
            let mut v3: Option<usize> = None;
            'best_search: for &v4 in self.vertices.keys() {
                let mt = MeshTriangle::new(v1, v2, v4);
                if v1 == v4 || v2 == v4 {
                    continue;
                }
                if self.boundaries_forward.get(&v4).is_some() {
                    if !self.boundaries_forward[&v4] == v1 && !self.boundaries_forward[&v2] == v4 {
                        continue;
                    }
                }
                for e in mt.edges() {
                    if self.exclude.contains(&e) {
                        continue 'best_search;
                    }
                }
                if let Some(v3) = v3 {
                    let inside1 = (self.vertices[&v3] - self.vertices[&v1])
                        .cross(self.vertices[&v4] - self.vertices[&v1])
                        < 0.0;
                    let inside2 = (self.vertices[&v3] - self.vertices[&v2])
                        .cross(self.vertices[&v4] - self.vertices[&v2])
                        > 0.0;
                    match (inside1, inside2) {
                        (false, false) => {
                            //totally outside
                            continue 'best_search;
                        }
                        (true, true) => {
                            //totally inside
                        }
                        (false, true) => {
                            if self.adjacency[&v1].contains(&v3) {
                                //previous is forced by required edge
                                continue 'best_search;
                            }
                        }
                        (true, false) => {
                            if self.adjacency[&v2].contains(&v3) {
                                //previous is forced by required edge
                                continue 'best_search;
                            }
                        }
                    }
                }
                v3 = Some(v4);
            }
            let v3 = v3.unwrap();
            println!("Cutting {} {} {}", v1, v2, v3);
            println!("{:?}", self.boundaries_forward);
            if self.boundaries_forward.get(&v3).is_some() {
                let ear1 = self.boundaries_forward[&v3] == v1;
                let ear2 = self.boundaries_forward[&v2] == v3;
                if ear1 && ear2 {
                    self.boundaries_forward.remove(&v1);
                    self.boundaries_forward.remove(&v2);
                    self.boundaries_forward.remove(&v3);
                } else if ear1 {
                    self.boundaries_forward.remove(&v1);
                    self.boundaries_forward.insert(v3, v2);
                } else if ear2 {
                    self.boundaries_forward.remove(&v2);
                    self.boundaries_forward.insert(v1, v3);
                } else {
                    unreachable!();
                }
            } else {
                self.boundaries_forward.insert(v1, v3);
                self.boundaries_forward.insert(v3, v2);
            }
            self.add_edge(v1, v3);
            self.add_edge(v2, v3);
            self.exclude.insert(MeshEdge::new(v1, v2));
            self.triangles.push(MeshTriangle::new(v1, v2, v3));
            println!("{:?}", self.boundaries_forward);
        }
        self.triangles
        // let mut candidate_edges = vec![];
        // for &v1 in self.vertices.keys() {
        //     for &v2 in self.vertices.keys() {
        //         let e1 = MeshEdge::new(v1, v2);
        //         let s1 = Segment2::new(self.vertices[&v1], self.vertices[&v2]);
        //         if v1 == v2 {
        //             continue;
        //         }
        //         if self.edges.contains(&e1) {
        //             continue;
        //         }
        //         if self.exclude.contains(&e1) {
        //             continue;
        //         }
        //         candidate_edges.push((e1, s1));
        //     }
        // }
        // candidate_edges
        //     .sort_by_cached_key(|(e1, s1)| (NotNan::new(s1.as_ray().dir().length()).unwrap(), *e1));
        // 'next_edge: for (e1, s1) in candidate_edges {
        //     for e2 in self.edges.iter() {
        //         if e1.shares_vertex(e2) {
        //             continue;
        //         }
        //         let s2 = Segment2::new(
        //             self.vertices[&e2.vertices()[0]],
        //             self.vertices[&e2.vertices()[1]],
        //         );
        //         if s1.intersect_time(s2).is_some() {
        //             continue 'next_edge;
        //         }
        //     }
        //     self.edges.insert(e1);
        // }
        // for e in &self.edges {
        //     let [v1, v2] = e.vertices();
        //     self.adjacency.entry(v1).or_default().insert(v2);
        //     self.adjacency.entry(v2).or_default().insert(v1);
        // }
        // for (&v1, ns1) in self.adjacency.iter() {
        //     for &v2 in self.adjacency.get(&v1).unwrap() {
        //         'next_triangle: for &v3 in self.adjacency.get(&v2).unwrap() {
        //             if ns1.contains(&v3) {
        //                 let mut tri = [v1, v2, v3];
        //                 tri.sort();
        //                 let tri2 = Triangle2::new(tri.map(|v| self.vertices[&v]));
        //                 for (&v4, &p4) in &self.vertices {
        //                     if tri.iter().any(|&v| v == v4) {
        //                         continue;
        //                     }
        //                     if tri2.intersects_point(p4) {
        //                         continue 'next_triangle;
        //                     }
        //                 }
        //                 self.triangles.insert(tri);
        //             }
        //         }
        //     }
        // }
        // let mut result = vec![];
        // for &tri in self.triangles.iter() {
        //     let tri2 = Triangle2::new(tri.map(|v| self.vertices[&v]));
        //     let mut mtri = MeshTriangle::from(tri);
        //     if tri2.area() < 0.0 {
        //         mtri.invert();
        //     }
        //     result.push(mtri);
        // }
        // result.sort();
        // result
    }
}
