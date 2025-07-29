use crate::sdf::AsSdf;
use crate::sdf::transform::TransformImpl;
use patina_geo::sphere::Circle;
use patina_scalar::Scalar;
use patina_vec::vec::Vector;
use patina_vec::vec2::{Vec2, Vector2};
use patina_vec::vec3::Vec3;
use std::any::type_name;

#[derive(Debug, Clone)]
pub struct Extrude {
    origin: Vec3,
    axis1: Vec3,
    axis2: Vec3,
    extrude: f64,
}

impl Extrude {
    pub fn new(origin: Vec3, axis1: Vec3, axis2: Vec3, extrude: f64) -> Self {
        Extrude {
            origin,
            axis1,
            axis2,
            extrude,
        }
    }
}

impl TransformImpl<2, 3> for Extrude {
    fn evaluate<T: Scalar>(&self, p: Vector<T, 3>, mut inner: impl FnOnce(Vector<T, 2>) -> T) -> T {
        let p3 = p - self.origin.into_scalars();
        let p2 = Vector2::new(
            p3.clone().dot(self.axis1.into_scalars()),
            p3.clone().dot(self.axis2.into_scalars()),
        );
        let z = self.axis1.cross(self.axis2).into_scalars().dot(p3.clone());
        let z2 = z.clone() - T::from_f64(self.extrude);
        let d2 = inner(p2);
        let d3 = z.clone().piecewise(
            d2.clone()
                .piecewise(-z.clone(), Vector2::new(d2.clone(), z.clone()).length()),
            z2.clone().piecewise(
                d2.clone().maximum(-z.clone()).maximum(z2.clone()),
                d2.clone()
                    .piecewise(z2.clone(), Vector2::new(d2, z2.clone()).length()),
            ),
        );
        d3
    }
}

#[test]
fn test_extrude() {
    let extrude = Circle::new(Vec2::new(0.0, 0.0), 1.0)
        .as_sdf()
        .extrude_z(-1.0..1.0);
    assert_eq!(-1.0, extrude.evaluate(Vec3::new(0.0, 0.0, 0.0)));
}
