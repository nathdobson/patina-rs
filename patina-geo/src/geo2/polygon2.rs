use anyhow::anyhow;
use itertools::Itertools;
use patina_vec::vec2::Vec2;
use rand::{Rng, SeedableRng};
use rand_xorshift::XorShiftRng;
use std::fmt::{Display, Formatter};
use std::ops::Range;
use crate::geo2::segment2::Segment2;

#[derive(Debug, Clone)]
pub struct Polygon2(Vec<Vec2>);

impl Polygon2 {
    pub fn new(points: Vec<Vec2>) -> Self {
        Self(points)
    }
    pub fn points(&self) -> &[Vec2] {
        &self.0
    }
    pub fn segments(&self) -> impl Clone + Iterator<Item = Segment2> {
        self.points()
            .iter()
            .cloned()
            .circular_tuple_windows()
            .map(|(p1, p2)| Segment2::new(p1, p2))
    }
    pub fn signed_area(&self) -> f64 {
        self.points()
            .iter()
            .circular_tuple_windows()
            .map(|(p1, p2)| p1.cross(*p2))
            .sum::<f64>()
            / 2.0
    }
    pub fn check_self_separate(&self) -> anyhow::Result<()> {
        for (v1, v2) in self.points().iter().tuple_combinations() {
            if v1 == v2 {
                return Err(anyhow!("vertex {} is the same as vertex {}", v1, v2));
            }
        }
        for &v in self.points() {
            for e in self.segments() {
                if e.p1() != v && e.p2() != v {
                    if e.distance(v) < 10e-10 {
                        return Err(anyhow!("vertex {:?} is on edge {:?}", v, e));
                    }
                }
            }
        }
        for (e1, e2) in self.segments().tuple_combinations() {
            if e1.p1() != e2.p1() && e1.p1() != e2.p2() && e1.p2() != e2.p1() && e1.p2() != e2.p2()
            {
                if e1.intersects(&e2) {
                    return Err(anyhow!("edge {} intersects edge {}", e1, e2));
                }
            }
        }
        Ok(())
    }
    pub fn check_simple(&self) -> anyhow::Result<()> {
        if self.signed_area() <= 0.0 {
            return Err(anyhow!("clockwise"));
        }
        self.check_self_separate()
    }
    fn random_complex<R: Rng>(r: &mut R, size: usize) -> Self {
        let mut poly: Vec<Vec2> = vec![];
        for _ in 0..size {
            poly.push(r.random());
        }
        Polygon2::new(poly)
    }
    fn random_discrete_complex<R: Rng>(r: &mut R, xs: usize, ys: usize, size: usize) -> Self {
        let mut poly: Vec<Vec2> = vec![];
        for _ in 0..size {
            poly.push(Vec2::new(
                r.random_range(0..=xs) as f64 / xs as f64,
                r.random_range(0..=ys) as f64 / ys as f64,
            ));
        }
        Polygon2::new(poly)
    }
    pub fn random<R: Rng>(r: &mut R, size: usize) -> Self {
        loop {
            let poly = Self::random_complex(r, size);
            if poly.check_simple().is_ok() {
                return poly;
            }
        }
    }
    pub fn random_discrete<R: Rng>(r: &mut R, xs: usize, ys: usize, size: usize) -> Self {
        loop {
            let poly = Self::random_discrete_complex(r, xs, ys, size);
            if poly.check_simple().is_ok() {
                return poly;
            }
        }
    }
    pub fn test_cases(sizes: Range<usize>, seeds: Range<u64>) -> impl Iterator<Item = Self> {
        sizes.flat_map(move |size| {
            seeds.clone().map(move |seed| {
                println!("size {} seed {}", size, seed);
                let mut rng = XorShiftRng::seed_from_u64(seed);
                let xs = rng.random_range(4..10);
                let poly = Polygon2::random_discrete(&mut rng, xs, 10, size);
                poly
            })
        })
    }
}

impl Display for Polygon2 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for x in &self.0 {
            writeln!(f, "{}", x)?;
        }
        Ok(())
    }
}
