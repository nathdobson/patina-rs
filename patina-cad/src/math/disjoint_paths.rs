use crate::math::edmonds_karp_vertex::EdmondsKarpVertex;
use itertools::Itertools;
use std::iter;

#[derive(Debug)]
pub struct DisjointPaths {
    ekv: EdmondsKarpVertex,
    begins: Vec<usize>,
}

impl DisjointPaths {
    pub fn new(vertices: usize) -> Self {
        let mut dp = DisjointPaths {
            ekv: EdmondsKarpVertex::new(vertices),
            begins: vec![],
        };
        dp
    }
    pub fn add_begin(&mut self, begin: usize) {
        self.begins.push(begin);
        self.ekv.add_source(begin);
    }
    pub fn add_end(&mut self, end: usize) {
        self.ekv.add_sink(end);
    }
    pub fn add_vertex(&mut self, vertex: usize) {
        self.ekv.add_vertex_capacity(vertex, 1.0);
    }
    pub fn add_edge(&mut self, from: usize, to: usize) {
        self.ekv.add_edge_capacity(from, to, 1.0)
    }
    pub fn solve(&mut self) {
        self.ekv.solve();
    }
    pub fn paths(&self) -> Vec<Vec<usize>> {
        let mut paths = vec![];
        for &begin in &self.begins {
            let mut path = vec![];
            let mut prev = begin;
            while let Some((next, _)) = self.ekv.edge_flows(prev).at_most_one().ok().unwrap() {
                path.push(prev);
                prev = next;
            }
            path.push(prev);
            if path.len() > 1 {
                paths.push(path);
            }
        }
        paths
    }
}
