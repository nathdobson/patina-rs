use crate::geo2::segment2::Segment2;
use crate::geo2::triangle2::Triangle2;
use crate::math::vec2::Vec2;
use crate::meshes::mesh_edge::MeshEdge;
use crate::meshes::mesh_triangle::MeshTriangle;
use ordered_float::NotNan;
use std::collections::{HashMap, HashSet};

#[derive(Debug)]
pub struct Triangulation {
    vertices: HashMap<usize, Vec2>,
    boundaries: HashSet<usize>,
    edges: HashSet<MeshEdge>,
    adjacency: HashMap<usize, HashSet<usize>>,
    exclude: HashSet<MeshEdge>,
    triangles: HashSet<[usize; 3]>,
}

impl Triangulation {
    pub fn new() -> Self {
        Triangulation {
            vertices: HashMap::new(),
            boundaries: HashSet::new(),
            edges: HashSet::new(),
            adjacency: HashMap::new(),
            exclude: HashSet::new(),
            triangles: HashSet::new(),
        }
    }
    pub fn add_vertex(&mut self, v: usize, p: Vec2) {
        self.vertices.insert(v, p);
    }
    pub fn add_boundary(&mut self, v: usize) {
        self.boundaries.insert(v);
    }
    pub fn add_edge(&mut self, v1: usize, v2: usize) {
        self.edges.insert(MeshEdge::new(v1, v2));
    }
    pub fn exclude_edge(&mut self, v1: usize, v2: usize) {
        self.exclude.insert(MeshEdge::new(v1, v2));
    }
    pub fn solve(mut self) -> Vec<MeshTriangle> {
        let mut candidate_edges = vec![];
        for &v1 in self.vertices.keys() {
            for &v2 in self.vertices.keys() {
                let e1 = MeshEdge::new(v1, v2);
                let s1 = Segment2::new(self.vertices[&v1], self.vertices[&v2]);
                if v1 == v2 {
                    continue;
                }
                if self.edges.contains(&e1) {
                    continue;
                }
                if self.exclude.contains(&e1) {
                    continue;
                }
                candidate_edges.push((e1, s1));
            }
        }
        candidate_edges.sort_by_cached_key(|(e1, s1)| {
            (NotNan::new(s1.as_ray().dir().length()).unwrap(), *e1)
        });
        'next_edge: for (e1, s1) in candidate_edges {
            for e2 in self.edges.iter() {
                if e1.shares_vertex(e2) {
                    continue;
                }
                let s2 = Segment2::new(
                    self.vertices[&e2.vertices()[0]],
                    self.vertices[&e2.vertices()[1]],
                );
                if s1.intersect_time(s2).is_some() {
                    continue 'next_edge;
                }
            }
            self.edges.insert(e1);
        }
        for e in &self.edges {
            let [v1, v2] = e.vertices();
            self.adjacency.entry(v1).or_default().insert(v2);
            self.adjacency.entry(v2).or_default().insert(v1);
        }
        for (&v1, ns1) in self.adjacency.iter() {
            for &v2 in self.adjacency.get(&v1).unwrap() {
                'next_triangle: for &v3 in self.adjacency.get(&v2).unwrap() {
                    if ns1.contains(&v3) {
                        let mut tri = [v1, v2, v3];
                        tri.sort();
                        let tri2 = Triangle2::new(tri.map(|v| self.vertices[&v]));
                        for (&v4, &p4) in &self.vertices {
                            if tri.iter().any(|&v| v == v4) {
                                continue;
                            }
                            if tri2.intersects_point(p4) {
                                continue 'next_triangle;
                            }
                        }
                        self.triangles.insert(tri);
                    }
                }
            }
        }
        let mut result = vec![];
        for &tri in self.triangles.iter() {
            let tri2 = Triangle2::new(tri.map(|v| self.vertices[&v]));
            let mut mtri = MeshTriangle::from(tri);
            if tri2.area() < 0.0 {
                mtri.invert();
            }
            result.push(mtri);
        }
        result.sort();
        result
    }
}
