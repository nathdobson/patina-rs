use crate::geo2::segment2::Segment2;
use crate::geo3::aabb::AABB;
use crate::math::interval::Interval;
use crate::math::mat2::Mat2;
use crate::math::vec2::Vec2;
use crate::math::vec3::Vec3;

#[derive(Debug)]
pub struct Ray2 {
    origin: Vec2,
    dir: Vec2,
}

impl Ray2 {
    pub fn new(start: Vec2, dir: Vec2) -> Self {
        Ray2 { origin: start, dir }
    }
    pub fn origin(&self) -> Vec2 {
        self.origin
    }
    pub fn dir(&self) -> Vec2 {
        self.dir
    }
    pub fn at_time(&self, t: f64) -> Vec2 {
        self.origin + self.dir * t
    }
    pub fn intersect_time(&self, r2: &Ray2) -> Option<Vec2> {
        let mat = Mat2::from_cols(-self.dir, r2.dir);
        let ts = mat.invert() * (self.origin - r2.origin);
        (ts.x() >= 0.0 && ts.y() >= 0.0).then_some(ts)
    }
    pub fn intersect_segment(&self, seg: &Segment2) -> bool {
        if let Some(v) = self.intersect_time(&seg.as_ray()) {
            v.y() <= 1.0
        } else {
            false
        }
    }
    pub fn is_left(&self, p: Vec2) -> bool {
        let d2 = p - self.origin;
        self.dir.cross(d2) >= 0.0
    }
}

#[test]
fn test_ray2_intersect_time() {
    let r1 = Ray2::new(Vec2::new(0.0, 1.0), Vec2::new(2.0, 1.0));
    let r2 = Ray2::new(Vec2::new(1.0, 0.0), Vec2::new(1.0, 3.0));
    let ts = r1.intersect_time(&r2).unwrap();
    assert!((r1.at_time(ts[0]) - r2.at_time(ts[1])).length() < 1e-5);
}
