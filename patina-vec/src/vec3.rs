use crate::vec::Vector;
use std::ops::{Mul, Sub};

pub type Vector3<T> = Vector<T, 3>;
pub type Vec3 = Vector3<f64>;
impl<T> Vector3<T> {
    pub fn new(x: T, y: T, z: T) -> Self {
        Self::from([x, y, z])
    }
    pub fn cross(self, rhs: Self) -> Self
    where
        T: Clone + Sub<Output = T> + Mul<Output = T>,
    {
        Self::new(
            self.y() * rhs.z() - rhs.y() * self.z(),
            self.z() * rhs.x() - rhs.z() * self.x(),
            self.x() * rhs.y() - rhs.x() * self.y(),
        )
    }
}
