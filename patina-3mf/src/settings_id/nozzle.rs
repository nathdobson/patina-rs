use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};

#[derive(Serialize, Deserialize, Clone, Eq, Ord, PartialOrd, PartialEq)]
#[non_exhaustive]
pub enum Nozzle {
    #[serde(rename = "0.2 nozzle")]
    Nozzle0_2,
    #[serde(rename = "0.4 nozzle")]
    Nozzle0_4,
    #[serde(rename = "0.6 nozzle")]
    Nozzle0_6,
    #[serde(rename = "0.8 nozzle")]
    Nozzle0_8,
}

impl Display for Nozzle {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.serialize(f)
    }
}
