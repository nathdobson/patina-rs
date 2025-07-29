use crate::sdf::leaf::{SdfLeaf, SdfLeafImpl};
use crate::sdf::{AsSdf, Sdf};
use patina_geo::sphere::{NSphere, Sphere};
use patina_scalar::Scalar;
use patina_vec::vec::Vector;
use patina_vec::vec3::Vector3;

impl<const N: usize> AsSdf<N> for NSphere<N> {
    fn as_sdf(&self) -> Sdf<N> {
        Sdf::new(SdfLeaf::new(self.clone()))
    }
}

impl<const N: usize> SdfLeafImpl<N> for NSphere<N> {
    fn evaluate<T: Scalar>(&self, p: Vector<T, N>) -> T {
        (p - self.origin().into_scalars::<T>()).length() - T::from_f64(self.radius())
    }
}
