use crate::custom_serde::map_struct::{MapStruct, MapStructKeys};
use serde::de::value::CowStrDeserializer;
use serde::de::{DeserializeSeed, Error, IgnoredAny, MapAccess, SeqAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_cow::CowStr;
use serde_with::BorrowCow;
use serde_with::{DeserializeAs, serde_as};
use std::borrow::Cow;
use std::fmt::Formatter;
use std::marker::PhantomData;

struct MapStructDeserializer<KS, D> {
    inner: D,
    phantom: PhantomData<KS>,
}

struct MapStructMapAccess<'de, KS, SA> {
    inner: SA,
    value: Option<Cow<'de, str>>,
    phantom: PhantomData<KS>,
}

impl<'de, D: Deserializer<'de>, KS: MapStructKeys> Deserializer<'de>
    for MapStructDeserializer<KS, D>
{
    type Error = D::Error;

    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        Err(Error::custom("deserialize_any not supported"))
    }

    fn deserialize_bool<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        Err(Error::custom("deserialize_bool not supported"))
    }

    fn deserialize_i8<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        Err(Error::custom("deserialize_i8 not supported"))
    }

    fn deserialize_i16<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        Err(Error::custom("deserialize_i16 not supported"))
    }

    fn deserialize_i32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        Err(Error::custom("deserialize_i32 not supported"))
    }

    fn deserialize_i64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        Err(Error::custom("deserialize_i64 not supported"))
    }

    fn deserialize_u8<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        Err(Error::custom("deserialize_u8 not supported"))
    }

    fn deserialize_u16<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        Err(Error::custom("deserialize_u16 not supported"))
    }

    fn deserialize_u32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        Err(Error::custom("deserialize_u32 not supported"))
    }

    fn deserialize_u64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        Err(Error::custom("deserialize_u64 not supported"))
    }

    fn deserialize_f32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        Err(Error::custom("deserialize_f32 not supported"))
    }

    fn deserialize_f64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        Err(Error::custom("deserialize_f64 not supported"))
    }

    fn deserialize_char<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        Err(Error::custom("deserialize_char not supported"))
    }

    fn deserialize_str<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        Err(Error::custom("deserialize_str not supported"))
    }

    fn deserialize_string<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        Err(Error::custom("deserialize_string not supported"))
    }

    fn deserialize_bytes<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        Err(Error::custom("deserialize_bytes not supported"))
    }

    fn deserialize_byte_buf<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        Err(Error::custom("deserialize_byte_buf not supported"))
    }

    fn deserialize_option<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        Err(Error::custom("deserialize_option not supported"))
    }

    fn deserialize_unit<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        Err(Error::custom("deserialize_unit not supported"))
    }

    fn deserialize_unit_struct<V>(
        self,
        name: &'static str,
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        Err(Error::custom("deserialize_unit_struct not supported"))
    }

    fn deserialize_newtype_struct<V>(
        self,
        name: &'static str,
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        Err(Error::custom("deserialize_newtype_struct not supported"))
    }

    fn deserialize_seq<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        Err(Error::custom("deserialize_seq not supported"))
    }

    fn deserialize_tuple<V>(self, len: usize, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        Err(Error::custom("deserialize_tuple not supported"))
    }

    fn deserialize_tuple_struct<V>(
        self,
        name: &'static str,
        len: usize,
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        Err(Error::custom("deserialize_tuple_struct not supported"))
    }

    fn deserialize_map<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        Err(Error::custom("deserialize_map not supported"))
    }

    fn deserialize_struct<V>(
        self,
        name: &'static str,
        fields: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        struct SeqVisitor<KS, V> {
            map_visitor: V,
            phantom: PhantomData<KS>,
        }
        impl<'de, KS: MapStructKeys, V: Visitor<'de>> Visitor<'de> for SeqVisitor<KS, V> {
            type Value = V::Value;

            fn expecting(&self, formatter: &mut Formatter) -> std::fmt::Result {
                write!(formatter, "a sequence of tags with key/value attributes")
            }

            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: SeqAccess<'de>,
            {
                self.map_visitor.visit_map(MapStructMapAccess {
                    inner: seq,
                    value: None,
                    phantom: PhantomData::<KS>,
                })
            }
        }
        self.inner.deserialize_seq(SeqVisitor {
            map_visitor: visitor,
            phantom: PhantomData::<KS>,
        })
    }

    fn deserialize_enum<V>(
        self,
        name: &'static str,
        variants: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        Err(Error::custom("deserialize_enum not supported"))
    }

    fn deserialize_identifier<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        Err(Error::custom("deserialize_identifier not supported"))
    }

    fn deserialize_ignored_any<V>(mut self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.inner.deserialize_ignored_any(visitor)
    }
}

struct NameValuePair<'a, KS: MapStructKeys> {
    name: Cow<'a, str>,
    value: Cow<'a, str>,
    phantom: PhantomData<KS>,
}

impl<'de, KS: MapStructKeys> Deserialize<'de> for NameValuePair<'de, KS> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct V<KS>(PhantomData<KS>);
        impl<'de, KS: MapStructKeys> Visitor<'de> for V<KS> {
            type Value = NameValuePair<'de, KS>;

            fn expecting(&self, formatter: &mut Formatter) -> std::fmt::Result {
                write!(formatter, "a struct with key/value attributes")
            }
            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
            where
                A: MapAccess<'de>,
            {
                let mut name = None;
                let mut value = None;
                while let Some(k) = map.next_key::<CowStr>()? {
                    if k.0 == KS::NAME {
                        name = Some(map.next_value()?);
                    } else if k.0 == KS::VALUE {
                        value = Some(map.next_value()?);
                    } else {
                        map.next_value::<IgnoredAny>()?;
                    }
                }
                if let (Some(name), Some(value)) = (name, value) {
                    Ok(NameValuePair {
                        name,
                        value,
                        phantom: Default::default(),
                    })
                } else {
                    Err(A::Error::custom("missing field(s)"))
                }
            }
        }
        deserializer.deserialize_struct("KeyValuePair", &[KS::NAME, KS::VALUE], V(PhantomData))
    }
}

impl<'de, KS: MapStructKeys, SA: SeqAccess<'de>> MapAccess<'de>
    for MapStructMapAccess<'de, KS, SA>
{
    type Error = SA::Error;

    fn next_key_seed<K>(&mut self, seed: K) -> Result<Option<K::Value>, Self::Error>
    where
        K: DeserializeSeed<'de>,
    {
        if let Some(kvp) = self.inner.next_element::<NameValuePair<'de, KS>>()? {
            self.value = Some(kvp.value);
            Ok(Some(seed.deserialize(CowStrDeserializer::new(kvp.name))?))
        } else {
            Ok(None)
        }
    }

    fn next_value_seed<V>(&mut self, seed: V) -> Result<V::Value, Self::Error>
    where
        V: DeserializeSeed<'de>,
    {
        Ok(seed.deserialize(CowStrDeserializer::new(
            self.value
                .take()
                .ok_or_else(|| Error::custom("must retrieve key first"))?,
        ))?)
    }
}

impl<'de, KS: MapStructKeys, T: Deserialize<'de>> DeserializeAs<'de, T> for MapStruct<KS> {
    fn deserialize_as<D>(deserializer: D) -> Result<T, D::Error>
    where
        D: Deserializer<'de>,
    {
        T::deserialize(MapStructDeserializer {
            inner: deserializer,
            phantom: PhantomData::<KS>,
        })
    }
}
