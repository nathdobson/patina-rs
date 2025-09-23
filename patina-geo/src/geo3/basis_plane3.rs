use patina_vec::vec2::Vec2;
use patina_vec::vec3::Vec3;

pub struct BasisPlane3 {
    origin: Vec3,
    axis1: Vec3,
    axis2: Vec3,
}

impl BasisPlane3 {
    pub fn new(origin: Vec3, axis1: Vec3, axis2: Vec3) -> Self {
        BasisPlane3 {
            origin,
            axis1,
            axis2,
        }
    }
    pub fn origin(&self) -> Vec3 {
        self.origin
    }
    pub fn axis1(&self) -> Vec3 {
        self.axis1
    }
    pub fn axis2(&self) -> Vec3 {
        self.axis2
    }
    pub fn project(&self, point: Vec3) -> Vec2 {
        let point = point - self.origin;
        Vec2::new(self.axis1.dot(point), self.axis2.dot(point))
    }
}
