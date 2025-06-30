use serde::{Deserialize, Deserializer, Serialize, Serializer};

pub fn serialize<S: Serializer>(value: &Option<Vec<bool>>, ser: S) -> Result<S::Ok, S::Error> {
    value
        .as_ref()
        .map(|x| {
            x.iter()
                .map(|b| if *b { "1" } else { "0" })
                .collect::<Vec<_>>()
        })
        .serialize(ser)
}
pub fn deserialize<'de, D: Deserializer<'de>>(d: D) -> Result<Option<Vec<bool>>, D::Error> {
    Ok(Option::<Vec<String>>::deserialize(d)?.map(|x| x.into_iter().map(|x| x == "1").collect()))
}
