use serde::{Deserializer, Serialize, Serializer};
use serde_with::{DeserializeAs, SerializeAs};
use std::marker::PhantomData;

pub struct OptionOrNil<T>(PhantomData<T>);

impl<T, SA> SerializeAs<Option<T>> for OptionOrNil<SA>
where
    SA: SerializeAs<T>,
{
    fn serialize_as<S>(value: &Option<T>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        if let Some(value) = value {
            SA::serialize_as(value, serializer)
        } else {
            "nil".serialize(serializer)
        }
    }
}

impl<'de, T, SA> DeserializeAs<'de, Option<T>> for OptionOrNil<SA>
where
    SA: DeserializeAs<'de, T>,
{
    fn deserialize_as<D>(deserializer: D) -> Result<Option<T>, D::Error>
    where
        D: Deserializer<'de>,
    {
        todo!()
    }
}
