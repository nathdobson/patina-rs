use crate::geo3::ray3::Ray3;
use crate::math::vec3::Vec3;

#[derive(Debug)]
pub struct Plane {
    origin: Vec3,
    normal: Vec3,
}

impl Plane {
    pub fn new(origin: Vec3, normal: Vec3) -> Self {
        Plane { origin, normal }
    }
    pub fn origin(&self) -> Vec3 {
        self.origin
    }
    pub fn normal(&self) -> Vec3 {
        self.normal
    }
    pub fn intersect_ray(&self, ray: &Ray3) -> Option<f64> {
        let t = (self.origin - ray.origin()).dot(self.normal) / ray.dir().dot(self.normal);
        if t.is_infinite() {
            return None;
        }
        let result = (t >= 0.0).then_some(t)?;
        assert!(
            result.is_finite(),
            "self={:?} ray={:?} time={:?}",
            self,
            ray,
            result
        );
        Some(result)
    }
}
