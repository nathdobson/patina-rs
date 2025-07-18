use crate::geo1::interval::Interval;
use crate::ray::Ray;
use patina_vec::vec::Vector;
use patina_vec::vec2::Vec2;
use std::cmp::Ordering;
use std::fmt::{Display, Formatter};

#[derive(Debug, Clone)]
pub struct Segment<const N: usize>([Vector<f64, N>; 2]);

impl<const N: usize> Segment<N> {
    pub fn new(p1: Vector<f64, N>, p2: Vector<f64, N>) -> Self {
        Self([p1, p2])
    }
    pub fn points(&self) -> &[Vector<f64, N>; 2] {
        &self.0
    }
    pub fn p1(&self) -> Vector<f64, N> {
        self.0[0]
    }
    pub fn p2(&self) -> Vector<f64, N> {
        self.0[1]
    }
    pub fn as_ray(&self) -> Ray<N> {
        Ray::new(self.p1(), self.p2() - self.p1())
    }

    pub fn distance(&self, other: Vector<f64, N>) -> f64 {
        let u = self.p2() - self.p1();
        let v = other - self.p1();
        let proj_fract = u.dot(v) / (u.dot(u));
        if proj_fract <= 0.0 {
            other.distance(self.p1())
        } else if proj_fract >= 1.0 {
            other.distance(self.p2())
        } else {
            v.distance(u * proj_fract)
        }
    }
    pub fn midpoint(&self) -> Vector<f64, N> {
        (self.p1() + self.p2()) / 2.0
    }
}

impl<const N: usize> Display for Segment<N> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "\n{}\n{}\n", self.p1(), self.p2())
    }
}
