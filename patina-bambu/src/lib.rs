#![deny(unused_must_use)]
#![feature(exit_status_error)]
#![allow(dead_code)]
#![allow(unused_mut)]
#![allow(unused_imports)]
#![allow(unused_variables)]

pub mod cli;

use itertools::Itertools;
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
use patina_3mf::project_settings::color::Color;
use patina_3mf::project_settings::support_interface_pattern::SupportInterfacePattern;
use patina_3mf::project_settings::support_style::SupportStyle;
use patina_3mf::project_settings::support_type::SupportType;
use patina_3mf::relationships::Relationships;
use patina_3mf::settings_id::filament_settings_id::{
    FilamentBrand, FilamentMaterial, FilamentSettingsId,
};
use patina_3mf::settings_id::nozzle::Nozzle;
use patina_3mf::settings_id::print_settings_id::{PrintQuality, PrintSettingsId};
use patina_3mf::settings_id::printer::Printer;
use patina_3mf::settings_id::printer_settings_id::PrinterSettingsId;
use patina_mesh::mesh::Mesh;
use patina_vec::vec2::Vec2;

#[test]
fn nothing() {}

pub struct BambuSupport {
    independent_support_layer_height: Option<usize>,
    support_bottom_z_distance: Option<usize>,
    support_filament: Option<usize>,
    support_interface_filament: Option<usize>,
    support_interface_pattern: Option<SupportInterfacePattern>,
    support_interface_spacing: Option<usize>,
    support_style: Option<SupportStyle>,
    support_top_z_distance: Option<usize>,
    support_type: Option<SupportType>,
    support_expansion: Option<f64>,
}

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
    color: Option<Color>,
    support: Option<bool>,
    settings_id: Option<FilamentSettingsId>,
    diameter: Option<f64>,
    shrink: Option<String>,
}
pub struct BambuBuilder {
    printer_settings_id: Option<PrinterSettingsId>,
    print_settings_id: Option<PrintSettingsId>,
    plates: Vec<BambuPlate>,
    filaments: Vec<BambuFilament>,
    prime_tower_positions: Option<Vec<Vec2>>,
    support: Option<BambuSupport>,
}

impl BambuSupport {
    pub fn new() -> Self {
        BambuSupport {
            independent_support_layer_height: None,
            support_bottom_z_distance: None,
            support_filament: None,
            support_interface_filament: None,
            support_interface_pattern: None,
            support_interface_spacing: None,
            support_style: None,
            support_top_z_distance: None,
            support_type: None,
            support_expansion: None,
        }
    }
    pub fn independent_support_layer_height(&mut self, layer_height: usize) {
        self.independent_support_layer_height = Some(layer_height);
    }
    pub fn support_bottom_z_distance(&mut self, support_bottom_z_distance: usize) {
        self.support_bottom_z_distance = Some(support_bottom_z_distance);
    }
    pub fn support_filament(&mut self, support_filament: usize) {
        self.support_filament = Some(support_filament);
    }
    pub fn support_interface_filament(&mut self, support_interface_filament: usize) {
        self.support_interface_filament = Some(support_interface_filament);
    }
    pub fn support_interface_pattern(
        &mut self,
        support_interface_pattern: SupportInterfacePattern,
    ) {
        self.support_interface_pattern = Some(support_interface_pattern);
    }
    pub fn support_interface_spacing(&mut self, support_interface_spacing: usize) {
        self.support_interface_spacing = Some(support_interface_spacing);
    }
    pub fn support_style(&mut self, support_style: SupportStyle) {
        self.support_style = Some(support_style);
    }
    pub fn support_top_z_distance(&mut self, support_top_z_distance: usize) {
        self.support_top_z_distance = Some(support_top_z_distance);
    }
    pub fn support_type(&mut self, support_type: SupportType) {
        self.support_type = Some(support_type);
    }
    pub fn support_expansion(&mut self, support_expansion: f64) {
        self.support_expansion = Some(support_expansion);
    }
}

