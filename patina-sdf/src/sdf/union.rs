use crate::deriv::Deriv;
use crate::sdf::{Sdf, SdfImpl};
use inari::DecInterval;
use patina_scalar::Scalar;
use patina_vec::vec3::Vec3;
use patina_vec::vector3::Vector3;

struct SdfbUnion {
    a: Sdf,
    b: Sdf,
}

impl SdfbUnion {
    fn new(a: Sdf, b: Sdf) -> Self {
        Self { a, b }
    }
}

impl SdfImpl for SdfbUnion {
    fn evaluate(&self, p: Vec3) -> f64 {
        self.a.evaluate(p).minimum(self.b.evaluate(p))
    }

    fn evaluate_deriv(&self, p: Vector3<Deriv>) -> Deriv {
        self.a.evaluate_deriv(p).minimum(self.b.evaluate_deriv(p))
    }

    fn evaluate_constrain(&self, p: Vector3<DecInterval>) -> (Option<Sdf>, DecInterval) {
        let (a2, ai) = self.a.evaluate_constrain(p);
        let (b2, bi) = self.b.evaluate_constrain(p);
        if ai.precedes(bi) {
            (Some(a2.unwrap_or(self.a.clone())), ai)
        } else if bi.precedes(ai) {
            (Some(b2.unwrap_or(self.a.clone())), bi)
        } else if a2.is_some() || b2.is_some() {
            (
                Some(Sdf::new(SdfbUnion::new(
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
