use serde::{Deserialize, Deserializer, Serializer};
use serde::de::Error;
use serde_with::{DeserializeAs, SerializeAs};

pub struct BoolFromIntString;

impl SerializeAs<bool> for BoolFromIntString {
    fn serialize_as<S>(source: &bool, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(if *source { "1" } else { "0" })
    }
}

impl<'de> DeserializeAs<'de, bool> for BoolFromIntString {
    fn deserialize_as<D>(deserializer: D) -> Result<bool, D::Error>
    where
        D: Deserializer<'de>,
    {
        match String::deserialize(deserializer)?.as_str() {
            "1" => Ok(true),
            "0" => Ok(false),
            x => Err(Error::custom(format!(
                "unexpected BoolFromIntString {}",
                x
            ))),
        }
    }
}
