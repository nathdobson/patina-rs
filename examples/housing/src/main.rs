#![allow(dead_code)]
#![allow(unused_imports)]
#![deny(unused_must_use)]

use patina_geo::aabb::Aabb;
use patina_geo::geo3::aabb3::Aabb3;
use patina_geo::geo3::cylinder::Cylinder;
use patina_mesh::mesh::Mesh;
use patina_mesh::ser::encode_file;
use patina_sdf::marching_mesh::MarchingMesh;
use patina_sdf::sdf::leaf::SdfLeafImpl;
use patina_sdf::sdf::{AsSdf, Sdf, Sdf3};
use patina_vec::vec3::Vec3;
use std::path::Path;
use std::time::Instant;

struct HousingBuilder {
    drum_bounding_radius: f64,
    back_thickness: f64,
    aabb: Aabb3,
}

impl HousingBuilder {
    fn build_sdf(&self) -> Sdf3 {
        self.aabb
            .as_sdf()
            .difference(
                &Cylinder::new(
                    Vec3::new(0.0, 0.0, self.back_thickness),
                    Vec3::axis_z() * 1000.0,
                    self.drum_bounding_radius,
                )
                .as_sdf(),
            )
            .difference(
                &Aabb::new(
                    Vec3::new(-1000.0, -68.0, self.back_thickness),
                    Vec3::new(-1.0, -21.0, 1000.0),
                )
                .as_sdf(),
            )
    }
    pub fn build(&self) -> Mesh {
        let sdf = self.build_sdf();
        let mut marching = MarchingMesh::new(self.aabb);
        marching.min_render_depth(6).max_render_depth(8);
        marching.build(&sdf)
    }
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let start = Instant::now();
    let mesh = HousingBuilder {
        drum_bounding_radius: 56.0,
        back_thickness: 3.0,
        aabb: Aabb::new(Vec3::new(-35.0, -71.0, 0.0), Vec3::new(59.0, 59.0, 50.0)),
    }
    .build();
    println!("Built mesh in {:?}", start.elapsed());
    encode_file(&mesh, Path::new("examples/housing/output/sideA.stl")).await?;
    Ok(())
}
