use crate::sdf::leaf::{SdfLeaf, SdfLeafImpl};
use crate::sdf::rotate::Rotate;
use crate::sdf::transform::Transform;
use crate::sdf::{AsSdf, Sdf};
use patina_geo::geo3::cylinder::Cylinder;
use patina_scalar::Scalar;
use patina_vec::vec::Vector;

#[derive(Debug)]
struct CylinderCrossSection {
    radius: f64,
    height: f64,
}
impl SdfLeafImpl<2> for CylinderCrossSection {
    fn evaluate<T: Scalar>(&self, p: Vector<T, 2>) -> T {
        let radial = p.x() - T::from_f64(self.radius);
        let y2 = p.y() - T::from_f64(self.height);
        p.y().clone().piecewise(
            radial.clone().piecewise(
                -p.y().clone(),
                (radial.clone() * radial.clone() + p.y() * p.y()).sqrt(),
            ),
            y2.clone().piecewise(
                radial.clone(),
                radial.clone().piecewise(
                    y2.clone(),
                    (radial.clone() * radial.clone() + y2.clone() * y2.clone()).sqrt(),
                ),
            ),
        )
    }
}

impl AsSdf<3> for Cylinder {
    fn as_sdf(&self) -> Sdf<3> {
        Sdf::new(Transform::new(
            Rotate::new(self.origin(), self.axis()),
            Sdf::new(SdfLeaf::new(CylinderCrossSection {
                radius: self.radius(),
                height: self.axis().length(),
            })),
        ))
    }
}
