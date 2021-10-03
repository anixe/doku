use crate::Value;
use serde::{ser::*, Serializer};
use std::error::Error as StdError;
use std::fmt;

/// Transforms anything that implements `serde::Serialize` into Doku's `Value`.
///
/// The name's a bit of a misnomer, because this struct actually serializes
/// _into_ a `Value`, not a `Value` _itself_ -- but at least this way it's
/// consistent with the names of Serde's traits.
#[derive(Default)]
pub struct ValueSerializer;

impl Serializer for ValueSerializer {
    type Ok = Value;
    type Error = ValueSerializerError;
    type SerializeSeq = ValueSerializeSeq;
    type SerializeTuple = ValueSerializeTuple;
    type SerializeTupleStruct = ValueSerializeTupleStruct;
    type SerializeTupleVariant = ValueSerializeTupleVariant;
    type SerializeMap = ValueSerializeMap;
    type SerializeStruct = ValueSerializeStruct;
    type SerializeStructVariant = ValueSerializeStructVariant;

    fn serialize_bool(self, v: bool) -> Result<Self::Ok, Self::Error> {
        Ok(Value::Bool(v))
    }

    fn serialize_i8(self, v: i8) -> Result<Self::Ok, Self::Error> {
        Ok(Value::I8(v))
    }

    fn serialize_i16(self, v: i16) -> Result<Self::Ok, Self::Error> {
        Ok(Value::I16(v))
    }

    fn serialize_i32(self, v: i32) -> Result<Self::Ok, Self::Error> {
        Ok(Value::I32(v))
    }

    fn serialize_i64(self, v: i64) -> Result<Self::Ok, Self::Error> {
        Ok(Value::I64(v))
    }

    fn serialize_u8(self, v: u8) -> Result<Self::Ok, Self::Error> {
        Ok(Value::U8(v))
    }

    fn serialize_u16(self, v: u16) -> Result<Self::Ok, Self::Error> {
        Ok(Value::U16(v))
    }

    fn serialize_u32(self, v: u32) -> Result<Self::Ok, Self::Error> {
        Ok(Value::U32(v))
    }

    fn serialize_u64(self, v: u64) -> Result<Self::Ok, Self::Error> {
        Ok(Value::U64(v))
    }

    fn serialize_f32(self, v: f32) -> Result<Self::Ok, Self::Error> {
        Ok(Value::F32(v))
    }

    fn serialize_f64(self, v: f64) -> Result<Self::Ok, Self::Error> {
        Ok(Value::F64(v))
    }

    fn serialize_char(self, v: char) -> Result<Self::Ok, Self::Error> {
        Ok(Value::Char(v))
    }

    fn serialize_str(self, v: &str) -> Result<Self::Ok, Self::Error> {
        Ok(Value::String(v.to_string()))
    }

    fn serialize_bytes(self, v: &[u8]) -> Result<Self::Ok, Self::Error> {
        Ok(Value::String(String::from_utf8_lossy(v).to_string()))
    }

    fn serialize_none(self) -> Result<Self::Ok, Self::Error> {
        Ok(Value::None)
    }

    fn serialize_some<T: ?Sized>(
        self,
        value: &T,
    ) -> Result<Self::Ok, Self::Error>
    where
        T: Serialize,
    {
        value.serialize(self)
    }

    fn serialize_unit(self) -> Result<Self::Ok, Self::Error> {
        self.serialize_none()
    }

    fn serialize_unit_struct(
        self,
        _name: &'static str,
    ) -> Result<Self::Ok, Self::Error> {
        Ok(Value::Map(Default::default()))
    }

