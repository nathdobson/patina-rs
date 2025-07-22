use itertools::Itertools;
use patina_scalar::Scalar;
use patina_scalar::deriv::Deriv;
use rand::Rng;
use rand::distr::{Distribution, StandardUniform};
use std::fmt::{Debug, Display, Formatter};
use std::iter;
use std::iter::Sum;
use std::ops::{Add, AddAssign, Div, Index, IndexMut, Mul, Neg, Sub};

#[derive(Copy, Clone, Eq, Ord, PartialEq, PartialOrd, Hash)]
pub struct Vector<T, const N: usize>([T; N]);

impl<const N: usize> Vector<f64, N> {
    pub fn into_scalars<T2: Scalar>(self) -> Vector<T2, N> {
        self.map(T2::from_f64)
    }
    pub fn into_variable(&self) -> Vector<Deriv<N>, N> {
        Vector::from_fn(|axis| Deriv::variable(self[axis], axis))
    }
    pub fn random_normal<R: Rng>(rng: &mut R) -> Self {
        loop {
            let result = Self::from_fn(|_| rng.random_range(-1.0..1.0));
            let length2 = result.length_squared();
            if length2 <= 1.0 {
                return result / length2;
            }
        }
    }
}

impl<T, const N: usize> Vector<T, N>
where
    [T; N - 1]: Sized,
{
    pub fn x(&self) -> T
    where
        T: Clone,
    {
        self.0[0].clone()
    }
    pub fn axis_x() -> Self
    where
        T: Scalar,
    {
        Self::axis(0)
    }
}

impl<T, const N: usize> Vector<T, N>
where
    [T; N - 2]: Sized,
{
    pub fn y(&self) -> T
    where
        T: Clone,
    {
        self.0[1].clone()
    }
    pub fn axis_y() -> Self
    where
        T: Scalar,
    {
        Self::axis(1)
    }
}

impl<T, const N: usize> Vector<T, N>
where
    [T; N - 3]: Sized,
{
    pub fn z(&self) -> T
    where
        T: Clone,
    {
        self.0[2].clone()
    }
    pub fn axis_z() -> Self
    where
        T: Scalar,
    {
        Self::axis(2)
    }
}

impl<T, const N: usize> Vector<T, N>
where
    [T; N - 4]: Sized,
{
    pub fn w(&self) -> T
    where
        T: Clone,
    {
        self.0[3].clone()
    }
    pub fn axis_w() -> Self
    where
        T: Scalar,
    {
        Self::axis(3)
    }
}

impl<T, const N: usize> Vector<T, N> {
    pub fn from_fn(f: impl FnMut(usize) -> T) -> Self {
        (0..N).map(f).collect()
    }
    pub fn splat(x: T) -> Self
    where
        T: Clone,
    {
        iter::repeat(x).take(N).collect()
    }
    pub fn normalize(self) -> Self
    where
        T: Scalar,
    {
        self.clone() / self.length()
    }
    pub fn length(self) -> T
    where
        T: Scalar,
    {
        self.length_squared().sqrt()
    }
    pub fn length_squared(self) -> T
    where
        T: Scalar,
    {
        self.into_iter()
            .map(|x| x.clone() * x)
            .fold(T::from_f64(0.0), T::add)
    }
    pub fn zip_with<T2, T3>(
        self,
        rhs: Vector<T2, N>,
        mut f: impl FnMut(T, T2) -> T3,
    ) -> Vector<T3, N> {
        self.into_iter()
            .zip(rhs.into_iter())
            .map(|(x, y)| f(x, y))
            .collect()
    }
    pub fn minimum(self, rhs: Self) -> Self
    where
        T: Scalar,
    {
        self.zip_with(rhs, |x, y| x.minimum(y))
    }
    pub fn maximum(self, rhs: Self) -> Self
    where
        T: Scalar,
    {
        self.zip_with(rhs, |x, y| x.maximum(y))
    }
    pub fn map<T2, F: FnMut(T) -> T2>(self, f: F) -> Vector<T2, N> {
        self.into_iter().map(f).collect()
    }
    pub fn zero() -> Self
    where
        T: Scalar,
    {
        Self::splat(T::from_f64(0.0))
    }
    pub fn axis(axis: usize) -> Self
    where
        T: Scalar,
    {
        let mut result = Self::zero();
        result[axis] = T::from_f64(1.0);
        result
    }

