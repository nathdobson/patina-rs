use crate::settings_id::nozzle::Nozzle;
use crate::settings_id::printer::{MachinePrinter, Printer};
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use serde_with::SerializeAs;
use std::fmt::{Display, Formatter};

#[derive(Clone)]
pub struct PrinterSettingsId {
    pub printer: Printer,
    pub nozzle: Option<Nozzle>,
}

impl PrinterSettingsId {
    pub fn new(printer: Printer) -> Self {
        PrinterSettingsId {
            printer,
            nozzle: None,
        }
    }
}

impl Display for PrinterSettingsId {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        MachinePrinter::serialize_as(&self.printer, &mut *f)?;
        if let Some(nozzle) = &self.nozzle {
            write!(f, " ")?;
            nozzle.serialize(&mut *f)?;
        }
        Ok(())
    }
}

impl Serialize for PrinterSettingsId {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.collect_str(self)
    }
}

impl<'de> Deserialize<'de> for PrinterSettingsId {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        todo!()
    }
}
