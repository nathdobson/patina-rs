use crate::geo2::segment2::Segment2;
use crate::ray::Ray;
use patina_vec::mat2::Matrix2;
use patina_vec::vec::Vector;
use patina_vec::vec2::Vec2;
use std::cmp::Ordering;

pub type Ray2 = Ray<2>;

impl Ray2 {
    pub fn above(&self, v: Vec2) -> Ordering {
        (v - self.origin()).cross(self.dir()).total_cmp(&0.0)
    }
    pub fn intersect_line(&self, other: &Self) -> Option<(f64, f64, Vec2)> {
        let mat2 = Matrix2::from_cols([self.dir(), -other.dir()]);
        let [t1, t2] = (mat2.invert2() * (other.origin() - self.origin())).into();
        if !t1.is_finite() || !t2.is_finite() {
            return None;
        }
        Some((t1, t2, self.at_time(t1)))
    }
    pub fn intersect_segment(&self, segment: &Segment2) -> Option<(f64, f64, Vec2)> {
        let (t1, t2, p) = self.intersect_line(&segment.as_ray())?;
        (t1 >= 0.0 && t2 >= 0.0 && t2 <= 1.0).then_some((t1, t2, p))
    }
}