    fn serialize_unit_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
    ) -> Result<Self::Ok, Self::Error> {
        // TODO
        Ok(Value::None)
    }

    fn serialize_newtype_struct<T: ?Sized>(
        self,
        _name: &'static str,
        value: &T,
    ) -> Result<Self::Ok, Self::Error>
    where
        T: Serialize,
    {
        Ok(Value::Array(vec![value.serialize(self)?]))
    }

    fn serialize_newtype_variant<T: ?Sized>(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _value: &T,
    ) -> Result<Self::Ok, Self::Error>
    where
        T: Serialize,
    {
        // TODO
        Ok(Value::None)
    }

    fn serialize_seq(
        self,
        _len: Option<usize>,
    ) -> Result<Self::SerializeSeq, Self::Error> {
        Ok(Default::default())
    }

    fn serialize_tuple(
        self,
        _len: usize,
    ) -> Result<Self::SerializeTuple, Self::Error> {
        Ok(Default::default())
    }

    fn serialize_tuple_struct(
        self,
        _name: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleStruct, Self::Error> {
        // TODO
        Ok(Default::default())
    }

    fn serialize_tuple_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleVariant, Self::Error> {
        // TODO
        Ok(Default::default())
    }

    fn serialize_map(
        self,
        _len: Option<usize>,
    ) -> Result<Self::SerializeMap, Self::Error> {
        Ok(Default::default())
    }

    fn serialize_struct(
        self,
        _name: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStruct, Self::Error> {
        Ok(Default::default())
    }

    fn serialize_struct_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStructVariant, Self::Error> {
        // TODO
        Ok(Default::default())
    }
}

// ----- //

#[derive(Default)]
pub struct ValueSerializeSeq {
    items: Vec<Value>,
}

impl SerializeSeq for ValueSerializeSeq {
    type Ok = Value;
    type Error = ValueSerializerError;

    fn serialize_element<T: ?Sized>(
        &mut self,
        value: &T,
    ) -> Result<(), Self::Error>
    where
        T: Serialize,
    {
        value.serialize(ValueSerializer).map(|value| {
            self.items.push(value);
        })
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(Value::Array(self.items))
    }
}

// ----- //

#[derive(Default)]
pub struct ValueSerializeTuple {
    items: Vec<Value>,
}

impl SerializeTuple for ValueSerializeTuple {
    type Ok = Value;
    type Error = ValueSerializerError;

    fn serialize_element<T: ?Sized>(
        &mut self,
        value: &T,
    ) -> Result<(), Self::Error>
    where
        T: Serialize,
    {
        value.serialize(ValueSerializer).map(|value| {
            self.items.push(value);
        })
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(Value::Array(self.items))
    }
}

// ----- //

#[derive(Default)]
pub struct ValueSerializeTupleStruct {
    items: Vec<Value>,
}

impl SerializeTupleStruct for ValueSerializeTupleStruct {
    type Ok = Value;
    type Error = ValueSerializerError;

    fn serialize_field<T: ?Sized>(
        &mut self,
        value: &T,
    ) -> Result<(), Self::Error>
    where
        T: Serialize,
    {
        value.serialize(ValueSerializer).map(|value| {
            self.items.push(value);
        })
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(Value::Array(self.items))
    }
}

// ---- //

#[derive(Default)]
pub struct ValueSerializeTupleVariant {
    items: Vec<Value>,
}

impl SerializeTupleVariant for ValueSerializeTupleVariant {
    type Ok = Value;
    type Error = ValueSerializerError;

    fn serialize_field<T: ?Sized>(
        &mut self,
        value: &T,
    ) -> Result<(), Self::Error>
    where
        T: Serialize,
    {
        value.serialize(ValueSerializer).map(|value| {
            self.items.push(value);
        })
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(Value::Array(self.items))
    }
}

// ----- //

#[derive(Default)]
pub struct ValueSerializeMap {
    pending_key: Option<Value>,
    items: Vec<(Value, Value)>,
}

impl SerializeMap for ValueSerializeMap {
    type Ok = Value;
    type Error = ValueSerializerError;

    fn serialize_key<T: ?Sized>(&mut self, key: &T) -> Result<(), Self::Error>
    where
        T: Serialize,
    {
        key.serialize(ValueSerializer).map(|key| {
            self.pending_key = Some(key);
        })
    }

    fn serialize_value<T: ?Sized>(
        &mut self,
        value: &T,
    ) -> Result<(), Self::Error>
    where
        T: Serialize,
    {
        value.serialize(ValueSerializer).map(|value| {
            let key = self.pending_key.take().unwrap_or_default();
            self.items.push((key, value));
        })
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(Value::Map(self.items))
    }
}

// ----- //

#[derive(Default)]
pub struct ValueSerializeStruct {
    fields: Vec<(Value, Value)>,
}

impl SerializeStruct for ValueSerializeStruct {
    type Ok = Value;
    type Error = ValueSerializerError;

    fn serialize_field<T: ?Sized>(
        &mut self,
        key: &'static str,
        value: &T,
    ) -> Result<(), Self::Error>
    where
        T: Serialize,
    {
        value.serialize(ValueSerializer).map(|value| {
            self.fields.push((Value::String(key.into()), value));
        })
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(Value::Map(self.fields))
    }
}

// ----- //

#[derive(Default)]
pub struct ValueSerializeStructVariant {
    fields: Vec<(Value, Value)>,
}

impl SerializeStructVariant for ValueSerializeStructVariant {
    type Ok = Value;
    type Error = ValueSerializerError;

    fn serialize_field<T: ?Sized>(
        &mut self,
        key: &'static str,
        value: &T,
    ) -> Result<(), Self::Error>
    where
        T: Serialize,
    {
        value.serialize(ValueSerializer).map(|value| {
            self.fields.push((Value::String(key.into()), value));
        })
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(Value::Map(self.fields))
    }
}

#[derive(Debug)]
pub enum ValueSerializerError {
    //
}

impl fmt::Display for ValueSerializerError {
    fn fmt(&self, _: &mut fmt::Formatter<'_>) -> fmt::Result {
        unreachable!()
    }
}

impl StdError for ValueSerializerError {
    //
}

impl Error for ValueSerializerError {
    fn custom<T>(_: T) -> Self
    where
        T: fmt::Display,
    {
        unreachable!()
    }
}
