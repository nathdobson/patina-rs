use crate::mat::Matrix;
use crate::vec3::Vec3;
use crate::vec4::Vec4;

pub type Matrix4<T> = Matrix<T, 4>;
pub type Mat4 = Matrix4<f64>;

impl Matrix4<f64> {
    pub fn as_affine(&self) -> Option<[f64; 12]> {
        if self.row(3) != Vec4::axis_w() {
            return None;
        }
        let mut result = [0f64; 12];
        for r in 0..3 {
            for c in 0..4 {
                result[c * 3 + r] = self[(r, c)];
            }
        }
        Some(result)
    }
    pub fn translate(v: Vec3) -> Self {
        Self::from_cols([
            Vec4::axis_x(),
            Vec4::axis_y(),
            Vec4::axis_z(),
            Vec4::new(v.x(), v.y(), v.z(), 1.0),
        ])
    }
}
