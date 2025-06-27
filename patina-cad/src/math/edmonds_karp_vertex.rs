use crate::math::edmonds_karp::EdmondsKarp;
use itertools::Itertools;

#[derive(Debug)]
pub struct EdmondsKarpVertex {
    ek: EdmondsKarp,
}

impl EdmondsKarpVertex {
    pub fn new() -> Self {
        let mut ekv = EdmondsKarpVertex {
            ek: EdmondsKarp::new(),
        };
        ekv
    }
    pub fn add_vertex_capacity(&mut self, vertex: usize, capacity: f64) {
        self.ek.add_capacity(vertex * 2, vertex * 2 + 1, capacity);
    }
    pub fn add_edge_capacity(&mut self, v1: usize, v2: usize, capacity: f64) {
        self.ek.add_capacity(v1 * 2 + 1, v2 * 2, capacity);
    }
    pub fn add_source(&mut self, vertex: usize) {
        self.ek.add_source(vertex * 2)
    }
    pub fn add_sink(&mut self, vertex: usize) {
        self.ek.add_sink(vertex * 2 + 1)
    }
    pub fn solve(&mut self) {
        self.ek.solve();
    }
    pub fn vertex_flow(&self, vertex: usize) -> f64 {
        self.ek
            .flows_from(vertex * 2)
            .at_most_one()
            .ok()
            .unwrap()
            .map(|x| x.1)
            .unwrap_or(0.0)
    }
    pub fn edge_flows(&self, from: usize) -> impl Iterator<Item = (usize, f64)> {
        self.ek.flows_from(from * 2 + 1).map(|(to, flow)| {
            assert_eq!(to % 2, 0);
            (to / 2, flow)
        })
    }
}

#[test]
fn test_basic() {
    let mut ekv = EdmondsKarpVertex::new();
    ekv.add_source(0);
    ekv.add_vertex_capacity(0, 1.0);
    ekv.add_edge_capacity(0, 1, 2.0);
    ekv.add_vertex_capacity(1, 3.0);
    ekv.add_sink(1);
    ekv.solve();
    assert_eq!(ekv.vertex_flow(0), 1.0);
    assert_eq!(ekv.edge_flows(0).collect::<Vec<_>>(), vec![(1, 1.0)]);
    assert_eq!(ekv.vertex_flow(1), 1.0);
}
