#![allow(dead_code)]
#![allow(unused_imports)]
#![deny(unused_must_use)]

use anyhow::Context;
use patina_geo::aabb::Aabb;
use patina_geo::geo2::polygon2::Polygon2;
use patina_geo::geo3::aabb3::Aabb3;
use patina_geo::geo3::cylinder::Cylinder;
use patina_geo::geo3::plane::Plane;
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

struct Tab {
    size: f64,
    thickness: f64,
    wall_size: f64,
    bottom_x: f64,
    top_x: f64,
    right_y: f64,
}

struct Catch {
    bottom_thickness: f64,
    max_x: f64,
}

struct Mount {
    off_x: f64,
    off_y: f64,
    length: f64,
    motor_radius: f64,
    motor_fit: f64,
    rad1: f64,
    rad2: f64,
    extra_back: f64,
}

struct Thread {
    ruthex_depth: f64,
    ruthex_radius: f64,
    through_radius: f64,
    countersink_radius: f64,
    countersink_depth: f64,
}

struct Brace {
    width: f64,
    extent: f64,
    indent: f64,
}

struct Port {
    start_x: f64,
    width: f64,
    length: f64,
}

struct Tube {
    width: f64,
    wall_bottom: f64,
    wall_top: f64,
    wire_inlet1: f64,
    wire_inlet2: f64,
    tab_width: f64,
}

struct HallMount {
    width: f64,
    thickness: f64,
    length: f64,

    hole1_x: f64,
    off_y: f64,
    rad1: f64,
    rad2: f64,
    tilt_deg: f64,
    extra_cone: f64,
}

struct DrumGuide {
    length: f64,
    rad_inner: f64,
    rad_outer: f64,
}

struct HallChannel {
    width: f64,
    length: f64,
}
struct HousingBuilder {
    aabb: Aabb3,
    inf: f64,
    m2: Thread,
    m3: Thread,
    m4: Thread,
    drum_bounding_radius: f64,
    back_thickness: f64,
    catch: Catch,
    mount: Mount,
    brace: Brace,
    port: Port,
    tab: Tab,
    tube: Tube,
    hall_mount: HallMount,
    drum_guide: DrumGuide,
    hall_channel: HallChannel,
}

