use serde::de::Error;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::fmt::Display;
use std::str::FromStr;

pub fn serialize<S: Serializer, T: Display>(b: &Option<Vec<T>>, ser: S) -> Result<S::Ok, S::Error> {
    b.as_ref()
        .map(|x| x.iter().map(|x| x.to_string()).collect::<Vec<_>>())
        .serialize(ser)
}
pub fn deserialize<'de, D: Deserializer<'de>, T: FromStr>(d: D) -> Result<Option<Vec<T>>, D::Error>
where
    <T as FromStr>::Err: Display,
{
    Option::<Vec<String>>::deserialize(d)?.map(|x| {
        x.into_iter()
            .map(|x| x.parse().map_err(D::Error::custom))
            .collect::<Result<Vec<T>, D::Error>>()
    }).transpose()
}