    pub fn axes() -> [Self; N]
    where
        T: Scalar,
    {
        (0..N).map(|i| Self::axis(i)).collect_array().unwrap()
    }
    pub fn dot(self, rhs: Self) -> T
    where
        T: Clone + Add<Output = T> + Mul<Output = T>,
    {
        self.mul_elements(rhs).into_iter().reduce(T::add).unwrap()
    }
    pub fn distance(self, other: Self) -> T
    where
        T: Scalar,
    {
        (self - other).length()
    }
    pub fn abs(self) -> Self
    where
        T: Scalar,
    {
        self.map(|x| x.abs())
    }
    pub fn mul_elements(self, rhs: Self) -> Self
    where
        T: Mul<Output = T>,
    {
        self.zip_with(rhs, |x, y| x * y)
    }
    pub fn into_inner(self) -> [T; N] {
        self.0
    }
}

impl<T, const N: usize> From<[T; N]> for Vector<T, N> {
    fn from(value: [T; N]) -> Self {
        Vector(value)
    }
}
impl<T, const N: usize> From<Vector<T, N>> for [T; N] {
    fn from(value: Vector<T, N>) -> Self {
        value.0
    }
}

impl<T, const N: usize> AsRef<[T; N]> for Vector<T, N> {
    fn as_ref(&self) -> &[T; N] {
        &self.0
    }
}

impl<T, const N: usize> IntoIterator for Vector<T, N> {
    type Item = T;
    type IntoIter = core::array::IntoIter<T, N>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl<T: Neg<Output = T>, const N: usize> Neg for Vector<T, N> {
    type Output = Vector<T, N>;
    fn neg(self) -> Self::Output {
        self.map(|x| x.neg())
    }
}

impl<T: Sub<Output = T>, const N: usize> Sub for Vector<T, N> {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        self.zip_with(rhs, |x, y| x - y)
    }
}

impl<T: Add<Output = T>, const N: usize> Add for Vector<T, N> {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        self.zip_with(rhs, |x, y| x + y)
    }
}

impl<T: Clone + Scalar, const N: usize> Div<T> for Vector<T, N> {
    type Output = Self;
    fn div(self, rhs: T) -> Self::Output {
        self * rhs.recip()
    }
}

impl<T: Clone + Mul<Output = T>, const N: usize> Mul<T> for Vector<T, N> {
    type Output = Self;

    fn mul(self, rhs: T) -> Self::Output {
        self.into_iter().map(|x| x * rhs.clone()).collect()
    }
}

impl<T: Clone + Add<Output = T>, const N: usize> AddAssign for Vector<T, N> {
    fn add_assign(&mut self, rhs: Self) {
        for i in 0..N {
            self[i] = self[i].clone() + rhs[i].clone();
        }
    }
}

impl<T: Clone + Add<Output = T>, const N: usize> AddAssign<&Vector<T, N>> for Vector<T, N> {
    fn add_assign(&mut self, rhs: &Vector<T, N>) {
        *self += rhs.clone();
    }
}

impl<T, const N: usize> Index<usize> for Vector<T, N> {
    type Output = T;
    fn index(&self, index: usize) -> &Self::Output {
        self.0.index(index)
    }
}

impl<T, const N: usize> IndexMut<usize> for Vector<T, N> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        self.0.index_mut(index)
    }
}

impl<T: Add<Output = T>, const N: usize> Sum for Vector<T, N> {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.into_iter().reduce(Self::add).unwrap()
    }
}

impl<'a, T: Scalar, const N: usize> Sum<&'a Self> for Vector<T, N> {
    fn sum<I: Iterator<Item = &'a Self>>(iter: I) -> Self {
        iter.into_iter().cloned().sum()
    }
}

impl<T, const N: usize> FromIterator<T> for Vector<T, N> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        Self(iter.into_iter().collect_array().unwrap())
    }
}

impl<T: Display, const N: usize> Debug for Vector<T, N> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "[")?;
        for x in &self.0[0..N - 1] {
            write!(f, "{:.5}, ", x)?;
        }
        write!(f, "{:.5}]", self.0[N - 1])?;
        Ok(())
    }
}

impl<T: Display, const N: usize> Display for Vector<T, N> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for x in &self.0[0..N - 1] {
            write!(f, "{:.5}\t", x)?;
        }
        write!(f, "{:.5}", self.0[N - 1])?;
        Ok(())
    }
}

impl<T, const N: usize> Distribution<Vector<T, N>> for StandardUniform
where
    StandardUniform: Distribution<T>,
{
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Vector<T, N> {
        Vector::from_fn(|_| rng.random())
    }
}
