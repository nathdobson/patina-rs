#![deny(unused_must_use)]
#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![feature(float_minimum_maximum)]

mod marching;

pub mod sdf;
mod deriv;
mod marching_mesh;
mod solver;
// pub mod sdf;
// pub mod geo;
#[cfg(test)]
mod test;
