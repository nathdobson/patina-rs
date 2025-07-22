use crate::sdf::transform::TransformImpl;
use crate::sdf::{Sdf, Sdf2, SdfImpl};
use inari::DecInterval;
use patina_scalar::Scalar;
use patina_scalar::deriv::Deriv;
use patina_vec::vec::Vector;
use patina_vec::vec3::Vec3;

#[derive(Debug, Clone)]
pub struct Rotate {
    origin: Vec3,
    axis: Vec3,
}

impl Rotate {
    pub fn new(origin: Vec3, axis: Vec3) -> Self {
        Rotate {
            origin,
            axis: axis.normalize(),
        }
    }
}

impl TransformImpl<2, 3> for Rotate {
    fn evaluate<T: Scalar>(&self, p: Vector<T, 3>, mut inner: impl FnOnce(Vector<T, 2>) -> T) -> T {
        let relative = p - self.origin.into_scalars();
        let elevation = relative.clone().dot(self.axis.into_scalars());
        let radius = (relative - self.axis.into_scalars() * elevation.clone()).length();
        inner(Vector::<T, 2>::new(radius, elevation))
    }
}
