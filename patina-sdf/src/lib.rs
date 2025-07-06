#![deny(unused_must_use)]
#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![feature(float_minimum_maximum)]

mod marching;

pub mod sdf;
mod marching_mesh;
// pub mod sdf;
// pub mod geo;
#[cfg(test)]
mod test;
