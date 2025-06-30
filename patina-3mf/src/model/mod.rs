use crate::xmlns::Xmlns;
use build::Build;
use resources::Resources;
use serde::{Deserialize, Serialize};

pub mod build;
pub mod mesh;
pub mod resources;

// Derived from https://docs.rs/threemf/latest/threemf/
#[derive(Serialize, Deserialize)]
#[serde(rename = "model", rename_all = "lowercase")]
#[non_exhaustive]
pub struct Model {
    #[serde(rename = "@xmlns")]
    pub xmlns: Xmlns,
    #[serde(rename = "@unit", default)]
    pub unit: Unit,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub metadata: Vec<Metadata>,
    pub resources: Resources,
    pub build: Build,
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

impl Model {
    pub fn new() -> Self {
        Model {
            xmlns: Xmlns::Model,
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

