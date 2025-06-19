use crate::math::interval::Interval;
use crate::math::vec3::Vec3;
use std::any::type_name;

pub trait ConvexPoly {
    fn normals(&self) -> impl AsRef<[Vec3]>;
    fn project_onto(&self, vector: Vec3) -> Interval;
}

pub fn sat_intersects<A: ConvexPoly, B: ConvexPoly>(a: &A, b: &B) -> bool {
    sat_intersects_partial(a, b) && sat_intersects_partial(b, a)
}

fn sat_intersects_partial<A: ConvexPoly, B: ConvexPoly>(a: &A, b: &B) -> bool {
    for normal in a.normals().as_ref() {
        let ia = a.project_onto(*normal);
        let ib = b.project_onto(*normal);
        if !ia.intersects(ib) {
            return false;
        }
    }
    true
}
