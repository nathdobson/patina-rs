use crate::bool_from_int_string::BoolFromIntString;
use crate::color::Color;
use crate::option_or_nil::OptionOrNil;
use crate::settings_id::filament_settings_id::FilamentSettingsId;
use crate::settings_id::print_settings_id::PrintSettingsId;
use crate::settings_id::printer_settings_id::PrinterSettingsId;
use serde::de::Error;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use serde_with::DisplayFromStr;
use serde_with::{DeserializeAs, SerializeAs, serde_as};

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
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde_as(as = "Option<DisplayFromStr>")]
    pub timelapse_type: Option<usize>,
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
        }
    }
    pub fn filament_colour(mut self, filament_colour: Option<Vec<Option<Color>>>) -> Self {
        self.filament_colour = filament_colour;
        self
    }
    pub fn filament_is_support(mut self, filament_support: Option<Vec<Option<bool>>>) -> Self {
        self.filament_is_support = filament_support;
        self
    }
    pub fn filament_settings_id(
        mut self,
        filament_settings_id: Option<Vec<Option<FilamentSettingsId>>>,
    ) -> Self {
        self.filament_settings_id = filament_settings_id;
        self
    }
    pub fn filament_shrink(mut self, filament_shrink: Option<Vec<Option<String>>>) -> Self {
        self.filament_shrink = filament_shrink;
        self
    }
    pub fn filament_diameter(mut self, filament_diameter: Option<Vec<Option<f64>>>) -> Self {
        self.filament_diameter = filament_diameter;
        self
    }
    pub fn filament_type(mut self, filament_type: Option<Vec<String>>) -> Self {
        self.filament_type = filament_type;
        self
    }
    pub fn flush_volumes_matrix(mut self, flush_volumes_matrix: Option<Vec<f64>>) -> Self {
        self.flush_volumes_matrix = flush_volumes_matrix;
        self
    }
    pub fn nozzle_diameter(mut self, nozzle_diameter: Option<Vec<f64>>) -> Self {
        self.nozzle_diameter = nozzle_diameter;
        self
    }
    pub fn print_settings_id(mut self, print_settings_id: Option<PrintSettingsId>) -> Self {
        self.print_settings_id = print_settings_id;
        self
    }
    pub fn printable_height(mut self, printable_height: Option<f64>) -> Self {
        self.printable_height = printable_height;
        self
    }
    pub fn printer_settings_id(mut self, printer_settings_id: Option<PrinterSettingsId>) -> Self {
        self.printer_settings_id = printer_settings_id;
        self
    }
    pub fn enable_prime_tower(mut self, enable_prime_tower: Option<bool>) -> Self {
        self.enable_prime_tower = enable_prime_tower;
        self
    }
    pub fn wipe_tower_x(mut self, wipe_tower_x: Option<Vec<f64>>) -> Self {
        self.wipe_tower_x = wipe_tower_x;
        self
    }
    pub fn wipe_tower_y(mut self, wipe_tower_y: Option<Vec<f64>>) -> Self {
        self.wipe_tower_y = wipe_tower_y;
        self
    }
    pub fn prime_tower_rib_wall(mut self, prime_tower_rib_wall: Option<bool>) -> Self {
        self.prime_tower_rib_wall = prime_tower_rib_wall;
        self
    }
    pub fn prime_tower_infill_gap(mut self, prime_tower_infill_gap: Option<String>) -> Self {
        self.prime_tower_infill_gap = prime_tower_infill_gap;
        self
    }
    pub fn enable_timelapse(mut self, enable_timelapse: Option<bool>) -> Self {
        self.enable_timelapse = enable_timelapse;
        self
    }
    pub fn timelapse_type(mut self, timelapse_type: Option<usize>) -> Self {
        self.timelapse_type = timelapse_type;
        self
    }
}
