use serde::{Deserialize, Serialize};
use crate::model::mesh::Mesh;

#[derive(Serialize, Deserialize, Default)]
#[non_exhaustive]
pub struct Color {
    #[serde(rename = "@color", default)]
    pub color: String,
}

#[derive(Serialize, Deserialize, Default)]
#[non_exhaustive]
pub struct Colorgroup {
    #[serde(rename = "@id")]
    pub id: usize,
    #[serde(rename = "m:color", default)]
    pub color: Vec<Color>,
}

#[derive(Serialize, Deserialize)]
#[non_exhaustive]
pub struct Resources {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub basematerials: Vec<BaseMaterials>,
    #[serde(default, rename = "m:colorgroup")]
    pub colorgroup: Vec<Colorgroup>,
    #[serde(default)]
    pub object: Vec<Object>,
}

impl Resources {
    pub fn new() -> Resources {
        Resources {
            basematerials: vec![],
            colorgroup: vec![],
            object: vec![],
        }
    }
    pub fn basematerials(mut self, basematerials: Vec<BaseMaterials>) -> Resources {
        self.basematerials = basematerials;
        self
    }
    pub fn colorgroup(mut self, colorgroup: Vec<Colorgroup>) -> Resources {
        self.colorgroup = colorgroup;
        self
    }
    pub fn object(mut self, object: Vec<Object>) -> Resources {
        self.object = object;
        self
    }
}

#[derive(Serialize, Deserialize)]
#[non_exhaustive]
pub struct BaseMaterials {
    #[serde(rename = "@id")]
    pub id: usize,
    pub base: Vec<BaseMaterial>,
}

#[derive(Serialize, Deserialize)]
#[non_exhaustive]
pub struct BaseMaterial {
    #[serde(rename = "@name")]
    pub name: String,
    #[serde(rename = "@displaycolor")]
    pub displaycolor: String,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
#[non_exhaustive]
pub enum ObjectType {
    Model,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
#[non_exhaustive]
pub struct Object {
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
    pub object_type: Option<ObjectType>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub mesh: Option<Mesh>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub components: Option<Components>,
}

impl Object {
    pub fn new(id: usize) -> Self {
        Object {
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
    pub fn object_type(mut self, object_type: Option<ObjectType>) -> Self {
        self.object_type = object_type;
        self
    }
    pub fn mesh(mut self, mesh: Option<Mesh>) -> Self {
        self.mesh = mesh;
        self
    }
    pub fn components(mut self, components: Option<Components>) -> Self {
        self.components = components;
        self
    }
}

#[derive(Serialize, Deserialize)]
#[non_exhaustive]
pub struct Components {
    pub component: Vec<Component>,
}

impl Components {
    pub fn new(component: Vec<Component>) -> Components {
        Components { component }
    }
}

#[derive(Serialize, Deserialize)]
#[non_exhaustive]
pub struct Component {
    #[serde(rename = "@objectid")]
    pub objectid: usize,

    #[serde(rename = "@transform", skip_serializing_if = "Option::is_none")]
    pub transform: Option<[f64; 12]>,
}

impl Component {
    pub fn new(objectid: usize) -> Component {
        Component {
            objectid,
            transform: None,
        }
    }
}
