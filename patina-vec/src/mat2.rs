use crate::mat::Matrix;
use crate::vec2::{Vec2, Vector2};
use patina_scalar::Scalar;

pub type Matrix2<T> = Matrix<T, 2>;
pub type Mat2 = Matrix2<f64>;
impl<T> Matrix2<T> {
    pub fn invert2(&self) -> Self
    where
        T: Scalar,
    {
        let (a, b, c, d) = (
            self[(0, 0)].clone(),
            self[(0, 1)].clone(),
            self[(1, 0)].clone(),
            self[(1, 1)].clone(),
        );
        let det = a.clone() * d.clone() - b.clone() * c.clone();

        Self::from_rows([Vector2::new(d, -b), Vector2::new(-c, a)]) / det
    }
}

#[test]
fn test_invert() {
    let mat = Mat2::from_rows([Vec2::new(1.0, 2.0), Vec2::new(3.0, 4.0)]);
    assert_eq!(Mat2::id(), mat * mat.invert2());
}
