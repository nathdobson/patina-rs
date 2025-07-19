use crate::mesh::Mesh;
use crate::ser::Encode;
use anyhow::anyhow;
use patina_vec::vec3::Vec3;
use std::io;
use std::path::{Path, PathBuf};
use tokio::fs::File;
use tokio::io::{AsyncWrite, AsyncWriteExt, BufWriter};

async fn write_vec3<W: Unpin + AsyncWrite>(vec3: Vec3, w: &mut W) -> io::Result<()> {
    for c in vec3 {
        w.write_all(&(c as f32).to_le_bytes()).await?;
    }
    Ok(())
}

impl Encode for Mesh {
    fn extension() -> &'static str {
        "stl"
    }

    fn encode<W: Unpin + Send + AsyncWrite>(
        &self,
        w: &mut W,
    ) -> impl Send + Future<Output = anyhow::Result<()>> {
        async move {
            w.write_all(&[0u8; 80]).await?;
            w.write_all(&(self.triangles().len() as u32).to_le_bytes())
                .await?;
            for vs in self.triangles() {
                let normal = (self.vertices()[vs[0]] - self.vertices()[vs[1]])
                    .cross(self.vertices()[vs[0]] - self.vertices()[vs[2]])
                    .normalize();
                write_vec3(normal, w).await?;
                for v in *vs {
                    write_vec3(self.vertices()[v], w).await?;
                }
                w.write_all(&[0u8; 2]).await?;
            }
            Ok(())
        }
    }
}
