#![allow(dead_code, unused_imports, unused_mut,unused_variables)]

pub mod content_types;
pub mod model;
pub mod model_settings;
pub mod project_settings;
pub mod relationships;
pub mod xmlns;
mod bool_from_int_string;

use crate::content_types::ContentTypes;
use crate::model::Model;
use crate::model_settings::ModelSettings;
use crate::project_settings::ProjectSettings;
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
    pub project_settings: Option<ProjectSettings>,
}

impl ModelContainer {
    pub fn new(model: Model) -> Self {
        ModelContainer {
            model,
            content_types: None,
            relationships: None,
            model_settings: None,
            project_settings: None,
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
    pub fn project_settings(mut self, project_settings: Option<ProjectSettings>) -> Self {
        self.project_settings = project_settings;
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
        if let Some(project_settings) = &self.project_settings {
            zip.start_file("Metadata/project_settings.config", opts.clone())?;
            zip.write_all(serde_json::to_string_pretty(project_settings)?.as_bytes())?;
        }
        zip.finish()?;
        Ok(buffer)
    }
}
