use crate::geo1::interval::Interval;
use crate::segment2::Segment;
use patina_vec::vec2::Vec2;
use std::cmp::Ordering;

pub type Segment2 = Segment<2>;

impl Segment2 {
    pub fn intersect_segment(&self, other: &Segment2) -> Option<(f64, f64, Vec2)> {
        let (t1, t2, p) = self.as_ray().intersect_line(&other.as_ray())?;
        (0.0 <= t1 && t1 <= 1.0 && 0.0 <= t2 && t2 <= 1.0).then_some((t1, t2, p))
    }
    pub fn intersects(&self, other: &Self) -> bool {
        let r1 = self.as_ray();
        let r2 = other.as_ray();
        let a11 = r1.above(other.p1());
        let a12 = r1.above(other.p2());
        let a21 = r2.above(self.p1());
        let a22 = r2.above(self.p2());
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
            let ix1 = Interval::new(self.p1().x(), self.p2().x());
            let iy1 = Interval::new(self.p1().y(), self.p2().y());
            let ix2 = Interval::new(other.p1().x(), other.p2().x());
            let iy2 = Interval::new(other.p1().y(), other.p2().y());
            if (ix1.length(), ix2.length()) < (iy1.length(), iy2.length()) {
                iy1.intersect(iy2).length() > 0.0
            } else {
                ix1.intersect(ix2).length() > 0.0
            }
        }
    }
}
