use crate::geo3::aabb::AABB;
use crate::geo3::triangle::Triangle;
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
    println!("sat_intersects_partial {:?} {:?}", type_name::<A>(), type_name::<A>());
    for normal in a.normals().as_ref() {
        let mut ia = a.project_onto(*normal);
        if ia.min() >= ia.max() {
            ia = Interval::new(ia.min(), ia.min());
        }
        let ib = b.project_onto(*normal);
        assert!(ia.min().is_finite());
        assert!(ia.max().is_finite());
        assert!(ib.min().is_finite());
        assert!(ib.max().is_finite());
        if !ia.intersects(ib) {
            return false;
        }
    }
    true
}

// #[test]
// fn test_sat() {
//     let tri1 = Triangle::new([
//         Vec3::new(0.02161, 5.32373, -8.45479),
//         Vec3::new(5.28103, 8.52436, 0.00847),
//         Vec3::new(8.54528, 0.07896, -5.18875),
//     ]);
//     let aabb = AABB::new(
//         Vec3::new(9.01104, 0.15527, -1.91011),
//         Vec3::new(10.56239, 2.65693, -1.33314),
//     );
//     let tri2 = Triangle::new([
//         Vec3::new(9.06785, 1.05991, -1.91011),
//         Vec3::new(10.56239, 1.93594, -1.35965),
//         Vec3::new(10.50745, 0.15527, -1.33314),
//     ]);
//     assert!(!sat_intersects(&tri1, &tri2));
// }
