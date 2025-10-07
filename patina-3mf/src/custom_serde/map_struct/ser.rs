use crate::custom_serde::map_struct::{MapStruct, MapStructKeys};
use serde::ser::{Error, Impossible, SerializeSeq, SerializeStruct};
use serde::{Deserialize, Serialize, Serializer};
use serde_with::SerializeAs;
use std::marker::PhantomData;

struct MapStructSerializeStruct<KS, SS> {
    inner: SS,
    phantom: PhantomData<KS>,
}
struct MapStructSerializer<KS, S> {
    inner: S,
    phantom: PhantomData<KS>,
}

impl<KS: MapStructKeys, S: Serializer> Serializer for MapStructSerializer<KS, S> {
    type Ok = S::Ok;
    type Error = S::Error;
    type SerializeSeq = Impossible<S::Ok, S::Error>;
    type SerializeTuple = Impossible<S::Ok, S::Error>;
    type SerializeTupleStruct = Impossible<S::Ok, S::Error>;
    type SerializeTupleVariant = Impossible<S::Ok, S::Error>;
    type SerializeMap = Impossible<S::Ok, S::Error>;
    type SerializeStruct = MapStructSerializeStruct<KS, S::SerializeSeq>;
    type SerializeStructVariant = Impossible<S::Ok, S::Error>;

    fn serialize_bool(self, v: bool) -> Result<Self::Ok, Self::Error> {
        Err(Error::custom("serialize_bool not supported"))
    }

    fn serialize_i8(self, v: i8) -> Result<Self::Ok, Self::Error> {
        Err(Error::custom("serialize_i8 not supported"))
    }

    fn serialize_i16(self, v: i16) -> Result<Self::Ok, Self::Error> {
        Err(Error::custom("serialize_i16 not supported"))
    }

    fn serialize_i32(self, v: i32) -> Result<Self::Ok, Self::Error> {
        Err(Error::custom("serialize_i32 not supported"))
    }

    fn serialize_i64(self, v: i64) -> Result<Self::Ok, Self::Error> {
        Err(Error::custom("serialize_i64 not supported"))
    }

    fn serialize_u8(self, v: u8) -> Result<Self::Ok, Self::Error> {
        Err(Error::custom("serialize_u8 not supported"))
    }

    fn serialize_u16(self, v: u16) -> Result<Self::Ok, Self::Error> {
        Err(Error::custom("serialize_u16 not supported"))
    }

    fn serialize_u32(self, v: u32) -> Result<Self::Ok, Self::Error> {
        Err(Error::custom("serialize_u32 not supported"))
    }

    fn serialize_u64(self, v: u64) -> Result<Self::Ok, Self::Error> {
        Err(Error::custom("serialize_u64 not supported"))
    }

    fn serialize_f32(self, v: f32) -> Result<Self::Ok, Self::Error> {
        Err(Error::custom("serialize_f32 not supported"))
    }

    fn serialize_f64(self, v: f64) -> Result<Self::Ok, Self::Error> {
        Err(Error::custom("serialize_f64 not supported"))
    }

    fn serialize_char(self, v: char) -> Result<Self::Ok, Self::Error> {
        Err(Error::custom("serialize_char not supported"))
    }

    fn serialize_str(self, v: &str) -> Result<Self::Ok, Self::Error> {
        Err(Error::custom("serialize_str not supported"))
    }

    fn serialize_bytes(self, v: &[u8]) -> Result<Self::Ok, Self::Error> {
        Err(Error::custom("serialize_bytes not supported"))
    }

    fn serialize_none(self) -> Result<Self::Ok, Self::Error> {
        Err(Error::custom("serialize_none not supported"))
    }

    fn serialize_some<T>(self, value: &T) -> Result<Self::Ok, Self::Error>
    where
        T: ?Sized + Serialize,
    {
        Err(Error::custom("serialize_some not supported"))
    }

    fn serialize_unit(self) -> Result<Self::Ok, Self::Error> {
        Err(Error::custom("serialize_unit not supported"))
    }

    fn serialize_unit_struct(self, name: &'static str) -> Result<Self::Ok, Self::Error> {
        Err(Error::custom("serialize_unit_struct not supported"))
    }

    fn serialize_unit_variant(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
    ) -> Result<Self::Ok, Self::Error> {
        Err(Error::custom("serialize_unit_variant not supported"))
    }

    fn serialize_newtype_struct<T>(
        self,
        name: &'static str,
        value: &T,
    ) -> Result<Self::Ok, Self::Error>
    where
        T: ?Sized + Serialize,
    {
        Err(Error::custom("serialize_newtype_struct not supported"))
    }

    fn serialize_newtype_variant<T>(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
        value: &T,
    ) -> Result<Self::Ok, Self::Error>
    where
        T: ?Sized + Serialize,
    {
        Err(Error::custom("serialize_newtype_variant not supported"))
    }

    fn serialize_seq(self, len: Option<usize>) -> Result<Self::SerializeSeq, Self::Error> {
        Err(Error::custom("serialize_seq not supported"))
    }

    fn serialize_tuple(self, len: usize) -> Result<Self::SerializeTuple, Self::Error> {
        Err(Error::custom("serialize_tuple not supported"))
    }

    fn serialize_tuple_struct(
        self,
        name: &'static str,
        len: usize,
    ) -> Result<Self::SerializeTupleStruct, Self::Error> {
        Err(Error::custom("serialize_tuple_struct not supported"))
    }

    fn serialize_tuple_variant(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
        len: usize,
    ) -> Result<Self::SerializeTupleVariant, Self::Error> {
        Err(Error::custom("serialize_tuple_variant not supported"))
    }

    fn serialize_map(self, len: Option<usize>) -> Result<Self::SerializeMap, Self::Error> {
        Err(Error::custom("serialize_map not supported"))
    }

    fn serialize_struct(
        self,
        name: &'static str,
        len: usize,
    ) -> Result<Self::SerializeStruct, Self::Error> {
        Ok(MapStructSerializeStruct {
            inner: self.inner.serialize_seq(Some(len))?,
            phantom: PhantomData,
        })
    }

    fn serialize_struct_variant(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
        len: usize,
    ) -> Result<Self::SerializeStructVariant, Self::Error> {
        Err(Error::custom("serialize_struct_variant not supported"))
    }
}

pub struct NameValuePair<KS, V> {
    name: &'static str,
    value: V,
    phantom: PhantomData<KS>,
}

impl<KS: MapStructKeys, V: Serialize> Serialize for NameValuePair<KS, V> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut serializer = serializer.serialize_struct("NameValuePair", 2)?;
        serializer.serialize_field(KS::NAME, &self.name)?;
        serializer.serialize_field(KS::VALUE, &self.value)?;
        serializer.end()
    }
}

impl<KS: MapStructKeys, SS: SerializeSeq> SerializeStruct for MapStructSerializeStruct<KS, SS> {
    type Ok = SS::Ok;
    type Error = SS::Error;

    fn serialize_field<T>(&mut self, key: &'static str, value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + Serialize,
    {
        self.inner.serialize_element(&NameValuePair {
            name: key,
            value,
            phantom: PhantomData::<KS>,
        })
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        self.inner.end()
    }
}

impl<KS: MapStructKeys, T: Serialize> SerializeAs<T> for MapStruct<KS> {
    fn serialize_as<S>(source: &T, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        source.serialize(MapStructSerializer {
            inner: serializer,
            phantom: PhantomData::<KS>,
        })
    }
}
