use crate::sdf::{Sdf, SdfImpl};
use inari::DecInterval;
use patina_geo::aabb::Aabb;
use patina_geo::geo3::aabb3::Aabb3;
use patina_geo::geo3::cylinder::Cylinder;
use patina_geo::geo3::plane::Plane;
use patina_geo::geo3::sphere::Sphere;
use patina_scalar::Scalar;
use patina_scalar::deriv::Deriv;
use patina_vec::vec::Vector;
use patina_vec::vec3::{Vec3, Vector3};
use std::fmt::{Debug, Formatter};

pub trait SdfLeafImpl<const N: usize>: 'static + Sync + Send + Sized + Debug {
    fn evaluate<T: Scalar>(&self, p: Vector<T, N>) -> T;
}

pub struct SdfLeaf<const N: usize, T> {
    inner: T,
}

impl<const N: usize, T: SdfLeafImpl<N>> SdfLeaf<N, T> {
    pub fn new(inner: T) -> SdfLeaf<N, T> {
        SdfLeaf { inner }
    }
}

impl<const N: usize, T: SdfLeafImpl<N>> SdfImpl<N> for SdfLeaf<N, T> {
    fn evaluate(&self, p: Vector<f64, N>) -> f64 {
        self.inner.evaluate(Vector::from(<[f64; N]>::from(p)))
    }
    fn evaluate_deriv1(&self, p: Vector<Deriv<1>, N>) -> Deriv<1> {
        self.inner.evaluate(p)
    }
    fn evaluate_deriv2(&self, p: Vector<Deriv<2>, N>) -> Deriv<2> {
        self.inner.evaluate(p)
    }
    fn evaluate_deriv3(&self, p: Vector<Deriv<3>, N>) -> Deriv<3> {
        self.inner.evaluate(p)
    }
    fn evaluate_constrain(&self, p: Vector<DecInterval, N>) -> (Option<Sdf<N>>, DecInterval) {
        (None, self.inner.evaluate(p))
    }

    fn complexity(&self) -> usize {
        1
    }
}

impl<const N: usize, T: Debug> Debug for SdfLeaf<N, T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.inner.fmt(f)
    }
}
