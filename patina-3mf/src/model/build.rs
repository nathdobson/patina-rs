use serde::{Deserialize, Serialize};

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
        with = "crate::bool_as_int"
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