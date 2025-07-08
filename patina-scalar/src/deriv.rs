use crate::Scalar;
use itertools::Itertools;
use std::fmt::{Display, Formatter};
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

/// A value representing both the numeric result of a computation and the derivative of that
/// computation.
#[derive(Debug, Clone)]
pub struct Deriv<const N: usize> {
    value: f64,
    deriv: [f64; N],
}

impl<const N: usize> Deriv<N> {
    pub fn constant(value: f64) -> Deriv<N> {
        Deriv {
            value,
            deriv: [0.0; N],
        }
    }
    pub fn variable(value: f64, index: usize) -> Deriv<N> {
        let mut deriv = [0.0; N];
        deriv[index] = 1.0;
        Deriv { value, deriv }
    }
    pub fn nan() -> Self {
        Deriv {
            value: std::f64::NAN,
            deriv: [std::f64::NAN; N],
        }
    }
    pub fn value(&self) -> f64 {
        self.value
    }
    pub fn deriv(&self) -> &[f64; N] {
        &self.deriv
    }
}

impl<const N: usize> Add for Deriv<N> {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Deriv {
            value: self.value + rhs.value,
            deriv: (0..N)
                .map(|axis| self.deriv[axis] + rhs.deriv[axis])
                .collect_array()
                .unwrap(),
        }
    }
}

impl<const N: usize> Neg for Deriv<N> {
    type Output = Self;
    fn neg(self) -> Self::Output {
        Deriv {
            value: -self.value,
            deriv: (0..N)
                .map(|axis| -self.deriv[axis])
                .collect_array()
                .unwrap(),
        }
    }
}

impl<const N: usize> Sub<Self> for Deriv<N> {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        Deriv {
            value: self.value - rhs.value,
            deriv: (0..N)
                .map(|axis| self.deriv[axis] - rhs.deriv[axis])
                .collect_array()
                .unwrap(),
        }
    }
}

impl<const N: usize> Mul<Self> for Deriv<N> {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        Deriv {
            value: self.value * rhs.value,
            deriv: (0..N)
                .map(|axis| self.value * rhs.deriv[axis] + self.deriv[axis] * rhs.value)
                .collect_array()
                .unwrap(),
        }
    }
}

impl<const N: usize> Div<Self> for Deriv<N> {
    type Output = Self;
    fn div(self, rhs: Self) -> Self::Output {
        Deriv {
            value: self.value / rhs.value,
            deriv: (0..N)
                .map(|axis| {
                    (self.deriv[axis] * rhs.value - self.value * rhs.deriv[axis])
                        / (rhs.value * rhs.value)
                })
                .collect_array()
                .unwrap(),
        }
    }
}

impl<const N: usize> AddAssign<Self> for Deriv<N> {
    fn add_assign(&mut self, rhs: Self) {
        *self = self.clone() + rhs
    }
}

impl<const N: usize> SubAssign<Self> for Deriv<N> {
    fn sub_assign(&mut self, rhs: Self) {
        *self = self.clone() - rhs
    }
}

impl<const N: usize> MulAssign<Self> for Deriv<N> {
    fn mul_assign(&mut self, rhs: Self) {
        *self = self.clone() * rhs
    }
}

impl<const N: usize> DivAssign<Self> for Deriv<N> {
    fn div_assign(&mut self, rhs: Self) {
        *self = self.clone() / rhs
    }
}

impl<const N: usize> Display for Deriv<N> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}<{:?}>", self.value, self.deriv)
    }
}

impl<const N: usize> Scalar for Deriv<N> {
    fn recip(self) -> Self {
        Deriv {
            value: self.value.recip(),
            deriv: (0..N)
                .map(|axis| -self.deriv[axis] / (self.value * self.value))
                .collect_array()
                .unwrap(),
        }
    }

    fn minimum(self, other: Self) -> Self {
        if self.value < other.value {
            self
        } else if self.value > other.value {
            other
        } else if self.value == other.value {
            Deriv {
                value: self.value,
                deriv: (0..N)
                    .map(|axis| {
                        if self.deriv[axis] == other.deriv[axis] {
                            self.deriv[axis]
                        } else {
                            f64::NAN
                        }
                    })
                    .collect_array()
                    .unwrap(),
            }
        } else {
            Deriv {
                value: f64::NAN,
                deriv: [f64::NAN; N],
            }
        }
    }

    fn maximum(self, other: Self) -> Self {
        if self.value < other.value {
            other
        } else if self.value > other.value {
            self
        } else if self.value == other.value {
            Deriv {
                value: self.value,
                deriv: (0..N)
                    .map(|axis| {
                        if self.deriv[axis] == other.deriv[axis] {
                            self.deriv[axis]
                        } else {
                            f64::NAN
                        }
                    })
                    .collect_array()
                    .unwrap(),
            }
        } else {
            Deriv {
                value: f64::NAN,
                deriv: [f64::NAN; N],
            }
        }
    }

    fn piecewise(self, neg: Self, pos: Self) -> Self {
        if self.value < 0.0 {
            neg
        } else if self.value > 0.0 {
            pos
        } else if self.value == 0.0 {
            let value = if neg.value == pos.value {
                pos.value
            } else {
                f64::NAN
            };
            let deriv = (0..N)
                .map(|axis| {
                    if neg.deriv[axis] == pos.deriv[axis] {
                        neg.deriv[axis]
                    } else {
                        f64::NAN
                    }
                })
                .collect_array()
                .unwrap();
            Deriv { value, deriv }
        } else {
            Deriv {
                value: f64::NAN,
                deriv: [f64::NAN; N],
            }
        }
    }

    fn from_f64(value: f64) -> Self {
        Deriv::constant(value)
    }

    fn sqrt(self) -> Self {
        let value = self.value.sqrt();
        Deriv {
            value,
            deriv: (0..N)
                .map(|axis| self.deriv[axis] / (2.0 * value))
                .collect_array()
                .unwrap(),
        }
    }

    fn abs(self) -> Self {
        if self.value < 0.0 {
            Deriv {
                value: -self.value,
                deriv: (0..N)
                    .map(|axis| -self.deriv[axis])
                    .collect_array()
                    .unwrap(),
            }
        } else if self.value > 0.0 {
            self
        } else if self.value == 0.0 {
            Deriv {
                value: 0.0,
                deriv: (0..N)
                    .map(|axis| {
                        if self.deriv[axis] == 0.0 {
                            0.0
                        } else {
                            f64::NAN
                        }
                    })
                    .collect_array()
                    .unwrap(),
            }
        } else {
            Deriv {
                value: f64::NAN,
                deriv: [f64::NAN; N],
            }
        }
    }
}
