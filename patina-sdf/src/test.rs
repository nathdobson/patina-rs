use crate::marching_mesh::MarchingMesh;
use crate::sdf::Sdf;
use patina_geo::geo3::plane::Plane;
use patina_geo::geo3::sphere::Sphere;
use patina_mesh::ser::stl::write_stl_file;
use patina_vec::vec3::Vec3;
use target_test_dir::with_test_dir;

#[tokio::test]
#[with_test_dir]
async fn test() -> anyhow::Result<()> {
    let test_dir = get_test_dir!();
    //let sdf = Sdf::new_sphere(&Sphere::new(Vec3::new(0.0, 0.0, 0.0), 5.2)).compile();
    let sdf = Sdf::new_plane(&Plane::new(
        Vec3::new(0.0, 0.0, 0.0),
        Vec3::new(1.0, 1.0, 1.0),
    ))
    .compile();
    let naive = MarchingMesh::new(
        Vec3::new(-3.0, -3.0, -3.0),
        Vec3::new(1.0, 1.0, 1.0),
        [6, 6, 6],
    );
    let mesh = naive.build(&sdf);
    write_stl_file(&mesh, &test_dir.join("mesh.stl")).await?;
    Ok(())
}
