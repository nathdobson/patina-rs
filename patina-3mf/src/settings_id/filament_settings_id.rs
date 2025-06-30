use crate::settings_id::nozzle::Nozzle;
use crate::settings_id::printer::{FilamentPrinter, Printer};
use itertools::Diff;
use serde::de::Error;
use serde::de::value::StrDeserializer;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use serde_with::{DeserializeAs, SerializeAs};
use std::fmt::{Display, Formatter};

#[derive(Serialize, Deserialize, Clone)]
#[non_exhaustive]
pub enum FilamentBrand {
    Bambu,
}

#[derive(Serialize, Deserialize, Clone)]
#[non_exhaustive]
pub enum FilamentMaterial {
    #[serde(rename = "ABS")]
    Abs,
    #[serde(rename = "PLA")]
    Pla,
    #[serde(rename = "PLA Basic")]
    PlaBasic,
    #[serde(rename = "Support for PLA")]
    SupportForPla,
}

#[derive(Clone)]
#[non_exhaustive]
pub struct FilamentSettingsId {
    pub brand: FilamentBrand,
    pub material: FilamentMaterial,
    pub printer: Printer,
    pub nozzle: Option<Nozzle>,
}

impl FilamentSettingsId {
    pub fn new(
        brand: FilamentBrand,
        material: FilamentMaterial,
        printer: Printer,
        nozzle: Option<Nozzle>,
    ) -> Self {
        FilamentSettingsId {
            brand,
            material,
            printer,
            nozzle,
        }
    }
}

impl Display for FilamentSettingsId {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.brand.serialize(&mut *f)?;
        write!(f, " ")?;
        self.material.serialize(&mut *f)?;
        write!(f, " @BBL ")?;
        FilamentPrinter::serialize_as(&self.printer, &mut *f)?;
        if let Some(nozzle) = &self.nozzle {
            write!(f, " ")?;
            nozzle.serialize(&mut *f)?;
        }
        Ok(())
    }
}

impl Serialize for FilamentSettingsId {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.collect_str(self)
    }
}

impl<'de> Deserialize<'de> for FilamentSettingsId {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let input = String::deserialize(deserializer)?;
        let (brand, input) = input
            .split_once(" ")
            .ok_or_else(|| D::Error::custom("missing brand in FilamentSettingsId"))?;
        let brand = FilamentBrand::deserialize(StrDeserializer::new(brand))?;
        let (material, input) = input
            .split_once(" @BBL ")
            .ok_or_else(|| D::Error::custom("missing material in FilamentSettingsId"))?;
        let material = FilamentMaterial::deserialize(StrDeserializer::new(material))?;
        let (printer, nozzle) = if let Some((printer, nozzle)) = input.split_once(" ") {
            (printer, Some(nozzle))
        } else {
            (input, None)
        };
        let printer = FilamentPrinter::deserialize_as(StrDeserializer::new(printer))?;
        let nozzle = if let Some(nozzle) = nozzle {
            Some(Nozzle::deserialize(StrDeserializer::new(nozzle))?)
        } else {
            None
        };
        Ok(FilamentSettingsId::new(brand, material, printer, nozzle))
    }
}
