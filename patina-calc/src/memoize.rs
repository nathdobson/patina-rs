use std::collections::HashMap;
use std::collections::hash_map::Entry;
use std::hash::Hash;

#[derive(Debug)]
pub struct Memoize<K, V> {
    map: HashMap<K, Option<V>>,
}

impl<K: Eq + Hash, V: Clone> Memoize<K, V> {
    pub fn new() -> Memoize<K, V> {
        Memoize {
            map: HashMap::new(),
        }
    }
    pub fn begin(&mut self, key: K) -> Option<V> {
        match self.map.entry(key) {
            Entry::Occupied(o) => Some(
                o.into_mut()
                    .as_ref()
                    .unwrap_or_else(|| panic!("self referential memoization"))
                    .clone(),
            ),
            Entry::Vacant(v) => {
                v.insert(None);
                None
            }
        }
    }
    pub fn end(&mut self, key: &K, value: V) -> V {
        let value_mut = self
            .map
            .get_mut(&key)
            .unwrap_or_else(|| panic!("Called end before start"));
        *value_mut = Some(value);
        value_mut.as_ref().unwrap().clone()
    }
}
