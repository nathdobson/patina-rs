use patina_scalar::Scalar;
use std::fmt::{Display, Formatter};
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

#[derive(Debug, Copy, Clone)]
pub struct Deriv {
    value: f64,
    deriv: f64,
}

impl Deriv {
    pub fn constant(value: f64) -> Deriv {
        Deriv { value, deriv: 0.0 }
    }
    pub fn variable(value: f64) -> Deriv {
        Deriv { value, deriv: 1.0 }
    }
    pub fn nan() -> Deriv {
        Deriv {
            value: std::f64::NAN,
            deriv: std::f64::NAN,
        }
    }
    pub fn value(&self) -> f64 {
        self.value
    }
    pub fn deriv(&self) -> f64 {
        self.deriv
    }
}

impl Add for Deriv {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Deriv {
            value: self.value + rhs.value,
            deriv: self.deriv + rhs.deriv,
        }
    }
}

impl Neg for Deriv {
    type Output = Self;
    fn neg(self) -> Self::Output {
        Deriv {
            value: -self.value,
            deriv: -self.deriv,
        }
    }
}

impl Sub<Self> for Deriv {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        Deriv {
            value: self.value - rhs.value,
            deriv: self.deriv - rhs.deriv,
        }
    }
}

impl Mul<Self> for Deriv {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        Deriv {
            value: self.value * rhs.value,
            deriv: self.value * rhs.deriv + self.deriv * rhs.value,
        }
    }
}

impl Div<Self> for Deriv {
    type Output = Self;
    fn div(self, rhs: Self) -> Self::Output {
        Deriv {
            value: self.value / rhs.value,
            deriv: (self.deriv * rhs.value - self.value * rhs.deriv) / (rhs.value * rhs.value),
        }
    }
}

impl AddAssign<Self> for Deriv {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs
    }
}

impl SubAssign<Self> for Deriv {
    fn sub_assign(&mut self, rhs: Self) {
        *self = *self - rhs
    }
}

impl MulAssign<Self> for Deriv {
    fn mul_assign(&mut self, rhs: Self) {
        *self = *self * rhs
    }
}

impl DivAssign<Self> for Deriv {
    fn div_assign(&mut self, rhs: Self) {
        *self = *self / rhs
    }
}

impl Display for Deriv {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}<{}>", self.value, self.deriv)
    }
}

impl Scalar for Deriv {
    fn recip(self) -> Self {
        Deriv {
            value: self.value.recip(),
            deriv: -self.deriv / (self.value * self.value),
        }
    }

    fn minimum(self, other: Self) -> Self {
        if self.value < other.value {
            self
        } else if self.value > other.value {
            other
        } else if self.value == other.value {
            if self.deriv == other.deriv {
                self
            } else {
                Deriv {
                    value: self.value,
                    deriv: f64::NAN,
                }
            }
        } else {
            Deriv {
                value: f64::NAN,
                deriv: f64::NAN,
            }
        }
    }

    fn maximum(self, other: Self) -> Self {
        if self.value < other.value {
            other
        } else if self.value > other.value {
            self
        } else if self.value == other.value {
            if self.deriv == other.deriv {
                self
            } else {
                Deriv {
                    value: self.value,
                    deriv: f64::NAN,
                }
            }
        } else {
            Deriv {
                value: f64::NAN,
                deriv: f64::NAN,
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
            let deriv = if neg.deriv == pos.deriv {
                pos.deriv
            } else {
                f64::NAN
            };
            Deriv { value, deriv }
        } else {
            Deriv {
                value: f64::NAN,
                deriv: f64::NAN,
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
            deriv: self.deriv / (2.0 * value),
        }
    }

    fn abs(self) -> Self {
        if self.value < 0.0 {
            Deriv {
                value: -self.value,
                deriv: -self.deriv,
            }
        } else if self.value > 0.0 {
            self
        } else if self.value == 0.0 {
            if self.deriv == 0.0 {
                Deriv {
                    value: 0.0,
                    deriv: 0.0,
                }
            } else {
                Deriv {
                    value: 0.0,
                    deriv: f64::NAN,
                }
            }
        } else {
            Deriv {
                value: f64::NAN,
                deriv: f64::NAN,
            }
        }
    }
}
