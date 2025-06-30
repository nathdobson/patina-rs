use crate::xmlns::Xmlns;
use build::ModelBuild;
use resources::ModelResources;
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
    pub unit: ModelUnit,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub metadata: Vec<ModelMetadata>,
    pub resources: ModelResources,
    pub build: ModelBuild,
}

#[derive(Serialize, Deserialize)]
#[non_exhaustive]
pub struct ModelMetadata {
    #[serde(rename = "@name")]
    pub name: String,
    #[serde(rename = "#text")]
    pub value: Option<String>,
}

impl ModelMetadata {
    pub fn new(name: String) -> Self {
        ModelMetadata { name, value: None }
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
            resources: ModelResources::new(),
            build: Default::default(),
        }
    }
    pub fn xmlns(mut self, xmlns: Xmlns) -> Self {
        self.xmlns = xmlns;
        self
    }
    pub fn unit(mut self, unit: ModelUnit) -> Self {
        self.unit = unit;
        self
    }
    pub fn metadata(mut self, metadata: Vec<ModelMetadata>) -> Self {
        self.metadata = metadata;
        self
    }
    pub fn resources(mut self, resources: ModelResources) -> Self {
        self.resources = resources;
        self
    }
    pub fn build(mut self, build: ModelBuild) -> Self {
        self.build = build;
        self
    }
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
#[non_exhaustive]
pub enum ModelUnit {
    Micron,
    Millimeter,
    Centimeter,
    Inch,
    Foot,
    Meter,
}

impl Default for ModelUnit {
    fn default() -> Self {
        ModelUnit::Millimeter
    }
}

