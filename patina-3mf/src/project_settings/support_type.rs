use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub enum SupportType {
    #[serde(rename = "normal(auto)")]
    NormalAuto,
    #[serde(rename = "normal(manual)")]
    NormalManual,
    #[serde(rename = "tree(auto)")]
    TreeAuto,
    #[serde(rename = "tree(manual)")]
    TreeManual,
}
