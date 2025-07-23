use crate::sdf::leaf::SdfLeafImpl;
use patina_scalar::Scalar;
use patina_vec::vec::Vector;

#[derive(Debug)]
pub struct SdfEmpty<const N: usize>;

impl<const N: usize> SdfLeafImpl<N> for SdfEmpty<N> {
    fn evaluate<T: Scalar>(&self, p: Vector<T, N>) -> T {
        T::from_f64(f64::INFINITY)
    }
}

#[derive(Debug)]
pub struct SdfFull<const N: usize>;

impl<const N: usize> SdfLeafImpl<N> for SdfFull<N> {
    fn evaluate<T: Scalar>(&self, p: Vector<T, N>) -> T {
        T::from_f64(-f64::INFINITY)
    }
}
