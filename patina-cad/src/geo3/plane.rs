use crate::geo3::ray3::Ray3;
use crate::math::float_bool::{Epsilon, FloatBool};
use crate::math::vec3::Vec3;

#[derive(Debug)]
pub struct Plane {
    origin: Vec3,
    normal: Vec3,
}

impl Plane {
    pub fn intersect(&self, other: &Plane) -> Ray3 {
        todo!();
    }
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
    pub fn intersect_ray(&self, ray: &Ray3, eps: Epsilon) -> (FloatBool, f64) {
        let t = (self.origin - ray.origin()).dot(self.normal) / ray.dir().dot(self.normal);
        if !t.is_finite() {
            return (FloatBool::from(false), f64::NAN);
        }
        (eps.less(0.0, t), t)
    }
}
