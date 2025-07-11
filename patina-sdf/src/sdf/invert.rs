use crate::sdf::{Sdf, SdfImpl};
use inari::DecInterval;
use patina_scalar::deriv::Deriv;
use patina_vec::vec3::{Vec3, Vector3};

#[derive(Debug)]
pub struct SdfInvert {
    inner: Sdf,
}

impl SdfInvert {
    pub fn new(inner: Sdf) -> Self {
        SdfInvert { inner }
    }
    pub fn into_sdf(self) -> Sdf {
        Sdf::new(self)
    }
}

impl SdfImpl for SdfInvert {
    fn evaluate(&self, p: Vec3) -> f64 {
        -self.inner.evaluate(p)
    }

    fn evaluate_deriv1(&self, p: Vector3<Deriv<1>>) -> Deriv<1> {
        -self.inner.evaluate_deriv1(p)
    }

    fn evaluate_deriv3(&self, p: Vector3<Deriv<3>>) -> Deriv<3> {
        -self.inner.evaluate_deriv3(p)
    }

    fn evaluate_constrain(&self, p: Vector3<DecInterval>) -> (Option<Sdf>, DecInterval) {
        let (inner, range) = self.inner.evaluate_constrain(p);
        (inner.map(|x| x.invert()), -range)
    }
}
