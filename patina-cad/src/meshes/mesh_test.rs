use patina_vec::vec3::Vec3;
use crate::meshes::bimesh::Bimesh;
use crate::meshes::mesh::Mesh;
use crate::meshes::mesh_triangle::MeshTriangle;
use crate::ser::stl::write_stl_file;
use rand::seq::SliceRandom;
use rand::{Rng, SeedableRng};
use rand_xorshift::XorShiftRng;
use std::path::PathBuf;

