use arrayvec::ArrayVec;
use itertools::Itertools;
use serde::de::Error;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::fmt::{Display, Formatter};

#[derive(Clone)]
#[non_exhaustive]
pub struct Color {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
}

impl Color {
    pub fn new(red: u8, green: u8, blue: u8) -> Self {
        Color { red, green, blue }
    }
}

impl Display for Color {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "#{:02x}{:02x}{:02x}", self.red, self.green, self.blue)
    }
}

impl Serialize for Color {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.collect_str(&self)
    }
}

impl<'de> Deserialize<'de> for Color {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let str = String::deserialize(deserializer)?;
        let str = str
            .strip_prefix("#")
            .ok_or_else(|| D::Error::custom("color does not start with #"))?;
        let [red, green, blue] = str
            .as_bytes()
            .chunks_exact(2)
            .into_iter()
            .map(|color| {
                Ok(str::from_utf8(color)
                    .map_err(D::Error::custom)?
                    .parse()
                    .map_err(D::Error::custom)?)
            })
            .collect::<Result<ArrayVec<u8, 3>, D::Error>>()?
            .into_inner()
            .map_err(|_| D::Error::custom("color not correct length"))?;
        Ok(Color { red, green, blue })
    }
}
