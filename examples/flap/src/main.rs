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
use patina_cad::geo3::aabb::AABB;
use patina_cad::math::mat4::Mat4;
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

pub struct LetterBuilder {
    index: usize,
    letter: char,
    width: f64,
    length: f64,
    thickness: f64,
    support_thickness: f64,
    incut: f64,
    extension: f64,
    axle_diameter: f64,
    drum_diameter: f64,
}

impl LetterBuilder {
    fn blank(&self, thickness: f64) -> Mesh {
        let mut mesh = AABB::new(
            Vec3::new(-self.width / 2.0, 0.0, 0.0),
            Vec3::new(self.width / 2.0, self.length, thickness),
        )
        .as_mesh();
        mesh
    }
    pub fn build(&self) -> Vec<BambuPart> {
        let mut body = BambuPart::new(self.blank(self.thickness));
        body.material(Some(2));
        body.name(Some(format!("part({})", self.letter)));
        body.transform(Some(
            Mat4::translate(Vec3::new(
                90.0,
                90.0 - self.length / 2.0,
                (self.index as f64) * (self.thickness + self.support_thickness)
                    + self.support_thickness,
            ))
            .as_affine()
            .unwrap(),
        ));

        let mut support = BambuPart::new(self.blank(self.support_thickness));
        support.material(Some(3));
        support.name(Some(format!("support({})", self.letter)));
        support.transform(Some(
            Mat4::translate(Vec3::new(
                90.0,
                90.0 - self.length / 2.0,
                (self.index as f64) * (self.thickness + self.support_thickness),
            ))
            .as_affine()
            .unwrap(),
        ));

        vec![support, body]
    }
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
    let pla_support = FilamentSettingsId::new(
        FilamentBrand::Bambu,
        FilamentMaterial::SupportForPla,
        printer.clone(),
        None,
    );

    bambu.printer_settings_id(Some(machine.clone()));
    bambu.print_settings_id(Some(process.clone()));
    bambu.prime_tower_positions(Some(vec![Vec2::new(5.0, 5.0)]));
    bambu.add_plate({
        let mut plate = BambuPlate::new();
        let mut object = BambuObject::new();
        object.name(Some("stack".to_string()));
        for (index, letter) in ['a', 'b'].iter().enumerate() {
            let parts = LetterBuilder {
                index,
                letter: *letter,
                width: 39.0,
                length: 35.0,
                thickness: 1.0,
                support_thickness: 0.2,
                incut: 2.0,
                extension: 1.2,
                axle_diameter: 1.2,
                drum_diameter: 18.0,
            }
            .build();
            for part in parts {
                object.add_part(part);
            }
        }
        plate.add_object(object);
        plate
    });
    bambu.add_filament({
        let mut filament = BambuFilament::new();
        filament.color(Some(Color::new(255, 255, 255)));
        filament.support(Some(false));
        filament.settings_id(Some(pla_matte.clone()));
        filament.diameter(Some(1.75));
        filament.shrink(Some("100%".to_string()));
        filament
    });
    bambu.add_filament({
        let mut filament = BambuFilament::new();
        filament.color(Some(Color::new(85, 140, 76)));
        filament.support(Some(false));
        filament.settings_id(Some(pla_basic.clone()));
        filament.diameter(Some(1.75));
        filament.shrink(Some("100%".to_string()));
        filament
    });
    bambu.add_filament({
        let mut filament = BambuFilament::new();
        filament.color(Some(Color::new(255, 255, 255)));
        filament.support(Some(true));
        filament.settings_id(Some(pla_support.clone()));
        filament.diameter(Some(1.75));
        filament.shrink(Some("100%".to_string()));
        filament
    });
    // bambu.add_filament(BambuFilament::new().color(Color::new(255, 255, 255))
    //     ,
    //     false,
    //     pla_matte.clone(),
    // ));
    // bambu.add_filament(BambuFilament::new(
    //     Color::new(85, 140, 76),
    //     false,
    //     pla_basic.clone(),
    // ));
    // bambu.add_filament(BambuFilament::new(
    //     Color::new(255, 255, 255),
    //     true,
    //     pla_support.clone(),
    // ));
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
