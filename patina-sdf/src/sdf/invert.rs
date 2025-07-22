use crate::sdf::{Sdf, SdfImpl};
use inari::DecInterval;
use patina_scalar::deriv::Deriv;
use patina_vec::vec::Vector;
use patina_vec::vec3::{Vec3, Vector3};

#[derive(Debug)]
pub struct SdfInvert<const N: usize> {
    inner: Sdf<N>,
}

impl<const N: usize> SdfInvert<N> {
    pub fn new(inner: Sdf<N>) -> Self {
        SdfInvert { inner }
    }
}

impl<const N: usize> SdfImpl<N> for SdfInvert<N> {
    fn evaluate(&self, p: Vector<f64, N>) -> f64 {
        -self.inner.evaluate(p)
    }

    fn evaluate_deriv1(&self, p: Vector<Deriv<1>, N>) -> Deriv<1> {
        -self.inner.evaluate_deriv1(p)
    }

    fn evaluate_deriv2(&self, p: Vector<Deriv<2>, N>) -> Deriv<2> {
        -self.inner.evaluate_deriv2(p)
    }

    fn evaluate_deriv3(&self, p: Vector<Deriv<3>, N>) -> Deriv<3> {
        -self.inner.evaluate_deriv3(p)
    }

    fn evaluate_constrain(&self, p: Vector<DecInterval, N>) -> (Option<Sdf<N>>, DecInterval) {
        let (inner, range) = self.inner.evaluate_constrain(p);
        (inner.map(|x| x.invert()), -range)
    }

    fn complexity(&self) -> usize {
        1 + self.inner.complexity()
    }
}
