use patina_vec::vec3::Vec3;

#[derive(Debug)]
pub struct Ray3 {
    origin: Vec3,
    dir: Vec3,
}

impl Ray3 {
    pub fn new(start: Vec3, dir: Vec3) -> Self {
        Ray3 { origin: start, dir }
    }
    pub fn origin(&self) -> Vec3 {
        self.origin
    }
    pub fn dir(&self) -> Vec3 {
        self.dir
    }
    pub fn project(&self, p: Vec3) -> f64 {
        (p - self.origin).dot(self.dir)
    }
    pub fn at_time(&self, t: f64) -> Vec3 {
        self.origin + self.dir * t
    }
}
