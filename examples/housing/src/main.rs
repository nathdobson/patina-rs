#![allow(dead_code)]
#![allow(unused_imports)]
#![deny(unused_must_use)]

use anyhow::Context;
use patina_geo::aabb::Aabb;
use patina_geo::geo2::polygon2::Polygon2;
use patina_geo::geo3::aabb3::Aabb3;
use patina_geo::geo3::cylinder::Cylinder;
use patina_geo::sphere::Circle;
use patina_mesh::decimate::Decimate;
use patina_mesh::half_edge_mesh::HalfEdgeMesh;
use patina_mesh::mesh::Mesh;
use patina_mesh::ser::encode_file;
use patina_sdf::marching_mesh::MarchingMesh;
use patina_sdf::sdf::leaf::SdfLeafImpl;
use patina_sdf::sdf::truncated_cone::TruncatedCone;
use patina_sdf::sdf::{AsSdf, Sdf, Sdf3};
use patina_vec::vec2::Vec2;
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
        let mut sdf = self.aabb.as_sdf().difference(
            &Cylinder::new(
                Vec3::new(0.0, 0.0, self.back_thickness),
                Vec3::axis_z() * 1000.0,
                self.drum_bounding_radius,
            )
            .as_sdf(),
        );
        sdf = sdf.difference(
            &Aabb::new(
                Vec3::new(-1000.0, -68.0, self.back_thickness),
                Vec3::new(-1.0, -40.0, 1000.0),
            )
            .as_sdf(),
        );
        let mount_off_x = 8.0;
        let mount_off_y = 17.5;
        let mount_length = 18.0;
        let motor_radius = 14.0;
        let soft_fit = 0.2;
        let mount_rad1 = 6.0;
        let mount_rad2 = 4.9;
        let pilot_depth_m4 = 8.1;
        let pilot_radius_m4 = 5.6 / 2.0;
        sdf = sdf.union(
            &TruncatedCone::new(
                Vec3::new(mount_off_x, mount_off_y, self.back_thickness),
                Vec3::new(0.0, 0.0, mount_length),
                mount_rad1,
                mount_rad2,
            )
            .as_sdf(),
        );
        sdf = sdf.union(
            &TruncatedCone::new(
                Vec3::new(mount_off_x, -mount_off_y, self.back_thickness),
                Vec3::new(0.0, 0.0, mount_length),
                mount_rad1,
                mount_rad2,
            )
            .as_sdf(),
        );
        sdf = sdf.difference(
            &Cylinder::new(
                Vec3::new(0.0, 0.0, self.back_thickness),
                Vec3::new(0.0, 0.0, mount_length * 2.0),
                motor_radius + soft_fit,
            )
            .as_sdf(),
        );
        sdf = sdf.difference(
            &Cylinder::new(
                Vec3::new(
                    mount_off_x,
                    mount_off_y,
                    self.back_thickness + mount_length - pilot_depth_m4,
                ),
                Vec3::new(0.0, 0.0, pilot_depth_m4),
                pilot_radius_m4,
            )
            .as_sdf(),
        );
        sdf = sdf.difference(
            &Cylinder::new(
                Vec3::new(
                    mount_off_x,
                    -mount_off_y,
                    self.back_thickness + mount_length - pilot_depth_m4,
                ),
                Vec3::new(0.0, 0.0, pilot_depth_m4),
                pilot_radius_m4,
            )
            .as_sdf(),
        );
        let brace_width = 2.0;
        let brace_extent = 6.0;
        let brace_indent = 0.2;
        sdf = sdf.union(
            &Polygon2::new(vec![
                Vec2::new(mount_off_y + mount_rad2 - brace_indent, self.back_thickness),
                Vec2::new(
                    mount_off_y + mount_rad2 - brace_indent + brace_extent,
                    self.back_thickness,
                ),
                Vec2::new(
                    mount_off_y + mount_rad2 - brace_indent,
                    self.back_thickness + mount_length,
                ),
            ])
            .as_sdf()
            .extrude_x(mount_off_x - brace_width / 2.0..mount_off_x + brace_width / 2.0),
        );
        sdf = sdf.union(
            &Polygon2::new(vec![
                Vec2::new(
                    -mount_off_y - mount_rad2 + brace_indent,
                    self.back_thickness,
                ),
                Vec2::new(
                    -mount_off_y - mount_rad2 + brace_indent - brace_extent,
                    self.back_thickness,
                ),
                Vec2::new(
                    -mount_off_y - mount_rad2 + brace_indent,
                    self.back_thickness + mount_length,
                ),
            ])
            .as_sdf()
            .extrude_x(mount_off_x - brace_width / 2.0..mount_off_x + brace_width / 2.0),
        );
        sdf = sdf.union(
            &Polygon2::new(vec![
                Vec2::new(self.back_thickness, mount_off_x + mount_rad2 - brace_indent),
                Vec2::new(
                    self.back_thickness,
                    mount_off_x + mount_rad2 - brace_indent + brace_extent,
                ),
                Vec2::new(
                    self.back_thickness + mount_length,
                    mount_off_x + mount_rad2 - brace_indent,
                ),
            ])
            .as_sdf()
            .extrude_y(-mount_off_y - brace_width / 2.0..-mount_off_y + brace_width / 2.0),
        );
        sdf = sdf.union(
            &Polygon2::new(vec![
                Vec2::new(self.back_thickness, mount_off_x - mount_rad2 + brace_indent),
                Vec2::new(
                    self.back_thickness,
                    mount_off_x - mount_rad2 + brace_indent - brace_extent,
                ),
                Vec2::new(
                    self.back_thickness + mount_length,
                    mount_off_x - mount_rad2 + brace_indent,
                ),
            ])
            .as_sdf()
            .extrude_y(-mount_off_y - brace_width / 2.0..-mount_off_y + brace_width / 2.0),
        );
        sdf = sdf.union(
            &Polygon2::new(vec![
                Vec2::new(self.back_thickness, mount_off_x + mount_rad2 - brace_indent),
                Vec2::new(
                    self.back_thickness,
                    mount_off_x + mount_rad2 - brace_indent + brace_extent,
                ),
                Vec2::new(
                    self.back_thickness + mount_length,
                    mount_off_x + mount_rad2 - brace_indent,
                ),
            ])
            .as_sdf()
            .extrude_y(mount_off_y - brace_width / 2.0..mount_off_y + brace_width / 2.0),
        );
        sdf = sdf.union(
            &Polygon2::new(vec![
                Vec2::new(self.back_thickness, mount_off_x - mount_rad2 + brace_indent),
                Vec2::new(
                    self.back_thickness,
                    mount_off_x - mount_rad2 + brace_indent - brace_extent,
                ),
                Vec2::new(
                    self.back_thickness + mount_length,
                    mount_off_x - mount_rad2 + brace_indent,
                ),
            ])
            .as_sdf()
            .extrude_y(mount_off_y - brace_width / 2.0..mount_off_y + brace_width / 2.0),
        );
        sdf
    }
    pub fn build(&self) -> Mesh {
        let sdf = self.build_sdf();
        let mut marching = MarchingMesh::new(Aabb::new(
            self.aabb.min() + Vec3::splat(-0.1),
            self.aabb.max() + Vec3::splat(0.1),
        ));
        marching
            // .min_render_depth(6)
            // .max_render_depth(7)
            // .subdiv_max_dot(0.9);
        .min_render_depth(6)
        .max_render_depth(10)
        .subdiv_max_dot(0.999);
        let mesh = marching.build(&sdf);
        // let mut hem = HalfEdgeMesh::new(&mesh);
        // let mut decimate = Decimate::new(&mut hem);
        // decimate.max_degree(13);
        // decimate.min_score(0.9999);
        // decimate.run_arbitrary();
        // let mesh = hem.as_mesh();
        mesh
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
    println!("{:?}", mesh.check_manifold());
    println!("Built mesh in {:?}", start.elapsed());
    encode_file(&mesh, Path::new("examples/housing/output/sideA.stl")).await?;
    Ok(())
}
