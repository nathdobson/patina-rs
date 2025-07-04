use crate::sdf::{Sdf, constant3, position3};
use patina_calc::Expr;
use patina_geo::geo3::plane::Plane;
use patina_geo::geo3::sphere::Sphere;
use patina_scalar::Scalar;

impl Sdf {
    pub fn new_sphere(sphere: &Sphere) -> Self {
        Sdf::new(
            (position3() - constant3(sphere.origin())).length() - Expr::constant(sphere.radius()),
        )
    }
    pub fn new_plane(plane: &Plane) -> Self {
        Sdf::new((position3() - constant3(plane.origin())).dot(constant3(plane.normal())))
    }
}
