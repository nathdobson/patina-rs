#![deny(unconditional_recursion, unused_must_use)]
#![allow(unused_imports, unused_mut)]
#![allow(unused_variables)]

use glam::DVec3;
use patina_cad::geo3::cylinder::Cylinder;
use patina_cad::geo3::sphere::Sphere;
use patina_cad::math::float_bool::Epsilon;
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
    let mut face = Sphere::new(Vec3::zero(), 10.0).as_mesh(3);
    let mut ear1 = Sphere::new(Vec3::new(10.0, 3.0, 3.0), 3.0).as_mesh(2);
    let mut ear2 = Sphere::new(Vec3::new(-10.0, 3.0, 3.0), 3.0).as_mesh(2);
    let mut eye1 = Cylinder::new(Vec3::new(3.0, 3.0, 8.0), Vec3::axis_z() * 2.0, 1.5).as_mesh(17);
    let mut eye2 = Cylinder::new(Vec3::new(-3.0, 3.0, 8.0), Vec3::axis_z() * 2.0, 1.5).as_mesh(17);
    let mut nose = Sphere::new(Vec3::new(0.0, 0.0, 9.0), 2.0).as_mesh(2);
    let mut rng = XorShiftRng::seed_from_u64(1);
    // eye1.perturb(&mut rng, 0.0001);
    // eye2.perturb(&mut rng, 0.0001);
    // ear1.perturb(&mut rng, 0.0001);
    // ear2.perturb(&mut rng, 0.0001);
    // face.perturb(&mut rng, 0.0001);
    // nose.perturb(&mut rng, 0.0001);
    eye1.check_manifold().unwrap();
    let eps = Epsilon::new(1e-10);
    let total = face
        .union(&ear1, eps)
        .union(&ear2, eps)
        .union(&eye1, eps)
        .union(&eye2, eps)
        .union(&nose, eps);
    let dir = PathBuf::from("examples").join("face").join("output");
    tokio::fs::create_dir_all(&dir).await.ok();
    write_stl_file(&total, &dir.join("face.stl")).await?;
    Ok(())
}
