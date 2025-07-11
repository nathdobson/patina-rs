#![feature(exit_status_error)]
#![deny(unused_must_use)]
#![allow(unused_mut)]
#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(unreachable_code)]
#![allow(unused_variables)]

use anyhow::anyhow;
use patina_3mf::ModelContainer;
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
use patina_3mf::project_settings::color::Color;
use patina_3mf::project_settings::support_interface_pattern::SupportInterfacePattern;
use patina_3mf::project_settings::support_style::SupportStyle;
use patina_3mf::project_settings::support_type::SupportType;
use patina_3mf::relationships::{Relationship, Relationships};
use patina_3mf::settings_id::filament_settings_id::{
    FilamentBrand, FilamentMaterial, FilamentSettingsId,
};
use patina_3mf::settings_id::nozzle::Nozzle;
use patina_3mf::settings_id::print_settings_id::{PrintQuality, PrintSettingsId};
use patina_3mf::settings_id::printer::Printer;
use patina_3mf::settings_id::printer_settings_id::PrinterSettingsId;
use patina_bambu::cli::{BambuStudioCommand, DebugLevel, Slice};
use patina_bambu::{BambuBuilder, BambuFilament, BambuObject, BambuPart, BambuPlate, BambuSupport};
use patina_geo::geo3::aabb::Aabb;
use patina_mesh::ser::stl::{write_stl, write_stl_file};
use patina_sdf::marching_mesh::MarchingMesh;
use patina_sdf::sdf::Sdf;
use patina_sdf::sdf::leaf::SdfLeafImpl;
use patina_vec::mat4::Mat4;
use patina_vec::vec2::Vec2;
use patina_vec::vec3::Vec3;
use rand::rng;
use rusttype::{Font, OutlineBuilder, Point, Scale};
use std::collections::HashSet;
use std::env;
use std::io::{Cursor, Read, Write};
use std::path::Path;
use std::time::Instant;
use tokio::fs;
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
    font: Font<'static>,
}

impl LetterBuilder {
    fn blank(&self, thickness: f64) -> Sdf {
        let mut sdf = Aabb::new(
            Vec3::new(-self.width / 2.0, 0.0, 0.0),
            Vec3::new(self.width / 2.0, self.length, thickness),
        )
        .into_sdf();
        sdf = sdf.difference(
            &Aabb::new(
                Vec3::new(self.width / 2.0 - self.incut, -1.0, -self.thickness),
                Vec3::new(self.width, self.extension, self.thickness * 2.0),
            )
            .into_sdf(),
        );
        sdf = sdf.difference(
            &Aabb::new(
                Vec3::new(-self.width, -1.0, -self.thickness),
                Vec3::new(
                    -self.width / 2.0 + self.incut,
                    self.extension,
                    self.thickness * 2.0,
                ),
            )
            .into_sdf(),
        );
        sdf = sdf.difference(
            &Aabb::new(
                Vec3::new(
                    self.width / 2.0 - self.incut,
                    self.extension + self.axle_diameter,
                    -self.thickness,
                ),
                Vec3::new(self.width, self.drum_diameter, self.thickness * 2.0),
            )
            .into_sdf(),
        );
        sdf = sdf.difference(
            &Aabb::new(
                Vec3::new(
                    -self.width,
                    self.extension + self.axle_diameter,
                    -self.thickness,
                ),
                Vec3::new(
                    -self.width / 2.0 + self.incut,
                    self.drum_diameter,
                    self.thickness * 2.0,
                ),
            )
            .into_sdf(),
        );
        sdf
    }
    fn letter(&self) -> Sdf {
        struct Builder;
        impl OutlineBuilder for Builder {
            fn move_to(&mut self, x: f32, y: f32) {
                println!("move_to({x}, {y})");
            }

            fn line_to(&mut self, x: f32, y: f32) {
                println!("line_to({x}, {y})");
            }

            fn quad_to(&mut self, x1: f32, y1: f32, x: f32, y: f32) {
                println!("quad_to({x1}, {y1}, {x}, {y})");
            }

            fn curve_to(&mut self, x1: f32, y1: f32, x2: f32, y2: f32, x: f32, y: f32) {
                println!("curve_to({x1}, {y1}, {x2}, {y2}, {x}, {y})");
            }

            fn close(&mut self) {
                println!("close");
            }
        }
        let mut builder = Builder;
        let glyph = self
            .font
            .glyph(self.letter)
            .scaled(Scale::uniform(10.0))
            .build_outline(&mut builder);
        todo!();
    }
    pub async fn build(&self) -> Vec<BambuPart> {
        let sdf = self.blank(self.thickness);
        let letter = self.letter();
        let start = Instant::now();

        let mesh = MarchingMesh::new(Aabb::new(
            Vec3::new(-self.width * 1.01, -self.width * 1.01, -0.01),
            Vec3::new(self.width * 1.01, self.width * 1.01, 40.01),
        ))
        .build(&sdf);
        println!("Built mesh in {}", start.elapsed().as_secs_f64());
        write_stl_file(
            &mesh,
            Path::new(&format!("examples/flap/output/flap_{}.stl", self.letter)),
        )
        .await
        .unwrap();
        let mut body = BambuPart::new(mesh);
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

        // let mut support = BambuPart::new(self.blank(self.support_thickness));
        // support.material(Some(3));
        // support.name(Some(format!("support({})", self.letter)));
        // support.transform(Some(
        //     Mat4::translate(Vec3::new(
        //         90.0,
        //         90.0 - self.length / 2.0,
        //         (self.index as f64) * (self.thickness + self.support_thickness),
        //     ))
        //     .as_affine()
        //     .unwrap(),
        // ));

        vec![body]
    }
}

