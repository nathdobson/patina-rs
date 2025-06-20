#![deny(unconditional_recursion, unused_must_use)]
#![allow(unused_imports, unused_mut)]
#![allow(unused_variables)]

use glam::DVec3;
use patina_cad::geo3::cylinder::Cylinder;
use patina_cad::geo3::sphere::Sphere;
use patina_cad::math::vec3::Vec3;
use patina_cad::meshes::bimesh::Bimesh;
use patina_cad::meshes::subdivision::subdivide;
use patina_cad::ser::stl::write_stl_file;
use rand::SeedableRng;
use rand_xorshift::XorShiftRng;
use std::path::PathBuf;
use tokio::io;

#[tokio::main]
async fn main() -> io::Result<()> {
    // let mut face = Cylinder::new(Vec3::zero(), Vec3::new(0.0, 0.0, 0.1), 10.0).as_mesh(7);
    // let eye = Cylinder::new(Vec3::new(3.0, 3.0, -0.1), Vec3::new(0.0, 0.0, 0.3), 1.0).as_mesh(3);
    let mut face = Sphere::new(Vec3::zero(), 10.0).as_mesh(2);
    let mut eye = Sphere::new(Vec3::new(9.0, 1.0, 1.0), 3.0).as_mesh(2);
    let mut rng = XorShiftRng::seed_from_u64(123);
    face.perturb(&mut rng, 0.001);
    eye.perturb(&mut rng, 0.001);
    face.check_manifold().unwrap();
    eye.check_manifold().unwrap();
    let bimesh = Bimesh::new(&face, &eye);
    let dir = PathBuf::from("examples").join("face").join("output");
    tokio::fs::create_dir_all(&dir).await.ok();
    write_stl_file(&bimesh.union(), &dir.join("union.stl")).await?;
    write_stl_file(&bimesh.intersect(), &dir.join("intersect.stl")).await?;
    write_stl_file(&bimesh.forward_difference(), &dir.join("forward_difference.stl")).await?;
    write_stl_file(&bimesh.reverse_difference(), &dir.join("reverse_difference.stl")).await?;
    // write_stl_file(&mesh, PathBuf::from("examples/face/output.stl").as_path()).await?;
    Ok(())
}
