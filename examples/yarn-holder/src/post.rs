mod consts;

use crate::consts::{
    BASE_RADIUS, BASE_THICKNESS, CATCH_LENGTH, CATCH_WIDTH, CLIP_HOLE_RADIUS, EPS, POST_LENGTH,
    POST_THICKNESS, POST_WIDTH,
};
use patina_geo::geo3::aabb3::Aabb3;
use patina_geo::geo3::cylinder::Cylinder;
use patina_mesh::ser::encode_file;
use patina_sdf::marching_mesh::MarchingMesh;
use patina_sdf::sdf::{AsSdf, Sdf};
use patina_vec::vec3::Vec3;
use std::path::Path;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let post = Aabb3::new(
        Vec3::new(0.0, -POST_WIDTH / 2.0, 0.0),
        Vec3::new(POST_LENGTH, POST_WIDTH / 2.0, POST_THICKNESS),
    );
    let mut sdf = post.as_sdf();
    sdf = sdf.union(
        &Cylinder::new(
            Vec3::new(POST_LENGTH, 0.0, 0.0),
            Vec3::axis_z() * POST_THICKNESS,
            POST_WIDTH / 2.0,
        )
        .as_sdf(),
    );
    sdf = sdf.difference(
        &Cylinder::new(
            Vec3::new(POST_LENGTH, 0.0, 0.0),
            Vec3::axis_z() * POST_THICKNESS,
            CLIP_HOLE_RADIUS,
        )
        .as_sdf(),
    );
    sdf = sdf.union(
        &Aabb3::new(
            Vec3::new(-CATCH_WIDTH, -CATCH_LENGTH, 0.0),
            Vec3::new(0.0, CATCH_LENGTH, POST_THICKNESS),
        )
        .as_sdf(),
    );
    let mut mesh = MarchingMesh::new(Aabb3::new(
        post.min() - Vec3::new(CATCH_WIDTH + EPS, CATCH_LENGTH + EPS, EPS),
        post.max() + Vec3::new(POST_WIDTH + EPS, CATCH_LENGTH + EPS, EPS),
    ));
    mesh.min_render_depth(6)
        .max_render_depth(8)
        .subdiv_max_dot(0.99);
    let mesh = mesh.build(&sdf);
    encode_file(&mesh, Path::new("examples/yarn-holder/output/post.stl")).await?;
    Ok(())
}