impl HousingBuilder {
    fn main_body(&self) -> Sdf3 {
        let mut sdf = self.aabb.as_sdf().difference(
            &Cylinder::new(
                Vec3::new(0.0, 0.0, self.back_thickness),
                Vec3::axis_z() * self.inf,
                self.drum_bounding_radius,
            )
            .as_sdf(),
        );
        sdf = sdf.difference(
            &Aabb::new(
                Vec3::new(
                    -self.inf,
                    self.aabb.min().y() + self.catch.bottom_thickness,
                    self.back_thickness,
                ),
                Vec3::new(-self.catch.max_x, 0.0, self.inf),
            )
            .as_sdf(),
        );
        sdf
    }
    fn mount(&self, y: f64) -> Sdf3 {
        let mut sdf = Sdf::empty();
        sdf = sdf.union(
            &TruncatedCone::new(
                Vec3::new(self.mount.off_x, y, self.back_thickness),
                Vec3::new(0.0, 0.0, self.mount.length),
                self.mount.rad1,
                self.mount.rad2,
            )
            .as_sdf(),
        );
        sdf = sdf.difference(
            &Cylinder::new(
                Vec3::new(
                    self.mount.off_x,
                    y,
                    self.back_thickness + self.mount.length - self.m3.ruthex_depth,
                ),
                Vec3::new(0.0, 0.0, self.m3.ruthex_depth),
                self.m3.ruthex_radius,
            )
            .as_sdf(),
        );
        sdf = sdf.union(
            &Cylinder::new(
                Vec3::new(self.mount.off_x, 0.0, self.back_thickness),
                Vec3::new(0.0, 0.0, self.mount.extra_back),
                self.mount.motor_radius + self.mount.motor_fit,
            )
            .as_sdf(),
        );
        sdf
    }
    fn brace_x(&self, y: f64, dy: f64) -> Sdf3 {
        let mut sdf = Sdf::empty();
        sdf = sdf.union(
            &Polygon2::new(vec![
                Vec2::new(
                    y + dy * (self.mount.rad2 - self.brace.indent),
                    self.back_thickness,
                ),
                Vec2::new(
                    y + dy * (self.mount.rad2 - self.brace.indent + self.brace.extent),
                    self.back_thickness,
                ),
                Vec2::new(
                    y + dy * (self.mount.rad2 - self.brace.indent),
                    self.back_thickness + self.mount.length,
                ),
            ])
            .as_sdf()
            .extrude_x(
                self.mount.off_x - self.brace.width / 2.0
                    ..self.mount.off_x + self.brace.width / 2.0,
            ),
        );
        sdf
    }
    fn brace_y(&self, y: f64, dx: f64) -> Sdf3 {
        let mut sdf = Sdf::empty();
        sdf = sdf.union(
            &Polygon2::new(vec![
                Vec2::new(
                    self.mount.off_x + dx * (self.mount.rad2 - self.brace.indent),
                    self.back_thickness,
                ),
                Vec2::new(
                    self.mount.off_x
                        + dx * (self.mount.rad2 - self.brace.indent + self.brace.extent),
                    self.back_thickness,
                ),
                Vec2::new(
                    self.mount.off_x + dx * (self.mount.rad2 - self.brace.indent),
                    self.back_thickness + self.mount.length,
                ),
            ])
            .as_sdf()
            .extrude_y(-y - self.brace.width / 2.0..-y + self.brace.width / 2.0),
        );
        sdf
    }
    fn mounts(&self) -> Sdf3 {
        let mut sdf = Sdf::empty();
        sdf = sdf.union(&self.mount(self.mount.off_y));
        sdf = sdf.union(&self.mount(-self.mount.off_y));
        // sdf = sdf.union(&self.brace_x(self.mount.off_y, 1.0));
        // sdf = sdf.union(&self.brace_x(-self.mount.off_y, -1.0));
        // sdf = sdf.union(&self.brace_y(self.mount.off_y, 1.0));
        // sdf = sdf.union(&self.brace_y(self.mount.off_y, -1.0));
        // sdf = sdf.union(&self.brace_y(-self.mount.off_y, 1.0));
        // sdf = sdf.union(&self.brace_y(-self.mount.off_y, -1.0));
        sdf
    }
    fn wiring_pos(&self) -> Sdf3 {
        let mut sdf = Sdf::empty();
        sdf = sdf.union(
            &Polygon2::new(vec![
                Vec2::new(
                    self.tube.width / 2.0 - self.tube.wire_inlet1,
                    self.tube.wall_bottom,
                ),
                Vec2::new(
                    self.tube.width / 2.0 - self.tube.wire_inlet1 - self.tube.tab_width,
                    self.tube.wall_bottom,
                ),
                Vec2::new(
                    self.tube.width / 2.0 - self.tube.wire_inlet1 - self.tube.tab_width,
                    self.back_thickness - self.tube.wall_top - self.tube.wire_inlet2,
                ),
            ])
            .as_sdf()
            .extrude_x(self.port.start_x + self.port.width..self.aabb.max().x()),
        );
        sdf
    }
    fn wiring_neg(&self) -> Sdf3 {
        let mut sdf = Sdf::empty();
        sdf = sdf.union(
            &Aabb::new(
                Vec3::new(self.port.start_x, -self.port.length / 2.0, -self.inf),
                Vec3::new(
                    self.port.start_x + self.port.width,
                    self.port.length / 2.0,
                    self.inf,
                ),
            )
            .as_sdf(),
        );
        sdf = sdf.union(
            &Aabb::new(
                Vec3::new(
                    self.port.start_x + self.port.width,
                    -self.tube.width / 2.0,
                    self.tube.wall_bottom,
                ),
                Vec3::new(
                    self.inf,
                    self.tube.width / 2.0,
                    self.back_thickness - self.tube.wall_top,
                ),
            )
            .as_sdf(),
        );
        sdf = sdf.union(
            &Aabb::new(
                Vec3::new(
                    self.port.start_x + self.port.width,
                    self.tube.width / 2.0 - self.tube.wire_inlet1,
                    -self.inf,
                ),
                Vec3::new(
                    self.inf,
                    self.tube.width / 2.0,
                    (self.tube.wall_bottom + self.back_thickness - self.tube.wall_top) / 2.0,
                ),
            )
            .as_sdf(),
        );
        sdf = sdf.union(
            &Aabb::new(
                Vec3::new(
                    self.port.start_x - self.hall_channel.length,
                    -self.hall_channel.width / 2.0,
                    self.back_thickness,
                ),
                Vec3::new(
                    self.port.start_x,
                    self.hall_channel.width / 2.0,
                    self.back_thickness + self.mount.extra_back,
                ),
            )
            .as_sdf(),
        );
        sdf
    }
    fn tab(&self, mut sdf: Sdf3, origin: Vec2, axis: Vec3) -> Sdf3 {
        let axis2 = Vec3::axis_z();
        let axis1 = -axis.cross(axis2);
        sdf = sdf.union(
            &Polygon2::new(vec![
                Vec2::new(-self.tab.size, 0.0),
                Vec2::new(self.tab.size, 0.0),
                Vec2::new(0.0, self.tab.size),
            ])
            .as_sdf()
            .extrude(
                Vec3::new(origin.x(), origin.y(), self.aabb.max().z()),
                axis1,
                axis2,
                self.tab.thickness,
            ),
        );
        sdf = sdf.difference(
            &Polygon2::new(vec![
                Vec2::new(-self.tab.size, 0.0),
                Vec2::new(self.tab.size, 0.0),
                Vec2::new(0.0, self.tab.size),
            ])
            .as_sdf()
            .extrude(
                Vec3::new(origin.x(), origin.y(), 0.0),
                axis1,
                axis2,
                self.tab.thickness,
            ),
        );

        sdf = sdf.difference(
            &Cylinder::new(
                Vec3::new(
                    origin.x(),
                    origin.y(),
                    self.aabb.max().z() + self.tab.wall_size,
                ),
                axis * self.tab.thickness * 2.0,
                self.m3.through_radius,
            )
            .as_sdf(),
        );
        sdf = sdf.difference(
            &Cylinder::new(
                Vec3::new(
                    origin.x(),
                    origin.y(),
                    self.aabb.max().z() + self.tab.wall_size,
                ),
                axis * self.m3.countersink_depth,
                self.m3.countersink_radius,
            )
            .as_sdf(),
        );
        sdf = sdf.difference(
            &Cylinder::new(
                Vec3::new(origin.x(), origin.y(), self.tab.wall_size),
                axis * (self.tab.thickness + self.m3.ruthex_depth),
                self.m3.ruthex_radius,
            )
            .as_sdf(),
        );
        sdf
    }
    fn hall_mount(&self) -> Sdf3 {
        let mut sdf = Sdf::empty();
        sdf = sdf.union(
            &Aabb::new(
                Vec3::new(
                    self.hall_mount.hole1_x
                        - self.hall_mount.width
                        - self.hall_mount.thickness / 2.0,
                    self.hall_mount.off_y - self.hall_mount.thickness / 2.0,
                    self.back_thickness,
                ),
                Vec3::new(
                    self.hall_mount.hole1_x + self.hall_mount.thickness / 2.0,
                    self.hall_mount.off_y + self.hall_mount.thickness / 2.0,
                    self.back_thickness + self.hall_mount.length,
                ),
            )
            .as_sdf(),
        );
        sdf = sdf.union(
            &TruncatedCone::new(
                Vec3::new(
                    self.hall_mount.hole1_x,
                    self.hall_mount.off_y,
                    self.back_thickness,
                ),
                Vec3::axis_z() * (self.hall_mount.length + self.hall_mount.extra_cone),
                self.hall_mount.rad1,
                self.hall_mount.rad2,
            )
            .as_sdf(),
        );
        sdf = sdf.union(
            &TruncatedCone::new(
                Vec3::new(
                    self.hall_mount.hole1_x - self.hall_mount.width,
                    self.hall_mount.off_y,
                    self.back_thickness,
                ),
                Vec3::axis_z() * (self.hall_mount.length + self.hall_mount.extra_cone),
                self.hall_mount.rad1,
                self.hall_mount.rad2,
            )
            .as_sdf(),
        );
        let norm = Vec2::from_deg(self.hall_mount.tilt_deg);
        let norm = Vec3::new(0.0, norm.x(), norm.y());
        sdf = sdf.difference(
            &Plane::new(
                Vec3::new(
                    0.0,
                    self.hall_mount.off_y,
                    self.back_thickness + self.hall_mount.length,
                ),
                -norm,
            )
            .as_sdf(),
        );

        sdf = sdf.difference(
            &Cylinder::new(
                Vec3::new(
                    self.hall_mount.hole1_x - self.hall_mount.width,
                    self.hall_mount.off_y,
                    self.back_thickness + self.hall_mount.length,
                ),
                -norm * self.m2.ruthex_depth,
                self.m2.ruthex_radius,
            )
            .as_sdf(),
        );
        sdf = sdf.difference(
            &Cylinder::new(
                Vec3::new(
                    self.hall_mount.hole1_x,
                    self.hall_mount.off_y,
                    self.back_thickness + self.hall_mount.length,
                ),
                -norm * self.m2.ruthex_depth,
                self.m2.ruthex_radius,
            )
            .as_sdf(),
        );
        sdf
    }
    fn motor_clearance(&self) -> Sdf3 {
        Cylinder::new(
            Vec3::new(
                self.mount.off_x,
                0.0,
                self.back_thickness + self.mount.extra_back,
            ),
            Vec3::new(0.0, 0.0, self.inf),
            self.mount.motor_radius + self.mount.motor_fit,
        )
        .as_sdf()
    }
    fn drum_guide(&self) -> Sdf3 {
        let mut sdf = Sdf::empty();
        sdf = sdf.union(
            &Cylinder::new(
                Vec3::new(0.0, 0.0, self.back_thickness),
                Vec3::new(0.0, 0.0, self.drum_guide.length),
                self.drum_guide.rad_outer,
            )
            .as_sdf(),
        );
        sdf = sdf.difference(
            &Cylinder::new(
                Vec3::new(0.0, 0.0, -self.inf),
                Vec3::new(0.0, 0.0, 2.0 * self.inf),
                self.drum_guide.rad_inner,
            )
            .as_sdf(),
        );
        sdf
    }
    fn drillium(&self) -> Sdf3 {
        let mut sdf = Sdf::empty();
        sdf = sdf.union(
            &Cylinder::new(
                Vec3::new(42.0, 57.0, -self.inf),
                Vec3::axis_z() * self.inf * 2.0,
                10.0,
            )
            .as_sdf(),
        );
        sdf = sdf.union(
            &Cylinder::new(
                Vec3::new(44.0, -57.0, -self.inf),
                Vec3::axis_z() * self.inf * 2.0,
                12.0,
            )
                .as_sdf(),
        );
        sdf
    }
    fn build_sdf(&self) -> Sdf3 {
        let mut sdf = self.main_body();
        sdf = sdf.union(&self.mounts());
        sdf = sdf.difference(&self.wiring_neg());
        sdf = sdf.union(&self.wiring_pos());
        sdf = self.tab(
            sdf,
            Vec2::new(self.tab.bottom_x, self.aabb.min().y()),
            Vec3::axis_y(),
        );
        sdf = self.tab(
            sdf,
            Vec2::new(self.tab.top_x, self.aabb.max().y()),
            -Vec3::axis_y(),
        );
        sdf = self.tab(
            sdf,
            Vec2::new(self.aabb.max().x(), self.tab.right_y),
            -Vec3::axis_x(),
        );
        sdf = sdf.union(&self.hall_mount());
        sdf = sdf.difference(&self.motor_clearance());
        sdf = sdf.union(&self.drum_guide());
        sdf = sdf.difference(&self.drillium());
        sdf
    }
    pub fn build(&self) -> Mesh {
        let sdf = self.build_sdf();
        let mut marching = MarchingMesh::new(Aabb::new(
            self.aabb.min() + Vec3::splat(-0.1),
            self.aabb.max() + Vec3::new(0.1, 0.1, self.tab.size + 0.1),
        ));
        marching
            // .min_render_depth(6)
            // .max_render_depth(7)
            // .subdiv_max_dot(0.9);
            .min_render_depth(7)
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
        inf: 1000.0,
        aabb: Aabb::new(Vec3::new(-35.0, -71.0, 0.0), Vec3::new(59.0, 70.0, 50.0)),
        m2: Thread {
            ruthex_depth: 4.0,
            ruthex_radius: 3.2 / 2.0,
            through_radius: 2.2 / 2.0,
            countersink_radius: 4.0 / 2.0,
            countersink_depth: 1.6,
        },
        m3: Thread {
            ruthex_depth: 5.7,
            ruthex_radius: 4.6 / 2.0,
            through_radius: 3.2 / 2.0,
            countersink_radius: 6.0 / 2.0,
            countersink_depth: 2.0,
        },
        m4: Thread {
            ruthex_depth: 8.1,
            ruthex_radius: 5.6 / 2.0,
            through_radius: 4.2 / 2.0,
            countersink_radius: 8.0 / 2.0,
            countersink_depth: 2.5,
        },
        drum_bounding_radius: 56.0,
        back_thickness: 4.0,
        tab: Tab {
            size: 14.0,
            thickness: 5.0,
            wall_size: 6.0,
            bottom_x: 20.0,
            top_x: -20.0,
            right_y: 45.0,
        },
        catch: Catch {
            bottom_thickness: 3.0,
            max_x: 1.0,
        },
        mount: Mount {
            off_x: 8.0,
            off_y: 17.5,
            length: 22.0,
            motor_radius: 14.0,
            motor_fit: 0.05,
            rad1: 8.0,
            rad2: 5.0,
            extra_back: 4.0,
        },
        brace: Brace {
            width: 2.0,
            extent: 6.0,
            indent: 0.2,
        },
        port: Port {
            start_x: 16.0,
            width: 7.0,
            length: 16.0,
        },
        tube: Tube {
            width: 8.0,
            wall_bottom: 1.0,
            wall_top: 1.0,
            wire_inlet1: 1.0,
            wire_inlet2: 0.2,
            tab_width: 2.0,
        },
        hall_mount: HallMount {
            width: 10.0,
            thickness: 6.0,
            length: 20.0,
            hole1_x: -8.5,
            off_y: -10.0,
            rad1: 6.0,
            rad2: 4.0,
            tilt_deg: 45.0,
            extra_cone: 3.5,
        },
        drum_guide: DrumGuide {
            length: 20.0,
            rad_inner: 53.0 / 2.0,
            rad_outer: 55.0 / 2.0,
        },
        hall_channel: HallChannel {
            width: 6.0,
            length: 30.0,
        },
    }
    .build();
    println!("{:?}", mesh.check_manifold());
    println!("Built mesh in {:?}", start.elapsed());
    encode_file(&mesh, Path::new("examples/housing/output/sideA.stl")).await?;
    Ok(())
}
