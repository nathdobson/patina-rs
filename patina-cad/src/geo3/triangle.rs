use crate::geo3::plane::Plane;
use crate::geo3::ray3::Ray3;
use crate::geo3::segment3::Segment3;
use crate::math::interval::Interval;
use crate::math::vec2::Vec2;
use crate::math::vec3::Vec3;
use crate::sat::ConvexPoly;
use std::fmt::{Debug, Formatter};

#[derive(Copy, Clone)]
pub struct Triangle([Vec3; 3]);

impl Triangle {
    pub fn new(points: [Vec3; 3]) -> Triangle {
        Triangle(points)
    }
    pub fn points(&self) -> &[Vec3; 3] {
        &self.0
    }
    pub fn normal(&self) -> Vec3 {
        (self.0[1] - self.0[0])
            .cross(self.0[2] - self.0[0])
            .normalize()
    }
    pub fn plane(&self) -> Plane {
        Plane::new(self.0[0], self.normal())
    }
    pub fn intersect_ray(&self, ray: &Ray3) -> Option<f64> {
        let plane = self.plane();
        let time = plane.intersect_ray(ray)?;
        let pos = ray.at_time(time);
        for i in 0..3 {
            let v1 = self.0[i];
            let v2 = self.0[(i + 1) % 3];
            let edge = v2 - v1;
            let disp = pos - v1;
            if edge.cross(disp).dot(plane.normal()) < 0.0 {
                return None;
            }
        }
        Some(time)
    }
    pub fn intersect_segment(&self, segment: &Segment3) -> Option<f64> {
        let t = self.intersect_ray(&segment.as_ray())?;
        (t < 1.0).then_some(t)
    }
    pub fn project(&self, p: Vec3) -> Vec2 {
        let p = p - self.0[0];
        let x = self.points()[1] - self.points()[0];
        let y0 = self.points()[2] - self.points()[0];
        let z = x.cross(y0);
        let y = z.cross(x);
        Vec2::new(p.dot(x.normalize()), p.dot(y.normalize()))
    }
    pub fn midpoint(&self) -> Vec3 {
        self.points().iter().sum::<Vec3>() / 3.0
    }
}

impl ConvexPoly for Triangle {
    fn normals(&self) -> impl AsRef<[Vec3]> {
        [self.normal()]
    }

    fn project_onto(&self, vector: Vec3) -> Interval {
        let mut interval = Interval::empty();
        for p in self.points() {
            interval = interval.union(p.dot(vector).into());
        }
        interval
    }
}

impl Debug for Triangle {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

#[test]
fn test_triangle_segment_intersect() {
    let tri = Triangle::new([
        Vec3::new(1.0, 0.0, 0.0),
        Vec3::new(0.0, 1.0, 0.0),
        Vec3::new(0.0, 0.0, 1.0),
    ]);
    let seg = Segment3::new(Vec3::new(0.0, 0.0, 0.0), Vec3::new(1.0, 1.0, 1.0));
    assert!((tri.intersect_segment(&seg).unwrap() - 1.0 / 3.0).abs() < 1e-5);
}
