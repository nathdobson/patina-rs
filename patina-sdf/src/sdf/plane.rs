use crate::sdf::leaf::{SdfLeaf, SdfLeafImpl};
use crate::sdf::{AsSdf, Sdf};
use patina_geo::geo3::plane::Plane;
use patina_scalar::Scalar;
use patina_vec::vec3::Vector3;

impl AsSdf<3> for Plane {
    fn as_sdf(&self) -> Sdf<3> {
        Sdf::new(SdfLeaf::new(self.clone()))
    }
}

impl SdfLeafImpl<3> for Plane {
    fn evaluate<T: Scalar>(&self, p: Vector3<T>) -> T {
        (p - self.origin().into_scalars::<T>()).dot(self.normal().into_scalars::<T>())
    }
}
