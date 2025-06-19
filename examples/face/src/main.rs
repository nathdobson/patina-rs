#![deny(unconditional_recursion, unused_must_use)]
#![allow(unused_imports, unused_mut)]

use glam::DVec3;
use patina_cad::geo3::cube::cube;
use patina_cad::geo3::cylinder::cylinder;
use patina_cad::geo3::sphere::{icosahedron, icosphere, spherify};
use patina_cad::stl::write_stl_file;
use patina_cad::subdivision::subdivide;
use std::path::PathBuf;
use tokio::io;

#[tokio::main]
async fn main() -> io::Result<()> {
    let mesh = icosphere(4);
    mesh.check_manifold().unwrap();
    write_stl_file(&mesh, PathBuf::from("examples/face/output.stl").as_path()).await?;
    Ok(())
}
