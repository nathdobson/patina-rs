// br#"<?xml version="1.0" encoding="UTF-8"?>
// <config>
//   <object id="9">
//     <metadata key="name" value="part1"/>
//     <metadata key="extruder" value="1"/>
//     <part id="1" subtype="normal_part">
//       <metadata key="name" value="part1.stl"/>
//       <metadata key="extruder" value="1"/>
//     </part>
//     <part id="2" subtype="normal_part">
//       <metadata key="name" value="part2.stl"/>
//       <metadata key="extruder" value="2"/>
//     </part>
//   </object>
//   <plate>
//     <metadata key="plater_id" value="1"/>
//     <model_instance>
//       <metadata key="object_id" value="9"/>
//     </model_instance>
//   </plate>
//   <assemble>
//    <assemble_item object_id="9" />
//   </assemble>
// </config>
// "#;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[serde(rename = "config")]
#[non_exhaustive]
pub struct ModelSettings {
    pub object: Vec<ObjectSettings>,
    pub plate: Vec<Plate>,
    pub assemble: Option<Assemble>,
}

impl ModelSettings {
    pub fn new() -> Self {
        ModelSettings {
            object: vec![],
            plate: vec![],
            assemble: None,
        }
    }
    pub fn object(mut self, object: Vec<ObjectSettings>) -> Self {
        self.object = object;
        self
    }
    pub fn plate(mut self, plate: Vec<Plate>) -> Self {
        self.plate = plate;
        self
    }
    pub fn assemble(mut self, assemble: Option<Assemble>) -> Self {
        self.assemble = assemble;
        self
    }
}

#[derive(Serialize, Deserialize)]
#[non_exhaustive]
pub struct ObjectSettings {
    #[serde(rename = "@id")]
    pub id: String,
    pub metadata: Vec<SettingsMetadata>,
    pub part: Vec<Part>,
}

impl ObjectSettings {
    pub fn new(id: String) -> Self {
        ObjectSettings {
            id,
            metadata: vec![],
            part: vec![],
        }
    }
    pub fn metadata(mut self, metadata: Vec<SettingsMetadata>) -> Self {
        self.metadata = metadata;
        self
    }
    pub fn part(mut self, part: Vec<Part>) -> Self {
        self.part = part;
        self
    }
}

#[derive(Serialize, Deserialize)]
#[non_exhaustive]
#[serde(rename_all = "snake_case")]
pub enum SettingsMetadataKey {
    Name,
    Extruder,
    ObjectId,
    PlaterId,
}

#[derive(Serialize, Deserialize)]
#[non_exhaustive]
pub struct SettingsMetadata {
    #[serde(rename = "@key")]
    pub key: SettingsMetadataKey,
    #[serde(rename = "@value")]
    pub value: Option<String>,
}

impl SettingsMetadata {
    pub fn new(key: SettingsMetadataKey) -> Self {
        SettingsMetadata { key, value: None }
    }
    pub fn value(mut self, value: Option<String>) -> Self {
        self.value = value;
        self
    }
}

#[derive(Serialize, Deserialize)]
#[non_exhaustive]
pub struct Part {
    #[serde(rename = "@id")]
    pub id: String,
    #[serde(rename = "@subtype")]
    pub subtype: String,
    pub metadata: Vec<SettingsMetadata>,
}

impl Part {
    pub fn new(id: String) -> Self {
        Part {
            id,
            subtype: "".to_string(),
            metadata: vec![],
        }
    }
    pub fn subtype(mut self, subtype: String) -> Self {
        self.subtype = subtype;
        self
    }
    pub fn metadata(mut self, metadata: Vec<SettingsMetadata>) -> Self {
        self.metadata = metadata;
        self
    }
}

#[derive(Serialize, Deserialize)]
#[non_exhaustive]
pub struct Plate {
    pub metadata: Vec<SettingsMetadata>,
    pub model_instance: Vec<ModelInstance>,
}

#[derive(Serialize, Deserialize)]
#[non_exhaustive]
pub struct ModelInstance {
    pub metadata: Vec<SettingsMetadata>,
}

impl ModelInstance {
    pub fn new() -> Self {
        ModelInstance { metadata: vec![] }
    }
    pub fn metadata(mut self, metadata: Vec<SettingsMetadata>) -> Self {
        self.metadata = metadata;
        self
    }
}

impl Plate {
    pub fn new() -> Self {
        Plate {
            metadata: vec![],
            model_instance: vec![],
        }
    }
    pub fn metadata(mut self, metadata: Vec<SettingsMetadata>) -> Self {
        self.metadata = metadata;
        self
    }
    pub fn model_instance(mut self, model_instance: Vec<ModelInstance>) -> Self {
        self.model_instance = model_instance;
        self
    }
}

#[derive(Serialize, Deserialize)]
#[non_exhaustive]
pub struct Assemble {
    pub assemble_item: Vec<AssembleItem>,
}

impl Assemble {
    pub fn new() -> Self {
        Assemble {
            assemble_item: vec![],
        }
    }
    pub fn assemble_item(mut self, assemble_item: Vec<AssembleItem>) -> Self {
        self.assemble_item = assemble_item;
        self
    }
}

#[derive(Serialize, Deserialize)]
#[non_exhaustive]
pub struct AssembleItem {
    #[serde(rename = "@object_id")]
    pub object_id: String,
}

impl AssembleItem {
    pub fn new(object_id: String) -> Self {
        AssembleItem { object_id }
    }
}
