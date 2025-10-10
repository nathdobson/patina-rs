use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "snake_case")]
pub enum SupportInterfacePattern {
    Rectilinear,
    Concentric,
    RectilinearInterlaced,
    Grid,
}
