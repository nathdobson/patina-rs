use patina_geo::geo3::triangle3::Triangle3;
use patina_scalar::Scalar;
use patina_vec::vec::Vector;
use crate::sdf::leaf::SdfLeafImpl;

impl SdfLeafImpl<2> for Triangle3{
    fn evaluate<T: Scalar>(&self, p: Vector<T, 2>) -> T {
        todo!()
    }
}