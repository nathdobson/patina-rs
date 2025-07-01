#![feature(iter_array_chunks)]
#![allow(dead_code, unused_imports, unused_mut, unused_variables)]

mod bool_from_int_string;
pub mod color;
pub mod content_types;
pub mod model;
pub mod model_settings;
pub mod project_settings;
pub mod relationships;
pub mod settings_id;
pub mod xmlns;
mod option_or_nil;

use crate::content_types::ContentTypes;
use crate::model::Model;
use crate::model_settings::ModelSettings;
use crate::project_settings::ProjectSettings;
use crate::relationships::Relationships;
use serde::{Deserialize, Serialize};
#[deny(unused_must_use)]
use std::io::{Cursor, Write};
use xml::EmitterConfig;
use zip::ZipWriter;
use zip::write::SimpleFileOptions;

// const PROJECT_SETTINGS:&[u8]=br##"{
//     "filament_settings_id": [
//         "Bambu PLA Matte @BBL A1M",
//         "Bambu PLA Basic @BBL A1M",
//         "Bambu Support For PLA @BBL A1M"
//     ],
//     "flush_volumes_matrix": [
//         "0",
//         "149",
//         "230",
//         "465",
//         "0",
//         "230",
//         "700",
//         "700",
//         "0"
//     ],
//     "nozzle_diameter": [
//         "0.4"
//     ],
//     "print_settings_id": "0.20mm Standard @BBL A1M",
//     "printable_height": "180",
//     "printer_settings_id": "Bambu Lab A1 mini 0.4 nozzle",
//     "role_base_wipe_speed": "1",
//     "scan_first_layer": "0",
//     "scarf_angle_threshold": "155",
//     "seam_gap": "15%",
//     "seam_position": "aligned",
//     "seam_slope_conditional": "1",
//     "seam_slope_entire_loop": "0",
//     "seam_slope_inner_walls": "1",
//     "seam_slope_steps": "10",
//     "silent_mode": "0",
//     "single_extruder_multi_material": "1",
//     "skeleton_infill_density": "15%",
//     "skeleton_infill_line_width": "0.45",
//     "skin_infill_density": "15%",
//     "skin_infill_depth": "2",
//     "skin_infill_line_width": "0.45",
//     "skirt_distance": "2",
//     "skirt_height": "1",
//     "skirt_loops": "0",
//     "slice_closing_radius": "0.049",
//     "slicing_mode": "regular",
//     "slow_down_for_layer_cooling": [
//         "1",
//         "1",
//         "1"
//     ],
//     "slow_down_layer_time": [
//         "6",
//         "6",
//         "8"
//     ],
//     "slow_down_min_speed": [
//         "20",
//         "20",
//         "20"
//     ],
//     "small_perimeter_speed": [
//         "50%"
//     ],
//     "small_perimeter_threshold": [
//         "0"
//     ],
//     "smooth_coefficient": "80",
//     "smooth_speed_discontinuity_area": "1",
//     "solid_infill_filament": "1",
//     "sparse_infill_acceleration": [
//         "100%"
//     ],
//     "sparse_infill_anchor": "400%",
//     "sparse_infill_anchor_max": "20",
//     "sparse_infill_density": "15%",
//     "sparse_infill_filament": "1",
//     "sparse_infill_line_width": "0.45",
//     "sparse_infill_pattern": "grid",
//     "sparse_infill_speed": [
//         "270"
//     ],
//     "spiral_mode": "0",
//     "spiral_mode_max_xy_smoothing": "200%",
//     "spiral_mode_smooth": "0",
//     "standby_temperature_delta": "-5",
//     "start_end_points": [
//         "30x-3",
//         "54x245"
//     ],
//     "supertack_plate_temp": [
//         "45",
//         "45",
//         "45"
//     ],
//     "supertack_plate_temp_initial_layer": [
//         "45",
//         "45",
//         "45"
//     ],
//     "support_air_filtration": "0",
//     "support_angle": "0",
//     "support_base_pattern": "default",
//     "support_base_pattern_spacing": "2.5",
//     "support_bottom_interface_spacing": "0.5",
//     "support_bottom_z_distance": "0.2",
//     "support_chamber_temp_control": "0",
//     "support_critical_regions_only": "0",
//     "support_expansion": "0",
//     "support_filament": "0",
//     "support_interface_bottom_layers": "2",
//     "support_interface_filament": "0",
//     "support_interface_loop_pattern": "0",
//     "support_interface_not_for_body": "1",
//     "support_interface_pattern": "auto",
//     "support_interface_spacing": "0.5",
//     "support_interface_speed": [
//         "80"
//     ],
//     "support_interface_top_layers": "2",
//     "support_line_width": "0.42",
//     "support_object_first_layer_gap": "0.2",
//     "support_object_xy_distance": "0.35",
//     "support_on_build_plate_only": "0",
//     "support_remove_small_overhang": "1",
//     "support_speed": [
//         "150"
//     ],
//     "support_style": "default",
//     "support_threshold_angle": "30",
//     "support_top_z_distance": "0.2",
//     "support_type": "tree(auto)",
//     "symmetric_infill_y_axis": "0",
//     "temperature_vitrification": [
//         "45",
//         "45",
//         "45"
//     ],
//     "template_custom_gcode": "",
//     "textured_plate_temp": [
//         "65",
//         "65",
//         "60"
//     ],
//     "textured_plate_temp_initial_layer": [
//         "65",
//         "65",
//         "60"
//     ],
//     "thick_bridges": "0",
//     "thumbnail_size": [
//         "50x50"
//     ],
//     "time_lapse_gcode": ";===================== date: 20250206 =====================\n{if !spiral_mode && print_sequence != \"by object\"}\n; don't support timelapse gcode in spiral_mode and by object sequence for I3 structure printer\n; SKIPPABLE_START\n; SKIPTYPE: timelapse\nM622.1 S1 ; for prev firware, default turned on\nM1002 judge_flag timelapse_record_flag\nM622 J1\nG92 E0\nG1 Z{max_layer_z + 0.4}\nG1 X0 Y{first_layer_center_no_wipe_tower[1]} F18000 ; move to safe pos\nG1 X-13.0 F3000 ; move to safe pos\nM400\nM1004 S5 P1  ; external shutter\nM400 P300\nM971 S11 C11 O0\nG92 E0\nG1 X0 F18000\nM623\n\n; SKIPTYPE: head_wrap_detect\nM622.1 S1\nM1002 judge_flag g39_3rd_layer_detect_flag\nM622 J1\n    ; enable nozzle clog detect at 3rd layer\n    {if layer_num == 2}\n      M400\n      G90\n      M83\n      M204 S5000\n      G0 Z2 F4000\n      G0 X187 Y178 F20000\n      G39 S1 X187 Y178\n      G0 Z2 F4000\n    {endif}\n\n\n    M622.1 S1\n    M1002 judge_flag g39_detection_flag\n    M622 J1\n      {if !in_head_wrap_detect_zone}\n        M622.1 S0\n        M1002 judge_flag g39_mass_exceed_flag\n        M622 J1\n        {if layer_num > 2}\n            G392 S0\n            M400\n            G90\n            M83\n            M204 S5000\n            G0 Z{max_layer_z + 0.4} F4000\n            G39.3 S1\n            G0 Z{max_layer_z + 0.4} F4000\n            G392 S0\n          {endif}\n        M623\n    {endif}\n    M623\nM623\n; SKIPPABLE_END\n{endif}\n\n\n",
//     "timelapse_type": "0",
//     "top_area_threshold": "200%",
//     "top_color_penetration_layers": "5",
//     "top_one_wall_type": "all top",
//     "top_shell_layers": "5",
//     "top_shell_thickness": "1",
//     "top_solid_infill_flow_ratio": "1",
//     "top_surface_acceleration": [
//         "2000"
//     ],
//     "top_surface_jerk": "9",
//     "top_surface_line_width": "0.42",
//     "top_surface_pattern": "monotonicline",
//     "top_surface_speed": [
//         "200"
//     ],
//     "travel_acceleration": [
//         "10000"
//     ],
//     "travel_jerk": "9",
//     "travel_speed": [
//         "700"
//     ],
//     "travel_speed_z": [
//         "0"
//     ],
//     "tree_support_branch_angle": "45",
//     "tree_support_branch_diameter": "2",
//     "tree_support_branch_diameter_angle": "5",
//     "tree_support_branch_distance": "5",
//     "tree_support_wall_count": "0",
//     "upward_compatible_machine": [
//         "Bambu Lab P1S 0.4 nozzle",
//         "Bambu Lab P1P 0.4 nozzle",
//         "Bambu Lab X1 0.4 nozzle",
//         "Bambu Lab X1 Carbon 0.4 nozzle",
//         "Bambu Lab X1E 0.4 nozzle",
//         "Bambu Lab A1 0.4 nozzle",
//         "Bambu Lab H2D 0.4 nozzle"
//     ],
//     "use_firmware_retraction": "0",
//     "use_relative_e_distances": "1",
//     "version": "02.01.01.52",
//     "vertical_shell_speed": [
//         "80%"
//     ],
//     "wall_distribution_count": "1",
//     "wall_filament": "1",
//     "wall_generator": "classic",
//     "wall_loops": "2",
//     "wall_sequence": "inner wall/outer wall",
//     "wall_transition_angle": "10",
//     "wall_transition_filter_deviation": "25%",
//     "wall_transition_length": "100%",
//     "wipe": [
//         "1"
//     ],
//     "wipe_distance": [
//         "2"
//     ],
//     "wipe_speed": "80%",
//     "wipe_tower_no_sparse_layers": "0",
//     "wipe_tower_rotation_angle": "0",
//     "wipe_tower_x": [
//         "5"
//     ],
//     "wipe_tower_y": [
//         "5"
//     ],
//     "xy_contour_compensation": "0",
//     "xy_hole_compensation": "0",
//     "z_direction_outwall_speed_continuous": "0",
//     "z_hop": [
//         "0.4"
//     ],
//     "z_hop_types": [
//         "Auto Lift"
//     ]
// }
// "##;
const PROJECT_SETTINGS:&[u8]=br##"{
    "filament_colour": [
        "#ffffff",
        "#558c4c",
        "#ffffff"
    ],
    "filament_is_support": [
        "0",
        "0",
        "1"
    ],
    "filament_settings_id": [
        "Bambu PLA Matte @BBL A1M",
        "Bambu PLA Basic @BBL A1M",
        "Bambu Support For PLA @BBL A1M"
    ],
    "filament_shrink": [
        "100%",
        "100%",
        "100%"
    ],
    "filament_diameter": [
        "1.75",
        "1.75",
        "1.75"
    ],
    "flush_volumes_matrix": [
        "0",
        "135",
        "230",
        "445",
        "0",
        "230",
        "700",
        "700",
        "0"
    ],
    "nozzle_diameter": [
        "0.4"
    ],
    "print_settings_id": "0.20mm Standard @BBL A1M",
    "printable_height": "180",
    "printer_settings_id": "Bambu Lab A1 mini 0.4 nozzle",
    "wipe_tower_x": [
        "5"
    ],
    "wipe_tower_y": [
        "5"
    ]
}
"##;

