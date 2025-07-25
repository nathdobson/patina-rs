use crate::geo3::basis_plane3::BasisPlane3;
use crate::geo3::plane::Plane;
use crate::geo3::segment3::Segment3;
use patina_vec::vec3::Vec3;
use std::fmt::{Debug, Display, Formatter};

#[derive(Copy, Clone)]
pub struct Triangle3([Vec3; 3]);

impl Triangle3 {
    pub fn new(points: [Vec3; 3]) -> Triangle3 {
        Triangle3(points)
    }
    pub fn points(&self) -> &[Vec3; 3] {
        &self.0
    }
    pub fn p1(&self) -> Vec3 {
        self.0[0]
    }
    pub fn p2(&self) -> Vec3 {
        self.0[1]
    }
    pub fn p3(&self) -> Vec3 {
        self.0[2]
    }
    pub fn normal(&self) -> Vec3 {
        (self.0[1] - self.0[0])
            .cross(self.0[2] - self.0[0])
            .normalize()
    }
    pub fn plane(&self) -> Plane {
        Plane::new(self.0[0], self.normal())
    }
    pub fn basis_plane(&self) -> BasisPlane3 {
        let origin = self.p1();
        let axis1 = (self.p2() - origin).normalize();
        let axis2_init = self.p3() - origin;
        let normal = axis1.cross(axis2_init);
        let axis2 = normal.cross(axis1).normalize();
        BasisPlane3::new(origin, axis1, axis2)
    }
    pub fn midpoint(&self) -> Vec3 {
        self.points().iter().sum::<Vec3>() / 3.0
    }
    pub fn edges(&self) -> [Segment3; 3] {
        [
            Segment3::new(self.points()[0], self.points()[1]),
            Segment3::new(self.points()[1], self.points()[2]),
            Segment3::new(self.points()[2], self.points()[0]),
        ]
    }
    pub fn area_vector(&self) -> Vec3 {
        (self.points()[1] - self.points()[0]).cross(self.points()[2] - self.points()[0])
    }
    pub fn area(&self) -> f64 {
        (self.points()[1] - self.points()[0])
            .cross(self.points()[2] - self.points()[0])
            .length()
    }
}

impl Debug for Triangle3 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

impl Display for Triangle3 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}\n{}\n{}\n",
            self.points()[0],
            self.points()[1],
            self.points()[2]
        )
    }
}
