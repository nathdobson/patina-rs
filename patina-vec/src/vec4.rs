use crate::vec::Vector;

pub type Vector4<T> = Vector<T, 4>;
pub type Vec4 = Vector4<f64>;
impl<T> Vector4<T> {
    pub fn new(x: T, y: T, z: T, w: T) -> Self {
        Self::from([x, y, z, w])
    }
}
