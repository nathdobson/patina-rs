use std::cmp::Ordering;
use std::fmt::{Debug, Formatter};
use std::ops::BitAnd;

#[derive(Copy, Clone)]
pub struct FloatBool(f64);

impl FloatBool {
    pub fn new(x: f64) -> Self {
        assert!(!x.is_nan());
        FloatBool(x.clamp(0.0, 1.0))
    }
    pub fn and(self, other: Self) -> Self {
        FloatBool(self.0.min(other.0))
    }
    pub fn or(self, other: Self) -> Self {
        FloatBool(self.0.max(other.0))
    }
    pub fn not(self) -> Self {
        Self::new(1.0 - self.0)
    }
    pub fn maybe(self) -> bool {
        self.0 > 0.0
    }
    pub fn xor(self, other: Self) -> Self {
        FloatBool::new(self.0 + other.0 - 2.0 * self.0 * other.0)
    }
    pub fn round(self) -> bool {
        self.0 > 0.5
    }
    pub fn is_true(self) -> bool {
        self.0 == 1.0
    }
    pub fn is_false(self) -> bool {
        self.0 == 0.0
    }
    pub fn definite(self) -> bool {
        self.is_false() || self.is_true()
    }
    pub fn matches(self, other: Self) -> bool {
        (self.is_false() && other.is_false())
            || (self.is_true() && other.is_true())
            || (!self.definite() && !other.definite())
    }
}

#[derive(Copy, Clone, Debug)]
pub struct Epsilon(f64);

impl Epsilon {
    pub fn new(x: f64) -> Self {
        assert!(x >= 0.0);
        assert!(x <= 1.0);
        Epsilon(x)
    }
    pub fn less(&self, a: f64, b: f64) -> FloatBool {
        FloatBool::new((b - a) / self.0 + 0.5)
    }
}

impl From<bool> for FloatBool {
    fn from(value: bool) -> Self {
        Self::new(value as u8 as f64)
    }
}

impl Debug for FloatBool {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:.5}", self.0)
    }
}

impl PartialEq<Self> for FloatBool {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl Eq for FloatBool {}

impl PartialOrd for FloatBool {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.0.partial_cmp(&other.0)
    }
}

impl Ord for FloatBool {
    fn cmp(&self, other: &Self) -> Ordering {
        self.0.partial_cmp(&other.0).unwrap()
    }
}
