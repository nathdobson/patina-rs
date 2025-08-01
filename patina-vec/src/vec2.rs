use crate::vec::Vector;
use std::ops::{Mul, Sub};
use std::f64;

pub type Vector2<T> = Vector<T, 2>;
pub type Vec2 = Vector2<f64>;

impl<T> Vector2<T> {
    pub fn new(x: T, y: T) -> Self {
        Self::from([x, y])
    }
}

impl Vec2 {
    pub fn slope(&self) -> f64 {
        if self.x() == 0.0 {
            if self.y() > 0.0 {
                f64::INFINITY
            } else if self.y() < 0.0 {
                f64::NEG_INFINITY
            } else {
                f64::NAN
            }
        } else {
            self.y() / self.x()
        }
    }
    pub fn from_deg(x: f64) -> Self {
        Self::from_rad(x * (2.0 * f64::consts::PI / 360.0))
    }
    pub fn from_rad(x: f64) -> Self {
        Self::new(x.cos(), x.sin())
    }
}

impl<T> Vector2<T> {
    pub fn cross(self, rhs: Self) -> T
    where
        T: Clone + Sub<Output = T> + Mul<Output = T>,
    {
        self.x() * rhs.y() - rhs.x() * self.y()
    }
}
