use crate::vec::Vector;
use crate::vec2::Vector2;
use crate::vec3::Vec3;
use crate::vec4::Vec4;
use itertools::Itertools;
use patina_scalar::Scalar;
use std::fmt::{Debug, Display, Formatter};
use std::ops::{Add, Div, Index, Mul, Sub};

#[derive(PartialEq, PartialOrd, Copy, Clone)]
pub struct Matrix<T, const N: usize>([Vector<T, N>; N]);

pub type Matrix3<T> = Matrix<T, 3>;
pub type Matrix4<T> = Matrix<T, 4>;

impl<T, const N: usize> Matrix<T, N> {
    pub fn from_fn(mut f: impl FnMut(usize, usize) -> T) -> Self {
        Self(
            (0..N)
                .map(|row| Vector::from_fn(|col| f(row, col)))
                .collect_array()
                .unwrap(),
        )
    }
    pub fn from_rows(rows: [Vector<T, N>; N]) -> Self
    where
        T: Clone,
    {
        Self(rows)
    }
    pub fn from_cols(cols: [Vector<T, N>; N]) -> Self
    where
        T: Clone,
    {
        Self(cols).transpose()
    }
    pub fn transpose(&self) -> Self
    where
        T: Clone,
    {
        Self((0..N).map(|col| self.col(col)).collect_array().unwrap())
    }
    pub fn row(&self, r: usize) -> Vector<T, N>
    where
        T: Clone,
    {
        self.0[r].clone()
    }
    pub fn col(&self, c: usize) -> Vector<T, N>
    where
        T: Clone,
    {
        Vector::from(self.0.clone().map(|row| row[c].clone()))
    }

    pub fn id() -> Self
    where
        T: Scalar,
    {
        Self::from_rows(Vector::<T, N>::axes())
    }
}

impl<T, const N: usize> Index<(usize, usize)> for Matrix<T, N> {
    type Output = T;
    fn index(&self, index: (usize, usize)) -> &Self::Output {
        &self.0[index.0][index.1]
    }
}

impl<T: Scalar, const N: usize> Div<T> for Matrix<T, N> {
    type Output = Self;
    fn div(self, rhs: T) -> Self::Output {
        self * rhs.recip()
    }
}

impl<T: Scalar, const N: usize> Mul<T> for Matrix<T, N> {
    type Output = Self;
    fn mul(self, rhs: T) -> Self::Output {
        Matrix(self.0.map(|row| row * rhs.clone()))
    }
}

impl<T: Scalar, const N: usize> Mul<Vector<T, N>> for Matrix<T, N> {
    type Output = Vector<T, N>;

    fn mul(self, rhs: Vector<T, N>) -> Self::Output {
        Vector::from(self.0.map(|row| row.dot(rhs.clone())))
    }
}

impl<T: Scalar, const N: usize> Mul<Matrix<T, N>> for Matrix<T, N> {
    type Output = Matrix<T, N>;
    fn mul(self, rhs: Matrix<T, N>) -> Self::Output {
        Self::from_fn(|row, col| self.row(row).dot(rhs.col(col)))
    }
}

impl<T: Scalar, const N: usize> Add<Matrix<T, N>> for Matrix<T, N> {
    type Output = Self;
    fn add(self, rhs: Matrix<T, N>) -> Self::Output {
        Self::from_fn(|row, col| self[(row, col)].clone() + rhs[(row, col)].clone())
    }
}

impl<T: Scalar, const N: usize> Sub<Matrix<T, N>> for Matrix<T, N> {
    type Output = Self;
    fn sub(self, rhs: Matrix<T, N>) -> Self::Output {
        Self::from_fn(|row, col| self[(row, col)].clone() - rhs[(row, col)].clone())
    }
}

impl<T: Display, const N: usize> Debug for Matrix<T, N> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

impl<T: Display, const N: usize> Display for Matrix<T, N> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}
