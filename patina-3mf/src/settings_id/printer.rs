use serde::{Deserialize, Deserializer, Serialize, Serializer};
use serde_with::{DeserializeAs, SerializeAs};

#[derive(Clone)]
#[non_exhaustive]
pub enum Printer {
    A1,
    A1Mini,
    H2D,
    P1P,
    X1,
    X1Carbon,
    X1E,
}

pub struct MachinePrinter {}
pub struct FilamentPrinter {}

impl SerializeAs<Printer> for MachinePrinter {
    fn serialize_as<S>(source: &Printer, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(match source {
            Printer::A1 => "Bambu Lab A1",
            Printer::A1Mini => "Bambu Lab A1 mini",
            Printer::H2D => "Bambu Lab H2D",
            Printer::P1P => "Bambu Lab P1P",
            Printer::X1 => "Bambu Lab X1",
            Printer::X1Carbon => "Bambu Lab X1 Carbon",
            Printer::X1E => "Bambu Lab X1E",
        })
    }
}

impl SerializeAs<Printer> for FilamentPrinter {
    fn serialize_as<S>(source: &Printer, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(match source {
            Printer::A1 => "A1",
            Printer::A1Mini => "A1M",
            Printer::H2D => "H2D",
            Printer::P1P => "P1P",
            Printer::X1 => "X1",
            Printer::X1Carbon => "X1C",
            Printer::X1E => "X1E",
        })
    }
}

impl<'de> DeserializeAs<'de, Printer> for FilamentPrinter {
    fn deserialize_as<D>(deserializer: D) -> Result<Printer, D::Error>
    where
        D: Deserializer<'de>,
    {
        todo!()
    }
}

impl<'de> DeserializeAs<'de, Printer> for MachinePrinter {
    fn deserialize_as<D>(deserializer: D) -> Result<Printer, D::Error>
    where
        D: Deserializer<'de>,
    {
        todo!()
    }
}
