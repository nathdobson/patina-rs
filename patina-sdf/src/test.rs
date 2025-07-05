use crate::marching_mesh::MarchingMesh;
use crate::sdf::Sdf;
use patina_geo::geo3::aabb::Aabb;
use patina_geo::geo3::plane::Plane;
use patina_geo::geo3::sphere::Sphere;
use patina_mesh::ser::stl::write_stl_file;
use patina_vec::vec3::Vec3;
use target_test_dir::with_test_dir;

#[tokio::test]
#[with_test_dir]
async fn test_mesh() -> anyhow::Result<()> {
    let test_dir = get_test_dir!();
    let sphere = Sdf::new_sphere(&Sphere::new(Vec3::new(0.0, 0.0, 0.0), 2.0)).compile();
    let plane = Sdf::new_plane(&Plane::new(
        Vec3::new(0.0, 0.0, 0.0),
        Vec3::new(1.0, 1.0, 1.0),
    ))
    .compile();
    let prism = Sdf::new_aabb(&Aabb::new(
        Vec3::new(-1.0, -2.0, -3.0),
        Vec3::new(2.0, 1.0, 0.0),
    ))
    .compile();
    println!("{}", prism.program());
    let naive = MarchingMesh::new(
        Vec3::new(-3.0, -3.0, -3.0),
        Vec3::new(0.251, 0.251, 0.251),
        [30, 30, 30],
    );
    let mesh = naive.build(&prism);
    write_stl_file(&mesh, &test_dir.join("mesh.stl")).await?;
    Ok(())
}
