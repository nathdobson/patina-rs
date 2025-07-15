use crate::custom_serde::bool_from_int_string::BoolFromIntString;
use crate::custom_serde::option_or_nil::OptionOrNil;
use crate::project_settings::support_interface_pattern::SupportInterfacePattern;
use crate::project_settings::support_style::SupportStyle;
use crate::project_settings::support_type::SupportType;
use crate::settings_id::filament_settings_id::FilamentSettingsId;
use crate::settings_id::print_settings_id::PrintSettingsId;
use crate::settings_id::printer_settings_id::PrinterSettingsId;
use color::Color;
use serde::de::Error;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use serde_with::DisplayFromStr;
use serde_with::{DeserializeAs, SerializeAs, serde_as};

pub mod color;
pub mod support_interface_pattern;
pub mod support_style;
pub mod support_type;

#[serde_as]
#[derive(Serialize, Deserialize)]
#[non_exhaustive]
pub struct ProjectSettings {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filament_colour: Option<Vec<Option<Color>>>,
    #[serde_as(as = "Option<Vec<OptionOrNil<BoolFromIntString>>>")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filament_is_support: Option<Vec<Option<bool>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filament_settings_id: Option<Vec<Option<FilamentSettingsId>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filament_shrink: Option<Vec<Option<String>>>,
    #[serde_as(as = "Option<Vec<OptionOrNil<DisplayFromStr>>>")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filament_diameter: Option<Vec<Option<f64>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filament_type: Option<Vec<String>>,
    #[serde_as(as = "Option<Vec<DisplayFromStr>>")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flush_volumes_matrix: Option<Vec<f64>>,
    #[serde_as(as = "Option<Vec<DisplayFromStr>>")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nozzle_diameter: Option<Vec<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub print_settings_id: Option<PrintSettingsId>,
    #[serde_as(as = "Option<DisplayFromStr>")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub printable_height: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub printer_settings_id: Option<PrinterSettingsId>,
    #[serde_as(as = "Option<BoolFromIntString>")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_prime_tower: Option<bool>,
    #[serde_as(as = "Option<Vec<DisplayFromStr>>")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wipe_tower_x: Option<Vec<f64>>,
    #[serde_as(as = "Option<Vec<DisplayFromStr>>")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wipe_tower_y: Option<Vec<f64>>,
    #[serde_as(as = "Option<BoolFromIntString>")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prime_tower_rib_wall: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prime_tower_infill_gap: Option<String>,
    #[serde_as(as = "Option<BoolFromIntString>")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_timelapse: Option<bool>,
    #[serde_as(as = "Option<DisplayFromStr>")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timelapse_type: Option<usize>,
    #[serde_as(as = "Option<BoolFromIntString>")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_support: Option<bool>,
    #[serde_as(as = "Option<DisplayFromStr>")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub independent_support_layer_height: Option<usize>,
    #[serde_as(as = "Option<DisplayFromStr>")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub support_bottom_z_distance: Option<usize>,
    #[serde_as(as = "Option<DisplayFromStr>")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub support_filament: Option<usize>,
    #[serde_as(as = "Option<DisplayFromStr>")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub support_interface_filament: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub support_interface_pattern: Option<SupportInterfacePattern>,
    #[serde_as(as = "Option<DisplayFromStr>")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub support_interface_spacing: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub support_style: Option<SupportStyle>,
    #[serde_as(as = "Option<DisplayFromStr>")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub support_top_z_distance: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub support_type: Option<SupportType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub different_settings_to_system: Option<Vec<String>>,
    #[serde_as(as = "Option<DisplayFromStr>")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub support_expansion: Option<f64>,
}

impl ProjectSettings {
    pub fn new() -> Self {
        ProjectSettings {
            filament_colour: None,
            filament_is_support: None,
            filament_settings_id: None,
            filament_shrink: None,
            filament_diameter: None,
            filament_type: None,
            flush_volumes_matrix: None,
            nozzle_diameter: None,
            print_settings_id: None,
            printable_height: None,
            printer_settings_id: None,
            enable_prime_tower: None,
            wipe_tower_x: None,
            wipe_tower_y: None,
            prime_tower_rib_wall: None,
            prime_tower_infill_gap: None,
            enable_timelapse: None,
            timelapse_type: None,
            enable_support: None,
            independent_support_layer_height: None,
            support_bottom_z_distance: None,
            support_filament: None,
            support_interface_filament: None,
            support_interface_pattern: None,
            support_interface_spacing: None,
            support_style: None,
            support_top_z_distance: None,
            support_type: None,
            different_settings_to_system: None,
            support_expansion: None,
        }
    }
}
