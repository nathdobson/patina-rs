use patina_vec::vec::Vector;
use patina_vec::vec3::Vec3;

#[derive(Debug, Clone)]
pub struct NSphere<const N: usize> {
    origin: Vector<f64, N>,
    radius: f64,
}

pub type Sphere = NSphere<3>;
pub type Circle = NSphere<2>;

impl<const N: usize> NSphere<N> {
    pub fn new(origin: Vector<f64, N>, radius: f64) -> NSphere<N> {
        NSphere { origin, radius }
    }
    pub fn origin(&self) -> Vector<f64, N> {
        self.origin
    }
    pub fn radius(&self) -> f64 {
        self.radius
    }
}
