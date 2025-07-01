use crate::math::macros::impl_ref_binop;
use crate::math::vec3::Vec3;
use crate::math::vec4::Vec4;
use itertools::Itertools;
use std::ops::{Add, Div, Index, Mul, Sub};

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Mat4([Vec4; 4]);

impl Mat4 {
    pub fn from_rows(r1: Vec4, r2: Vec4, r3: Vec4, r4: Vec4) -> Mat4 {
        Mat4([r1, r2, r3, r4])
    }
    pub fn from_cols(c1: Vec4, c2: Vec4, c3: Vec4, c4: Vec4) -> Mat4 {
        Self::from_rows(c1, c2, c3, c4).transpose()
    }
    pub fn transpose(&self) -> Mat4 {
        Mat4::from_rows(self.col(0), self.col(1), self.col(2), self.col(3))
    }
    pub fn row(&self, r: usize) -> Vec4 {
        self.0[r]
    }
    pub fn col(&self, c: usize) -> Vec4 {
        Vec4::new(
            self.row(0)[c],
            self.row(1)[c],
            self.row(2)[c],
            self.row(3)[c],
        )
    }
    pub fn id() -> Self {
        Mat4::from_rows(
            Vec4::axis_x(),
            Vec4::axis_y(),
            Vec4::axis_z(),
            Vec4::axis_w(),
        )
    }
    pub fn as_affine(&self) -> Option<[f64; 12]> {
        if self.row(3) != Vec4::axis_w() {
            return None;
        }
        let mut result = [0f64; 12];
        for r in 0..3 {
            for c in 0..4 {
                result[c * 3 + r] = self[(r, c)];
            }
        }
        Some(result)
    }
    pub fn translate(v: Vec3) -> Self {
        Self::from_cols(
            Vec4::axis_x(),
            Vec4::axis_y(),
            Vec4::axis_z(),
            Vec4::new(v.x(), v.y(), v.z(), 1.0),
        )
    }
}

impl Sub for Mat4 {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        self.into_iter()
            .zip(rhs.into_iter())
            .map(|(a, b)| a - b)
            .collect()
    }
}

impl Add for Mat4 {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        self.into_iter()
            .zip(rhs.into_iter())
            .map(|(a, b)| a + b)
            .collect()
    }
}

impl Div<f64> for Mat4 {
    type Output = Self;
    fn div(self, rhs: f64) -> Self::Output {
        self * rhs.recip()
    }
}

impl Mul<f64> for Mat4 {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self::Output {
        self.into_iter().map(|x| x * rhs).collect()
    }
}

impl Mul<Mat4> for f64 {
    type Output = Mat4;
    fn mul(self, rhs: Mat4) -> Self::Output {
        rhs * self
    }
}

impl Mul<Mat4> for Vec4 {
    type Output = Vec4;

    fn mul(self, rhs: Mat4) -> Self::Output {
        rhs.transpose() * self
    }
}

impl Mul<Vec4> for Mat4 {
    type Output = Vec4;

    fn mul(self, rhs: Vec4) -> Self::Output {
        self.0.map(|row| row.dot(rhs)).into()
    }
}

impl Mul<Mat4> for Mat4 {
    type Output = Mat4;

    fn mul(self, rhs: Mat4) -> Self::Output {
        Self::from_rows(
            self.row(0) * rhs,
            self.row(1) * rhs,
            self.row(2) * rhs,
            self.row(3) * rhs,
        )
    }
}

impl Index<(usize, usize)> for Mat4 {
    type Output = f64;
    fn index(&self, (row, col): (usize, usize)) -> &Self::Output {
        &self.0[row][col]
    }
}

impl_ref_binop! {
    impl Add<Mat4> for Mat4{
        type Output = Mat4;
        fn add();
    }
    impl Div<f64> for Mat4{
        type Output = Mat4;
        fn div();
    }
    impl Mul<f64> for Mat4{
        type Output = Mat4;
        fn mul();
    }
    impl Mul<Mat4> for f64{
        type Output = Mat4;
        fn mul();
    }
    impl Sub<Mat4> for Mat4{
        type Output = Mat4;
        fn sub();
    }
}

impl IntoIterator for Mat4 {
    type Item = Vec4;
    type IntoIter = <[Vec4; 4] as IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl FromIterator<Vec4> for Mat4 {
    fn from_iter<I: IntoIterator<Item = Vec4>>(iter: I) -> Self {
        let mut iter = iter.into_iter();
        let x = iter.next().unwrap();
        let y = iter.next().unwrap();
        let z = iter.next().unwrap();
        let w = iter.next().unwrap();
        assert!(iter.next().is_none());
        Mat4::from_rows(x, y, z, w)
    }
}

#[test]
fn test_affine() {
    assert_eq!(
        Mat4::translate(Vec3::new(1.0, 2.0, 3.0)).as_affine(),
        Some([
            1.0, 0.0, 0.0, //
            0.0, 1.0, 0.0, //
            0.0, 0.0, 1.0, //
            1.0, 2.0, 3.0 //
        ])
    );
}
