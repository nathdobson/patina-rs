use patina_scalar::Scalar;
use crate::vec::Vector;
use crate::vec3::Vector3;

pub type Vector4<T> = Vector<T, 4>;
pub type Vec4 = Vector4<f64>;
impl<T> Vector4<T> {
    pub fn new(x: T, y: T, z: T, w: T) -> Self {
        Self::from([x, y, z, w])
    }
    pub fn from_vec3(v: Vector3<T>) -> Self
    where
        T: Scalar,
    {
        Self::from([v.x(), v.y(), v.z(), T::from_f64(0.0)])
    }
}
