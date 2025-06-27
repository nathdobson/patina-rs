use crate::math::disjoint_paths::DisjointPaths;
use std::collections::{HashMap, HashSet};

#[derive(Debug)]
pub struct LoopBuilder {
    strong_forward: HashMap<usize, usize>,
    strong_reverse: HashMap<usize, usize>,
    weak: DisjointPaths,
}

impl LoopBuilder {
    pub fn new() -> Self {
        LoopBuilder {
            strong_forward: HashMap::new(),
            strong_reverse: HashMap::new(),
            weak: DisjointPaths::new(),
        }
    }
    pub fn add_vertex(&mut self, vertex: usize) {
        self.weak.add_vertex(vertex);
    }
    pub fn add_strong(&mut self, v1: usize, v2: usize) {
        assert!(self.strong_forward.insert(v1, v2).is_none());
        assert!(self.strong_reverse.insert(v2, v1).is_none());
    }
    pub fn add_weak(&mut self, v1: usize, v2: usize) {
        self.weak.add_edge(v1, v2);
    }
    pub fn solve(mut self) -> Vec<Vec<usize>> {
        let mut partial_loops = vec![];
        let mut full_loops = vec![];
        let mut visited = HashSet::new();
        for (&i1, &i2) in self.strong_forward.iter() {
            if !visited.insert(i1) {
                continue;
            }
            let mut start = i1;
            while let Some(&prev) = self.strong_reverse.get(&start) {
                start = prev;
                if start == i1 {
                    break;
                }
            }
            let mut loop1 = vec![];
            let mut prev = start;
            loop {
                loop1.push(prev);
                visited.insert(prev);
                if let Some(&next) = self.strong_forward.get(&prev) {
                    prev = next;
                    if next == start {
                        full_loops.push(loop1);
                        break;
                    }
                } else {
                    partial_loops.push(loop1);
                    break;
                }
            }
        }
        for partial_loop in &partial_loops {
            let begin = *partial_loop.first().unwrap();
            let end = *partial_loop.last().unwrap();
            self.weak.add_begin(end);
            self.weak.add_end(begin);
        }
        self.weak.solve();
        let weak_paths = self.weak.paths();
        let mut weak_path_table = HashMap::new();
        for weak_path in weak_paths {
            let begin = *weak_path.first().unwrap();
            weak_path_table.insert(begin, weak_path);
        }
        for mut partial_loop in partial_loops {
            let cont = weak_path_table
                .get(partial_loop.last().unwrap())
                .expect("Missing continuation");
            assert_eq!(
                cont.last(),
                partial_loop.first(),
                "{:?} vs {:?}",
                cont,
                partial_loop
            );
            partial_loop.extend(&cont[1..cont.len() - 1]);
            full_loops.push(partial_loop);
        }
        full_loops
    }
}
