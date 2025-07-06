pub mod leaf;
pub mod union;

use inari::DecInterval;
use patina_scalar::Scalar;
use patina_scalar::deriv::Deriv;
use patina_vec::vec3::Vec3;
use patina_vec::vector3::Vector3;
use std::fmt::{Debug, Formatter};
use std::sync::Arc;

/// The [signed distance function (SDF)](https://iquilezles.org/articles/distfunctions/)
///  of a solid `S ⊆ ℝ³` is the function `F: ℝ³ → ℝ` where
/// * `F(p) < 0` iff `p` is below the surface of `S`.
/// * `F(p) = 0` iff `p` is on the surface of `S`.
/// * `F(p) > 0` iff `p` is above the surface of `S`.
/// * `|F(p)|` is the distance from `p` to the surface of `S`.
///
/// A signed distance function bound (SDFB) of a solid `S ⊆ ℝ³` is a lower bound `G: ℝ³ → ℝ` on the SDF of `S` where
/// * `G(p) < 0` iff `p` is below the surface of `S`.
/// * `G(p) = 0` iff `p` is on the surface of `S`.
/// * `G(p) > 0` iff `p` is above the surface of `S`.
/// * `|G(p)| < |F(p)|`
///
/// In addition to the above requirements, an SDFB should be designed to be as tight a bound as possible,
/// to improve performance of algoritthms relying on these bounds.

#[derive(Clone)]
pub struct Sdf(Arc<SdfInner<dyn SdfImpl>>);

impl Sdf {
    pub fn new<I: SdfImpl>(imp: I) -> Self {
        Sdf(Arc::new(SdfInner { imp }))
    }
    pub fn evaluate(&self, p: Vec3) -> f64 {
        self.0.imp.evaluate(p)
    }
    pub fn evaluate_deriv(&self, p: Vector3<Deriv>) -> Deriv {
        self.0.imp.evaluate_deriv(p)
    }
    pub fn evaluate_constrain(&self, p: Vector3<DecInterval>) -> (Option<Sdf>, DecInterval) {
        self.0.imp.evaluate_constrain(p)
    }
}

struct SdfInner<S: ?Sized> {
    imp: S,
}

pub trait SdfImpl: 'static + Sync + Send + Debug {
    fn evaluate(&self, p: Vec3) -> f64;
    fn evaluate_deriv(&self, p: Vector3<Deriv>) -> Deriv;
    fn evaluate_constrain(&self, p: Vector3<DecInterval>) -> (Option<Sdf>, DecInterval);
}

impl Debug for Sdf {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.0.imp.fmt(f)
    }
}
