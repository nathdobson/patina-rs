use inari::DecInterval;
use ordered_float::NotNan;
use patina_scalar::Scalar;
use std::fmt::{Display, Formatter};
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

#[derive(Copy, Clone, Eq, Ord, PartialEq, PartialOrd, Hash, Debug)]
pub struct Exact(NotNan<f64>);

#[derive(Copy, Clone, Eq, Ord, PartialEq, PartialOrd, Hash, Debug)]
pub struct NotExact;

impl Exact {
    pub fn interval(self) -> DecInterval {
        DecInterval::try_from((self.0.into_inner(), self.0.into_inner())).unwrap()
    }
}

impl TryFrom<f64> for Exact {
    type Error = NotExact;

    fn try_from(value: f64) -> Result<Self, Self::Error> {
        if value.is_infinite() {
            Err(NotExact)
        } else {
            Ok(Exact(NotNan::new(value).ok().ok_or(NotExact)?))
        }
    }
}

impl TryFrom<DecInterval> for Exact {
    type Error = NotExact;

    fn try_from(value: DecInterval) -> Result<Self, Self::Error> {
        if value.is_singleton() {
            Self::try_from(value.sup())
        } else {
            Err(NotExact)
        }
    }
}

impl Add for Exact {
    type Output = Exact;
    fn add(self, rhs: Self) -> Self::Output {
        Self::try_from(self.interval() + rhs.interval()).unwrap()
    }
}
impl Sub for Exact {
    type Output = Exact;
    fn sub(self, rhs: Self) -> Self::Output {
        Self::try_from(self.interval() - rhs.interval()).unwrap()
    }
}

impl Mul for Exact {
    type Output = Exact;
    fn mul(self, rhs: Self) -> Self::Output {
        Self::try_from(self.interval() * rhs.interval()).unwrap()
    }
}

impl Div for Exact {
    type Output = Exact;
    fn div(self, rhs: Self) -> Self::Output {
        Self::try_from(self.interval() / rhs.interval()).unwrap()
    }
}

impl Neg for Exact {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self::try_from(-self.interval()).unwrap()
    }
}

impl AddAssign<Self> for Exact {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

impl SubAssign<Self> for Exact {
    fn sub_assign(&mut self, rhs: Self) {
        *self = *self - rhs;
    }
}

impl MulAssign<Self> for Exact {
    fn mul_assign(&mut self, rhs: Self) {
        *self = *self * rhs;
    }
}

impl DivAssign<Self> for Exact {
    fn div_assign(&mut self, rhs: Self) {
        *self = *self / rhs;
    }
}

impl Display for Exact {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Display::fmt(&self.0, f)
    }
}

impl Scalar for Exact {
    fn recip(self) -> Self {
        Exact::try_from(self.interval().recip()).unwrap()
    }

    fn maximum(self, other: Self) -> Self {
        Exact::try_from(self.interval().maximum(other.interval())).unwrap()
    }

    fn minimum(self, other: Self) -> Self {
        Exact::try_from(self.interval().minimum(other.interval())).unwrap()
    }

    fn piecewise(self, neg: Self, pos: Self) -> Self {
        todo!()
    }

    fn from_f64(value: f64) -> Self {
        Self::try_from(value).unwrap()
    }

    fn sqrt(self) -> Self {
        Exact::try_from(self.interval().sqrt()).unwrap()
    }

    fn abs(self) -> Self {
        Exact::try_from(self.interval().abs()).unwrap()
    }
}
