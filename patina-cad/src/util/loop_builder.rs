use std::collections::HashMap;
use std::hash::Hash;

#[derive(Clone, Debug)]
pub struct LoopBuilder<T> {
    inner: HashMap<T, T>,
}

impl<T: Eq + Hash + Clone> LoopBuilder<T> {
    pub fn new() -> Self {
        LoopBuilder {
            inner: HashMap::new(),
        }
    }
    pub fn insert(&mut self, a: T, b: T) {
        assert!(self.inner.insert(a, b).is_none());
    }
    pub fn build(mut self) -> Vec<Vec<T>> {
        let mut result = vec![];
        while let Some(start) = self.inner.keys().next() {
            let start = start.clone();
            let mut next = start.clone();
            let mut loopv = vec![];
            loop {
                loopv.push(next.clone());
                next = self.inner.remove(&next).unwrap();
                if next == start {
                    break;
                }
            }
            result.push(loopv);
        }
        result
    }
}
