use inari::DecInterval;
use crate::Scalar;

impl Scalar for DecInterval {
    fn recip(self) -> Self {
        DecInterval::recip(self)
    }

    fn minimum(self, other: Self) -> Self {
        DecInterval::min(self, other)
    }

    fn maximum(self, other: Self) -> Self {
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

    fn sqrt(self) -> Self {
        DecInterval::sqrt(self)
    }

    fn from_f64(value: f64) -> Self {
        DecInterval::try_from((value, value)).unwrap()
    }
}
