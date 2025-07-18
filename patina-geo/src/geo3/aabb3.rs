use crate::aabb::Aabb;
use crate::geo1::interval::Interval;
use patina_vec::vec3::Vec3;

pub type Aabb3 = Aabb<3>;

impl Aabb<3> {
    pub fn surface_area(&self) -> f64 {
        let d = self.max() - self.min();
        let d = d.maximum(Vec3::splat(0.0));
        d.x() * d.y() + d.x() * d.z() + d.y() * d.z()
    }
    pub fn vertices(&self) -> [Vec3; 8] {
        let min = self.min();
        let max = self.max();
        [
            Vec3::new(min.x(), min.y(), min.z()),
            Vec3::new(min.x(), min.y(), max.z()),
            Vec3::new(min.x(), max.y(), min.z()),
            Vec3::new(min.x(), max.y(), max.z()),
            Vec3::new(max.x(), min.y(), min.z()),
            Vec3::new(max.x(), min.y(), max.z()),
            Vec3::new(max.x(), max.y(), min.z()),
            Vec3::new(max.x(), max.y(), max.z()),
        ]
    }
    pub fn octants(&self) -> [Self; 8] {
        let [x1, y1, z1] = self.min().into();
        let [x2, y2, z2] = self.center().into();
        let [x3, y3, z3] = self.max().into();
        let x12 = Interval::new(x1, x2);
        let x23 = Interval::new(x2, x3);
        let y12 = Interval::new(y1, y2);
        let y23 = Interval::new(y2, y3);
        let z12 = Interval::new(z1, z2);
        let z23 = Interval::new(z2, z3);
        [
            Aabb::from_intervals([x12, y12, z12]),
            Aabb::from_intervals([x12, y12, z23]),
            Aabb::from_intervals([x12, y23, z12]),
            Aabb::from_intervals([x12, y23, z23]),
            Aabb::from_intervals([x23, y12, z12]),
            Aabb::from_intervals([x23, y12, z23]),
            Aabb::from_intervals([x23, y23, z12]),
            Aabb::from_intervals([x23, y23, z23]),
        ]
    }
}
