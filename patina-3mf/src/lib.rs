#![allow(dead_code, unused_imports, unused_mut)]

mod bool_as_int;
pub mod content_types;
pub mod model;
pub mod model_settings;
pub mod relationships;
pub mod xmlns;

use crate::content_types::ContentTypes;
use crate::model::Model;
use crate::model_settings::ModelSettings;
use crate::relationships::Relationships;
use serde::{Deserialize, Serialize};
#[deny(unused_must_use)]
use std::io::{Cursor, Write};
use xml::EmitterConfig;
use zip::ZipWriter;
use zip::write::SimpleFileOptions;

#[non_exhaustive]
pub struct ModelContainer {
    pub model: Model,
    pub content_types: Option<ContentTypes>,
    pub relationships: Option<Relationships>,
    pub model_settings: Option<ModelSettings>,
}

const MODEL_CONFIG: &[u8] = //
    br#"<?xml version="1.0" encoding="UTF-8"?>
<config>
  <object id="9">
    <metadata key="name" value="part1"/>
    <metadata key="extruder" value="1"/>
    <part id="1" subtype="normal_part">
      <metadata key="name" value="part1.stl"/>
      <metadata key="extruder" value="1"/>
    </part>
    <part id="2" subtype="normal_part">
      <metadata key="name" value="part2.stl"/>
      <metadata key="extruder" value="2"/>
    </part>
  </object>
  <plate>
    <metadata key="plater_id" value="1"/>
    <model_instance>
      <metadata key="object_id" value="9"/>
    </model_instance>
  </plate>
  <assemble>
   <assemble_item object_id="9" />
  </assemble>
</config>
"#;

const PROJECT_CONFIG: &[u8] = br##"{
    "filament_colour": [
        "#0000FF",
        "#FFFFFF",
        "#8E9089",
        "#000000",
        "#000000"
    ],
    "filament_is_support": [
        "0",
        "0",
        "0",
        "0",
        "0"
    ],
    "filament_settings_id": [
        "Bambu PLA Basic @BBL A1M",
        "Bambu PLA Basic @BBL A1M",
        "Bambu PLA Basic @BBL A1M",
        "Bambu PLA Basic @BBL A1M",
        "Bambu PLA Basic @BBL A1M"
    ],
    "filament_type": [
        "PLA",
        "PLA",
        "PLA",
        "PLA",
        "TPU"
    ],
    "flush_volumes_matrix": [
        "0",
        "100",
        "100",
        "100",
        "100",
        "100",
        "0",
        "100",
        "100",
        "100",
        "100",
        "100",
        "0",
        "100",
        "100",
        "100",
        "100",
        "100",
        "0",
        "100",
        "100",
        "100",
        "100",
        "100",
        "0"
    ],
    "nozzle_diameter": [
        "0.4"
    ],
    "print_settings_id": "0.20mm Standard @BBL A1M",
    "printable_height": "180",
    "printer_settings_id": "Bambu Lab A1 mini 0.4 nozzle",
    "enable_prime_tower": "1",
    "wipe_tower_x": [
        "50"
    ],
    "wipe_tower_y": [
        "50"
    ]
}"##;

const SLICE_CONFIG:&[u8]=br##"<?xml version="1.0" encoding="UTF-8"?>
<config>
  <header>
    <header_item key="X-BBL-Client-Type" value="slicer"/>
    <header_item key="X-BBL-Client-Version" value="01.10.01.50"/>
  </header>
  <plate>
    <metadata key="index" value="1"/>
    <metadata key="printer_model_id" value="C12"/>
    <metadata key="nozzle_diameters" value="0.4"/>
    <metadata key="timelapse_type" value="0"/>
    <metadata key="prediction" value="9541"/>
    <metadata key="weight" value="38.24"/>
    <metadata key="outside" value="false"/>
    <metadata key="support_used" value="false"/>
    <metadata key="label_object_enabled" value="false"/>
    <object identify_id="150" name="part1" skipped="false" />
    <filament id="1" tray_info_idx="GFL99" type="PLA" color="#0000FF" used_m="2.44" used_g="7.26" />
    <filament id="2" tray_info_idx="GFL03" type="PLA" color="#FFFFFF" used_m="6.10" used_g="18.35" />
    <filament id="3" tray_info_idx="GFA00" type="PLA" color="#8E9089" used_m="1.51" used_g="4.58" />
    <filament id="4" tray_info_idx="GFA00" type="PLA" color="#000000" used_m="2.66" used_g="8.05" />
    <warning msg="bed_temperature_too_high_than_filament" level="1" error_code ="1000C001"  />
  </plate>
</config>
"##;

impl ModelContainer {
    pub fn new(model: Model) -> Self {
        ModelContainer {
            model,
            content_types: None,
            relationships: None,
            model_settings: None,
        }
    }
    pub fn content_types(mut self, content_types: Option<ContentTypes>) -> Self {
        self.content_types = content_types;
        self
    }
    pub fn relationships(mut self, relationships: Option<Relationships>) -> Self {
        self.relationships = relationships;
        self
    }
    pub fn model_settings(mut self, model_settings: Option<ModelSettings>) -> Self {
        self.model_settings = model_settings;
        self
    }
    fn to_xml_string<T: Serialize>(&self, value: &T) -> anyhow::Result<String> {
        Ok(serde_xml_rs::SerdeXml::new()
            .emitter(EmitterConfig::new().perform_indent(true))
            .to_string(value)?)
    }
    pub fn encode(&self) -> anyhow::Result<Vec<u8>> {
        let mut buffer = vec![];
        let mut zip = ZipWriter::new(Cursor::new(&mut buffer));
        let opts = SimpleFileOptions::default();
        if let Some(content_types) = &self.content_types {
            zip.start_file("[Content_Types].xml", opts.clone())?;
            zip.write_all(self.to_xml_string(content_types)?.as_bytes())?;
        }
        if let Some(relationships) = &self.relationships {
            zip.add_directory("_rels", opts.clone())?;
            zip.start_file("_rels/.rels", opts.clone())?;
            zip.write_all(self.to_xml_string(relationships)?.as_bytes())?;
        }
        zip.add_directory("3D/", opts.clone())?;
        zip.start_file("3D/3dmodel.model", opts.clone())?;
        zip.write_all(self.to_xml_string(&self.model)?.as_bytes())?;
        zip.add_directory("Metadata", opts.clone())?;
        if let Some(model_settings) = &self.model_settings {
            zip.start_file("Metadata/model_settings.config", opts.clone())?;
            zip.write_all(self.to_xml_string(model_settings)?.as_bytes())?;
        }
        zip.start_file("Metadata/project_settings.config", opts.clone())?;
        zip.write_all(PROJECT_CONFIG)?;
        zip.finish()?;
        Ok(buffer)
    }
}
