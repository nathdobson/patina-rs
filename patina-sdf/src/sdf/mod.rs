pub mod invert;
pub mod leaf;
pub mod union;
mod polygon;

use crate::sdf::invert::SdfInvert;
use crate::sdf::union::SdfUnion;
use inari::DecInterval;
use patina_scalar::Scalar;
use patina_scalar::deriv::Deriv;
use patina_vec::vec3::{Vec3, Vector3};
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
    pub fn evaluate_deriv1(&self, p: Vector3<Deriv<1>>) -> Deriv<1> {
        self.0.imp.evaluate_deriv1(p)
    }
    pub fn evaluate_deriv3(&self, p: Vector3<Deriv<3>>) -> Deriv<3> {
        self.0.imp.evaluate_deriv3(p)
    }
    pub fn evaluate_constrain(&self, p: Vector3<DecInterval>) -> (Option<Sdf>, DecInterval) {
        self.0.imp.evaluate_constrain(p)
    }
    pub fn normal(&self, position: Vec3) -> Vec3 {
        Vec3::from(
            self.evaluate_deriv3(Vector3::new(
                Deriv::variable(position[0], 0),
                Deriv::variable(position[1], 1),
                Deriv::variable(position[2], 2),
            ))
            .deriv()
            .clone(),
        )
        .normalize()
    }
    pub fn union(&self, other: &Sdf) -> Sdf {
        SdfUnion::new(self.clone(), other.clone()).into_sdf()
    }
    pub fn invert(&self) -> Sdf {
        SdfInvert::new(self.clone()).into_sdf()
    }
    pub fn difference(&self, other: &Sdf) -> Sdf {
        self.invert().union(other).invert()
    }
}

struct SdfInner<S: ?Sized> {
    imp: S,
}

pub trait SdfImpl: 'static + Sync + Send + Debug {
    fn evaluate(&self, p: Vec3) -> f64;
    fn evaluate_deriv1(&self, p: Vector3<Deriv<1>>) -> Deriv<1>;
    fn evaluate_deriv3(&self, p: Vector3<Deriv<3>>) -> Deriv<3>;
    fn evaluate_constrain(&self, p: Vector3<DecInterval>) -> (Option<Sdf>, DecInterval);
}

impl Debug for Sdf {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.0.imp.fmt(f)
    }
}