impl BambuFilament {
    pub fn new() -> BambuFilament {
        BambuFilament {
            color: None,
            support: None,
            settings_id: None,
            diameter: None,
            shrink: None,
        }
    }
    pub fn color(&mut self, color: Option<Color>) {
        self.color = color;
    }
    pub fn support(&mut self, support: Option<bool>) {
        self.support = support;
    }
    pub fn settings_id(&mut self, settings_id: Option<FilamentSettingsId>) {
        self.settings_id = settings_id;
    }
    pub fn diameter(&mut self, diameter: Option<f64>) {
        self.diameter = diameter;
    }
    pub fn shrink(&mut self, shrink: Option<String>) {
        self.shrink = shrink;
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
    pub fn name(&mut self, name: Option<String>) {
        self.name = name;
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
            prime_tower_positions: None,
            support: None,
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
    pub fn prime_tower_positions(&mut self, position: Option<Vec<Vec2>>) {
        self.prime_tower_positions = position;
    }
    pub fn support(&mut self, support: BambuSupport) {
        self.support = Some(support);
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
        let filament_shrink = self.filaments.iter().map(|x| x.shrink.clone()).collect();
        let filament_diameter = self.filaments.iter().map(|x| x.diameter.clone()).collect();
        let flush_volumes_matrix: Vec<_> = (0..self.filaments.len())
            .flat_map(|f1| {
                (0..self.filaments.len()).map(move |f2| if f1 == f2 { 0.0 } else { 100.0 })
            })
            .collect();
        let mut project_settings = ProjectSettings::new();
        project_settings.filament_colour = Some(filament_color);
        project_settings.filament_is_support = Some(filament_is_support);
        project_settings.filament_settings_id = Some(filament_settings_id);
        project_settings.filament_shrink = Some(filament_shrink);
        project_settings.filament_diameter = Some(filament_diameter);
        project_settings.flush_volumes_matrix = Some(flush_volumes_matrix);
        project_settings.nozzle_diameter = Some(vec![0.4]);
        project_settings.print_settings_id = self.print_settings_id.clone();
        project_settings.printable_height = Some(180.0);
        project_settings.printer_settings_id = self.printer_settings_id.clone();
        project_settings.wipe_tower_x = self
            .prime_tower_positions
            .as_ref()
            .map(|ps| ps.iter().map(|p| p.x()).collect());
        project_settings.wipe_tower_y = self
            .prime_tower_positions
            .as_ref()
            .map(|ps| ps.iter().map(|p| p.y()).collect());
        let mut different_settings_to_system = vec![];
        if let Some(support) = self.support {
            project_settings.enable_support = Some(true);
            different_settings_to_system.push("enable_support");
            if let Some(independent_support_layer_height) = support.independent_support_layer_height
            {
                project_settings.independent_support_layer_height =
                    Some(independent_support_layer_height);
                different_settings_to_system.push("independent_support_layer_height");
            }
            if let Some(support_bottom_z_distance) = support.support_bottom_z_distance {
                project_settings.support_bottom_z_distance = Some(support_bottom_z_distance);
                different_settings_to_system.push("support_bottom_z_distance");
            }

            if let Some(support_bottom_z_distance) = support.support_bottom_z_distance {
                project_settings.support_bottom_z_distance = Some(support_bottom_z_distance);
                different_settings_to_system.push("support_bottom_z_distance");
            }
            if let Some(support_filament) = support.support_filament {
                project_settings.support_filament = Some(support_filament);
                different_settings_to_system.push("support_filament");
            }
            if let Some(support_filament) = support.support_filament {
                project_settings.support_filament = Some(support_filament);
                different_settings_to_system.push("support_filament");
            }
            if let Some(support_interface_filament) = support.support_interface_filament {
                project_settings.support_interface_filament = Some(support_interface_filament);
                different_settings_to_system.push("support_interface_filament");
            }
            if let Some(support_interface_pattern) = support.support_interface_pattern {
                project_settings.support_interface_pattern = Some(support_interface_pattern);
                different_settings_to_system.push("support_interface_pattern");
            }
            if let Some(support_interface_spacing) = support.support_interface_spacing {
                project_settings.support_interface_spacing = Some(support_interface_spacing);
                different_settings_to_system.push("support_interface_spacing");
            }
            if let Some(support_style) = support.support_style {
                project_settings.support_style = Some(support_style);
                different_settings_to_system.push("support_style");
            }
            if let Some(support_top_z_distance) = support.support_top_z_distance {
                project_settings.support_top_z_distance = Some(support_top_z_distance);
                different_settings_to_system.push("support_top_z_distance");
            }
            if let Some(support_type) = support.support_type {
                project_settings.support_type = Some(support_type);
                different_settings_to_system.push("support_type");
            }
            if let Some(support_expansion) = support.support_expansion {
                project_settings.support_expansion = Some(support_expansion);
                different_settings_to_system.push("support_expansion");
            }
        }

        project_settings.different_settings_to_system = Some(vec![
            different_settings_to_system.into_iter().join(";"),
            "".to_string(),
            "".to_string(),
            "".to_string(),
        ]);

        let mut model_cont = ModelContainer::new(model);
        model_cont.content_types = Some(content_types);
        model_cont.relationships = Some(relationships);
        model_cont.model_settings = Some(model_settings);
        model_cont.project_settings = Some(project_settings);

        let encoded = model_cont.encode()?;
        Ok(encoded)
    }
}
