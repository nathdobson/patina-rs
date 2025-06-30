use serde::{Deserialize, Serialize};
use serde_with::serde_as;
use crate::bool_from_int_string::BoolFromIntString;

#[derive(Serialize, Deserialize, Default)]
#[non_exhaustive]
pub struct ModelBuild {
    #[serde(default)]
    pub item: Vec<ModelItem>,
}

impl ModelBuild {
    pub fn new() -> Self {
        ModelBuild { item: vec![] }
    }
    pub fn item(mut self, item: Vec<ModelItem>) -> Self {
        self.item = item;
        self
    }
}

#[serde_as]
#[derive(Serialize, Deserialize)]
#[non_exhaustive]
pub struct ModelItem {
    #[serde(rename = "@objectid")]
    pub objectid: usize,

    #[serde(rename = "@transform", skip_serializing_if = "Option::is_none")]
    pub transform: Option<[f64; 12]>,

    #[serde(rename = "@partnumber", skip_serializing_if = "Option::is_none")]
    pub partnumber: Option<String>,

    #[serde_as(as = "Option<BoolFromIntString>")]
    #[serde(rename = "@printable", skip_serializing_if = "Option::is_none")]
    pub printable: Option<bool>,
}

impl ModelItem {
    pub fn new(objectid: usize) -> ModelItem {
        ModelItem {
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
