mod aabb;
mod cylinder;
mod empty;
pub mod invert;
pub mod leaf;
mod plane;
mod polygon;
mod rotate;
mod sphere;
mod transform;
pub mod truncated_cone;
pub mod union;

use crate::sdf::empty::{SdfEmpty, SdfFull};
use crate::sdf::invert::SdfInvert;
use crate::sdf::leaf::{SdfLeaf, SdfLeafImpl};
use crate::sdf::rotate::Rotate;
use crate::sdf::transform::Transform;
use crate::sdf::union::SdfUnion;
use inari::DecInterval;
use patina_geo::aabb::Aabb;
use patina_geo::geo3::cylinder::Cylinder;
use patina_scalar::Scalar;
use patina_scalar::deriv::Deriv;
use patina_vec::vec::Vector;
use patina_vec::vec2::Vec2;
use patina_vec::vec3::{Vec3, Vector3};
use std::any::{Any, TypeId};
use std::fmt::{Debug, Formatter};
use std::mem;
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
pub struct Sdf<const N: usize>(Arc<SdfInner<dyn SdfImpl<N>>>);

pub type Sdf3 = Sdf<3>;
pub type Sdf2 = Sdf<2>;

impl<const N: usize> Sdf<N> {
    pub fn new<I: SdfImpl<N>>(imp: I) -> Self {
        Sdf(Arc::new(SdfInner { imp }))
    }
    pub fn evaluate(&self, p: Vector<f64, N>) -> f64 {
        self.0.imp.evaluate(p)
    }
    #[inline(never)]
    pub fn evaluate_deriv1(&self, p: Vector<Deriv<1>, N>) -> Deriv<1> {
        self.0.imp.evaluate_deriv1(p)
    }
    pub fn evaluate_deriv2(&self, p: Vector<Deriv<2>, N>) -> Deriv<2> {
        self.0.imp.evaluate_deriv2(p)
    }
    pub fn evaluate_deriv3(&self, p: Vector<Deriv<3>, N>) -> Deriv<3> {
        self.0.imp.evaluate_deriv3(p)
    }
    pub fn evaluate_constrain(&self, p: Vector<DecInterval, N>) -> (Option<Sdf<N>>, DecInterval) {
        self.0.imp.evaluate_constrain(p)
    }
    pub fn union(&self, other: &Sdf<N>) -> Sdf<N> {
        SdfUnion::new(self.clone(), other.clone()).into_sdf()
    }
    pub fn invert(&self) -> Sdf<N> {
        Sdf::new(SdfInvert::new(self.clone()))
    }
    pub fn difference(&self, other: &Sdf<N>) -> Sdf<N> {
        self.invert().union(other).invert()
    }
    pub fn empty() -> Sdf<N> {
        Sdf::new(SdfLeaf::new(SdfEmpty))
    }
    pub fn full() -> Sdf<N> {
        Sdf::new(SdfLeaf::new(SdfFull))
    }
    pub fn complexity(&self) -> usize {
        self.0.imp.complexity()
    }
}

impl Sdf<3> {
    pub fn normal(&self, position: Vec3) -> Vec3 {
        Vector::from(
            self.evaluate_deriv3(position.into_variable())
                .deriv()
                .clone(),
        )
        .normalize()
    }
}

impl Sdf<2> {
    pub fn normal(&self, position: Vec2) -> Vec2 {
        Vector::from(
            self.evaluate_deriv2(position.into_variable())
                .deriv()
                .clone(),
        )
        .normalize()
    }
    pub fn rotate(&self, origin: Vec3, axis: Vec3) -> Sdf<3> {
        Sdf::new(Transform::new(Rotate::new(origin, axis), self.clone()))
    }
}

struct SdfInner<S: ?Sized> {
    imp: S,
}

pub trait SdfImpl<const N: usize>: 'static + Sync + Send + Debug {
    fn evaluate(&self, p: Vector<f64, N>) -> f64;
    fn evaluate_deriv1(&self, p: Vector<Deriv<1>, N>) -> Deriv<1>;
    fn evaluate_deriv2(&self, p: Vector<Deriv<2>, N>) -> Deriv<2>;
    fn evaluate_deriv3(&self, p: Vector<Deriv<3>, N>) -> Deriv<3>;
    fn evaluate_constrain(&self, p: Vector<DecInterval, N>) -> (Option<Sdf<N>>, DecInterval);
    fn complexity(&self) -> usize;
}

impl<const N: usize> Debug for Sdf<N> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.0.imp.fmt(f)
    }
}

pub trait AsSdf<const N: usize> {
    fn as_sdf(&self) -> Sdf<N>;
}
