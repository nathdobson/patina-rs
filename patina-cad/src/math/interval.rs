use std::fmt::{Debug, Formatter};

#[derive(Copy, Clone)]
pub struct Interval {
    min: f64,
    max: f64,
}

impl Interval {
    pub fn new(min: f64, max: f64) -> Self {
        Interval { min, max }
    }
    pub fn empty() -> Self {
        Self {
            min: f64::INFINITY,
            max: f64::NEG_INFINITY,
        }
    }
    pub fn union(self, other: Self) -> Self {
        Interval {
            min: self.min.min(other.min),
            max: self.max.max(other.max),
        }
    }
    pub fn intersect(self, other: Self) -> Self {
        Interval {
            min: self.max.max(other.min),
            max: self.min.min(other.max),
        }
    }
    pub fn intersects(self, other: Self) -> bool {
        let i = self.intersect(other);
        !i.is_empty()
    }
    pub fn is_empty(&self) -> bool {
        self.min > self.max
    }
}

impl From<f64> for Interval {
    fn from(value: f64) -> Self {
        Interval {
            min: value,
            max: value,
        }
    }
}

impl Debug for Interval {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        [self.min, self.max].fmt(f)
    }
}
