use crate::xmlns::Xmlns;
use serde::{Deserialize, Serialize};

const CONTENT_TYPES_FILE: &[u8] = //
    br#"<?xml version="1.0" encoding="utf-8"?>
    <Types xmlns="http://schemas.openxmlformats.org/package/2006/content-types">
        <Default
            Extension="model"
            ContentType="application/vnd.ms-package.3dmanufacturing-3dmodel+xml" />
        <Default
            Extension="rels"
            ContentType="application/vnd.openxmlformats-package.relationships+xml" />
        <Default
            Extension="texture"
            ContentType="application/vnd.ms-package.3dmanufacturing-3dmodeltexture" />
        <Default Extension="png" ContentType="image/png"/>
    </Types>
"#;

#[derive(Serialize, Deserialize)]
#[serde(rename = "Types")]
#[non_exhaustive]
pub struct ContentTypes {
    #[serde(rename = "@xmlns")]
    pub xmlns: Xmlns,
    pub defaults: Vec<ContentTypeDefault>,
}

impl ContentTypes {
    pub fn new() -> Self {
        ContentTypes {
            xmlns: Xmlns::ContentTypes,
            defaults: vec![],
        }
    }
    pub fn xmlns(mut self, xmlns: Xmlns) -> Self {
        self.xmlns = xmlns;
        self
    }
    pub fn defaults(mut self, defaults: Vec<ContentTypeDefault>) -> Self {
        self.defaults = defaults;
        self
    }
}

#[derive(Serialize, Deserialize)]
#[serde(rename = "Default")]
#[non_exhaustive]
pub struct ContentTypeDefault {
    #[serde(rename = "@Extension")]
    extension: String,
    #[serde(rename = "@ContentType")]
    content_type: String,
}

impl ContentTypeDefault {
    pub fn new(extension: String, content_type: String) -> Self {
        ContentTypeDefault {
            extension,
            content_type,
        }
    }
}
