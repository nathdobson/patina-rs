#![deny(unused_must_use)]
#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![feature(float_minimum_maximum)]
#![allow(unused_mut)]
#![allow(unreachable_code)]

mod marching;

pub mod sdf;
pub mod marching_mesh;
// pub mod sdf;
// pub mod geo;
#[cfg(test)]
mod test;
mod subdivide;
mod octree;
mod exact;
mod transvoxel;
