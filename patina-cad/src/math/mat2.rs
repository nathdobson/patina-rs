use crate::math::macros::impl_ref_binop;
use crate::math::vec2::Vec2;
use std::ops::{Add, Div, Index, Mul, Sub};

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Mat2([Vec2; 2]);

impl Mat2 {
    pub fn from_rows(r1: Vec2, r2: Vec2) -> Mat2 {
        Mat2([r1, r2])
    }
    pub fn from_cols(c1: Vec2, c2: Vec2) -> Mat2 {
        Self::from_rows(c1, c2).transpose()
    }
    pub fn transpose(&self) -> Mat2 {
        Mat2::from_rows(self.col(0), self.col(1))
    }
    pub fn row(&self, r: usize) -> Vec2 {
        self.0[r]
    }
    pub fn col(&self, c: usize) -> Vec2 {
        Vec2::new(self.row(0)[c], self.row(1)[c])
    }
    pub fn invert(&self) -> Mat2 {
        let (a, b, c, d) = (self[(0, 0)], self[(0, 1)], self[(1, 0)], self[(1, 1)]);
        let det = a * d - b * c;
        Mat2::from_rows(Vec2::new(d, -b), Vec2::new(-c, a)) / det
    }
    pub fn id() -> Self {
        Mat2::from_rows(Vec2::axis_x(), Vec2::axis_y())
    }
}

impl Sub for Mat2 {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        self.into_iter()
            .zip(rhs.into_iter())
            .map(|(a, b)| a - b)
            .collect()
    }
}

impl Add for Mat2 {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        self.into_iter()
            .zip(rhs.into_iter())
            .map(|(a, b)| a + b)
            .collect()
    }
}

impl Div<f64> for Mat2 {
    type Output = Self;
    fn div(self, rhs: f64) -> Self::Output {
        self * rhs.recip()
    }
}

impl Mul<f64> for Mat2 {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self::Output {
        self.into_iter().map(|x| x * rhs).collect()
    }
}

impl Mul<Mat2> for f64 {
    type Output = Mat2;
    fn mul(self, rhs: Mat2) -> Self::Output {
        rhs * self
    }
}

impl Mul<Mat2> for Vec2 {
    type Output = Vec2;

    fn mul(self, rhs: Mat2) -> Self::Output {
        rhs.transpose() * self
    }
}

impl Mul<Vec2> for Mat2 {
    type Output = Vec2;

    fn mul(self, rhs: Vec2) -> Self::Output {
        self.0.map(|row| row.dot(rhs)).into()
    }
}

impl Mul<Mat2> for Mat2 {
    type Output = Mat2;

    fn mul(self, rhs: Mat2) -> Self::Output {
        Self::from_rows(self.row(0) * rhs, self.row(1) * rhs)
    }
}

impl Index<(usize, usize)> for Mat2 {
    type Output = f64;
    fn index(&self, (row, col): (usize, usize)) -> &Self::Output {
        &self.0[row][col]
    }
}

impl_ref_binop! {
    impl Add<Mat2> for Mat2{
        type Output = Mat2;
        fn add();
    }
    impl Div<f64> for Mat2{
        type Output = Mat2;
        fn div();
    }
    impl Mul<f64> for Mat2{
        type Output = Mat2;
        fn mul();
    }
    impl Mul<Mat2> for f64{
        type Output = Mat2;
        fn mul();
    }
    impl Sub<Mat2> for Mat2{
        type Output = Mat2;
        fn sub();
    }
}

impl IntoIterator for Mat2 {
    type Item = Vec2;
    type IntoIter = <[Vec2; 2] as IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl FromIterator<Vec2> for Mat2 {
    fn from_iter<I: IntoIterator<Item = Vec2>>(iter: I) -> Self {
        let mut iter = iter.into_iter();
        let x = iter.next().unwrap();
        let y = iter.next().unwrap();
        assert!(iter.next().is_none());
        Mat2::from_rows(x, y)
    }
}

#[test]
fn test_invert_mat2() {
    let mat = Mat2::from_rows(Vec2::new(3.0, 1.0), Vec2::new(4.0, 2.0));
    let inv = mat.invert();
    let id = mat * inv;
    assert_eq!(id, Mat2::id());
}

#[test]
fn test_invert_mat2_solve() {
    let mat = Mat2::from_rows(Vec2::new(2.0, 3.0), Vec2::new(4.0, 9.0));
    println!("{:?}", mat.invert());
    let v = mat.invert() * Vec2::new(6.0, 15.0);
    assert_eq!(v, Vec2::new(1.5, 1.0));
}
