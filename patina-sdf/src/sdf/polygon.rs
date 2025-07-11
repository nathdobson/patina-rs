use crate::sdf::leaf::SdfLeafImpl;
use itertools::Itertools;
use patina_scalar::Scalar;
use patina_vec::vec2::Vec2;
use patina_vec::vec3::Vector3;

#[derive(Debug)]
pub struct Polygon {
    vertices: Vec<Vec2>,
}

impl SdfLeafImpl for Polygon {
    fn evaluate<T: Scalar>(&self, p: Vector3<T>) -> T {
        todo!();
        // for (&v1, &v2) in self.vertices.iter().circular_tuple_windows() {
        //     let delta = v2 - v1;
        //     let p2 = p - v1.into_scalars();
        //
        // }
    }
}
