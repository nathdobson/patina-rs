use crate::meshes::mesh::Mesh;
use std::io;
use std::path::Path;
use tokio::fs::File;
use tokio::io::AsyncWrite;
use tokio::io::AsyncWriteExt;
use tokio::io::BufWriter;
use patina_vec::vec3::Vec3;

async fn write_vec3<W: Unpin + AsyncWrite>(vec3: Vec3, w: &mut W) -> io::Result<()> {
    for c in vec3 {
        w.write_all(&(c as f32).to_le_bytes()).await?;
    }
    Ok(())
}

pub async fn write_stl<W: Unpin + AsyncWrite>(mesh: &Mesh, w: &mut W) -> io::Result<()> {
    w.write_all(&[0u8; 80]).await?;
    w.write_all(&(mesh.triangles().len() as u32).to_le_bytes())
        .await?;
    for vs in mesh.triangles() {
        let normal = (mesh.vertices()[vs[0]] - mesh.vertices()[vs[1]])
            .cross(mesh.vertices()[vs[0]] - mesh.vertices()[vs[2]])
            .normalize();
        write_vec3(normal, w).await?;
        for v in *vs {
            write_vec3(mesh.vertices()[v], w).await?;
        }
        w.write_all(&[0u8; 2]).await?;
    }
    Ok(())
}

pub async fn write_stl_file(mesh: &Mesh, filename: &Path) -> io::Result<()> {
    let file = File::create(filename).await?;
    let mut file = BufWriter::new(file);
    write_stl(mesh, &mut file).await?;
    file.flush().await?;
    Ok(())
}
