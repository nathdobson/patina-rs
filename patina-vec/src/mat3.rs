use crate::mat::Matrix;
use crate::vec3::Vec3;

pub type Matrix3<T> = Matrix<T, 3>;
pub type Mat3 = Matrix3<f64>;

impl Mat3 {
    pub fn rotate(axis: Vec3, angle: f64) -> Self {
        let [x, y, z] = axis.into();
        let cos = angle.cos();
        let sin = angle.sin();
        Mat3::from_rows([
            Vec3::new(
                cos + x * x * (1.0 - cos),
                x * y * (1.0 - cos) - z * sin,
                x * z * (1.0 - cos) + y * sin,
            ),
            Vec3::new(
                y * x * (1.0 - cos) + z * sin,
                cos + y * y * (1.0 - cos),
                y * z * (1.0 - cos) - x * cos,
            ),
            Vec3::new(
                z * x * (1.0 - cos) - y * sin,
                z * y * (1.0 - cos) + x * sin,
                cos + z * z * (1.0 - cos),
            ),
        ])
    }
}
