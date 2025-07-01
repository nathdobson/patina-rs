#![deny(unused_must_use)]
#![feature(exit_status_error)]
#![allow(dead_code)]
#![allow(unused_mut)]
#![allow(unused_imports)]
#![allow(unused_variables)]

pub mod cli;

use patina_3mf::ModelContainer;
use patina_3mf::color::Color;
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
use patina_3mf::settings_id::filament_settings_id::{
    FilamentBrand, FilamentMaterial, FilamentSettingsId,
};
use patina_3mf::settings_id::nozzle::Nozzle;
use patina_3mf::settings_id::print_settings_id::{PrintQuality, PrintSettingsId};
use patina_3mf::settings_id::printer::Printer;
use patina_3mf::settings_id::printer_settings_id::PrinterSettingsId;
use patina_cad::math::vec2::Vec2;
use patina_cad::meshes::mesh::Mesh;

#[test]
fn nothing() {}

pub struct BambuPart {
    mesh: Mesh,
    name: Option<String>,
    material: Option<usize>,
    transform: Option<[f64; 12]>,
}

pub struct BambuObject {
    name: Option<String>,
    parts: Vec<BambuPart>,
    transform: Option<[f64; 12]>,
}

pub struct BambuPlate {
    objects: Vec<BambuObject>,
}

pub struct BambuFilament {
    color: Color,
    support: bool,
    settings_id: FilamentSettingsId,
}
pub struct BambuBuilder {
    printer_settings_id: Option<PrinterSettingsId>,
    print_settings_id: Option<PrintSettingsId>,
    plates: Vec<BambuPlate>,
    filaments: Vec<BambuFilament>,
    prime_tower_position: Option<Vec2>,
}

impl BambuFilament {
    pub fn new(color: Color, support: bool, settings_id: FilamentSettingsId) -> BambuFilament {
        BambuFilament {
            color,
            support,
            settings_id,
        }
    }
}

impl BambuPart {
    pub fn new(mesh: Mesh) -> Self {
        BambuPart {
            name: None,
            mesh,
            material: None,
            transform: None,
        }
    }
    pub fn name(&mut self, name: Option<String>) {
        self.name = name;
    }
    pub fn material(&mut self, material: Option<usize>) {
        self.material = material;
    }
    pub fn transform(&mut self, transform: Option<[f64; 12]>) {
        self.transform = transform;
    }
}

impl BambuObject {
    pub fn new() -> Self {
        BambuObject {
            name: None,
            parts: vec![],
            transform: None,
        }
    }
    pub fn transform(&mut self, transform: Option<[f64; 12]>) {
        self.transform = transform;
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
            printer_settings_id: None,
            print_settings_id: None,
            plates: vec![],
            filaments: vec![],
            prime_tower_position: None,
        }
    }
    pub fn add_plate(&mut self, p: BambuPlate) {
        self.plates.push(p);
    }
    pub fn add_filament(&mut self, m: BambuFilament) {
        self.filaments.push(m);
    }
    pub fn printer_settings_id(&mut self, id: Option<PrinterSettingsId>) {
        self.printer_settings_id = id;
    }
    pub fn print_settings_id(&mut self, id: Option<PrintSettingsId>) {
        self.print_settings_id = id;
    }
    pub fn prime_tower_position(&mut self, position: Option<Vec2>) {
        self.prime_tower_position = position;
    }
    pub fn build(self) -> anyhow::Result<Vec<u8>> {
        let application_metadata = ModelMetadata::new("Application".to_string())
            .value(Some("BambuStudio-02.01.01.52".to_string()));
        let version =
            ModelMetadata::new("BambuStudio:3mfVersion".to_string()).value(Some("1".to_string()));
        let mut model_objects = vec![];
        let mut model_items = vec![];
        let mut object_settings = vec![];
        let mut plate_settings = vec![];
        let mut assemble_items = vec![];
        for (plate_id_z, plate) in self.plates.iter().enumerate() {
            let plate_id = plate_id_z + 1;
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
                    components.push(ModelComponent::new(part_id).transform(part.transform));
                    part_settings.push(
                        Part::new(part_id.to_string())
                            .subtype("normal_part".to_string())
                            .metadata(vec![
                                SettingsMetadata::new("name".to_string()).value(part.name.clone()),
                                SettingsMetadata::new("extruder".to_string())
                                    .value(part.material.map(|x| x.to_string())),
                            ]),
                    );
                }
                let object_id = model_objects.len() + 1;
                model_objects.push(
                    ModelObject::new(object_id)
                        .object_type(Some(ModelObjectType::Model))
                        .components(Some(ModelComponents::new(components))),
                );
                model_items.push(
                    ModelItem::new(object_id)
                        .transform(object.transform)
                        .printable(Some(true)),
                );
                object_settings.push(
                    ObjectSettings::new(object_id.to_string())
                        .metadata(vec![
                            SettingsMetadata::new("name".to_string()).value(object.name.clone()),
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
                            .value(Some(plate_id.to_string())),
                    ])
                    .model_instance(model_instances),
            );
        }
        let model = Model::new()
            .metadata(vec![application_metadata, version])
            .resources(ModelResources::new().object(model_objects))
            .build(ModelBuild::new().item(model_items))
            .unit(ModelUnit::Millimeter);
        let content_types = ContentTypes::minimal();
        let relationships = Relationships::minimal();
        let model_settings = ModelSettings::new()
            .object(object_settings)
            .plate(plate_settings)
            .assemble(Some(Assemble::new().assemble_item(assemble_items)));
        let filament_color = self.filaments.iter().map(|x| x.color.clone()).collect();
        let filament_is_support = self.filaments.iter().map(|x| x.support.clone()).collect();
        let filament_settings_id = self
            .filaments
            .iter()
            .map(|x| x.settings_id.clone())
            .collect();
        let flush_volumes_matrix: Vec<_> = (0..self.filaments.len())
            .flat_map(|f1| {
                (0..self.filaments.len()).map(move |f2| if f1 == f2 { 0.0 } else { 100.0 })
            })
            .collect();
        let project_settings = ProjectSettings::new()
            .filament_colour(Some(filament_color))
            .filament_is_support(Some(filament_is_support))
            .filament_settings_id(Some(filament_settings_id))
            .flush_volumes_matrix(Some(flush_volumes_matrix))
            .nozzle_diameter(Some(vec![0.4]))
            .print_settings_id(self.print_settings_id.clone())
            .printable_height(Some(180.0))
            .printer_settings_id(self.printer_settings_id.clone())
            .enable_prime_tower(Some(true))
            .wipe_tower_x(self.prime_tower_position.map(|p| p.x()))
            .wipe_tower_y(self.prime_tower_position.map(|p| p.y()))
            .enable_timelapse(Some(true))
            .timelapse_type(Some(1));

        let model_cont = ModelContainer::new(model)
            .content_types(Some(content_types))
            .relationships(Some(relationships))
            .model_settings(Some(model_settings))
            .project_settings(Some(project_settings));

        let encoded = model_cont.encode()?;
        Ok(encoded)
    }
}
