use std::fmt::{Debug, Formatter};

#[derive(Eq, Ord, PartialEq, PartialOrd, Clone, Copy, Hash)]
pub struct SortedPair<T>([T; 2]);

impl<T: Ord> SortedPair<T> {
    pub fn new(a: T, b: T) -> Self {
        if a <= b {
            SortedPair([a, b])
        } else {
            SortedPair([b, a])
        }
    }
    pub fn first(&self) -> &T {
        &self.0[0]
    }
    pub fn second(&self) -> &T {
        &self.0[1]
    }
    pub fn inner(&self) -> &[T; 2] {
        &self.0
    }
    pub fn into_inner(self) -> [T; 2] {
        self.0
    }
}

impl<T: Ord> From<[T; 2]> for SortedPair<T> {
    fn from([v1, v2]: [T; 2]) -> Self {
        Self::new(v1, v2)
    }
}

impl<T: Debug> Debug for SortedPair<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{{{:?}, {:?}}}", &self.0[0], &self.0[1])
    }
}
