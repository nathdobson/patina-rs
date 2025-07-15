use crate::PolygonOutlineBuilder;
use anyhow::anyhow;
use patina_extrude::ExtrusionBuilder;
use patina_geo::geo2::polygon2::Polygon2;
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
        let mut extrusion = ExtrusionBuilder::new();
        let mut outline = PolygonOutlineBuilder::new();
        font.glyph(letter)
            .scaled(Scale::uniform(16.0))
            .positioned(Point { x: 0.0, y: 0.0 })
            .build_outline(&mut outline);

        for poly in outline.build() {
            extrusion.add_prism(poly, 0.0..1.0);
        }
        let extrusion = extrusion.build();
        extrusion.check_manifold()?;
        write_test_stl_file(&extrusion, &format!("{}.stl", letter as u32)).await?;
    }
    Ok(())
}