#[non_exhaustive]
pub struct ModelContainer {
    pub model: Model,
    pub content_types: Option<ContentTypes>,
    pub relationships: Option<Relationships>,
    pub model_settings: Option<ModelSettings>,
    pub project_settings: Option<ProjectSettings>,
}

impl ModelContainer {
    pub fn new(model: Model) -> Self {
        ModelContainer {
            model,
            content_types: None,
            relationships: None,
            model_settings: None,
            project_settings: None,
        }
    }
    pub fn content_types(mut self, content_types: Option<ContentTypes>) -> Self {
        self.content_types = content_types;
        self
    }
    pub fn relationships(mut self, relationships: Option<Relationships>) -> Self {
        self.relationships = relationships;
        self
    }
    pub fn model_settings(mut self, model_settings: Option<ModelSettings>) -> Self {
        self.model_settings = model_settings;
        self
    }
    pub fn project_settings(mut self, project_settings: Option<ProjectSettings>) -> Self {
        self.project_settings = project_settings;
        self
    }
    fn to_xml_string<T: Serialize>(&self, value: &T) -> anyhow::Result<String> {
        Ok(serde_xml_rs::SerdeXml::new()
            .emitter(EmitterConfig::new().perform_indent(true))
            .to_string(value)?)
    }
    pub fn encode(&self) -> anyhow::Result<Vec<u8>> {
        let mut buffer = vec![];
        let mut zip = ZipWriter::new(Cursor::new(&mut buffer));
        let opts = SimpleFileOptions::default();
        if let Some(content_types) = &self.content_types {
            zip.start_file("[Content_Types].xml", opts.clone())?;
            zip.write_all(self.to_xml_string(content_types)?.as_bytes())?;
        }
        if let Some(relationships) = &self.relationships {
            zip.add_directory("_rels", opts.clone())?;
            zip.start_file("_rels/.rels", opts.clone())?;
            zip.write_all(self.to_xml_string(relationships)?.as_bytes())?;
        }
        zip.add_directory("3D/", opts.clone())?;
        zip.start_file("3D/3dmodel.model", opts.clone())?;
        zip.write_all(self.to_xml_string(&self.model)?.as_bytes())?;
        zip.add_directory("Metadata", opts.clone())?;
        if let Some(model_settings) = &self.model_settings {
            zip.start_file("Metadata/model_settings.config", opts.clone())?;
            zip.write_all(self.to_xml_string(model_settings)?.as_bytes())?;
        }
        if let Some(project_settings) = &self.project_settings {
            zip.start_file("Metadata/project_settings.config", opts.clone())?;
            zip.write_all(serde_json::to_string_pretty(project_settings)?.as_bytes())?;
            // zip.write_all(PROJECT_SETTINGS)?;
        }
        zip.finish()?;
        Ok(buffer)
    }
}
