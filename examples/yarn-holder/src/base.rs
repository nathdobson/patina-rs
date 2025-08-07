mod consts;

use crate::consts::{
    BASE_RADIUS, BASE_RISE, BASE_RISE_WIDTH, BASE_THICKNESS, EPS, FITMENT, POST_THICKNESS,
    POST_WIDTH,
};
use patina_geo::geo3::aabb3::Aabb3;
use patina_geo::geo3::cylinder::Cylinder;
use patina_mesh::ser::encode_file;
use patina_sdf::marching_mesh::MarchingMesh;
use patina_sdf::sdf::AsSdf;
use patina_vec::vec3::Vec3;
use std::path::Path;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let mut sdf = Cylinder::new(
        Vec3::zero(),
        Vec3::axis_z() * (BASE_THICKNESS + BASE_RISE),
        BASE_RADIUS,
    )
    .as_sdf();
    sdf = sdf.difference(
        &Cylinder::new(
            Vec3::new(0.0, 0.0, BASE_THICKNESS),
            Vec3::axis_z() * BASE_RISE,
            BASE_RADIUS - BASE_RISE_WIDTH,
        )
        .as_sdf(),
    );
    sdf = sdf.difference(
        &Aabb3::new(
            Vec3::new(
                -POST_WIDTH / 2.0 - FITMENT,
                -POST_THICKNESS / 2.0 - FITMENT,
                0.0,
            ),
            Vec3::new(
                POST_WIDTH / 2.0 + FITMENT,
                POST_THICKNESS / 2.0 + FITMENT,
                BASE_THICKNESS,
            ),
        )
        .as_sdf(),
    );
    let mut mesh = MarchingMesh::new(Aabb3::new(
        Vec3::new(-BASE_RADIUS - EPS, -BASE_RADIUS - EPS, -EPS),
        Vec3::new(
            BASE_RADIUS + EPS,
            BASE_RADIUS + EPS,
            BASE_THICKNESS + BASE_RISE + EPS,
        ),
    ));
    mesh.min_render_depth(6)
        .max_render_depth(7)
        .subdiv_max_dot(0.9);
    let mesh = mesh.build(&sdf);
    encode_file(&mesh, Path::new("examples/yarn-holder/output/base.stl")).await?;
    Ok(())
}
