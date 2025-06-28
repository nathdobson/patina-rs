use crate::threemf::mesh::Mesh;
use serde::{Deserialize, Serialize};

// Derived from https://docs.rs/threemf/latest/threemf/
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
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
    #[serde(rename = "$value")]
    pub value: Option<String>,
}

#[derive(Serialize, Deserialize, Default)]
pub struct Resources {
    #[serde(default)]
    pub object: Vec<Object>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub basematerials: Option<()>,
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
}
