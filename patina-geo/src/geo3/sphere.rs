use patina_vec::vec3::Vec3;

#[derive(Debug, Clone)]
pub struct Sphere {
    origin: Vec3,
    radius: f64,
}

impl Sphere {
    pub fn new(origin: Vec3, radius: f64) -> Sphere {
        Sphere { origin, radius }
    }
    pub fn origin(&self) -> Vec3 {
        self.origin
    }
    pub fn radius(&self) -> f64 {
        self.radius
    }
}
