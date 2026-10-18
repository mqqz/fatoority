//! A real Serde serializer with an injected failure on a named struct field.
use serde::ser::{Serialize, SerializeStruct, Serializer};

pub struct FailField(pub &'static str);
pub struct Struct {
    inner: <serde_json::value::Serializer as Serializer>::SerializeStruct,
    field: &'static str,
}
impl SerializeStruct for Struct {
    type Ok = serde_json::Value;
    type Error = serde_json::Error;
    fn serialize_field<T: ?Sized + Serialize>(
        &mut self,
        key: &'static str,
        value: &T,
    ) -> Result<(), Self::Error> {
        if key == self.field {
            return Err(serde::ser::Error::custom(format!(
                "injected failure: {key}"
            )));
        }
        self.inner.serialize_field(key, value)
    }
    fn end(self) -> Result<Self::Ok, Self::Error> {
        self.inner.end()
    }
}
macro_rules! forward {
    ($($name:ident($($arg:ident: $ty:ty),*);)*) => {$(
        fn $name(self, $($arg: $ty),*) -> Result<Self::Ok, Self::Error> {
            serde_json::value::Serializer.$name($($arg),*)
        }
    )*};
}
impl Serializer for FailField {
    type Ok = serde_json::Value;
    type Error = serde_json::Error;
    type SerializeSeq = <serde_json::value::Serializer as Serializer>::SerializeSeq;
    type SerializeTuple = <serde_json::value::Serializer as Serializer>::SerializeTuple;
    type SerializeTupleStruct = <serde_json::value::Serializer as Serializer>::SerializeTupleStruct;
    type SerializeTupleVariant =
        <serde_json::value::Serializer as Serializer>::SerializeTupleVariant;
    type SerializeMap = <serde_json::value::Serializer as Serializer>::SerializeMap;
    type SerializeStruct = Struct;
    type SerializeStructVariant =
        <serde_json::value::Serializer as Serializer>::SerializeStructVariant;
    forward! {
        serialize_bool(v: bool); serialize_i8(v: i8); serialize_i16(v: i16);
        serialize_i32(v: i32); serialize_i64(v: i64); serialize_u8(v: u8);
        serialize_u16(v: u16); serialize_u32(v: u32); serialize_u64(v: u64);
        serialize_f32(v: f32); serialize_f64(v: f64); serialize_char(v: char);
        serialize_str(v: &str); serialize_bytes(v: &[u8]); serialize_none();
        serialize_unit(); serialize_unit_struct(name: &'static str);
        serialize_unit_variant(name: &'static str, index: u32, variant: &'static str);
    }
    fn serialize_some<T: ?Sized + Serialize>(self, v: &T) -> Result<Self::Ok, Self::Error> {
        serde_json::value::Serializer.serialize_some(v)
    }
    fn serialize_newtype_struct<T: ?Sized + Serialize>(
        self,
        name: &'static str,
        v: &T,
    ) -> Result<Self::Ok, Self::Error> {
        serde_json::value::Serializer.serialize_newtype_struct(name, v)
    }
    fn serialize_newtype_variant<T: ?Sized + Serialize>(
        self,
        name: &'static str,
        index: u32,
        variant: &'static str,
        v: &T,
    ) -> Result<Self::Ok, Self::Error> {
        serde_json::value::Serializer.serialize_newtype_variant(name, index, variant, v)
    }
    fn serialize_seq(self, len: Option<usize>) -> Result<Self::SerializeSeq, Self::Error> {
        serde_json::value::Serializer.serialize_seq(len)
    }
    fn serialize_tuple(self, len: usize) -> Result<Self::SerializeTuple, Self::Error> {
        serde_json::value::Serializer.serialize_tuple(len)
    }
    fn serialize_tuple_struct(
        self,
        name: &'static str,
        len: usize,
    ) -> Result<Self::SerializeTupleStruct, Self::Error> {
        serde_json::value::Serializer.serialize_tuple_struct(name, len)
    }
    fn serialize_tuple_variant(
        self,
        name: &'static str,
        index: u32,
        variant: &'static str,
        len: usize,
    ) -> Result<Self::SerializeTupleVariant, Self::Error> {
        serde_json::value::Serializer.serialize_tuple_variant(name, index, variant, len)
    }
    fn serialize_map(self, len: Option<usize>) -> Result<Self::SerializeMap, Self::Error> {
        serde_json::value::Serializer.serialize_map(len)
    }
    fn serialize_struct(
        self,
        name: &'static str,
        len: usize,
    ) -> Result<Self::SerializeStruct, Self::Error> {
        Ok(Struct {
            inner: serde_json::value::Serializer.serialize_struct(name, len)?,
            field: self.0,
        })
    }
    fn serialize_struct_variant(
        self,
        name: &'static str,
        index: u32,
        variant: &'static str,
        len: usize,
    ) -> Result<Self::SerializeStructVariant, Self::Error> {
        serde_json::value::Serializer.serialize_struct_variant(name, index, variant, len)
    }
}
