use crate::{BambuBuilder, BambuFilament, BambuObject, BambuPart, BambuPartType, BambuPlate};
use patina_3mf::model::resources::ModelObjectType;
use patina_3mf::model_settings::PartSubtype;
use patina_3mf::project_settings::color::Color;
use patina_3mf::settings_id::filament_settings_id::{
    FilamentBrand, FilamentMaterial, FilamentSettingsId,
};
use patina_3mf::settings_id::nozzle::Nozzle;
use patina_3mf::settings_id::print_settings_id::{PrintQuality, PrintSettingsId};
use patina_3mf::settings_id::printer::Printer;
use patina_3mf::settings_id::printer_settings_id::PrinterSettingsId;
use patina_geo::geo3::aabb3::Aabb3;
use patina_mesh::mesh::Mesh;
use patina_mesh::ser::create_test_path;
use patina_vec::mat4::Mat4;
use patina_vec::vec3::Vec3;
use serde::{Deserialize, Serialize};
use tokio::fs;

#[tokio::test]
async fn test_basic() -> anyhow::Result<()> {
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
    bambu.printer_settings_id(Some(machine.clone()));
    bambu.print_settings_id(Some(process.clone()));
    bambu.add_filament({
        let mut filament = BambuFilament::new();
        filament.color(Some(Color::new(128, 255, 128)));
        filament.support(Some(false));
        filament.settings_id(Some(pla_basic.clone()));
        filament.diameter(Some(1.75));
        filament.shrink(Some("100%".to_string()));
        filament
    });
    bambu.add_plate({
        let mut plate = BambuPlate::new();
        plate.add_object({
            let mut obj = BambuObject::new();
            obj.name(Some("obj".to_string()));
            obj.add_part({
                let cube = Mesh::from_aabb(Aabb3::new(Vec3::splat(0.0), Vec3::splat(10.0)));
                let mut part = BambuPart::new(cube);
                part.transform(Some(
                    Mat4::translate(Vec3::new(90.0, 90.0, 0.0))
                        .as_affine()
                        .unwrap(),
                ));
                part.name(Some("model".to_string()));
                part
            });
            obj.add_part({
                let cube = Mesh::from_aabb(Aabb3::new(Vec3::splat(0.0), Vec3::splat(10.0)));
                let mut part = BambuPart::new(cube);
                part.transform(Some(
                    Mat4::translate(Vec3::new(95.0, 95.0, 0.0))
                        .as_affine()
                        .unwrap(),
                ));
                part.typ(BambuPartType::Modifier);
                part.name(Some("modifier".to_string()));
                part
            });
            obj
        });
        plate
    });
    fs::write(create_test_path("bambu.3mf").await?, bambu.build()?).await?;
    Ok(())
}
