#![deny(unused_must_use)]
#![allow(unused_variables)]
#![allow(dead_code)]
pub mod vec3;
pub mod vec2;
pub mod mat2;
pub mod mat4;
pub mod vec4;
mod impl_ref_binop;

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
