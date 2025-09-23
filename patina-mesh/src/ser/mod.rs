use crate::mesh::Mesh;
use anyhow::{Context, anyhow};
use std::io;
use std::path::{Path, PathBuf};
use tokio::fs::File;
use tokio::io::{AsyncWrite, AsyncWriteExt, BufWriter};

pub mod stl;
mod svg;

pub trait Encode {
    fn extension() -> &'static str;
    fn encode<W: Unpin + Send + AsyncWrite>(
        &self,
        w: &mut W,
    ) -> impl Send + Future<Output = anyhow::Result<()>>;
}

pub async fn encode_file<T: Encode>(encode: &T, filename: &Path) -> anyhow::Result<()> {
    let result: anyhow::Result<()> = try {
        let file = File::create(filename).await?;
        let mut file = BufWriter::new(file);
        encode.encode(&mut file).await?;
        file.flush().await?;
        ()
    };
    result.with_context(|| format!("while saving file {:?}", filename))
}

pub async fn create_test_path(filename: &str) -> anyhow::Result<PathBuf> {
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
        test_name.push_str(&part.to_str().unwrap());
        test_name.push('_');
    }
    test_name.push_str(
        &std::thread::current()
            .name()
            .unwrap_or("<unknown>")
            .replace("::", "_"),
    );
    let test_dir = target_dir.join("test_outputs").join(test_name);
    tokio::fs::create_dir_all(&test_dir).await?;
    Ok(test_dir.join(filename))
}

pub async fn encode_test_file<T: Encode>(encode: &T, filename: &str) -> anyhow::Result<()> {
    encode_file(encode, &create_test_path(filename).await?).await?;
    Ok(())
}
