use patina_scalar::Scalar;
use std::fmt::{Debug, Display, Formatter};
use std::iter::Sum;
use std::ops::{Add, AddAssign, Div, Index, IndexMut, Mul, Neg, Sub};

#[derive(Clone)]
pub struct Vector3<T>([T; 3]);

impl<T: Scalar> Vector3<T> {
    pub fn new(x: T, y: T, z: T) -> Self {
        Self([x, y, z])
    }
    pub fn splat(x: T) -> Self {
        Self::new(x.clone(), x.clone(), x)
    }
    pub fn normalize(self) -> Self
    where
        T: Div,
    {
        self.clone() / self.length()
    }
    pub fn length(self) -> T {
        self.length_squared().sqrt()
    }
    pub fn length_squared(self) -> T {
        self.into_iter()
            .map(|x| x.clone() * x)
            .fold(T::from_f64(0.0), T::add)
    }
    pub fn cross(self, rhs: Self) -> Self {
        Self([
            self.y() * rhs.z() - rhs.y() * self.z(),
            self.z() * rhs.x() - rhs.z() * self.x(),
            self.x() * rhs.y() - rhs.x() * self.y(),
        ])
    }
    pub fn x(&self) -> T {
        self.0[0].clone()
    }
    pub fn y(&self) -> T {
        self.0[1].clone()
    }
    pub fn z(&self) -> T {
        self.0[2].clone()
    }
    pub fn min(self, rhs: Self) -> Self {
        self.into_iter()
            .zip(rhs.into_iter())
            .map(|(x, y)| x.minimum(y))
            .collect()
    }
    pub fn max(self, rhs: Self) -> Self {
        self.into_iter()
            .zip(rhs.into_iter())
            .map(|(x, y)| x.maximum(y))
            .collect()
    }
    pub fn map<T2: Scalar, F: FnMut(T) -> T2>(&self, f: F) -> Vector3<T2> {
        self.clone().into_iter().map(f).collect()
    }
    pub fn zero() -> Self {
        Self::splat(T::from_f64(0.0))
    }
    pub fn axis_x() -> Self {
        Vector3::new(T::from_f64(1.0), T::from_f64(0.0), T::from_f64(0.0))
    }
    pub fn axis_y() -> Self {
        Vector3::new(T::from_f64(0.0), T::from_f64(1.0), T::from_f64(0.0))
    }
    pub fn axis_z() -> Self {
        Vector3::new(T::from_f64(0.0), T::from_f64(0.0), T::from_f64(1.0))
    }
    pub fn axes() -> [Self; 3] {
        [Self::axis_x(), Self::axis_y(), Self::axis_z()]
    }
    pub fn dot(self, rhs: Self) -> T {
        self.x() * rhs.x() + self.y() * rhs.y() + self.z() * rhs.z()
    }
    pub fn distance(self, other: Self) -> T {
        (self - other).length()
    }
}

impl<T> From<[T; 3]> for Vector3<T> {
    fn from(value: [T; 3]) -> Self {
        Vector3(value)
    }
}

impl<T> From<Vector3<T>> for [T; 3] {
    fn from(value: Vector3<T>) -> Self {
        value.0
    }
}

impl<T> AsRef<[T; 3]> for Vector3<T> {
    fn as_ref(&self) -> &[T; 3] {
        &self.0
    }
}

impl<T> IntoIterator for Vector3<T> {
    type Item = T;
    type IntoIter = core::array::IntoIter<T, 3>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl<T: Scalar> Neg for Vector3<T> {
    type Output = Vector3<T>;
    fn neg(self) -> Self::Output {
        self.map(|x| x.neg())
    }
}

impl<T: Scalar> Sub for Vector3<T> {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        self.into_iter()
            .zip(rhs.into_iter())
            .map(|(a, b)| a - b)
            .collect()
    }
}

impl<T: Scalar> Add for Vector3<T> {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        self.into_iter()
            .zip(rhs.into_iter())
            .map(|(a, b)| a + b)
            .collect()
    }
}

impl<T: Scalar> Div<T> for Vector3<T> {
    type Output = Self;
    fn div(self, rhs: T) -> Self::Output {
        self * rhs.recip()
    }
}

impl<T: Scalar> Mul<T> for Vector3<T> {
    type Output = Self;

    fn mul(self, rhs: T) -> Self::Output {
        self.into_iter().map(|x| x * rhs.clone()).collect()
    }
}

// impl_ref_binop! {
//     impl Add<Vector3<T>> for Vector3<T>{
//         type Output = Vector3<T>;
//         fn add();
//     }
//     impl Div<f64> for Vector3<T>{
//         type Output = Vector3<T>;
//         fn div();
//     }
//     impl Mul<f64> for Vector3<T>{
//         type Output = Vector3<T>;
//         fn mul();
//     }
//     impl Mul<Vector3<T>> for f64{
//         type Output = Vector3<T>;
//         fn mul();
//     }
//     impl Sub<Vector3<T>> for Vector3<T>{
//         type Output = Vector3<T>;
//         fn sub();
//     }
// }

impl<T: Scalar> AddAssign for Vector3<T> {
    fn add_assign(&mut self, rhs: Self) {
        for i in 0..3 {
            self[i] += rhs[i].clone();
        }
    }
}

impl<T: Scalar> AddAssign<&Vector3<T>> for Vector3<T> {
    fn add_assign(&mut self, rhs: &Vector3<T>) {
        *self += rhs.clone();
    }
}

impl<T: Scalar> Index<usize> for Vector3<T> {
    type Output = T;
    fn index(&self, index: usize) -> &Self::Output {
        self.0.index(index)
    }
}

impl<T: Scalar> IndexMut<usize> for Vector3<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        self.0.index_mut(index)
    }
}

impl<T: Scalar> Sum for Vector3<T> {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        let mut result = Self::splat(T::from_f64(0.0));
        for x in iter {
            result += x;
        }
        result
    }
}

impl<'a, T: Scalar> Sum<&'a Self> for Vector3<T> {
    fn sum<I: Iterator<Item = &'a Self>>(iter: I) -> Self {
        let mut total = Vector3::<T>::zero();
        for x in iter {
            total += x;
        }
        total
    }
}

impl<T: Scalar> FromIterator<T> for Vector3<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let mut iter = iter.into_iter();
        let x = iter.next().unwrap();
        let y = iter.next().unwrap();
        let z = iter.next().unwrap();
        assert!(iter.next().is_none());
        Vector3::<T>::new(x, y, z)
    }
}

impl<T: Scalar> Debug for Vector3<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{:.5}, {:.5}, {:.5}]", self.x(), self.y(), self.z())
    }
}

impl<T: Scalar> Display for Vector3<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:.5}\t{:.5}\t{:.5}", self.x(), self.y(), self.z())
    }
}
