use itertools::Itertools;
use patina_3mf::settings_id::filament_settings_id::FilamentSettingsId;
use patina_3mf::settings_id::print_settings_id::PrintSettingsId;
use patina_3mf::settings_id::printer_settings_id::PrinterSettingsId;
use std::ffi::OsString;
use std::path::{Path, PathBuf};

#[derive(Copy, Clone)]
pub enum DebugLevel {
    Fatal = 0,
    Error = 1,
    Warning = 2,
    Info = 3,
    Debug = 4,
    Trace = 5,
}

#[derive(Copy, Clone)]
pub enum Slice {
    AllPlates,
    OnePlate(usize),
}

#[derive(Copy, Clone)]
pub enum Arrange {
    Disable = 0,
    Enable = 1,
    Auto = 2,
}

pub struct BambuStudioCommand {
    debug: Option<DebugLevel>,
    slice: Option<Slice>,
    arrange: Option<Arrange>,
    filaments: Vec<FilamentSettingsId>,
    machine: Option<PrinterSettingsId>,
    process: Option<PrintSettingsId>,
    export_3mf: Option<PathBuf>,
    input: Option<PathBuf>,
    export_slicedata: Option<PathBuf>,
    enable_timelapse: bool,
    timelapse_type: Option<usize>,
}

impl BambuStudioCommand {
    pub fn new() -> Self {
        BambuStudioCommand {
            debug: None,
            slice: None,
            arrange: None,
            filaments: vec![],
            machine: None,
            process: None,
            export_3mf: None,
            input: None,
            export_slicedata: None,
            enable_timelapse: false,
            timelapse_type: None,
        }
    }
    pub fn debug(&mut self, debug: DebugLevel) {
        self.debug = Some(debug);
    }
    pub fn slice(&mut self, slice: Slice) {
        self.slice = Some(slice);
    }
    pub fn arrange(&mut self, arrange: Arrange) {
        self.arrange = Some(arrange);
    }
    pub fn add_filament(&mut self, filament: FilamentSettingsId) {
        self.filaments.push(filament);
    }
    pub fn machine(&mut self, machine: PrinterSettingsId) {
        self.machine = Some(machine);
    }
    pub fn process(&mut self, process: PrintSettingsId) {
        self.process = Some(process);
    }
    pub fn export_3mf(&mut self, export_3mf: PathBuf) {
        self.export_3mf = Some(export_3mf);
    }
    pub fn export_slicedata(&mut self, export_slicedata: PathBuf) {
        self.export_slicedata = Some(export_slicedata);
    }
    pub fn input(&mut self, input: PathBuf) {
        self.input = Some(input);
    }
    pub fn enable_timelapse(&mut self) {
        self.enable_timelapse = true;
    }
    pub fn timelapse_type(&mut self, timelapse_type: usize) {
        self.timelapse_type = Some(timelapse_type);
    }
    pub async fn run(self) -> anyhow::Result<()> {
        let mut command = tokio::process::Command::new(
            "/Applications/BambuStudio.app/Contents/MacOS/BambuStudio",
        );
        if let Some(debug) = self.debug {
            command.arg("--debug").arg(&(debug as usize).to_string());
        }
        if let Some(slice) = self.slice {
            command.arg("--slice");
            match slice {
                Slice::AllPlates => {
                    command.arg("0");
                }
                Slice::OnePlate(plate) => {
                    command.arg("1");
                }
            }
        }
        if let Some(arrange) = &self.arrange {
            command
                .arg("--arrange")
                .arg((*arrange as usize).to_string());
        }
        let profiles_dir = Path::new("/Applications/BambuStudio.app/Contents/Resources/profiles/");
        let filament_dir = profiles_dir.join("BBL").join("filament");
        let machine_dir = profiles_dir.join("BBL").join("machine");
        let process_dir = profiles_dir.join("BBL").join("process");
        if !self.filaments.is_empty() {
            command.arg("--load-filaments");
            let mut filaments = OsString::new();
            for filament in &self.filaments {
                if !filaments.is_empty() {
                    filaments.push(";");
                }
                filaments.push(filament_dir.join(format!("{}.json", filament)));
            }
            command.arg(filaments);
        }
        let mut settings_paths = vec![];
        if let Some(machine) = &self.machine {
            settings_paths.push(
                machine_dir
                    .join(format!("{}.json", machine))
                    .into_os_string(),
            );
        }
        if let Some(process) = &self.process {
            settings_paths.push(
                process_dir
                    .join(format!("{}.json", process))
                    .into_os_string(),
            );
        }
        if !settings_paths.is_empty() {
            command.arg("--load-settings");
            let mut settings_paths_string = OsString::new();
            for settings_path in settings_paths {
                if !settings_paths_string.is_empty() {
                    settings_paths_string.push(";");
                }
                settings_paths_string.push(settings_path);
            }
            command.arg(settings_paths_string);
        }
        if let Some(export_3mf) = &self.export_3mf {
            command.arg("--export-3mf").arg(export_3mf);
        }
        if let Some(export_slicedata) = &self.export_slicedata {
            command.arg("--export-slicedata").arg(export_slicedata);
        }
        if let Some(input) = &self.input {
            command.arg(input);
        }
        if self.enable_timelapse {
            command.arg("--enable-timelapse");
        }
        if let Some(timelapse_type) = self.timelapse_type {
            command
                .arg("--timelapse-type")
                .arg(timelapse_type.to_string());
        }
        command.spawn()?.wait().await?.exit_ok()?;
        Ok(())
    }
}