async fn build_output() -> anyhow::Result<()> {
    let font =
        Font::try_from_vec(fs::read("/System/Library/Fonts/Supplemental/Phosphate.ttc").await?)
            .ok_or_else(|| anyhow!("bad font"))?;

    let mut bambu = BambuBuilder::new();
    let printer = Printer::A1Mini;
    let nozzle = Nozzle::Nozzle0_4;
    let mut machine = PrinterSettingsId::new(printer.clone());
    machine.nozzle = Some(nozzle.clone());
    let process = PrintSettingsId::new(0.2, PrintQuality::Standard, printer.clone(), nozzle);
    let pla_basic = FilamentSettingsId::new(
        FilamentBrand::Bambu,
        FilamentMaterial::PlaBasic,
        printer.clone(),
    );
    let pla_matte = FilamentSettingsId::new(
        FilamentBrand::Bambu,
        FilamentMaterial::PlaMatte,
        printer.clone(),
    );
    let pla_support = FilamentSettingsId::new(
        FilamentBrand::Bambu,
        FilamentMaterial::SupportForPla,
        printer.clone(),
    );

    bambu.printer_settings_id(Some(machine.clone()));
    bambu.print_settings_id(Some(process.clone()));
    bambu.prime_tower_positions(Some(vec![Vec2::new(5.0, 5.0)]));
    bambu.support({
        let mut support = BambuSupport::new();
        support.independent_support_layer_height(0);
        support.support_bottom_z_distance(0);
        support.support_filament(3);
        support.support_interface_filament(3);
        support.support_interface_pattern(SupportInterfacePattern::RectilinearInterlaced);
        support.support_interface_spacing(0);
        support.support_style(SupportStyle::Snug);
        support.support_top_z_distance(0);
        support.support_type(SupportType::NormalAuto);
        support
    });
    bambu.add_plate({
        let mut plate = BambuPlate::new();
        let mut object = BambuObject::new();
        object.name(Some("stack".to_string()));
        for (index, letter) in ['a', 'b'].iter().enumerate() {
            let parts = LetterBuilder {
                index,
                letter: *letter,
                width: 43.0,
                length: 35.0,
                thickness: 1.0,
                support_thickness: 0.2,
                incut: 2.0,
                extension: 1.2,
                axle_diameter: 1.2,
                drum_diameter: 18.0,
                font: font.clone(),
            }
            .build()
            .await;
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
