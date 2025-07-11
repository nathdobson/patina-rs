use crate::sdf::{Sdf, SdfImpl};
use inari::DecInterval;
use patina_scalar::Scalar;
use patina_scalar::deriv::Deriv;
use patina_vec::vec3::{Vec3, Vector3};

#[derive(Debug)]
pub struct SdfUnion {
    a: Sdf,
    b: Sdf,
}

impl SdfUnion {
    pub fn new(a: Sdf, b: Sdf) -> Self {
        Self { a, b }
    }
    pub fn into_sdf(self) -> Sdf {
        Sdf::new(self)
    }
}

impl SdfImpl for SdfUnion {
    fn evaluate(&self, p: Vec3) -> f64 {
        self.a.evaluate(p).minimum(self.b.evaluate(p))
    }

    fn evaluate_deriv1(&self, p: Vector3<Deriv<1>>) -> Deriv<1> {
        self.a
            .evaluate_deriv1(p.clone())
            .minimum(self.b.evaluate_deriv1(p.clone()))
    }

    fn evaluate_deriv3(&self, p: Vector3<Deriv<3>>) -> Deriv<3> {
        self.a
            .evaluate_deriv3(p.clone())
            .minimum(self.b.evaluate_deriv3(p.clone()))
    }

    fn evaluate_constrain(&self, p: Vector3<DecInterval>) -> (Option<Sdf>, DecInterval) {
        let (a2, ai) = self.a.evaluate_constrain(p);
        let (b2, bi) = self.b.evaluate_constrain(p);
        if ai.precedes(bi) {
            (Some(a2.unwrap_or(self.a.clone())), ai)
        } else if bi.precedes(ai) {
            (Some(b2.unwrap_or(self.b.clone())), bi)
        } else if a2.is_some() || b2.is_some() {
            (
                Some(Sdf::new(SdfUnion::new(
                    a2.unwrap_or(self.a.clone()),
                    b2.unwrap_or(self.b.clone()),
                ))),
                ai.minimum(bi),
            )
        } else {
            (None, ai.minimum(bi))
        }
    }
}
