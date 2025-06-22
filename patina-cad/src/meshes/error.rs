use std::fmt::{Debug, Display, Formatter};

#[derive(Debug, Eq, Ord, PartialEq, PartialOrd, Clone, Copy, Hash)]
pub enum ManifoldError {
    // A triangle contains the same vertex at least twice.
    DuplicateVertex,
    // A vertex occurs in no triangles.
    MissingVertex,
    // The fan around a vertex is not connected.
    BrokenFan(usize, usize),
    // The fan around a vertex has extra triangles.
    SplitFan(usize),
    // There is more than one fan around a vertex.
    DuplicateFan,
    // A triangle contains an unknown vertex.
    BadVertex,
    // A triangle has no area
    // EmptyTriangle,
}

impl std::error::Error for ManifoldError {}

impl Display for ManifoldError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Debug::fmt(self, f)
    }
}
