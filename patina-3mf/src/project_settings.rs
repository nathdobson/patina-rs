use crate::bool_from_int_string::BoolFromIntString;
use serde::de::Error;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use serde_with::{DeserializeAs, SerializeAs, serde_as};
use serde_with::DisplayFromStr;

#[serde_as]
#[derive(Serialize, Deserialize)]
#[non_exhaustive]
pub struct ProjectSettings {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filament_colour: Option<Vec<String>>,
    #[serde_as(as = "Option<Vec<BoolFromIntString>>")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filament_is_support: Option<Vec<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filament_settings_id: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filament_type: Option<Vec<String>>,
    #[serde_as(as = "Option<Vec<DisplayFromStr>>")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flush_volumes_matrix: Option<Vec<f64>>,
    #[serde_as(as = "Option<Vec<DisplayFromStr>>")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nozzle_diameter: Option<Vec<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub print_settings_id: Option<String>,
    #[serde_as(as = "Option<DisplayFromStr>")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub printable_height: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub printer_settings_id: Option<String>,
    #[serde_as(as = "Option<BoolFromIntString>")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_prime_tower: Option<bool>,
    #[serde_as(as = "Option<DisplayFromStr>")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wipe_tower_x: Option<f64>,
    #[serde_as(as = "Option<DisplayFromStr>")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wipe_tower_y: Option<f64>,
}

impl ProjectSettings {
    pub fn new() -> Self {
        ProjectSettings {
            filament_colour: None,
            filament_is_support: None,
            filament_settings_id: None,
            filament_type: None,
            flush_volumes_matrix: None,
            nozzle_diameter: None,
            print_settings_id: None,
            printable_height: None,
            printer_settings_id: None,
            enable_prime_tower: None,
            wipe_tower_x: None,
            wipe_tower_y: None,
        }
    }
    pub fn filament_colour(mut self, filament_colour: Option<Vec<String>>) -> Self {
        self.filament_colour = filament_colour;
        self
    }
    pub fn filament_is_support(mut self, filament_support: Option<Vec<bool>>) -> Self {
        self.filament_is_support = filament_support;
        self
    }
    pub fn filament_settings_id(mut self, filament_settings_id: Option<Vec<String>>) -> Self {
        self.filament_settings_id = filament_settings_id;
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
    pub fn print_settings_id(mut self, print_settings_id: Option<String>) -> Self {
        self.print_settings_id = print_settings_id;
        self
    }
    pub fn printable_height(mut self, printable_height: Option<f64>) -> Self {
        self.printable_height = printable_height;
        self
    }
    pub fn printer_settings_id(mut self, printer_settings_id: Option<String>) -> Self {
        self.printer_settings_id = printer_settings_id;
        self
    }
    pub fn enable_prime_tower(mut self, enable_prime_tower: Option<bool>) -> Self {
        self.enable_prime_tower = enable_prime_tower;
        self
    }
    pub fn wipe_tower_x(mut self, wipe_tower_x: Option<f64>) -> Self {
        self.wipe_tower_x = wipe_tower_x;
        self
    }
    pub fn wipe_tower_y(mut self, wipe_tower_y: Option<f64>) -> Self {
        self.wipe_tower_y = wipe_tower_y;
        self
    }
}
