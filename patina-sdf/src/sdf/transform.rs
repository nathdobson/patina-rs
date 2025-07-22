use crate::sdf::{Sdf, SdfImpl};
use inari::DecInterval;
use patina_scalar::Scalar;
use patina_scalar::deriv::Deriv;
use patina_vec::vec::Vector;
use std::fmt::Debug;

#[derive(Debug)]
pub struct Transform<const NI: usize, const NO: usize, T> {
    transform: T,
    inner: Sdf<NI>,
}

impl<const NI: usize, const NO: usize, T> Transform<NI, NO, T> {
    pub fn new(transform: T, inner: Sdf<NI>) -> Self {
        Transform { transform, inner }
    }
}

pub trait TransformImpl<const NI: usize, const NO: usize>:
    'static + Sync + Send + Debug + Clone
{
    fn evaluate<T: Scalar>(
        &self,
        input: Vector<T, NO>,
        inner: impl FnOnce(Vector<T, NI>) -> T,
    ) -> T;
}

impl<const NI: usize, const NO: usize, T: TransformImpl<NI, NO>> SdfImpl<NO>
    for Transform<NI, NO, T>
{
    fn evaluate(&self, p: Vector<f64, NO>) -> f64 {
        self.transform.evaluate(p, |x| self.inner.evaluate(x))
    }

    fn evaluate_deriv1(&self, p: Vector<Deriv<1>, NO>) -> Deriv<1> {
        self.transform
            .evaluate(p, |x| self.inner.evaluate_deriv1(x))
    }

    fn evaluate_deriv2(&self, p: Vector<Deriv<2>, NO>) -> Deriv<2> {
        self.transform
            .evaluate(p, |x| self.inner.evaluate_deriv2(x))
    }

    fn evaluate_deriv3(&self, p: Vector<Deriv<3>, NO>) -> Deriv<3> {
        self.transform
            .evaluate(p, |x| self.inner.evaluate_deriv3(x))
    }

    fn evaluate_constrain(&self, p: Vector<DecInterval, NO>) -> (Option<Sdf<NO>>, DecInterval) {
        let mut constrained = None;
        let int = self.transform.evaluate::<DecInterval>(p, |x| {
            let (c, int) = self.inner.evaluate_constrain(x);
            constrained = c;
            int
        });
        if let Some(constrained) = constrained {
            (
                Some(Sdf::new(Transform::new(
                    self.transform.clone(),
                    constrained,
                ))),
                int,
            )
        } else {
            (None, int)
        }
    }

    fn complexity(&self) -> usize {
        1 + self.inner.complexity()
    }
}
