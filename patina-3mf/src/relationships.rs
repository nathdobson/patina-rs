use crate::xmlns::Xmlns;
use serde::{Deserialize, Serialize};

// br#"<?xml version="1.0" encoding="UTF-8"?>
//             <Relationships xmlns="http://schemas.openxmlformats.org/package/2006/relationships">
//                 <Relationship Target="/3D/3dmodel.model" Id="rel-1" Type="http://schemas.microsoft.com/3dmanufacturing/2013/01/3dmodel"/>
//             </Relationships>
//         "#;

#[derive(Serialize, Deserialize)]
#[non_exhaustive]
pub struct Relationships {
    #[serde(rename = "@xmlns")]
    pub xmlns: Xmlns,
    #[serde(rename = "Relationship")]
    pub relationship: Vec<Relationship>,
}

impl Relationships {
    pub fn new() -> Self {
        Relationships {
            xmlns: Xmlns::Relationships,
            relationship: vec![],
        }
    }
    pub fn relationship(mut self, relationship: Vec<Relationship>) -> Self {
        self.relationship = relationship;
        self
    }
}

#[derive(Serialize, Deserialize)]
#[non_exhaustive]
pub struct Relationship {
    #[serde(rename = "@Target")]
    pub target: String,
    #[serde(rename = "@Id")]
    pub id: String,
    #[serde(rename = "@Type")]
    pub typ: String,
}

impl Relationship {
    pub fn new() -> Self {
        Relationship {
            target: "".to_string(),
            id: "".to_string(),
            typ: "".to_string(),
        }
    }
    pub fn target(mut self, target: String) -> Self {
        self.target = target;
        self
    }
    pub fn id(mut self, id: String) -> Self {
        self.id = id;
        self
    }
    pub fn typ(mut self, typ: String) -> Self {
        self.typ = typ;
        self
    }
}
