use patina_vec::vec::Vector;
use patina_vec::vec2::Vec2;
use std::cmp::Ordering;

pub struct Ray<const N: usize> {
    origin: Vector<f64, N>,
    dir: Vector<f64, N>,
}


impl<const N: usize> Ray<N> {
    pub fn new(origin: Vector<f64, N>, dir: Vector<f64, N>) -> Self {
        Self { origin, dir }
    }
    pub fn origin(&self) -> Vector<f64, N> {
        self.origin
    }
    pub fn dir(&self) -> Vector<f64, N> {
        self.dir
    }

    pub fn at_time(&self, t: f64) -> Vector<f64, N> {
        self.origin + self.dir * t
    }
    
}
