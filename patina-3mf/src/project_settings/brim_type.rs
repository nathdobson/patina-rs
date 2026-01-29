use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub enum BrimType {
    #[serde(rename = "auto_brim")]
    Auto,
    #[serde(rename = "brim_ears")]
    Painted,
    #[serde(rename = "outer_only")]
    OuterBrimOnly,
    #[serde(rename = "inner_only")]
    InnerBrimOnly,
    #[serde(rename = "outer_and_inner")]
    OuterAndInnerBrim,
    #[serde(rename = "no_brim")]
    NoBrim,
}
