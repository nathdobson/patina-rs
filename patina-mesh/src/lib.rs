#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unused_mut)]
#![allow(unused_imports)]
#![deny(unused_must_use)]
#![feature(try_blocks)]
#![feature(debug_closure_helpers)]
#![feature(float_minimum_maximum)]

pub mod directed_mesh_edge;
pub mod mesh;
pub mod mesh_edge;
pub mod mesh_geo;
pub mod mesh_triangle;
pub mod ser;
pub mod edge_table;
pub mod triangulation;
mod trap_decomp;
pub mod edge_mesh2;
mod monotone_decomp;
pub mod tri_mesh2;
pub mod bimesh2;
pub mod mesh_cut;
mod bvh;
mod util;
pub mod half_edge_mesh;
pub mod decimate;

