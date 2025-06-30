#![deny(unused_must_use)]
#![feature(exit_status_error)]
#![allow(dead_code)]
#![allow(unused_mut)]

use patina_3mf::ModelContainer;
use patina_3mf::content_types::ContentTypes;
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
use patina_3mf::relationships::Relationships;
use patina_cad::meshes::mesh::Mesh;

pub struct BambuPart {
    mesh: Mesh,
    material: usize,
}

pub struct BambuObject {
    parts: Vec<BambuPart>,
}

pub struct BambuPlate {
    objects: Vec<BambuObject>,
}

pub struct BambuMaterial {}
pub struct BambuBuilder {
    plates: Vec<BambuPlate>,
    materials: Vec<BambuMaterial>,
}

impl BambuPart {
    pub fn new(material: usize, mesh: Mesh) -> Self {
        BambuPart { mesh, material }
    }
}

impl BambuObject {
    pub fn new() -> Self {
        BambuObject { parts: vec![] }
    }
    pub fn add_part(&mut self, part: BambuPart) {
        self.parts.push(part);
    }
}

impl BambuPlate {
    pub fn new() -> Self {
        BambuPlate { objects: vec![] }
    }
    pub fn add_object(&mut self, obj: BambuObject) {
        self.objects.push(obj);
    }
}

impl BambuBuilder {
    pub fn new() -> Self {
        BambuBuilder {
            plates: vec![],
            materials: vec![],
        }
    }
    pub fn add_plate(&mut self, p: BambuPlate) {
        self.plates.push(p);
    }
    pub async fn build(self) -> anyhow::Result<()> {
        let mut application_metadata = ModelMetadata::new("Application".to_string())
            .value(Some("BambuStudio-01.10.01.50".to_string()));
        let mut model_objects = vec![];
        let mut model_items = vec![];
        let mut object_settings = vec![];
        let mut plate_settings = vec![];
        let mut assemble_items = vec![];
        for (plate_id, plate) in self.plates.iter().enumerate() {
            let mut model_instances = vec![];
            for object in &plate.objects {
                let mut components = vec![];
                let mut part_settings = vec![];
                for part in &object.parts {
                    let vertices = part
                        .mesh
                        .vertices()
                        .iter()
                        .map(|v| ModelVertex::new(v.x(), v.y(), v.z()))
                        .collect();
                    let triangles = part
                        .mesh
                        .triangles()
                        .iter()
                        .map(|tri| {
                            ModelTriangle::new(
                                tri.vertices()[0],
                                tri.vertices()[1],
                                tri.vertices()[2],
                            )
                        })
                        .collect();
                    let mesh = ModelMesh::new(
                        ModelVertices::new(vertices),
                        ModelTriangles::new(triangles),
                    );
                    let part_id = model_objects.len() + 1;
                    model_objects.push(
                        ModelObject::new(part_id)
                            .mesh(Some(mesh))
                            .object_type(Some(ModelObjectType::Model)),
                    );
                    components.push(ModelComponent::new(part_id));
                    part_settings.push(
                        Part::new(part_id.to_string())
                            .subtype("normal_part".to_string())
                            .metadata(vec![
                                SettingsMetadata::new("name".to_string())
                                    .value(Some("???".to_string())),
                                SettingsMetadata::new("extruder".to_string())
                                    .value(Some(part.material.to_string())),
                            ]),
                    );
                }
                let object_id = model_objects.len() + 1;
                model_objects.push(
                    ModelObject::new(object_id)
                        .object_type(Some(ModelObjectType::Model))
                        .components(Some(ModelComponents::new(components))),
                );
                model_items.push(ModelItem::new(object_id).printable(Some(true)));
                object_settings.push(
                    ObjectSettings::new(object_id.to_string())
                        .metadata(vec![
                            SettingsMetadata::new("name".to_string())
                                .value(Some("!?!!?".to_string())),
                            SettingsMetadata::new("extruder".to_string())
                                .value(Some("1".to_string())),
                        ])
                        .part(part_settings),
                );
                model_instances.push(ModelInstance::new().metadata(vec![
                        SettingsMetadata::new("object_id".to_string())
                            .value(Some(object_id.to_string())),
                    ]));
                assemble_items.push(AssembleItem::new(object_id.to_string()));
            }
            plate_settings.push(
                Plate::new()
                    .metadata(vec![
                        SettingsMetadata::new("plater_id".to_string())
                            .value(Some("1".to_string())),
                    ])
                    .model_instance(model_instances),
            );
        }
        let model = Model::new()
            .metadata(vec![application_metadata])
            .resources(ModelResources::new().object(model_objects))
            .build(ModelBuild::new().item(model_items))
            .unit(ModelUnit::Millimeter);
        let content_types = ContentTypes::minimal();
        let relationships = Relationships::minimal();
        let model_settings = ModelSettings::new()
            .object(object_settings)
            .plate(plate_settings)
            .assemble(Some(Assemble::new().assemble_item(assemble_items)));
        let project_settings = ProjectSettings::new()
            .filament_colour(Some(vec![
                "#0000FF".to_string(),
                "#FFFFFF".to_string(),
                "#8E9089".to_string(),
                "#000000".to_string(),
                "#000000".to_string(),
            ]))
            .filament_is_support(Some(vec![false, false, false, false, false]))
            .filament_settings_id(Some(vec![
                "Bambu PLA Basic @BBL A1M".to_string(),
                "Bambu PLA Basic @BBL A1M".to_string(),
                "Bambu PLA Basic @BBL A1M".to_string(),
                "Bambu PLA Basic @BBL A1M".to_string(),
                "Bambu PLA Basic @BBL A1M".to_string(),
            ]))
            .filament_type(Some(vec![
                "PLA".to_string(),
                "PLA".to_string(),
                "PLA".to_string(),
                "PLA".to_string(),
                "PLA".to_string(),
            ]))
            .flush_volumes_matrix(Some(vec![
                0.0, 100.0, 100.0, 100.0, 100.0, //
                100.0, 0.0, 100.0, 100.0, 100.0, //
                100.0, 100.0, 0.0, 100.0, 100.0, //
                100.0, 100.0, 100.0, 0.0, 100.0, //
                100.0, 100.0, 100.0, 100.0, 0.0, //
            ]))
            .nozzle_diameter(Some(vec![0.4]))
            .print_settings_id(Some("0.20mm Standard @BBL A1M".to_string()))
            .printable_height(Some(180.0))
            .printer_settings_id(Some("Bambu Lab A1 mini 0.4 nozzle".to_string()))
            .enable_prime_tower(Some(true))
            .wipe_tower_x(Some(50.0))
            .wipe_tower_y(Some(50.0));

        let model_cont = ModelContainer::new(model)
            .content_types(Some(content_types))
            .relationships(Some(relationships))
            .model_settings(Some(model_settings))
            .project_settings(Some(project_settings));

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
        let mut slicer = tokio::process::Command::new(
            "/Applications/BambuStudio.app/Contents/MacOS/BambuStudio",
        );
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
}
