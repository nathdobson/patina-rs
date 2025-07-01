#![feature(exit_status_error)]
#![deny(unused_must_use)]
#![allow(unused_mut)]
#![allow(unused_imports)]
#![allow(dead_code)]

use anyhow::anyhow;
use patina_3mf::ModelContainer;
use patina_3mf::color::Color;
use patina_3mf::content_types::{ContentTypeDefault, ContentTypes};
use patina_3mf::model::build::{ModelBuild, ModelItem};
use patina_3mf::model::mesh::{
    ModelMesh, ModelTriangle, ModelTriangles, ModelVertex, ModelVertices,
};
use patina_3mf::model::resources::{
    ModelComponent, ModelComponents, ModelObject, ModelObjectType, ModelResources,
};
use patina_3mf::model::{Model, ModelMetadata, ModelUnit};
use patina_3mf::model_settings::{
    Assemble, AssembleItem, ModelInstance, ModelSettings, ObjectSettings, Part, Plate,
    SettingsMetadata,
};
use patina_3mf::project_settings::ProjectSettings;
use patina_3mf::relationships::{Relationship, Relationships};
use patina_3mf::settings_id::filament_settings_id::{
    FilamentBrand, FilamentMaterial, FilamentSettingsId,
};
use patina_3mf::settings_id::nozzle::Nozzle;
use patina_3mf::settings_id::print_settings_id::{PrintQuality, PrintSettingsId};
use patina_3mf::settings_id::printer::Printer;
use patina_3mf::settings_id::printer_settings_id::PrinterSettingsId;
use patina_bambu::cli::{BambuStudioCommand, DebugLevel, Slice};
use patina_bambu::{BambuBuilder, BambuFilament, BambuObject, BambuPart, BambuPlate};
use patina_cad::math::vec2::Vec2;
use patina_cad::math::vec3::Vec3;
use patina_cad::meshes::mesh::Mesh;
use patina_cad::meshes::mesh_triangle::MeshTriangle;
use std::env;
use std::io::{Cursor, Read, Write};
use std::path::Path;
use zip::write::{FileOptions, SimpleFileOptions};
use zip::{ZipArchive, ZipWriter};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    build_output().await?;
    Ok(())
}

async fn build_output() -> anyhow::Result<()> {
    let mut bambu = BambuBuilder::new();
    let printer = Printer::A1Mini;
    let nozzle = Nozzle::Nozzle0_4;
    let machine = PrinterSettingsId::new(printer.clone(), Some(nozzle.clone()));
    let process = PrintSettingsId::new(0.2, PrintQuality::Standard, printer.clone(), nozzle);
    let pla_basic = FilamentSettingsId::new(
        FilamentBrand::Bambu,
        FilamentMaterial::PlaBasic,
        printer.clone(),
        None,
    );
    let pla_matte = FilamentSettingsId::new(
        FilamentBrand::Bambu,
        FilamentMaterial::PlaMatte,
        printer.clone(),
        None,
    );

    bambu.printer_settings_id(Some(machine.clone()));
    bambu.print_settings_id(Some(process.clone()));
    bambu.prime_tower_position(Some(Vec2::new(100.0, 100.0)));
    bambu.add_plate({
        let mut plate = BambuPlate::new();
        plate.add_object({
            let mut object = BambuObject::new();
            object.transform(Some([
                1.0, 0.0, 0.0, //
                0.0, 1.0, 0.0, //
                0.0, 0.0, 1.0, //
                60.0, 60.0, 0.0, //
            ]));
            object.add_part({
                let mut part = BambuPart::new(Mesh::new(
                    vec![
                        Vec3::new(0.0, 0.0, 0.0),
                        Vec3::new(10.0, 0.0, 0.0),
                        Vec3::new(0.0, 10.0, 0.0),
                        Vec3::new(0.0, 0.0, 10.0),
                    ],
                    vec![
                        MeshTriangle::new(0, 1, 2),
                        MeshTriangle::new(0, 3, 1),
                        MeshTriangle::new(2, 3, 0),
                        MeshTriangle::new(1, 3, 2),
                    ],
                ));
                part.name(Some("part1".to_string()));
                part.material(Some(1));
                part
            });
            object.add_part({
                let mut part = BambuPart::new(Mesh::new(
                    vec![
                        Vec3::new(0.0, 0.0, 0.0),
                        Vec3::new(-10.0, 0.0, 0.0),
                        Vec3::new(0.0, -10.0, 0.0),
                        Vec3::new(0.0, 0.0, 10.0),
                    ],
                    vec![
                        MeshTriangle::new(0, 1, 2),
                        MeshTriangle::new(0, 3, 1),
                        MeshTriangle::new(2, 3, 0),
                        MeshTriangle::new(1, 3, 2),
                    ],
                ));
                part.name(Some("part2".to_string()));
                part.material(Some(2));
                part
            });
            object
        });
        plate
    });
    bambu.add_filament(BambuFilament::new(
        Color::new(255, 255, 255),
        false,
        pla_matte.clone(),
    ));
    bambu.add_filament(BambuFilament::new(
        Color::new(85, 140, 76),
        false,
        pla_basic.clone(),
    ));
    tokio::fs::create_dir_all("examples/flap/output").await?;
    tokio::fs::write("examples/flap/output/original.3mf", bambu.build()?).await?;
    let mut command = BambuStudioCommand::new();
    command.debug(DebugLevel::Warning);
    command.machine(machine.clone());
    command.process(process.clone());
    command.add_filament(pla_basic.clone());
    command.add_filament(pla_matte.clone());
    command.slice(Slice::AllPlates);
    command.export_3mf(std::path::absolute(Path::new(
        "examples/flap/output/sliced.3mf",
    ))?);
    command.input("examples/flap/output/original.3mf".into());
    command.enable_timelapse();
    command.timelapse_type(1);
    command.run().await?;

    let sliced_out = tokio::fs::read("examples/flap/output/sliced.3mf").await?;
    let mut zip = ZipArchive::new(Cursor::new(sliced_out.as_slice()))?;
    for i in 0..zip.len() {
        let mut file = zip.by_index(i)?;
        if file.name().ends_with("gcode") {
            let mut contents = vec![];
            file.read_to_end(&mut contents)?;
            let path = file.enclosed_name().ok_or_else(|| anyhow!("bad path"))?;
            let path = path.file_name().ok_or_else(|| anyhow!("bad filename"))?;
            tokio::fs::write(Path::new("examples/flap/output/").join(path), contents).await?;
        }
    }
    Ok(())
}
