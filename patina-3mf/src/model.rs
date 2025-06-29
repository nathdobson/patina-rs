use crate::mesh::Mesh;
use serde::{Deserialize, Serialize};

// Derived from https://docs.rs/threemf/latest/threemf/
#[derive(Serialize, Deserialize)]
#[serde(rename = "model", rename_all = "lowercase")]
#[non_exhaustive]
pub struct Model {
    #[serde(rename = "@xmlns", default)]
    pub xmlns: Xmlns,
    #[serde(rename = "@unit", default)]
    pub unit: Unit,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub metadata: Vec<Metadata>,
    pub resources: Resources,
    pub build: Build,
}

impl Model {
    pub fn new() -> Self {
        Model {
            xmlns: Default::default(),
            unit: Default::default(),
            metadata: vec![],
            resources: Resources::new(),
            build: Default::default(),
        }
    }
    pub fn xmlns(mut self, xmlns: Xmlns) -> Self {
        self.xmlns = xmlns;
        self
    }
    pub fn unit(mut self, unit: Unit) -> Self {
        self.unit = unit;
        self
    }
    pub fn metadata(mut self, metadata: Vec<Metadata>) -> Self {
        self.metadata = metadata;
        self
    }
    pub fn resources(mut self, resources: Resources) -> Self {
        self.resources = resources;
        self
    }
    pub fn build(mut self, build: Build) -> Self {
        self.build = build;
        self
    }
}

#[derive(Serialize, Deserialize)]
#[non_exhaustive]
pub enum Xmlns {
    #[serde(rename = "http://schemas.microsoft.com/3dmanufacturing/core/2015/02")]
    Model,
}

impl Default for Xmlns {
    fn default() -> Self {
        Xmlns::Model
    }
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
#[non_exhaustive]
pub enum Unit {
    Micron,
    Millimeter,
    Centimeter,
    Inch,
    Foot,
    Meter,
}

impl Default for Unit {
    fn default() -> Self {
        Unit::Millimeter
    }
}

#[derive(Serialize, Deserialize)]
#[non_exhaustive]
pub struct Metadata {
    #[serde(rename = "@name")]
    pub name: String,
    #[serde(rename = "#text")]
    pub value: Option<String>,
}

impl Metadata {
    pub fn new(name: String) -> Self {
        Metadata { name, value: None }
    }
    pub fn value(mut self, value: Option<String>) -> Self {
        self.value = value;
        self
    }
}

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

#[derive(Serialize, Deserialize, Default)]
#[non_exhaustive]
pub struct Build {
    #[serde(default)]
    pub item: Vec<Item>,
}

impl Build {
    pub fn new() -> Self {
        Build { item: vec![] }
    }
    pub fn item(mut self, item: Vec<Item>) -> Self {
        self.item = item;
        self
    }
}

#[derive(Serialize, Deserialize)]
#[non_exhaustive]
pub struct Item {
    #[serde(rename = "@objectid")]
    pub objectid: usize,

    #[serde(rename = "@transform", skip_serializing_if = "Option::is_none")]
    pub transform: Option<[f64; 12]>,

    #[serde(rename = "@partnumber", skip_serializing_if = "Option::is_none")]
    pub partnumber: Option<String>,

    #[serde(
        rename = "@printable",
        skip_serializing_if = "Option::is_none",
        with = "bool_as_int"
    )]
    pub printable: Option<bool>,
}

impl Item {
    pub fn new(objectid: usize) -> Item {
        Item {
            objectid,
            transform: None,
            partnumber: None,
            printable: None,
        }
    }
    pub fn transform(mut self, transform: Option<[f64; 12]>) -> Self {
        self.transform = transform;
        self
    }
    pub fn partnumber(mut self, partnumber: Option<String>) -> Self {
        self.partnumber = partnumber;
        self
    }
    pub fn printable(mut self, printable: Option<bool>) -> Self {
        self.printable = printable;
        self
    }
}

pub mod bool_as_int {
    use serde::{Deserialize, Deserializer, Serialize, Serializer};

    pub fn serialize<S: Serializer>(b: &Option<bool>, ser: S) -> Result<S::Ok, S::Error> {
        b.map(|x| x as u8).serialize(ser)
    }
    pub fn deserialize<'de, D: Deserializer<'de>>(d: D) -> Result<Option<bool>, D::Error> {
        Ok(Option::<u8>::deserialize(d)?.map(|x| x != 0))
    }
}
