#![deny(unused_must_use)]
#![allow(unused_variables)]
#![allow(dead_code)]
// pub mod vec3;
mod impl_ref_binop;
pub mod mat2;
pub mod mat4;
pub mod vec2;
pub mod vec4;
pub mod vector3;

pub mod vec3 {
    use crate::vector3::Vector3;

    pub type Vec3 = Vector3<f64>;
}
