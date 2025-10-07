use crate::model::mesh::ModelMesh;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Default)]
#[non_exhaustive]
pub struct ModelColor {
    #[serde(rename = "@color", default)]
    pub color: String,
}

#[derive(Serialize, Deserialize, Default)]
#[non_exhaustive]
pub struct ModelColorGroup {
    #[serde(rename = "@id")]
    pub id: usize,
    #[serde(rename = "m:color", default)]
    pub color: Vec<ModelColor>,
}

#[derive(Serialize, Deserialize)]
#[non_exhaustive]
pub struct ModelResources {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub basematerials: Vec<ModelBaseMaterials>,
    #[serde(default, rename = "m:colorgroup")]
    pub colorgroup: Vec<ModelColorGroup>,
    #[serde(default)]
    pub object: Vec<ModelObject>,
}

impl ModelResources {
    pub fn new() -> ModelResources {
        ModelResources {
            basematerials: vec![],
            colorgroup: vec![],
            object: vec![],
        }
    }
    pub fn basematerials(mut self, basematerials: Vec<ModelBaseMaterials>) -> ModelResources {
        self.basematerials = basematerials;
        self
    }
    pub fn colorgroup(mut self, colorgroup: Vec<ModelColorGroup>) -> ModelResources {
        self.colorgroup = colorgroup;
        self
    }
    pub fn object(mut self, object: Vec<ModelObject>) -> ModelResources {
        self.object = object;
        self
    }
}

#[derive(Serialize, Deserialize)]
#[non_exhaustive]
pub struct ModelBaseMaterials {
    #[serde(rename = "@id")]
    pub id: usize,
    pub base: Vec<ModelBaseMaterial>,
}

#[derive(Serialize, Deserialize)]
#[non_exhaustive]
pub struct ModelBaseMaterial {
    #[serde(rename = "@name")]
    pub name: String,
    #[serde(rename = "@displaycolor")]
    pub displaycolor: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "lowercase")]
#[non_exhaustive]
pub enum ModelObjectType {
    Model,
    SolidSupport,
    Support,
    Surface,
    Other,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
#[non_exhaustive]
pub struct ModelObject {
    #[serde(rename = "@id")]
    pub id: usize,

    #[serde(rename = "@partnumber", skip_serializing_if = "Option::is_none")]
    pub partnumber: Option<String>,

    #[serde(rename = "@name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    #[serde(rename = "@pid", skip_serializing_if = "Option::is_none")]
    pub pid: Option<usize>,

    #[serde(rename = "@pindex", skip_serializing_if = "Option::is_none")]
    pub pindex: Option<usize>,

    #[serde(rename = "@type", skip_serializing_if = "Option::is_none")]
    pub object_type: Option<ModelObjectType>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub mesh: Option<ModelMesh>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub components: Option<ModelComponents>,
}

impl ModelObject {
    pub fn new(id: usize) -> Self {
        ModelObject {
            id,
            partnumber: None,
            name: None,
            pid: None,
            pindex: None,
            object_type: None,
            mesh: None,
            components: None,
        }
    }
    pub fn partnumber(mut self, partnumber: Option<String>) -> Self {
        self.partnumber = partnumber;
        self
    }
    pub fn name(mut self, name: Option<String>) -> Self {
        self.name = name;
        self
    }
    pub fn pid(mut self, pid: Option<usize>) -> Self {
        self.pid = pid;
        self
    }
    pub fn pindex(mut self, pindex: Option<usize>) -> Self {
        self.pindex = pindex;
        self
    }
    pub fn object_type(mut self, object_type: Option<ModelObjectType>) -> Self {
        self.object_type = object_type;
        self
    }
    pub fn mesh(mut self, mesh: Option<ModelMesh>) -> Self {
        self.mesh = mesh;
        self
    }
    pub fn components(mut self, components: Option<ModelComponents>) -> Self {
        self.components = components;
        self
    }
}

#[derive(Serialize, Deserialize)]
#[non_exhaustive]
pub struct ModelComponents {
    pub component: Vec<ModelComponent>,
}

impl ModelComponents {
    pub fn new(component: Vec<ModelComponent>) -> ModelComponents {
        ModelComponents { component }
    }
}

#[derive(Serialize, Deserialize)]
#[non_exhaustive]
pub struct ModelComponent {
    #[serde(rename = "@objectid")]
    pub objectid: usize,

    #[serde(rename = "@transform", skip_serializing_if = "Option::is_none")]
    pub transform: Option<[f64; 12]>,
}

impl ModelComponent {
    pub fn new(objectid: usize) -> ModelComponent {
        ModelComponent {
            objectid,
            transform: None,
        }
    }
    pub fn transform(mut self, transform: Option<[f64; 12]>) -> Self {
        self.transform = transform;
        self
    }
}
