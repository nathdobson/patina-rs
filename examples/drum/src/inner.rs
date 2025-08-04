#![deny(unused_must_use)]
#![allow(unused_mut)]
#![allow(dead_code)]

mod outer;

use patina_geo::aabb::Aabb;
use patina_geo::geo3::cylinder::Cylinder;
use patina_mesh::mesh::Mesh;
use patina_mesh::ser::encode_file;
use patina_sdf::marching_mesh::MarchingMesh;
use patina_sdf::sdf::truncated_cone::TruncatedCone;
use patina_sdf::sdf::{AsSdf, Sdf3};
use patina_threads::{THREAD_M2, ThreadMetrics};
use patina_vec::vec2::Vec2;
use patina_vec::vec3::Vec3;
use std::f64;
use std::path::Path;
use std::time::Instant;

pub struct DrumBuilder {
    eps: f64,
    drum_outer_radius: f64,
    drum_inner_radius: f64,
    flange_radius: f64,
    drum_height: f64,
    flange_height: f64,
    post_height: f64,
    post_rad1: f64,
    post_rad2: f64,
    strut_radius: f64,
    strut_thickness: f64,
    screw_off_x: f64,
    screw_off_y: f64,
    screw_hole_height: f64,
    screw_hole_radius: f64,
    screw_threads: &'static ThreadMetrics,
    axle_round_length: f64,
    axle_radius: f64,
    axle_length: f64,
    axle_flat_width: f64,
    letter_count: usize,
    flap_hole_radius: f64,
    flap_pos_radius: f64,
    magnet_ring_inner_radius: f64,
    magnet_ring_outer_radius: f64,
    magnet_depth: f64,
    magnet_radius: f64,
    magnet_height: f64,
}

impl DrumBuilder {
    pub fn screw(&self, x: f64, y: f64, mut sdf: Sdf3) -> Sdf3 {
        sdf = sdf.union(
            &Cylinder::new(
                Vec3::new(x * 0.96, y, self.flange_height),
                Vec3::axis_z() * self.screw_hole_height,
                self.screw_hole_radius,
            )
            .as_sdf(),
        );
        sdf = sdf.difference(
            &Cylinder::new(
                Vec3::new(x, y, 0.0),
                Vec3::axis_z() * self.post_height,
                self.screw_threads.through_radius,
            )
            .as_sdf(),
        );
        sdf = sdf.difference(
            &Cylinder::new(
                Vec3::new(x, y, 0.0),
                Vec3::axis_z() * self.screw_threads.countersink_depth,
                self.screw_threads.countersink_radius,
            )
            .as_sdf(),
        );
        sdf
    }

