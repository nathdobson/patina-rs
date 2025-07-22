use crate::sdf::{AsSdf, Sdf};
use crate::sdf::leaf::{SdfLeaf, SdfLeafImpl};
use patina_geo::aabb::Aabb;
use patina_scalar::Scalar;
use patina_vec::vec::Vector;

impl AsSdf<3> for Aabb<3> {
    fn as_sdf(&self) -> Sdf<3> {
        Sdf::new(SdfLeaf::new(self.clone()))
    }
}

impl SdfLeafImpl<3> for Aabb<3> {
    fn evaluate<T: Scalar>(&self, p: Vector<T, 3>) -> T {
        let center = self.center().into_scalars::<T>();
        let radius = (self.dimensions() / 2.0).into_scalars::<T>();
        let delta = (p - center).abs() - radius;
        let [x, y, z] = delta.into_inner();
        x.clone().piecewise(
            y.clone().piecewise(
                z.clone().piecewise(
                    //
                    x.clone().maximum(y.clone().maximum(z.clone())),
                    z.clone(),
                ),
                z.clone().piecewise(
                    y.clone(),
                    (y.clone() * y.clone() + z.clone() * z.clone()).sqrt(),
                ),
            ),
            y.clone().piecewise(
                z.clone().piecewise(
                    x.clone(),
                    (x.clone() * x.clone() + z.clone() * z.clone()).sqrt(),
                ),
                z.clone().piecewise(
                    (x.clone() * x.clone() + y.clone() * y.clone()).sqrt(),
                    (x.clone() * x.clone() + y.clone() * y.clone() + z.clone() * z.clone()).sqrt(),
                ),
            ),
        )
    }
}
