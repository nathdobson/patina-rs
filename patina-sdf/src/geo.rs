use crate::sdf::{Sdf, constant3, position3};
use patina_calc::Expr;
use patina_geo::geo3::aabb::Aabb;
use patina_geo::geo3::plane::Plane;
use patina_geo::geo3::sphere::Sphere;
use patina_scalar::Scalar;
use patina_vec::vec3::Vec3;
use patina_vec::vector3::Vector3;

impl Sdf {
    pub fn new_sphere(sphere: &Sphere) -> Self {
        Sdf::new(
            (position3() - constant3(sphere.origin())).length() - Expr::constant(sphere.radius()),
        )
    }
    pub fn new_plane(plane: &Plane) -> Self {
        Sdf::new((position3() - constant3(plane.origin())).dot(constant3(plane.normal())))
    }
    pub fn new_negative_octant(input: Vector3<Expr>) -> Sdf {
        let x2 = input.x() * input.x();
        let y2 = input.y() * input.y();
        let z2 = input.z() * input.z();
        Sdf::new(Expr::piecewise(
            input.x(),
            Expr::piecewise(
                input.y(),
                Expr::piecewise(
                    input.z(),
                    input.x().maximum(input.y()).maximum(input.z()),
                    input.z(),
                ),
                Expr::piecewise(input.z(), input.y(), (y2.clone() + z2.clone()).sqrt()),
            ),
            Expr::piecewise(
                input.y(),
                Expr::piecewise(input.z(), input.x(), (x2.clone() + z2.clone()).sqrt()),
                Expr::piecewise(
                    input.z(),
                    (x2.clone() + y2.clone()).sqrt(),
                    (x2.clone() + y2.clone() + z2.clone()).sqrt(),
                ),
            ),
        ))
    }
    pub fn new_aabb(aabb: &Aabb) -> Self {
        let disp =
            (position3() - constant3(aabb.center())).abs() - constant3(aabb.dimensions() / 2.0);
        // let outside = disp.clone().max(constant3(Vec3::splat(0.0))).length();
        // let inside = disp
        //     .x()
        //     .maximum(disp.y())
        //     .maximum(disp.z())
        //     .minimum(Expr::constant(0.0));
        // Sdf::new(outside + inside)
        Self::new_negative_octant(disp)
    }
}