    pub fn build_sdf(&self) -> Sdf3 {
        let mut sdf = Cylinder::new(
            Vec3::zero(),
            Vec3::axis_z() * self.drum_height,
            self.drum_outer_radius,
        )
        .as_sdf();
        sdf = sdf.union(
            &Cylinder::new(
                Vec3::zero(),
                Vec3::axis_z() * self.flange_height,
                self.flange_radius,
            )
            .as_sdf(),
        );
        sdf = sdf.difference(
            &Cylinder::new(
                Vec3::axis_z() * self.flange_height,
                Vec3::axis_z() * self.drum_height,
                self.drum_inner_radius,
            )
            .as_sdf(),
        );
        sdf = sdf.union(
            &TruncatedCone::new(
                Vec3::axis_z() * self.flange_height,
                Vec3::axis_z() * self.post_height,
                self.post_rad1,
                self.post_rad2,
            )
            .as_sdf(),
        );
        sdf = sdf.union(
            &Aabb::new(
                Vec3::new(
                    -self.strut_radius,
                    -self.strut_thickness / 2.0,
                    self.flange_height,
                ),
                Vec3::new(
                    self.strut_radius,
                    self.strut_thickness / 2.0,
                    self.magnet_height,
                ),
            )
            .as_sdf(),
        );
        sdf = sdf.union(
            &Aabb::new(
                Vec3::new(
                    -self.strut_thickness / 2.0,
                    -self.strut_radius,
                    self.flange_height,
                ),
                Vec3::new(
                    self.strut_thickness / 2.0,
                    self.strut_radius,
                    self.magnet_height,
                ),
            )
            .as_sdf(),
        );
        sdf = self.screw(-self.screw_off_x, self.screw_off_y, sdf);
        sdf = self.screw(self.screw_off_x, self.screw_off_y, sdf);
        sdf = sdf.difference(
            &Cylinder::new(
                Vec3::new(0.0, 0.0, self.flange_height + self.post_height),
                -Vec3::axis_z() * self.axle_length,
                self.axle_radius,
            )
            .as_sdf()
            .difference(
                &Aabb::new(
                    Vec3::new(
                        -self.axle_radius,
                        self.axle_flat_width / 2.0,
                        self.flange_height + self.post_height - self.axle_length,
                    ),
                    Vec3::new(
                        self.axle_radius,
                        1000.0,
                        self.flange_height + self.post_height - self.axle_round_length,
                    ),
                )
                .as_sdf(),
            )
            .difference(
                &Aabb::new(
                    Vec3::new(
                        -self.axle_radius,
                        -1000.0,
                        self.flange_height + self.post_height - self.axle_length,
                    ),
                    Vec3::new(
                        self.axle_radius,
                        -self.axle_flat_width / 2.0,
                        self.flange_height + self.post_height - self.axle_round_length,
                    ),
                )
                .as_sdf(),
            ),
        );
        for i in 0..self.letter_count {
            let pos =
                Vec2::from_rad(2.0 * f64::consts::PI * (i as f64) / (self.letter_count as f64))
                    * self.flap_pos_radius;
            sdf = sdf.difference(
                &Cylinder::new(
                    Vec3::new(pos.x(), pos.y(), 0.0),
                    Vec3::axis_z() * self.flange_height,
                    self.flap_hole_radius,
                )
                .as_sdf(),
            )
        }
        sdf = sdf.union(
            &Cylinder::new(
                Vec3::zero(),
                Vec3::axis_z() * self.magnet_height,
                self.magnet_ring_outer_radius,
            )
            .as_sdf()
            .difference(
                &Cylinder::new(
                    Vec3::zero(),
                    Vec3::axis_z() * self.magnet_height,
                    self.magnet_ring_inner_radius,
                )
                .as_sdf(),
            ),
        );
        sdf = sdf.difference(
            &Cylinder::new(
                Vec3::new(
                    (self.magnet_ring_inner_radius + self.magnet_ring_outer_radius) / 2.0,
                    0.0,
                    self.magnet_height,
                ),
                -Vec3::axis_z() * self.magnet_depth,
                self.magnet_radius,
            )
            .as_sdf(),
        );
        sdf
    }
    pub fn build(&self) -> Mesh {
        let sdf = self.build_sdf();
        let mut marching = MarchingMesh::new(Aabb::new(
            Vec3::new(
                -self.flange_radius - self.eps,
                -self.flange_radius - self.eps,
                -self.eps,
            ),
            Vec3::new(
                self.flange_radius + self.eps,
                self.flange_radius + self.eps,
                self.post_height + self.flange_height + self.eps,
            ),
        ));
        marching
            // .min_render_depth(6)
            // .max_render_depth(7)
            // .subdiv_max_dot(0.9);
            .min_render_depth(7)
            .max_render_depth(10)
            .subdiv_max_dot(0.999);
        let mesh = marching.build(&sdf);
        mesh
    }
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let start = Instant::now();
    let mesh = DrumBuilder {
        eps: 1.0,
        drum_outer_radius: 55.0 / 2.0,
        drum_inner_radius: 53.0 / 2.0,
        flange_radius: 69.0 / 2.0,
        drum_height: 8.0,
        flange_height: 1.6,
        post_height: 17.5,
        post_rad1: 16.5 / 2.0,
        post_rad2: 10.0 / 2.0,
        strut_radius: 54.8 / 2.0,
        strut_thickness: 1.0,
        screw_off_x: -25.0,
        screw_off_y: -4.0,
        screw_hole_height: 6.0,
        screw_hole_radius: 3.0,
        screw_threads: &THREAD_M2,
        axle_round_length: 1.6,
        axle_length: 8.3,
        axle_radius: 5.3 / 2.0,
        axle_flat_width: 3.3,
        letter_count: 45,
        flap_hole_radius: 2.0 / 2.0,
        flap_pos_radius: 64.0 / 2.0,
        magnet_ring_inner_radius: 19.0,
        magnet_ring_outer_radius: 22.0,
        magnet_depth: 1.0,
        magnet_radius: 1.1,
        magnet_height: 16.4,
    }
    .build();
    println!("{:?}", mesh.check_manifold());
    println!("Built mesh in {:?}", start.elapsed());
    encode_file(&mesh, Path::new("examples/drum/output/inner.stl")).await?;
    Ok(())
}
