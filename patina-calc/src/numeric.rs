use crate::operator::{OperatorBinary, OperatorUnary};
use inari::DecInterval;
use std::ops::{Add, Div, Mul, Neg, Sub};

/// Types that behave like a number (e.g. [f64] and [inari::DecInterval]).
pub trait Numeric:
    Clone
    + Neg<Output = Self>
    + Add<Self, Output = Self>
    + Sub<Self, Output = Self>
    + Mul<Self, Output = Self>
    + Div<Self, Output = Self>
{
    fn recip(self) -> Self;
    fn min(self, other: Self) -> Self;
    fn max(self, other: Self) -> Self;
    fn piecewise(self, neg: Self, pos: Self) -> Self;
    fn from_f64(value: f64) -> Self;
}

impl Numeric for f64 {
    fn recip(self) -> Self {
        f64::recip(self)
    }

    fn min(self, other: Self) -> Self {
        f64::min(self, other)
    }

    fn max(self, other: Self) -> Self {
        f64::max(self, other)
    }

    fn piecewise(self, neg: Self, pos: Self) -> Self {
        if self < 0.0 { neg } else { pos }
    }

    fn from_f64(value: f64) -> Self {
        value
    }
}

impl Numeric for DecInterval {
    fn recip(self) -> Self {
        DecInterval::recip(self)
    }

    fn min(self, other: Self) -> Self {
        DecInterval::min(self, other)
    }

    fn max(self, other: Self) -> Self {
        DecInterval::max(self, other)
    }

    fn piecewise(self, neg: Self, pos: Self) -> Self {
        if self.precedes(Self::from_f64(0.0)) {
            neg
        } else if Self::from_f64(1.0).precedes(Self::from_f64(0.0)) {
            pos
        } else {
            neg.convex_hull(pos)
        }
    }

    fn from_f64(value: f64) -> Self {
        DecInterval::try_from((value, value)).unwrap()
    }
}
