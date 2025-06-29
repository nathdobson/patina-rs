#![feature(exit_status_error)]
#![deny(unused_must_use)]

use patina_3mf::ModelContainer;
use patina_3mf::mesh::{Mesh, Triangle, Triangles, Vertex, Vertices};
use patina_3mf::model::{
    Build, Component, Components, Item, Metadata, Model, Object, ObjectType, Resources, Unit, Xmlns,
};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let mut application_metadata =
        Metadata::new("Application".to_string()).value(Some("BambuStudio-01.10.01.50".to_string()));
    let mesh1 = Mesh::new(
        Vertices::new(vec![
            Vertex::new(0.0, 0.0, 0.0),
            Vertex::new(10.0, 0.0, 0.0),
            Vertex::new(0.0, 10.0, 0.0),
            Vertex::new(0.0, 0.0, 10.0),
        ]),
        Triangles::new(vec![
            Triangle::new(0, 1, 2),
            Triangle::new(0, 3, 1),
            Triangle::new(2, 3, 0),
            Triangle::new(1, 3, 2),
        ]),
    );
    let mesh2 = Mesh::new(
        Vertices::new(vec![
            Vertex::new(0.0, 0.0, 0.0),
            Vertex::new(-10.0, 0.0, 0.0),
            Vertex::new(0.0, -10.0, 0.0),
            Vertex::new(0.0, 0.0, 10.0),
        ]),
        Triangles::new(vec![
            Triangle::new(0, 1, 2),
            Triangle::new(0, 3, 1),
            Triangle::new(2, 3, 0),
            Triangle::new(1, 3, 2),
        ]),
    );
    let object1 = Object::new(1)
        .mesh(Some(mesh1))
        .object_type(Some(ObjectType::Model));
    let object2 = Object::new(2)
        .mesh(Some(mesh2))
        .object_type(Some(ObjectType::Model));
    let object9 = Object::new(9)
        .components(Some(Components::new(vec![
            Component::new(1),
            Component::new(2),
        ])))
        .object_type(Some(ObjectType::Model));
    let model = Model::new()
        .xmlns(Xmlns::Model)
        .metadata(vec![application_metadata])
        .resources(Resources::new().object(vec![object1, object2, object9]))
        .build(Build::new().item(vec![Item::new(9).printable(Some(true))]))
        .unit(Unit::Millimeter);
    let model_cont = ModelContainer::new(model);
    let encoded = model_cont.encode()?;
    tokio::fs::write("examples/flap/output.3mf", encoded).await?;
    tokio::fs::remove_dir_all("examples/flap/output").await.ok();
    tokio::fs::create_dir("examples/flap/output").await.ok();

    tokio::process::Command::new("unzip")
        .arg("-q")
        .arg("../output.3mf")
        .current_dir("examples/flap/output")
        .spawn()?
        .wait()
        .await?
        .exit_ok()?;
    let mut slicer =
        tokio::process::Command::new("/Applications/BambuStudio.app/Contents/MacOS/BambuStudio");
    slicer.arg("--debug").arg("2");
    slicer.arg("--slice").arg("0");
    let filament = "/Users/nathan/Library/Application Support/BambuStudio/system/BBL/filament/Bambu PLA Basic @BBL A1M 0.2 nozzle.json";
    slicer.arg("--load-filaments").arg(filament);
    let machine = "/Users/nathan/Library/Application Support/BambuStudio/system/BBL/machine/Bambu Lab A1 mini 0.4 nozzle.json";
    let process = "/Users/nathan/Library/Application Support/BambuStudio/system/BBL/process/0.20mm Standard @BBL A1M.json";
    slicer.arg("--enable-timelapse");
    slicer.arg("--timelapse-type=1");
    slicer
        .arg("--load-settings")
        .arg(format!("{};{}", machine, process));
    slicer
        .arg("--export-3mf")
        .arg("/Users/nathan/Documents/workspace/patina/examples/flap/sliced.3mf");
    slicer.arg("examples/flap/output.3mf");
    slicer.spawn()?.wait().await?.exit_ok()?;
    tokio::fs::remove_dir_all("examples/flap/sliced").await.ok();
    tokio::fs::create_dir("examples/flap/sliced").await.ok();
    tokio::process::Command::new("unzip")
        .arg("-q")
        .arg("../sliced.3mf")
        .current_dir("examples/flap/sliced")
        .spawn()?
        .wait()
        .await?
        .exit_ok()?;
    Ok(())
}
