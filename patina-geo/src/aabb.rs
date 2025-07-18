use crate::geo1::interval::Interval;
use crate::ray::Ray;
use crate::segment2::Segment;
use patina_vec::vec::Vector;
use patina_vec::vec3::Vec3;
use rand::Rng;
use rand::prelude::Distribution;
use std::ops::Range;

#[derive(Copy, Clone, Debug)]
pub struct Aabb<const N: usize> {
    min: Vector<f64, N>,
    max: Vector<f64, N>,
}

impl<const N: usize> Aabb<N> {
    pub fn new(min: Vector<f64, N>, max: Vector<f64, N>) -> Self {
        Aabb { min, max }
    }
    pub fn from_intervals(is: [Interval; N]) -> Self {
        Self::new(is.map(|x| x.min()).into(), is.map(|x| x.max()).into())
    }
    pub fn min(&self) -> Vector<f64, N> {
        self.min
    }
    pub fn max(&self) -> Vector<f64, N> {
        self.max
    }
    pub fn center(&self) -> Vector<f64, N> {
        (self.min + self.max) / 2.0
    }
    pub fn from_point(p: Vector<f64, N>) -> Self {
        Self::new(p, p)
    }
    pub fn empty() -> Self {
        Self::new(Vector::splat(f64::INFINITY), Vector::splat(-f64::INFINITY))
    }
    pub fn union(&self, other: &Self) -> Self {
        Self::new(self.min.minimum(other.min), self.max.maximum(other.max))
    }
    pub fn intersect(&self, other: &Self) -> Self {
        Self::new(self.min.maximum(other.min), self.max.minimum(other.max))
    }
    pub fn dimensions(&self) -> Vector<f64, N> {
        (self.max - self.min).maximum(Vector::zero())
    }
    pub fn intersects(&self, other: &Self) -> bool {
        self.intersect(other)
            .dimensions()
            .into_iter()
            .all(|x| x >= 0.0)
    }
    pub fn surface_measure(&self) -> f64 {
        let d = self.dimensions().maximum(Vector::splat(0.0));
        if N == 0 {
            0.0
        } else if N == 1 {
            2.0 * d[0]
        } else if N == 2 {
            2.0 * (d[0] + d[1])
        } else if N == 3 {
            2.0 * (d[0] * d[1] + d[0] * d[2] + d[1] * d[2])
        } else {
            panic!();
        }
    }
    pub fn intersect_segment(&self, segment: &Segment<N>) -> Option<Interval> {
        let mut range = Interval::new(0.0, 1.0);
        for axis in 0..N {
            let x1 = segment.p1()[axis];
            let x2 = segment.p2()[axis];
            let min = self.min[axis];
            let max = self.max[axis];
            let diff = x2 - x1;
            if diff == 0.0 {
                if x1 >= min && x1 <= max {
                    continue;
                } else {
                    return None;
                }
            } else {
                let tmin = (min - x1) / diff;
                let tmax = (max - x1) / diff;
                range = range.intersect(Interval::new(tmin.min(tmax), tmin.max(tmax)));
            }
        }
        (range.min() <= range.max()).then_some(range)
    }
    pub fn intersect_ray(&self, ray: &Ray<N>) -> Option<Interval> {
        let mut range = Interval::new(0.0, f64::INFINITY);
        for axis in 0..N {
            let e = ray.origin()[axis];
            let d = ray.dir()[axis];
            let min = self.min[axis];
            let max = self.max[axis];
            if d == 0.0 {
                if e >= min && e <= max {
                    continue;
                } else {
                    return None;
                }
            } else {
                let tmin = (min - e) / d;
                let tmax = (max - e) / d;
                range = range.intersect(Interval::new(tmin.min(tmax), tmin.max(tmax)));
            }
        }
        (range.min() <= range.max()).then_some(range)
    }
}

impl<const N: usize> Distribution<Vector<f64, N>> for Aabb<N> {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Vector<f64, N> {
        (0..3)
            .map(|axis| rng.random_range(self.min[axis]..self.max[axis]))
            .collect()
    }
}

impl<const N: usize> FromIterator<Aabb<N>> for Aabb<N> {
    fn from_iter<T: IntoIterator<Item = Aabb<N>>>(iter: T) -> Self {
        iter.into_iter()
            .reduce(|x, y| x.union(&y))
            .unwrap_or(Aabb::empty())
    }
}

impl<const N: usize> FromIterator<Vector<f64, N>> for Aabb<N> {
    fn from_iter<T: IntoIterator<Item = Vector<f64, N>>>(iter: T) -> Self {
        iter.into_iter().map(|x| Aabb::from_point(x)).collect()
    }
}
