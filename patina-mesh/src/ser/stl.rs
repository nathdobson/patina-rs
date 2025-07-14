use crate::mesh::Mesh;
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

pub async fn write_test_stl_file(mesh: &Mesh, filename: &str) -> anyhow::Result<()> {
    let manifest_dir = PathBuf::from(std::env::var("CARGO_MANIFEST_DIR")?);
    let mut manifest_dir: &Path = &manifest_dir;
    let mut target_dir;
    let mut parts = vec![];
    loop {
        target_dir = manifest_dir.join("target");
        if target_dir.exists() {
            break;
        } else {
            if let Some(manifest_dir_parent) = manifest_dir.parent() {
                parts.push(manifest_dir.file_name().unwrap());
                manifest_dir = manifest_dir_parent;
            } else {
                return Err(anyhow!("Cannot find target directory"));
            }
        }
    }
    let mut test_name = String::new();
    for part in parts {
        test_name.push_str(part.to_str().unwrap());
        test_name.push('_');
    }
    test_name.push_str(std::thread::current().name().unwrap_or("<unknown>"));
    let test_dir = target_dir.join("test_outputs").join(test_name);
    tokio::fs::create_dir_all(&test_dir).await?;
    let file = test_dir.join(filename);
    write_stl_file(mesh, &file).await?;
    Ok(())
}
