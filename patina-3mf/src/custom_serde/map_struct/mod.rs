mod de;
mod ser;
#[cfg(test)]
mod test;

use serde::de::value::{CowStrDeserializer, StrDeserializer};
use serde::de::{DeserializeSeed, Error, MapAccess, SeqAccess, Visitor};
use serde::ser::{Impossible, SerializeSeq, SerializeStruct};
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use serde_with::BorrowCow;
use serde_with::{DeserializeAs, SerializeAs, serde_as};
use std::borrow::Cow;
use std::fmt::Formatter;
use std::marker::PhantomData;

pub struct MapStruct<T>(PhantomData<T>);

pub trait MapStructKeys {
    const NAME: &'static str;
    const VALUE: &'static str;
}
