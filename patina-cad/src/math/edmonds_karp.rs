use itertools::Itertools;
use ordered_float::NotNan;
use std::collections::hash_map::Entry;
use std::collections::{HashMap, HashSet};

#[derive(Default, Debug)]
struct EdgeLabel {
    flow: f64,
    capacity: f64,
}
#[derive(Debug)]
pub struct EdmondsKarp {
    edges: HashMap<usize, HashMap<usize, EdgeLabel>>,
    sources: HashSet<usize>,
    sinks: HashSet<usize>,
}

#[derive(Default)]
struct EdgeFlow {
    prev: Option<usize>,
    flow: f64,
}

impl EdmondsKarp {
    pub fn new() -> Self {
        EdmondsKarp {
            edges: HashMap::new(),
            sources: HashSet::new(),
            sinks: HashSet::new(),
        }
    }
    pub fn add_source(&mut self, source: usize) {
        self.sources.insert(source);
    }
    pub fn add_sink(&mut self, sink: usize) {
        self.sinks.insert(sink);
    }
    pub fn add_capacity(&mut self, from: usize, to: usize, cap: f64) {
        self.edges
            .entry(from)
            .or_default()
            .entry(to)
            .or_default()
            .capacity += cap;
        self.edges.entry(to).or_default().entry(from).or_default();
    }
    fn find_augmenting_path(&mut self) -> Option<(f64, Vec<usize>)> {
        let mut frontier: HashMap<usize, EdgeFlow> = self
            .sources
            .iter()
            .map(|&next| {
                (
                    next,
                    EdgeFlow {
                        prev: None,
                        flow: f64::INFINITY,
                    },
                )
            })
            .collect();
        let mut visited = HashSet::new();
        let mut frontiers = vec![];
        loop {
            if frontier.len() == 0 {
                return None;
            }
            let mut new_frontier = HashMap::<usize, EdgeFlow>::new();
            for (&prev, flow) in frontier.iter() {
                if visited.insert(prev) {
                    if let Some(nexts) = self.edges.get(&prev) {
                        for (&next, edge) in nexts {
                            let residual = (edge.capacity - edge.flow).min(flow.flow);
                            if residual > 0.0 {
                                let new_flow = EdgeFlow {
                                    prev: Some(prev),
                                    flow: residual,
                                };
                                let old_flow = new_frontier.entry(next).or_default();
                                if old_flow.flow < new_flow.flow {
                                    *old_flow = new_flow;
                                }
                            }
                        }
                    }
                }
            }
            frontiers.push(frontier);
            if let Some((&end, flow)) = frontiers
                .last()
                .unwrap()
                .iter()
                .filter(|&(&next, flow)| self.sinks.contains(&next))
                .max_by_key(|&(&next, flow)| NotNan::new(flow.flow).unwrap())
            {
                let mut rev_path = vec![];
                let mut second = Some(end);
                let mut frontier = frontiers.iter().rev();
                while let Some(first) = second {
                    rev_path.push(first);
                    second = frontier.next().unwrap().get(&first).unwrap().prev;
                }
                rev_path.reverse();
                return Some((flow.flow, rev_path));
            }
            frontier = new_frontier;
        }
    }
    pub fn solve(&mut self) {
        while let Some((flow, augmenting_path)) = self.find_augmenting_path() {
            assert!(flow > 0.0);
            for (&from, &to) in augmenting_path.iter().tuple_windows() {
                self.edges
                    .entry(from)
                    .or_default()
                    .entry(to)
                    .or_default()
                    .flow += flow;
                self.edges
                    .entry(to)
                    .or_default()
                    .entry(from)
                    .or_default()
                    .flow -= flow;
            }
        }
    }
    pub fn flows_from(&self, from: usize) -> impl Iterator<Item = (usize, f64)> {
        self.edges
            .get(&from)
            .unwrap()
            .iter()
            .filter_map(|(&to, flow)| (flow.flow > 0.0).then_some((to, flow.flow)))
    }
    fn test_flows(&self) -> Vec<(usize, usize, f64)> {
        let mut result = vec![];
        for &from in self.edges.keys() {
            for (to, flow) in self.flows_from(from) {
                result.push((from, to, flow));
            }
        }
        result.sort_by_key(|&(from, to, flow)| (from, to));
        result
    }
}

#[test]
fn test_edge_cases() {
    let mut ek = EdmondsKarp::new();
    ek.solve();

    let mut ek = EdmondsKarp::new();
    ek.add_source(0);
    ek.solve();

    let mut ek = EdmondsKarp::new();
    ek.add_sink(0);
    ek.solve();
}

#[test]
fn test_basic() {
    let mut ek = EdmondsKarp::new();
    ek.add_source(0);
    ek.add_capacity(0, 1, 1.0);
    ek.add_sink(1);
    ek.solve();
    assert_eq!(ek.flows_from(0).collect::<Vec<_>>(), vec![(1, 1.0)]);
    assert_eq!(ek.flows_from(1).collect::<Vec<_>>(), vec![]);
}

#[test]
fn test_multiple() {
    let mut ek = EdmondsKarp::new();
    ek.add_source(0);
    ek.add_source(1);
    ek.add_capacity(0, 2, 1.5);
    ek.add_capacity(1, 3, 2.5);
    ek.add_sink(2);
    ek.add_sink(3);
    ek.solve();
    assert_eq!(ek.flows_from(0).collect::<Vec<_>>(), vec![(2, 1.5)]);
    assert_eq!(ek.flows_from(1).collect::<Vec<_>>(), vec![(3, 2.5)]);
    assert_eq!(ek.flows_from(2).collect::<Vec<_>>(), vec![]);
    assert_eq!(ek.flows_from(3).collect::<Vec<_>>(), vec![]);
}

#[test]
fn test_augment() {
    let mut ek = EdmondsKarp::new();
    ek.add_source(0);
    ek.add_capacity(0, 1, 3.0);
    ek.add_capacity(0, 3, 3.0);
    ek.add_capacity(1, 2, 4.0);
    ek.add_capacity(2, 0, 3.0);
    ek.add_capacity(2, 3, 1.0);
    ek.add_capacity(2, 4, 2.0);
    ek.add_capacity(3, 4, 2.0);
    ek.add_capacity(3, 5, 6.0);
    ek.add_capacity(4, 1, 1.0);
    ek.add_capacity(4, 6, 1.0);
    ek.add_capacity(5, 6, 9.0);

    ek.add_sink(6);
    ek.solve();
    assert_eq!(
        ek.test_flows(),
        vec![
            (0, 1, 2.0),
            (0, 3, 3.0),
            (1, 2, 2.0),
            (2, 3, 1.0),
            (2, 4, 1.0),
            (3, 5, 4.0),
            (4, 6, 1.0),
            (5, 6, 4.0)
        ]
    );
    // assert_eq!(ek.flows_from(3).collect::<Vec<_>>(), vec![]);
    // assert_eq!(ek.flows_from(4).collect::<Vec<_>>(), vec![]);
    // assert_eq!(ek.flows_from(5).collect::<Vec<_>>(), vec![]);
    // assert_eq!(ek.flows_from(6).collect::<Vec<_>>(), vec![]);
    // assert_eq!(ek.flows_from(1).collect::<Vec<_>>(), vec![(2, 3.0)]);
    // assert_eq!(ek.flows_from(2).collect::<Vec<_>>(), vec![(3, 3.0)]);
    // assert_eq!(ek.flows_from(3).collect::<Vec<_>>(), vec![]);
}
