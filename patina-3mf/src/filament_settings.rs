use crate::settings_id::filament_settings_id::FilamentSettingsId;
use serde::{Deserialize, Serialize};
use serde_with::serde_as;
use crate::custom_serde::option_or_nil::OptionOrNil;
use serde_with::DisplayFromStr;
#[derive(Serialize, Deserialize)]
pub enum FilamentExtruderStandard {
    #[serde(rename = "Direct Drive Standard")]
    DirectDriveStandard,
}

#[serde_as]
#[derive(Serialize, Deserialize)]
pub struct FilamentSettings {
    pub filament_extruder_variant: FilamentExtruderStandard,
    #[serde_as(as = "Option<Vec<OptionOrNil<DisplayFromStr>>>")]
    pub filament_flow_ratio: Option<Vec<Option<f64>>>,
    #[serde_as(as = "Option<Vec<OptionOrNil<DisplayFromStr>>>")]
    pub filament_prime_volume: Option<Vec<Option<f64>>>,
    pub filament_settings_id: Vec<String>,
    pub from: String,
    pub inherits: Option<FilamentSettingsId>,
    pub name: String,
    pub version: String,
}
