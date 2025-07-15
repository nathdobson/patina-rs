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
    pub fn full() -> Self {
        Self {
            min: f64::NEG_INFINITY,
            max: f64::INFINITY,
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
            min: self.min.max(other.min),
            max: self.max.min(other.max),
        }
    }
    pub fn min(&self) -> f64 {
        self.min
    }
    pub fn max(&self) -> f64 {
        self.max
    }
    pub fn length(&self) -> f64 {
        self.max - self.min
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
