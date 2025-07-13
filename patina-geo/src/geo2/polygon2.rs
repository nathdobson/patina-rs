use crate::geo2::segment2::Segment2;
use itertools::Itertools;
use patina_vec::vec2::Vec2;
use rand::Rng;
use std::fmt::{Display, Formatter};

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
    pub fn random<R: Rng>(r: &mut R, size: usize) -> Self {
        'search: loop {
            let mut poly: Vec<Vec2> = vec![];
            for _ in 0..size {
                poly.push(r.random());
            }
            let poly = Polygon2::new(poly);
            if poly.signed_area() <= 0.0 {
                continue;
            }
            for (e1, e2) in poly.segments().tuple_combinations() {
                if e1.p1() != e2.p1()
                    && e1.p1() != e2.p2()
                    && e1.p2() != e2.p1()
                    && e1.p2() != e2.p2()
                {
                    if e1.intersects(&e2) {
                        continue 'search;
                    }
                }
            }
            return poly;
        }
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
