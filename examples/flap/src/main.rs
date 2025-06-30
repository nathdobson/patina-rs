#![feature(exit_status_error)]
#![deny(unused_must_use)]
#![allow(unused_mut)]
#![allow(unused_imports)]
#![allow(dead_code)]

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
use patina_bambu::{BambuBuilder, BambuFilament, BambuObject, BambuPart, BambuPlate};
use patina_cad::math::vec2::Vec2;
use patina_cad::math::vec3::Vec3;
use patina_cad::meshes::mesh::Mesh;
use patina_cad::meshes::mesh_triangle::MeshTriangle;
use std::io::{Cursor, Read, Write};
use std::path::Path;
use zip::write::{FileOptions, SimpleFileOptions};
use zip::{ZipArchive, ZipWriter};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    build_output().await?;
    Ok(())
}

async fn filter() -> anyhow::Result<()> {
    let mut output = vec![];
    let mut writer = ZipWriter::new(Cursor::new(&mut output));
    let file = tokio::fs::read("examples/flap/input.3mf").await?;
    let mut zip = ZipArchive::new(Cursor::new(file.as_slice()))?;
    for i in 0..zip.len() {
        let mut file = zip.by_index(i)?;
        match file.name() {
            "[Content_Types].xml"
            // | "Metadata/plate_1.png"
            // | "Metadata/plate_1_small.png"
            // | "Metadata/plate_no_light_1.png"
            // | "Metadata/top_1.png"
            // | "Metadata/pick_1.png"
            // | "Metadata/plate_1.json"
            | "3D/3dmodel.model"
            | "3D/_rels/3dmodel.model.rels"
            | "3D/Objects/object_1.model"
            | "Metadata/project_settings.config"
            // | "Metadata/filament_settings_1.config"
            | "Metadata/model_settings.config"
            // | "Metadata/cut_information.xml"
            // | "Metadata/slice_info.config"
            | "_rels/.rels" => {}
            _ => {
                println!("| {:?}", file.name());
                continue;
            }
        }
        if file.is_file() {
            writer.start_file(file.name(), SimpleFileOptions::default())?;
            let mut buf = vec![];
            file.read_to_end(&mut buf)?;
            writer.write_all(&buf)?;
        } else if file.is_dir() {
            writer.add_directory(file.name(), SimpleFileOptions::default())?;
        }
    }
    writer.finish()?;
    tokio::fs::write("examples/flap/filtered.3mf", &output).await?;
    Ok(())
}

async fn build_output() -> anyhow::Result<()> {
    let mut bambu = BambuBuilder::new();
    let printer = Printer::A1Mini;
    let nozzle = Nozzle::Nozzle0_4;
    bambu.printer_settings_id(Some(PrinterSettingsId::new(
        printer.clone(),
        Some(nozzle.clone()),
    )));
    bambu.print_settings_id(Some(PrintSettingsId::new(
        0.2,
        PrintQuality::Standard,
        printer,
        nozzle,
    )));
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
    for color in [
        Color::new(0, 0, 255),
        Color::new(0, 255, 0),
        Color::new(0, 255, 255),
        Color::new(255, 0, 0),
        Color::new(255, 0, 255),
    ] {
        bambu.add_filament(BambuFilament::new(
            color,
            false,
            FilamentSettingsId::new(
                FilamentBrand::Bambu,
                FilamentMaterial::PlaBasic,
                Printer::A1Mini,
                None,
            ),
        ));
    }
    bambu.build().await?;
    Ok(())
}
