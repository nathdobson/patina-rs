use crate::settings_id::nozzle::Nozzle;
use crate::settings_id::printer::{FilamentPrinter, Printer};
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use serde_with::SerializeAs;
use std::fmt::{Display, Formatter};

#[derive(Serialize, Deserialize, Clone)]
pub enum PrintQuality {
    Fine,
    #[serde(rename = "High Quality")]
    HighQuality,
    Standard,
    #[serde(rename = "Extra Fine")]
    ExtraFine,
    Optimal,
    #[serde(rename = "Balanced Quality")]
    BalancedQuality,
    Draft,
    #[serde(rename = "Extra Draft")]
    ExtraDraft,
    #[serde(rename = "Balanced Strength")]
    BalancedStrength,
    #[serde(rename = "Strength")]
    Strength,
}

#[derive(Clone)]
pub struct PrintSettingsId {
    layer_height: f64,
    quality: PrintQuality,
    printer: Printer,
    nozzle: Nozzle,
}

impl PrintSettingsId {
    pub fn new(layer_height: f64, quality: PrintQuality, printer: Printer, nozzle: Nozzle) -> Self {
        PrintSettingsId {
            layer_height,
            quality,
            printer,
            nozzle,
        }
    }
}

impl Display for PrintSettingsId {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:.2}mm ", self.layer_height)?;
        self.quality.serialize(&mut *f)?;
        write!(f, " @BBL ")?;
        FilamentPrinter::serialize_as(&self.printer, &mut *f)?;
        if self.nozzle != Nozzle::Nozzle0_4 {
            write!(f, " ")?;
            self.nozzle.serialize(&mut *f)?;
        }
        Ok(())
    }
}

impl Serialize for PrintSettingsId {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.collect_str(self)
    }
}

impl<'de> Deserialize<'de> for PrintSettingsId {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        todo!()
    }
}
