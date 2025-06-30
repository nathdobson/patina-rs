use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[non_exhaustive]
pub enum Xmlns {
    #[serde(rename = "http://schemas.microsoft.com/3dmanufacturing/core/2015/02")]
    Model,
    #[serde(rename = "http://schemas.openxmlformats.org/package/2006/content-types")]
    ContentTypes,
    #[serde(rename = "http://schemas.openxmlformats.org/package/2006/relationships")]
    Relationships,
}
