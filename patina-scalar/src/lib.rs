#![feature(float_minimum_maximum)]

pub mod deriv;
#[cfg(feature = "inari")]
mod impl_inari;
pub mod newton;

use std::fmt::{Debug, Display};
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

#[test]
fn test() {}

/// Types that behave like a number (e.g. [f64] and [impl_inari::DecInterval]).
pub trait Scalar:
    Clone
    + Neg<Output = Self>
    + Add<Self, Output = Self>
    + Sub<Self, Output = Self>
    + Mul<Self, Output = Self>
    + Div<Self, Output = Self>
    + AddAssign<Self>
    + SubAssign<Self>
    + MulAssign<Self>
    + DivAssign<Self>
    + Debug
    + Display
{
    fn recip(self) -> Self;
    fn minimum(self, other: Self) -> Self;
    fn maximum(self, other: Self) -> Self;
    fn piecewise(self, neg: Self, pos: Self) -> Self;
    fn from_f64(value: f64) -> Self;
    fn sqrt(self) -> Self;
    fn abs(self) -> Self;
}

impl Scalar for f64 {
    fn recip(self) -> Self {
        f64::recip(self)
    }

    fn minimum(self, other: Self) -> Self {
        f64::minimum(self, other)
    }

    fn maximum(self, other: Self) -> Self {
        f64::maximum(self, other)
    }

    fn piecewise(self, neg: Self, pos: Self) -> Self {
        if self < 0.0 { neg } else { pos }
    }

    fn from_f64(value: f64) -> Self {
        value
    }

    fn sqrt(self) -> Self {
        f64::sqrt(self)
    }

    fn abs(self) -> Self {
        f64::abs(self)
    }
}
