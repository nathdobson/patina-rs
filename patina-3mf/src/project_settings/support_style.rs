use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SupportStyle {
    Default,
    Grid,
    Snug,
}
