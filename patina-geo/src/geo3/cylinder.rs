use patina_vec::vec3::Vec3;

#[derive(Debug)]
pub struct Cylinder {
    origin: Vec3,
    axis: Vec3,
    radius: f64,
}

impl Cylinder {
    pub fn new(origin: Vec3, axis: Vec3, radius: f64) -> Self {
        Cylinder {
            origin,
            axis,
            radius,
        }
    }
    pub fn origin(&self) -> Vec3 {
        self.origin
    }
    pub fn axis(&self) -> Vec3 {
        self.axis
    }
    pub fn radius(&self) -> f64 {
        self.radius
    }


}
