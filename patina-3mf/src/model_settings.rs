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

use crate::custom_serde::map_struct::{MapStruct, MapStructKeys};
use serde::{Deserialize, Serialize};
use serde_with::DisplayFromStr;
use serde_with::serde_as;

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

struct MetadataKeys;

impl MapStructKeys for MetadataKeys {
    const NAME: &'static str = "@key";
    const VALUE: &'static str = "@value";
}

#[serde_as]
#[derive(Serialize, Deserialize)]
#[non_exhaustive]
pub struct ObjectSettings {
    #[serde(rename = "@id")]
    pub id: String,
    #[serde_as(as = "MapStruct<MetadataKeys>")]
    pub metadata: ObjectSettingsMetadata,
    pub part: Vec<Part>,
}

#[serde_as]
#[derive(Serialize, Deserialize)]
#[non_exhaustive]
pub struct ObjectSettingsMetadata {
    name: Option<String>,
    #[serde_as(as = "Option<DisplayFromStr>")]
    extruder: Option<usize>,
}

impl ObjectSettings {
    pub fn new(id: String) -> Self {
        ObjectSettings {
            id,
            metadata: ObjectSettingsMetadata {
                name: None,
                extruder: None,
            },
            part: vec![],
        }
    }
    pub fn metadata_name(mut self, name: Option<String>) -> Self {
        self.metadata.name = name;
        self
    }
    pub fn metadata_extruder(mut self, extruder: Option<usize>) -> Self {
        self.metadata.extruder = extruder;
        self
    }
    pub fn part(mut self, part: Vec<Part>) -> Self {
        self.part = part;
        self
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "snake_case")]
#[non_exhaustive]
pub enum PartSubtype {
    NormalPart,
    ModifierPart,
    SupportBlocker,
}

#[serde_as]
#[derive(Serialize, Deserialize)]
#[non_exhaustive]
pub struct Part {
    #[serde(rename = "@id")]
    pub id: String,
    #[serde(rename = "@subtype")]
    pub subtype: Option<PartSubtype>,
    #[serde_as(as = "MapStruct<MetadataKeys>")]
    pub metadata: PartSettingsMetadata,
}

#[serde_as]
#[derive(Serialize, Deserialize)]
#[non_exhaustive]
pub struct PartSettingsMetadata {
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<String>,
    #[serde_as(as = "Option<DisplayFromStr>")]
    #[serde(skip_serializing_if = "Option::is_none")]
    extruder: Option<usize>,
    #[serde_as(as = "Option<DisplayFromStr>")]
    #[serde(skip_serializing_if = "Option::is_none")]
    wall_loops: Option<usize>,
}

impl Part {
    pub fn new(id: String) -> Self {
        Part {
            id,
            subtype: None,
            metadata: PartSettingsMetadata {
                name: None,
                extruder: None,
                wall_loops: None,
            },
        }
    }
    pub fn metadata_name(mut self, name: Option<String>) -> Self {
        self.metadata.name = name;
        self
    }
    pub fn metadata_extruder(mut self, extruder: Option<usize>) -> Self {
        self.metadata.extruder = extruder;
        self
    }
    pub fn metadata_wall_loops(mut self, wall_loops: Option<usize>) -> Self {
        self.metadata.wall_loops = wall_loops;
        self
    }
    pub fn subtype(mut self, subtype: Option<PartSubtype>) -> Self {
        self.subtype = subtype;
        self
    }
}

#[serde_as]
#[derive(Serialize, Deserialize)]
#[non_exhaustive]
pub struct Plate {
    #[serde_as(as = "MapStruct<MetadataKeys>")]
    pub metadata: PlateSettingsMetadata,
    pub model_instance: Vec<ModelInstance>,
}

#[serde_as]
#[derive(Serialize, Deserialize)]
#[non_exhaustive]
pub struct PlateSettingsMetadata {
    #[serde_as(as = "Option<DisplayFromStr>")]
    #[serde(skip_serializing_if = "Option::is_none")]
    plater_id: Option<usize>,
}

#[serde_as]
#[derive(Serialize, Deserialize)]
#[non_exhaustive]
pub struct ModelInstance {
    #[serde_as(as = "MapStruct<MetadataKeys>")]
    pub metadata: ModelInstanceSettingsMetadata,
}

#[serde_as]
#[derive(Serialize, Deserialize)]
#[non_exhaustive]
pub struct ModelInstanceSettingsMetadata {
    #[serde_as(as = "Option<DisplayFromStr>")]
    #[serde(skip_serializing_if = "Option::is_none")]
    object_id: Option<usize>,
}

impl ModelInstance {
    pub fn new() -> Self {
        ModelInstance {
            metadata: ModelInstanceSettingsMetadata { object_id: None },
        }
    }
    pub fn metadata_object_id(mut self, object_id: Option<usize>) -> Self {
        self.metadata.object_id = object_id;
        self
    }
}

impl Plate {
    pub fn new() -> Self {
        Plate {
            metadata: PlateSettingsMetadata { plater_id: None },
            model_instance: vec![],
        }
    }

    pub fn metadata_plater_id(mut self, plater_id: Option<usize>) -> Self {
        self.metadata.plater_id = plater_id;
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
