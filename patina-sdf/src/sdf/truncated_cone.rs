use crate::sdf::leaf::{SdfLeaf, SdfLeafImpl};
use crate::sdf::rotate::Rotate;
use crate::sdf::transform::Transform;
use crate::sdf::{AsSdf, Sdf};
use patina_geo::geo3::cylinder::Cylinder;
use patina_scalar::Scalar;
use patina_vec::vec::Vector;
use patina_vec::vec2::{Vec2, Vector2};
use patina_vec::vec3::Vec3;

pub struct TruncatedCone {
    origin: Vec3,
    axis: Vec3,
    r1: f64,
    r2: f64,
}

impl TruncatedCone {
    pub fn new(origin: Vec3, axis: Vec3, r1: f64, r2: f64) -> Self {
        TruncatedCone {
            origin,
            axis,
            r1,
            r2,
        }
    }
}

#[derive(Debug)]
struct TruncatedConeCrossSection {
    height: f64,
    r1: f64,
    r2: f64,
}

impl SdfLeafImpl<2> for TruncatedConeCrossSection {
    fn evaluate<T: Scalar>(&self, p: Vector<T, 2>) -> T {
        let r1 = T::from_f64(self.r1);
        let r2 = T::from_f64(self.r2);
        let radial1 = p.x().clone() - r1.clone();
        let radial2 = p.x().clone() - r2.clone();
        let height = T::from_f64(self.height);
        let norm = Vec2::new(self.height, self.r1 - self.r2).normalize();
        let tang = Vec2::new(self.r2 - self.r1, self.height)
            .normalize()
            .into_scalars();
        let below = -p.y();
        let above = p.y() - T::from_f64(self.height);
        let right = Vector2::new(radial1.clone(), p.y().clone()).dot(norm.into_scalars());
        let below_right = Vector2::new(radial1.clone(), p.y().clone()).dot(tang.clone());
        let above_right =
            Vector2::new(radial2.clone(), p.y().clone() - height.clone()).dot(tang.clone());
        let from_bottom_corner =
            (radial1.clone() * radial1.clone() + below.clone() * below.clone()).sqrt();
        let from_top_corner =
            (radial2.clone() * radial2.clone() + above.clone() * above.clone()).sqrt();
        p.y().clone().piecewise(
            radial1.piecewise(
                below.clone(),
                below_right.piecewise(from_bottom_corner, right.clone()),
            ),
            above.clone().piecewise(
                right.clone().maximum(below.clone().maximum(above.clone())),
                radial2.piecewise(above.clone(), above_right.piecewise(from_top_corner, right)),
            ),
        )
    }
}

impl AsSdf<3> for TruncatedCone {
    fn as_sdf(&self) -> Sdf<3> {
        Sdf::new(Transform::new(
            Rotate::new(self.origin, self.axis),
            Sdf::new(SdfLeaf::new(TruncatedConeCrossSection {
                height: self.axis.length(),
                r1: self.r1,
                r2: self.r2,
            })),
        ))
    }
}
