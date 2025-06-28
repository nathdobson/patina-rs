#![feature(exit_status_error)]
#![deny(unused_must_use)]

use std::io::Cursor;
use threemf::Mesh;
use threemf::model::{Build, Item, Object, Resources, Triangle, Triangles, Unit, Vertex, Vertices};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let mut encoded: Vec<u8> = vec![];
    threemf::write(
        Cursor::new(&mut encoded),
        threemf::model::Model {
            xmlns: "http://schemas.microsoft.com/3dmanufacturing/core/2015/02".to_string(),
            metadata: vec![],
            resources: Resources {
                object: vec![Object {
                    id: 1,
                    partnumber: None,
                    name: None,
                    pid: None,
                    mesh: Some(Mesh {
                        vertices: Vertices {
                            vertex: vec![
                                Vertex {
                                    x: 0.0,
                                    y: 0.0,
                                    z: 0.0,
                                },
                                Vertex {
                                    x: 100.0,
                                    y: 0.0,
                                    z: 0.0,
                                },
                                Vertex {
                                    x: 0.0,
                                    y: 100.0,
                                    z: 0.0,
                                },
                                Vertex {
                                    x: 0.0,
                                    y: 0.0,
                                    z: 100.0,
                                },
                            ],
                        },
                        triangles: Triangles {
                            triangle: vec![
                                Triangle {
                                    v1: 0,
                                    v2: 1,
                                    v3: 2,
                                },
                                Triangle {
                                    v1: 1,
                                    v2: 0,
                                    v3: 3,
                                },
                                Triangle {
                                    v1: 2,
                                    v2: 1,
                                    v3: 3,
                                },
                                Triangle {
                                    v1: 0,
                                    v2: 2,
                                    v3: 3,
                                },
                            ],
                        },
                    }),
                    components: None,
                }],
                basematerials: None,
            },
            build: Build {
                item: vec![Item {
                    objectid: 1,
                    transform: None,
                    partnumber: None,
                }],
            },
            unit: Unit::Millimeter,
        },
    )?;

    // let model = Model {};
    // let encoded = model.encode()?;
    tokio::fs::write("examples/flap/output.3mf", encoded).await?;
    tokio::fs::remove_dir_all("examples/flap/output").await.ok();
    tokio::fs::create_dir("examples/flap/output").await.ok();

    tokio::process::Command::new("unzip")
        .arg("../output.3mf")
        .current_dir("examples/flap/output")
        .spawn()?
        .wait()
        .await?
        .exit_ok()?;
    let mut slicer =
        tokio::process::Command::new("/Applications/BambuStudio.app/Contents/MacOS/BambuStudio");
    slicer.arg("--debug").arg("1");
    slicer.arg("--slice").arg("0");
    let filament = "/Users/nathan/Library/Application Support/BambuStudio/system/BBL/filament/Bambu PLA Basic @BBL A1M 0.2 nozzle.json";
    slicer.arg("--load-filaments").arg(filament);
    let machine = "/Users/nathan/Library/Application Support/BambuStudio/system/BBL/machine/Bambu Lab A1 mini 0.4 nozzle.json";
    let process = "/Users/nathan/Library/Application Support/BambuStudio/system/BBL/process/0.20mm Standard @BBL A1M.json";
    slicer
        .arg("--load-settings")
        .arg(format!("{};{}", machine, process));
    slicer.arg("--export-3mf").arg("/Users/nathan/Documents/workspace/patina/examples/flap/sliced.3mf");
    slicer.arg("examples/flap/output.3mf");
    slicer.spawn()?.wait().await?.exit_ok()?;
    Ok(())
}
