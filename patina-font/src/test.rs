use crate::PolygonOutlineBuilder;
use anyhow::anyhow;
use patina_extrude::ExtrusionBuilder;
use patina_geo::geo2::polygon2::Polygon2;
use patina_geo::segment2::Segment2;
use patina_mesh::edge_mesh2::EdgeMesh2;
use patina_mesh::mesh_cut::MeshCut;
use patina_mesh::ser::stl::write_test_stl_file;
use patina_vec::vec2::Vec2;
use rusttype::{Font, Point, Scale};
use tokio::fs;

#[tokio::test]
async fn test() -> anyhow::Result<()> {
    let font =
        Font::try_from_vec(fs::read("/System/Library/Fonts/Supplemental/Phosphate.ttc").await?)
            .ok_or_else(|| anyhow!("bad font"))?;

    for letter in ' '..'~' {
        println!("Building letter {}", letter);
        let mut outline = PolygonOutlineBuilder::new(0.1);
        font.glyph(letter)
            .scaled(Scale::uniform(16.0))
            .positioned(Point { x: 0.0, y: 0.0 })
            .build_outline(&mut outline);

        let mut outline_mesh = EdgeMesh2::new();
        for poly in outline.build() {
            outline_mesh.add_polygon(poly.points().iter().cloned());
        }

        let mut inverted=EdgeMesh2::new();
        inverted.add_mesh(&outline_mesh,true);
        let top_half = MeshCut::new(
            &inverted,
            vec![
                Vec2::new(15.0, -5.0),
                Vec2::new(15.0, 15.0),
                Vec2::new(-5.0, 15.0),
                Vec2::new(-5.0, -5.0),
            ],
        )
        .build();

        let mut extrusion = ExtrusionBuilder::new();
        let p1 = extrusion.add_plane(0.0, true);
        let p2 = extrusion.add_plane(1.0, false);
        extrusion.add_prism(&top_half, (p1, false), (p2, false));

        let extrusion = extrusion.build();
        // extrusion.check_manifold()?;
        write_test_stl_file(&extrusion, &format!("{}.stl", letter as u32)).await?;
    }
    Ok(())
}
