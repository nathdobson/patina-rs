use crate::math::macros::impl_ref_binop;
use std::fmt::{Debug, Formatter};
use std::iter::Sum;
use std::ops::{Add, AddAssign, Div, Index, IndexMut, Mul, Neg, Sub};

#[derive(Copy, Clone, PartialEq)]
pub struct Vec2([f64; 2]);

impl Vec2 {
    pub fn splat(x: f64) -> Self {
        Self::new(x, x)
    }
    pub fn new(x: f64, y: f64) -> Vec2 {
        Vec2([x, y])
    }
    pub fn normalize(self) -> Self {
        self / self.length()
    }
    pub fn length(self) -> f64 {
        self.into_iter().map(|x| x * x).sum::<f64>().sqrt()
    }
    pub fn x(self) -> f64 {
        self.0[0]
    }
    pub fn y(self) -> f64 {
        self.0[1]
    }
    pub fn min(self, rhs: Self) -> Self {
        self.into_iter()
            .zip(rhs.into_iter())
            .map(|(x, y)| x.min(y))
            .collect()
    }
    pub fn max(self, rhs: Self) -> Self {
        self.into_iter()
            .zip(rhs.into_iter())
            .map(|(x, y)| x.max(y))
            .collect()
    }
    pub fn map<F: FnMut(f64) -> f64>(&self, f: F) -> Vec2 {
        self.into_iter().map(f).collect()
    }
    pub fn zero() -> Self {
        Self::splat(0.0)
    }
    pub fn axis_x() -> Self {
        Vec2::new(1.0, 0.0)
    }
    pub fn axis_y() -> Self {
        Vec2::new(0.0, 1.0)
    }
    pub fn dot(self, rhs: Self) -> f64 {
        self.x() * rhs.x() + self.y() * rhs.y()
    }
    pub fn distance(self, other: Self) -> f64 {
        (self - other).length()
    }
    pub fn cross(self, rhs: Self) -> f64 {
        self.x() * rhs.y() - self.y() * rhs.x()
    }
    pub fn angle(self) -> f64 {
        self.y().atan2(self.x())
    }
}

impl From<[f64; 2]> for Vec2 {
    fn from(value: [f64; 2]) -> Self {
        Vec2(value)
    }
}

impl From<Vec2> for [f64; 2] {
    fn from(value: Vec2) -> Self {
        value.0
    }
}

impl AsRef<[f64; 2]> for Vec2 {
    fn as_ref(&self) -> &[f64; 2] {
        &self.0
    }
}

impl IntoIterator for Vec2 {
    type Item = f64;
    type IntoIter = core::array::IntoIter<f64, 2>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl Neg for Vec2 {
    type Output = Vec2;
    fn neg(self) -> Self::Output {
        self.map(|x| x.neg())
    }
}

impl Sub for Vec2 {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        self.into_iter()
            .zip(rhs.into_iter())
            .map(|(a, b)| a - b)
            .collect()
    }
}

impl Add for Vec2 {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        self.into_iter()
            .zip(rhs.into_iter())
            .map(|(a, b)| a + b)
            .collect()
    }
}

impl Div<f64> for Vec2 {
    type Output = Self;
    fn div(self, rhs: f64) -> Self::Output {
        self * rhs.recip()
    }
}

impl Mul<f64> for Vec2 {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self::Output {
        self.into_iter().map(|x| x * rhs).collect()
    }
}

impl Mul<Vec2> for f64 {
    type Output = Vec2;

    fn mul(self, rhs: Vec2) -> Self::Output {
        rhs.into_iter().map(|x| x.mul(self)).collect()
    }
}

impl_ref_binop! {
    impl Add<Vec2> for Vec2{
        type Output = Vec2;
        fn add();
    }
    impl Div<f64> for Vec2{
        type Output = Vec2;
        fn div();
    }
    impl Mul<f64> for Vec2{
        type Output = Vec2;
        fn mul();
    }
    impl Mul<Vec2> for f64{
        type Output = Vec2;
        fn mul();
    }
    impl Sub<Vec2> for Vec2{
        type Output = Vec2;
        fn sub();
    }
}

impl AddAssign for Vec2 {
    fn add_assign(&mut self, rhs: Self) {
        for i in 0..3 {
            self[i] += rhs[i];
        }
    }
}

impl AddAssign<&Vec2> for Vec2 {
    fn add_assign(&mut self, rhs: &Vec2) {
        *self += *rhs;
    }
}

impl Index<usize> for Vec2 {
    type Output = f64;
    fn index(&self, index: usize) -> &Self::Output {
        self.0.index(index)
    }
}

impl IndexMut<usize> for Vec2 {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        self.0.index_mut(index)
    }
}

impl Sum for Vec2 {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        todo!()
    }
}

impl<'a> Sum<&'a Self> for Vec2 {
    fn sum<I: Iterator<Item = &'a Self>>(iter: I) -> Self {
        let mut total = Vec2::zero();
        for x in iter {
            total += x;
        }
        total
    }
}

impl FromIterator<f64> for Vec2 {
    fn from_iter<I: IntoIterator<Item = f64>>(iter: I) -> Self {
        let mut iter = iter.into_iter();
        let x = iter.next().unwrap();
        let y = iter.next().unwrap();
        assert!(iter.next().is_none());
        Vec2::new(x, y)
    }
}

impl Debug for Vec2 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{:.5}, {:.5}]", self.x(), self.y())
    }
}
