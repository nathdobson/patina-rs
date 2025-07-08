use crate::marching_mesh::MarchingMesh;
use crate::sdf::Sdf;
use crate::sdf::leaf::SdfLeafImpl;
use crate::sdf::union::SdfUnion;
use crate::subdivide::Subdivide;
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
    let sphere1 = Sphere::new(Vec3::new(-0.25, 0.0, 0.0), 0.5).into_sdf();
    let sphere2 = Sphere::new(Vec3::new(0.25, 0.0, 0.0), 0.5).into_sdf();
    // let plane = Plane::new(Vec3::new(0.0, 0.0, 0.0), Vec3::new(1.0, 1.0, 1.0)).into_sdf();
    // let prism = Sdf::new_aabb(&Aabb::new(
    //     Vec3::new(-0.5, -0.5, -0.5),
    //     Vec3::new(0.5, 0.5, 0.5),
    // ));
    // let sdf = sphere.difference(&plane);
    // let sdf = plane;
    // let csdf = sdf.compile();
    let sdf = SdfUnion::new(sphere1, sphere2).into_sdf();
    let scene = Aabb::new(Vec3::new(-1.0001, -1.01, -1.1), Vec3::new(1.0, 1.0, 1.0));
    let march = MarchingMesh::new(scene);
    // let naive = MarchingMesh::new(
    //     scene.min(),
    //     scene.dimensions() / (detail as f64),
    //     [detail, detail, detail],
    // );
    let mut mesh = march.build(&sdf);
    write_stl_file(&mesh, &test_dir.join("mesh.stl")).await?;
    // for i in 0..2 {
    //     mesh = Subdivide::new().subdivide(&mesh, &sdf);
    //     write_stl_file(&mesh, &test_dir.join(format!("mesh{}.stl", i))).await?;
    // }
    Ok(())
}
