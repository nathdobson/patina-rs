use crate::geo1::interval::Interval;
use crate::geo2::ray2::Ray2;
use patina_vec::vec2::Vec2;
use std::cmp::Ordering;
use std::fmt::{Display, Formatter};

#[derive(Debug, Clone)]
pub struct Segment2 {
    p1: Vec2,
    p2: Vec2,
}

impl Segment2 {
    pub fn new(p1: Vec2, p2: Vec2) -> Self {
        Self { p1, p2 }
    }
    pub fn p1(&self) -> Vec2 {
        self.p1
    }
    pub fn p2(&self) -> Vec2 {
        self.p2
    }
    pub fn as_ray(&self) -> Ray2 {
        Ray2::new(self.p1, self.p2 - self.p1)
    }
    pub fn intersects(&self, other: &Self) -> bool {
        let r1 = self.as_ray();
        let r2 = other.as_ray();
        let a11 = r1.above(other.p1);
        let a12 = r1.above(other.p2);
        let a21 = r2.above(self.p1);
        let a22 = r2.above(self.p2);
        fn intersects_ord(x: Ordering, y: Ordering) -> Option<bool> {
            match (x, y) {
                (Ordering::Less, Ordering::Less) => Some(false),
                (Ordering::Greater, Ordering::Greater) => Some(false),
                (Ordering::Equal, Ordering::Equal) => None,
                _ => Some(true),
            }
        }
        if let (Some(i1), Some(i2)) = (intersects_ord(a11, a12), intersects_ord(a21, a22)) {
            i1 && i2
        } else {
            let ix1 = Interval::new(self.p1.x(), self.p2.x());
            let iy1 = Interval::new(self.p1.y(), self.p2.y());
            let ix2 = Interval::new(other.p1.x(), other.p2.x());
            let iy2 = Interval::new(other.p1.y(), other.p2.y());
            if (ix1.length(), ix2.length()) < (iy1.length(), iy2.length()) {
                iy1.intersect(iy2).length() > 0.0
            } else {
                ix1.intersect(ix2).length() > 0.0
            }
        }
    }
    pub fn distance(&self, other: Vec2) -> f64 {
        let u = self.p2 - self.p1;
        let v = other - self.p1;
        let proj_fract = u.dot(v) / (u.dot(u));
        if proj_fract <= 0.0 {
            other.distance(self.p1)
        } else if proj_fract >= 1.0 {
            other.distance(self.p2)
        } else {
            other.distance(u * proj_fract)
        }
    }
}

impl Display for Segment2 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "\n{}\n{}\n", self.p1, self.p2)
    }
}
