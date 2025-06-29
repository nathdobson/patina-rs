use serde::{Deserialize, Serialize};
use crate::mesh::Mesh;

// Derived from https://docs.rs/threemf/latest/threemf/
#[derive(Serialize, Deserialize)]
#[serde(rename = "model", rename_all = "lowercase")]
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

#[derive(Serialize, Deserialize)]
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
pub struct Metadata {
    #[serde(rename = "@name")]
    pub name: String,
    #[serde(rename = "#text")]
    pub value: Option<String>,
}

#[derive(Serialize, Deserialize, Default)]
pub struct Color {
    #[serde(rename = "@color", default)]
    pub color: String,
}

#[derive(Serialize, Deserialize, Default)]
pub struct Colorgroup {
    #[serde(rename = "@id")]
    pub id: usize,
    #[serde(rename = "m:color", default)]
    pub color: Vec<Color>,
}

#[derive(Serialize, Deserialize, Default)]
pub struct Resources {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub basematerials: Vec<BaseMaterials>,
    #[serde(default, rename = "m:colorgroup")]
    pub colorgroup: Vec<Colorgroup>,
    #[serde(default)]
    pub object: Vec<Object>,
}

#[derive(Serialize, Deserialize)]
pub struct BaseMaterials {
    #[serde(rename = "@id")]
    pub id: usize,
    pub base: Vec<BaseMaterial>,
}

#[derive(Serialize, Deserialize)]
pub struct BaseMaterial {
    #[serde(rename = "@name")]
    pub name: String,
    #[serde(rename = "@displaycolor")]
    pub displaycolor: String,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ObjectType {
    Model,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
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

#[derive(Serialize, Deserialize)]
pub struct Components {
    pub component: Vec<Component>,
}

#[derive(Serialize, Deserialize)]
pub struct Component {
    #[serde(rename = "@objectid")]
    pub objectid: usize,

    #[serde(rename = "@transform", skip_serializing_if = "Option::is_none")]
    pub transform: Option<[f64; 12]>,
}

#[derive(Serialize, Deserialize, Default)]
pub struct Build {
    #[serde(default)]
    pub item: Vec<Item>,
}

#[derive(Serialize, Deserialize)]

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

pub mod bool_as_int {
    use serde::{Deserialize, Deserializer, Serialize, Serializer};

    pub fn serialize<S: Serializer>(b: &Option<bool>, ser: S) -> Result<S::Ok, S::Error> {
        b.map(|x| x as u8).serialize(ser)
    }
    pub fn deserialize<'de, D: Deserializer<'de>>(d: D) -> Result<Option<bool>, D::Error> {
        Ok(Option::<u8>::deserialize(d)?.map(|x| x != 0))
    }
}
