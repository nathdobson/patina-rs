use crate::sdf::{Sdf, SdfImpl};
use inari::DecInterval;
use patina_geo::geo3::plane::Plane;
use patina_geo::geo3::sphere::Sphere;
use patina_scalar::Scalar;
use patina_scalar::deriv::Deriv;
use patina_vec::vec3::Vec3;
use patina_vec::vector3::Vector3;
use std::fmt::{Debug, Formatter};

pub trait SdfLeafImpl: 'static + Sync + Send + Sized + Debug {
    fn evaluate<T: Scalar>(&self, p: Vector3<T>) -> T;
    fn into_sdf(self) -> Sdf {
        Sdf::new(SdfLeaf::new(self))
    }
}

pub struct SdfLeaf<T> {
    inner: T,
}

impl<T: SdfLeafImpl> SdfLeaf<T> {
    pub fn new(inner: T) -> SdfLeaf<T> {
        SdfLeaf { inner }
    }
}

impl<T: SdfLeafImpl> SdfImpl for SdfLeaf<T> {
    fn evaluate(&self, p: Vec3) -> f64 {
        self.inner.evaluate(Vector3::from(<[f64; 3]>::from(p)))
    }
    fn evaluate_deriv1(&self, p: Vector3<Deriv<1>>) -> Deriv<1> {
        self.inner.evaluate(p)
    }
    fn evaluate_deriv3(&self, p: Vector3<Deriv<3>>) -> Deriv<3> {
        self.inner.evaluate(p)
    }
    fn evaluate_constrain(&self, p: Vector3<DecInterval>) -> (Option<Sdf>, DecInterval) {
        (None, self.inner.evaluate(p))
    }
}

impl SdfLeafImpl for Sphere {
    fn evaluate<T: Scalar>(&self, p: Vector3<T>) -> T {
        (p - self.origin().into_scalars::<T>()).length() - T::from_f64(self.radius())
    }
}

impl SdfLeafImpl for Plane {
    fn evaluate<T: Scalar>(&self, p: Vector3<T>) -> T {
        (p - self.origin().into_scalars::<T>()).dot(self.normal().into_scalars::<T>())
    }
}

impl<T: Debug> Debug for SdfLeaf<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.inner.fmt(f)
    }
}
