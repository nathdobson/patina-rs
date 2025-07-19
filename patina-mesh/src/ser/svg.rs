use crate::edge_mesh2::EdgeMesh2;
use crate::ser::{Encode, encode_test_file};
use patina_geo::aabb::Aabb;
use patina_geo::geo2::aabb2::Aabb2;
use patina_geo::geo2::polygon2::Polygon2;
use patina_vec::vec2::Vec2;
use tokio::io::{AsyncWrite, AsyncWriteExt};

impl Encode for EdgeMesh2 {
    fn extension() -> &'static str {
        "svg"
    }

    fn encode<W: Unpin + Send + AsyncWrite>(
        &self,
        w: &mut W,
    ) -> impl Send + Future<Output = anyhow::Result<()>> {
        async move {
            let polys = self.as_polygons();
            let aabb = self.vertices().iter().cloned().collect::<Aabb2>();
            w.write_all(
                format!(
                    "<svg xmlns=\"http://www.w3.org/2000/svg\" viewBox=\"{} {} {} {}\">\n",
                    aabb.min().x(),
                    aabb.min().y(),
                    aabb.dimensions().x(),
                    aabb.dimensions().y()
                )
                .as_bytes(),
            )
            .await?;
            for poly in polys {
                let color = if poly.signed_area() > 0.0 {
                    "lime"
                } else {
                    "red"
                };
                w.write_all("<polygon points=\"".as_bytes()).await?;
                for point in poly.points() {
                    w.write_all(format!("{},{} ", point.x(), point.y()).as_bytes())
                        .await?;
                }
                w.write_all(
                    format!(
                        " \" style=\"fill:none;stroke:{};stroke-width:.1\" />\n",
                        color
                    )
                    .as_bytes(),
                )
                .await?;
            }
            w.write_all("</svg>\n".as_bytes()).await?;

            Ok(())
        }
    }
}

#[cfg(test)]
#[tokio::test]
async fn test_svg() -> anyhow::Result<()> {
    for rev in [false, true] {
        let mut mesh = EdgeMesh2::new();
        let points = vec![
            Vec2::new(0.0, 0.0),
            Vec2::new(10.0, 0.0),
            Vec2::new(10.0, 10.0),
            Vec2::new(0.0, 10.0),
        ];
        if rev {
            mesh.add_polygon(points.into_iter().rev());
        } else {
            mesh.add_polygon(points.into_iter());
        }
        encode_test_file(&mesh, &format!("test_{}.svg", rev)).await?;
    }
    Ok(())
}
