use crate::math::macros::impl_ref_binop;
use crate::math::vec2::Vec2;
use rand::Rng;
use rand::distr::{Distribution, StandardUniform};
use std::fmt::{Debug, Display, Formatter};
use std::iter::Sum;
use std::ops::{Add, AddAssign, Div, Index, IndexMut, Mul, Neg, Sub};

#[derive(Copy, Clone, PartialEq)]
pub struct Vec3([f64; 3]);

impl Vec3 {
    pub fn splat(x: f64) -> Self {
        Self::new(x, x, x)
    }
    pub fn new(x: f64, y: f64, z: f64) -> Vec3 {
        Vec3([x, y, z])
    }
    pub fn normalize(self) -> Self {
        self / self.length()
    }
    pub fn length(self) -> f64 {
        self.length_squared().sqrt()
    }
    pub fn length_squared(self) -> f64 {
        self.into_iter().map(|x| x * x).sum::<f64>()
    }
    pub fn cross(self, rhs: Vec3) -> Self {
        Self([
            self.y() * rhs.z() - rhs.y() * self.z(),
            self.z() * rhs.x() - rhs.z() * self.x(),
            self.x() * rhs.y() - rhs.x() * self.y(),
        ])
    }
    pub fn x(self) -> f64 {
        self.0[0]
    }
    pub fn y(self) -> f64 {
        self.0[1]
    }
    pub fn z(self) -> f64 {
        self.0[2]
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
    pub fn map<F: FnMut(f64) -> f64>(&self, f: F) -> Vec3 {
        self.into_iter().map(f).collect()
    }
    pub fn zero() -> Self {
        Self::splat(0.0)
    }
    pub fn axis_x() -> Self {
        Vec3::new(1.0, 0.0, 0.0)
    }
    pub fn axis_y() -> Self {
        Vec3::new(0.0, 1.0, 0.0)
    }
    pub fn axis_z() -> Self {
        Vec3::new(0.0, 0.0, 1.0)
    }
    pub fn axes() -> [Self; 3] {
        [Self::axis_x(), Self::axis_y(), Self::axis_z()]
    }
    pub fn dot(self, rhs: Self) -> f64 {
        self.x() * rhs.x() + self.y() * rhs.y() + self.z() * rhs.z()
    }
    pub fn distance(self, other: Self) -> f64 {
        (self - other).length()
    }
    pub fn is_finite(self) -> bool {
        self.0.iter().all(|x| x.is_finite())
    }
    pub fn random_unit(rng: &mut impl Rng) -> Self {
        loop {
            let v = Vec3::new(
                rng.random_range(-1.0..1.0),
                rng.random_range(-1.0..1.0),
                rng.random_range(-1.0..1.0),
            );
            if v.length() < 1.0 {
                continue;
            }
            return v.normalize();
        }
    }
    pub fn nan() -> Self {
        Self::splat(f64::NAN)
    }
}

impl From<[f64; 3]> for Vec3 {
    fn from(value: [f64; 3]) -> Self {
        Vec3(value)
    }
}

impl From<Vec3> for [f64; 3] {
    fn from(value: Vec3) -> Self {
        value.0
    }
}

impl AsRef<[f64; 3]> for Vec3 {
    fn as_ref(&self) -> &[f64; 3] {
        &self.0
    }
}

impl IntoIterator for Vec3 {
    type Item = f64;
    type IntoIter = core::array::IntoIter<f64, 3>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl Neg for Vec3 {
    type Output = Vec3;
    fn neg(self) -> Self::Output {
        self.map(|x| x.neg())
    }
}

impl Sub for Vec3 {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        self.into_iter()
            .zip(rhs.into_iter())
            .map(|(a, b)| a - b)
            .collect()
    }
}

impl Add for Vec3 {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        self.into_iter()
            .zip(rhs.into_iter())
            .map(|(a, b)| a + b)
            .collect()
    }
}

impl Div<f64> for Vec3 {
    type Output = Self;
    fn div(self, rhs: f64) -> Self::Output {
        self * rhs.recip()
    }
}

impl Mul<f64> for Vec3 {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self::Output {
        self.into_iter().map(|x| x * rhs).collect()
    }
}

impl Mul<Vec3> for f64 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Self::Output {
        rhs.into_iter().map(|x| x.mul(self)).collect()
    }
}

impl_ref_binop! {
    impl Add<Vec3> for Vec3{
        type Output = Vec3;
        fn add();
    }
    impl Div<f64> for Vec3{
        type Output = Vec3;
        fn div();
    }
    impl Mul<f64> for Vec3{
        type Output = Vec3;
        fn mul();
    }
    impl Mul<Vec3> for f64{
        type Output = Vec3;
        fn mul();
    }
    impl Sub<Vec3> for Vec3{
        type Output = Vec3;
        fn sub();
    }
}

impl AddAssign for Vec3 {
    fn add_assign(&mut self, rhs: Self) {
        for i in 0..3 {
            self[i] += rhs[i];
        }
    }
}

impl AddAssign<&Vec3> for Vec3 {
    fn add_assign(&mut self, rhs: &Vec3) {
        *self += *rhs;
    }
}

impl Index<usize> for Vec3 {
    type Output = f64;
    fn index(&self, index: usize) -> &Self::Output {
        self.0.index(index)
    }
}

impl IndexMut<usize> for Vec3 {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        self.0.index_mut(index)
    }
}

impl Sum for Vec3 {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        todo!()
    }
}

impl<'a> Sum<&'a Self> for Vec3 {
    fn sum<I: Iterator<Item = &'a Self>>(iter: I) -> Self {
        let mut total = Vec3::zero();
        for x in iter {
            total += x;
        }
        total
    }
}

impl FromIterator<f64> for Vec3 {
    fn from_iter<I: IntoIterator<Item = f64>>(iter: I) -> Self {
        let mut iter = iter.into_iter();
        let x = iter.next().unwrap();
        let y = iter.next().unwrap();
        let z = iter.next().unwrap();
        assert!(iter.next().is_none());
        Vec3::new(x, y, z)
    }
}

impl Debug for Vec3 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{:.5}, {:.5}, {:.5}]", self.x(), self.y(), self.z())
    }
}

impl Display for Vec3 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:.5}\t{:.5}\t{:.5}", self.x(), self.y(), self.z())
    }
}

impl Distribution<Vec3> for StandardUniform {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Vec3 {
        Vec3::new(rng.random(), rng.random(), rng.random())
    }
}
