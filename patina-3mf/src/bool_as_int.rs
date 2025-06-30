use serde::{Deserialize, Deserializer, Serialize, Serializer};

pub fn serialize<S: Serializer>(b: &Option<bool>, ser: S) -> Result<S::Ok, S::Error> {
    b.map(|x| x as u8).serialize(ser)
}
pub fn deserialize<'de, D: Deserializer<'de>>(d: D) -> Result<Option<bool>, D::Error> {
    Ok(Option::<u8>::deserialize(d)?.map(|x| x != 0))
}