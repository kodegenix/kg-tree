use super::*;

use serde::de::Error as DeError;
use serde::de::Unexpected;
use serde::de::IntoDeserializer;


pub fn from_tree<'de, T>(node: &NodeRef) -> self::error::Result<T>
    where T: serde::Deserialize<'de>
{
    T::deserialize(NodeDeserializer::new(node))
}


pub struct NodeDeserializer<'a> {
    node: &'a NodeRef,
}

impl<'a> NodeDeserializer<'a> {
    pub fn new(node: &'a NodeRef) -> NodeDeserializer<'a> {
        NodeDeserializer { node }
    }
}

impl<'a, 'de> serde::de::Deserializer<'de> for NodeDeserializer<'a> {
    type Error = self::error::Error;

    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
        where V: serde::de::Visitor<'de>
    {
        match *self.node.data().value() {
            Value::Null => visitor.visit_unit(),
            Value::Boolean(b) => visitor.visit_bool(b),
            Value::Integer(i) => visitor.visit_i64(i),
            Value::Float(f) => visitor.visit_f64(f),
            Value::String(ref s) => visitor.visit_str(s),
            Value::Binary(ref b) => visitor.visit_bytes(b),
            Value::Array(ref e) => {
                let a = Array::new(e);
                visitor.visit_seq(a)
            }
            Value::Object(ref p) => {
                let o = Object::new(p);
                visitor.visit_map(o)
            }
        }
    }

    fn deserialize_bool<V>(self, visitor: V) -> Result<V::Value, Self::Error>
        where V: serde::de::Visitor<'de>
    {
        match *self.node.data().value() {
            Value::Null => Err(DeError::invalid_type(Unexpected::Unit, &visitor)),
            Value::Boolean(b) => visitor.visit_bool(b),
            Value::Integer(i) => Err(DeError::invalid_type(Unexpected::Signed(i), &visitor)),
            Value::Float(f) => Err(DeError::invalid_type(Unexpected::Float(f), &visitor)),
            Value::String(ref s) => Err(DeError::invalid_type(Unexpected::Str(s), &visitor)),
            Value::Binary(ref b) => Err(DeError::invalid_type(Unexpected::Bytes(b), &visitor)),
            Value::Array(_) => Err(DeError::invalid_type(Unexpected::Seq, &visitor)),
            Value::Object(_) => Err(DeError::invalid_type(Unexpected::Map, &visitor)),
        }
    }

    fn deserialize_i8<V>(self, visitor: V) -> Result<V::Value, Self::Error>
        where V: serde::de::Visitor<'de>
    {
        match *self.node.data().value() {
            Value::Null => Err(DeError::invalid_type(Unexpected::Unit, &visitor)),
            Value::Boolean(b) => Err(DeError::invalid_type(Unexpected::Bool(b), &visitor)),
            Value::Integer(i) => visitor.visit_i8(i as i8),
            Value::Float(f) => visitor.visit_i8(f as i8),
            Value::String(ref s) => Err(DeError::invalid_type(Unexpected::Str(s), &visitor)),
            Value::Binary(ref b) => Err(DeError::invalid_type(Unexpected::Bytes(b), &visitor)),
            Value::Array(_) => Err(DeError::invalid_type(Unexpected::Seq, &visitor)),
            Value::Object(_) => Err(DeError::invalid_type(Unexpected::Map, &visitor)),
        }
    }

    fn deserialize_i16<V>(self, visitor: V) -> Result<V::Value, Self::Error>
        where V: serde::de::Visitor<'de>
    {
        match *self.node.data().value() {
            Value::Null => Err(DeError::invalid_type(Unexpected::Unit, &visitor)),
            Value::Boolean(b) => Err(DeError::invalid_type(Unexpected::Bool(b), &visitor)),
            Value::Integer(i) => visitor.visit_i16(i as i16),
            Value::Float(f) => visitor.visit_i16(f as i16),
            Value::String(ref s) => Err(DeError::invalid_type(Unexpected::Str(s), &visitor)),
            Value::Binary(ref b) => Err(DeError::invalid_type(Unexpected::Bytes(b), &visitor)),
            Value::Array(_) => Err(DeError::invalid_type(Unexpected::Seq, &visitor)),
            Value::Object(_) => Err(DeError::invalid_type(Unexpected::Map, &visitor)),
        }
    }

    fn deserialize_i32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
        where V: serde::de::Visitor<'de>
    {
        match *self.node.data().value() {
            Value::Null => Err(DeError::invalid_type(Unexpected::Unit, &visitor)),
            Value::Boolean(b) => Err(DeError::invalid_type(Unexpected::Bool(b), &visitor)),
            Value::Integer(i) => visitor.visit_i32(i as i32),
            Value::Float(f) => visitor.visit_i32(f as i32),
            Value::String(ref s) => Err(DeError::invalid_type(Unexpected::Str(s), &visitor)),
            Value::Binary(ref b) => Err(DeError::invalid_type(Unexpected::Bytes(b), &visitor)),
            Value::Array(_) => Err(DeError::invalid_type(Unexpected::Seq, &visitor)),
            Value::Object(_) => Err(DeError::invalid_type(Unexpected::Map, &visitor)),
        }
    }

    fn deserialize_i64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
        where V: serde::de::Visitor<'de>
    {
        match *self.node.data().value() {
            Value::Null => Err(DeError::invalid_type(Unexpected::Unit, &visitor)),
            Value::Boolean(b) => Err(DeError::invalid_type(Unexpected::Bool(b), &visitor)),
            Value::Integer(i) => visitor.visit_i64(i),
            Value::Float(f) => visitor.visit_i64(f as i64),
            Value::String(ref s) => Err(DeError::invalid_type(Unexpected::Str(s), &visitor)),
            Value::Binary(ref b) => Err(DeError::invalid_type(Unexpected::Bytes(b), &visitor)),
            Value::Array(_) => Err(DeError::invalid_type(Unexpected::Seq, &visitor)),
            Value::Object(_) => Err(DeError::invalid_type(Unexpected::Map, &visitor)),
        }
    }

    fn deserialize_u8<V>(self, visitor: V) -> Result<V::Value, Self::Error>
        where V: serde::de::Visitor<'de>
    {
        match *self.node.data().value() {
            Value::Null => Err(DeError::invalid_type(Unexpected::Unit, &visitor)),
            Value::Boolean(b) => Err(DeError::invalid_type(Unexpected::Bool(b), &visitor)),
            Value::Integer(i) => visitor.visit_u8(i as u8),
            Value::Float(f) => visitor.visit_u8(f as u8),
            Value::String(ref s) => Err(DeError::invalid_type(Unexpected::Str(s), &visitor)),
            Value::Binary(ref b) => Err(DeError::invalid_type(Unexpected::Bytes(b), &visitor)),
            Value::Array(_) => Err(DeError::invalid_type(Unexpected::Seq, &visitor)),
            Value::Object(_) => Err(DeError::invalid_type(Unexpected::Map, &visitor)),
        }
    }

    fn deserialize_u16<V>(self, visitor: V) -> Result<V::Value, Self::Error>
        where V: serde::de::Visitor<'de>
    {
        match *self.node.data().value() {
            Value::Null => Err(DeError::invalid_type(Unexpected::Unit, &visitor)),
            Value::Boolean(b) => Err(DeError::invalid_type(Unexpected::Bool(b), &visitor)),
            Value::Integer(i) => visitor.visit_u16(i as u16),
            Value::Float(f) => visitor.visit_u16(f as u16),
            Value::String(ref s) => Err(DeError::invalid_type(Unexpected::Str(s), &visitor)),
            Value::Binary(ref b) => Err(DeError::invalid_type(Unexpected::Bytes(b), &visitor)),
            Value::Array(_) => Err(DeError::invalid_type(Unexpected::Seq, &visitor)),
            Value::Object(_) => Err(DeError::invalid_type(Unexpected::Map, &visitor)),
        }
    }

    fn deserialize_u32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
        where V: serde::de::Visitor<'de>
    {
        match *self.node.data().value() {
            Value::Null => Err(DeError::invalid_type(Unexpected::Unit, &visitor)),
            Value::Boolean(b) => Err(DeError::invalid_type(Unexpected::Bool(b), &visitor)),
            Value::Integer(i) => visitor.visit_u32(i as u32),
            Value::Float(f) => visitor.visit_u32(f as u32),
            Value::String(ref s) => Err(DeError::invalid_type(Unexpected::Str(s), &visitor)),
            Value::Binary(ref b) => Err(DeError::invalid_type(Unexpected::Bytes(b), &visitor)),
            Value::Array(_) => Err(DeError::invalid_type(Unexpected::Seq, &visitor)),
            Value::Object(_) => Err(DeError::invalid_type(Unexpected::Map, &visitor)),
        }
    }

    fn deserialize_u64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
        where V: serde::de::Visitor<'de>
    {
        match *self.node.data().value() {
            Value::Null => Err(DeError::invalid_type(Unexpected::Unit, &visitor)),
            Value::Boolean(b) => Err(DeError::invalid_type(Unexpected::Bool(b), &visitor)),
            Value::Integer(i) => visitor.visit_u64(i as u64),
            Value::Float(f) => visitor.visit_u64(f as u64),
            Value::String(ref s) => Err(DeError::invalid_type(Unexpected::Str(s), &visitor)),
            Value::Binary(ref b) => Err(DeError::invalid_type(Unexpected::Bytes(b), &visitor)),
            Value::Array(_) => Err(DeError::invalid_type(Unexpected::Seq, &visitor)),
            Value::Object(_) => Err(DeError::invalid_type(Unexpected::Map, &visitor)),
        }
    }

    fn deserialize_f32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
        where V: serde::de::Visitor<'de>
    {
        match *self.node.data().value() {
            Value::Null => Err(DeError::invalid_type(Unexpected::Unit, &visitor)),
            Value::Boolean(b) => Err(DeError::invalid_type(Unexpected::Bool(b), &visitor)),
            Value::Integer(i) => visitor.visit_f32(i as f32),
            Value::Float(f) => visitor.visit_f32(f as f32),
            Value::String(ref s) => Err(DeError::invalid_type(Unexpected::Str(s), &visitor)),
            Value::Binary(ref b) => Err(DeError::invalid_type(Unexpected::Bytes(b), &visitor)),
            Value::Array(_) => Err(DeError::invalid_type(Unexpected::Seq, &visitor)),
            Value::Object(_) => Err(DeError::invalid_type(Unexpected::Map, &visitor)),
        }
    }

    fn deserialize_f64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
        where V: serde::de::Visitor<'de>
    {
        match *self.node.data().value() {
            Value::Null => Err(DeError::invalid_type(Unexpected::Unit, &visitor)),
            Value::Boolean(b) => Err(DeError::invalid_type(Unexpected::Bool(b), &visitor)),
            Value::Integer(i) => visitor.visit_f64(i as f64),
            Value::Float(f) => visitor.visit_f64(f),
            Value::String(ref s) => Err(DeError::invalid_type(Unexpected::Str(s), &visitor)),
            Value::Binary(ref b) => Err(DeError::invalid_type(Unexpected::Bytes(b), &visitor)),
            Value::Array(_) => Err(DeError::invalid_type(Unexpected::Seq, &visitor)),
            Value::Object(_) => Err(DeError::invalid_type(Unexpected::Map, &visitor)),
        }
    }

    fn deserialize_char<V>(self, visitor: V) -> Result<V::Value, Self::Error>
        where V: serde::de::Visitor<'de>
    {
        match *self.node.data().value() {
            Value::Null => Err(DeError::invalid_type(Unexpected::Unit, &visitor)),
            Value::Boolean(b) => Err(DeError::invalid_type(Unexpected::Bool(b), &visitor)),
            Value::Integer(i) => Err(DeError::invalid_type(Unexpected::Signed(i), &visitor)),
            Value::Float(f) => Err(DeError::invalid_type(Unexpected::Float(f), &visitor)),
            Value::String(ref s) => {
                let mut chars = s.chars();
                match (chars.next(), chars.next()) {
                    (Some(c), None) => visitor.visit_char(c),
                    (_, _) => Err(DeError::invalid_type(Unexpected::Str(s), &visitor)),
                }
            }
            Value::Binary(ref b) => Err(DeError::invalid_type(Unexpected::Bytes(b), &visitor)),
            Value::Array(_) => Err(DeError::invalid_type(Unexpected::Seq, &visitor)),
            Value::Object(_) => Err(DeError::invalid_type(Unexpected::Map, &visitor)),
        }
    }

    fn deserialize_str<V>(self, visitor: V) -> Result<V::Value, Self::Error>
        where V: serde::de::Visitor<'de>
    {
        match *self.node.data().value() {
            Value::Null => Err(DeError::invalid_type(Unexpected::Unit, &visitor)),
            Value::Boolean(b) => Err(DeError::invalid_type(Unexpected::Bool(b), &visitor)),
            Value::Integer(i) => Err(DeError::invalid_type(Unexpected::Signed(i), &visitor)),
            Value::Float(f) => Err(DeError::invalid_type(Unexpected::Float(f), &visitor)),
            Value::String(ref s) => visitor.visit_str(s),
            Value::Binary(ref b) => Err(DeError::invalid_type(Unexpected::Bytes(b), &visitor)),
            Value::Array(_) => Err(DeError::invalid_type(Unexpected::Seq, &visitor)),
            Value::Object(_) => Err(DeError::invalid_type(Unexpected::Map, &visitor)),
        }
    }

    fn deserialize_string<V>(self, visitor: V) -> Result<V::Value, Self::Error>
        where V: serde::de::Visitor<'de>
    {
        match *self.node.data().value() {
            Value::Null => Err(DeError::invalid_type(Unexpected::Unit, &visitor)),
            Value::Boolean(b) => Err(DeError::invalid_type(Unexpected::Bool(b), &visitor)),
            Value::Integer(i) => Err(DeError::invalid_type(Unexpected::Signed(i), &visitor)),
            Value::Float(f) => Err(DeError::invalid_type(Unexpected::Float(f), &visitor)),
            Value::String(ref s) => visitor.visit_string(s.to_string()),
            Value::Binary(ref b) => Err(DeError::invalid_type(Unexpected::Bytes(b), &visitor)),
            Value::Array(_) => Err(DeError::invalid_type(Unexpected::Seq, &visitor)),
            Value::Object(_) => Err(DeError::invalid_type(Unexpected::Map, &visitor)),
        }
    }

    fn deserialize_bytes<V>(self, visitor: V) -> Result<V::Value, Self::Error>
        where V: serde::de::Visitor<'de>
    {
        match *self.node.data().value() {
            Value::Null => Err(DeError::invalid_type(Unexpected::Unit, &visitor)),
            Value::Boolean(b) => Err(DeError::invalid_type(Unexpected::Bool(b), &visitor)),
            Value::Integer(i) => Err(DeError::invalid_type(Unexpected::Signed(i), &visitor)),
            Value::Float(f) => Err(DeError::invalid_type(Unexpected::Float(f), &visitor)),
            Value::String(ref s) => visitor.visit_bytes(s.as_bytes()),
            Value::Binary(ref b) => visitor.visit_bytes(b),
            Value::Array(_) => Err(DeError::invalid_type(Unexpected::Seq, &visitor)),
            Value::Object(_) => Err(DeError::invalid_type(Unexpected::Map, &visitor)),
        }
    }

    fn deserialize_byte_buf<V>(self, visitor: V) -> Result<V::Value, Self::Error>
        where V: serde::de::Visitor<'de>
    {
        match *self.node.data().value() {
            Value::Null => Err(DeError::invalid_type(Unexpected::Unit, &visitor)),
            Value::Boolean(b) => Err(DeError::invalid_type(Unexpected::Bool(b), &visitor)),
            Value::Integer(i) => Err(DeError::invalid_type(Unexpected::Signed(i), &visitor)),
            Value::Float(f) => Err(DeError::invalid_type(Unexpected::Float(f), &visitor)),
            Value::String(ref s) => visitor.visit_byte_buf(s.as_bytes().to_vec()),
            Value::Binary(ref b) => visitor.visit_byte_buf(b.clone()),
            Value::Array(_) => Err(DeError::invalid_type(Unexpected::Seq, &visitor)),
            Value::Object(_) => Err(DeError::invalid_type(Unexpected::Map, &visitor)),
        }
    }

    fn deserialize_option<V>(self, visitor: V) -> Result<V::Value, Self::Error>
        where V: serde::de::Visitor<'de>
    {
        match *self.node.data().value() {
            Value::Null => visitor.visit_none(),
            _ => visitor.visit_some(self),
        }
    }

    fn deserialize_unit<V>(self, visitor: V) -> Result<V::Value, Self::Error>
        where V: serde::de::Visitor<'de>
    {
        match *self.node.data().value() {
            Value::Null => visitor.visit_unit(),
            Value::Boolean(b) => Err(DeError::invalid_type(Unexpected::Bool(b), &visitor)),
            Value::Integer(i) => Err(DeError::invalid_type(Unexpected::Signed(i), &visitor)),
            Value::Float(f) => Err(DeError::invalid_type(Unexpected::Float(f), &visitor)),
            Value::String(ref s) => Err(DeError::invalid_type(Unexpected::Str(s), &visitor)),
            Value::Binary(ref b) => Err(DeError::invalid_type(Unexpected::Bytes(b), &visitor)),
            Value::Array(_) => Err(DeError::invalid_type(Unexpected::Seq, &visitor)),
            Value::Object(_) => Err(DeError::invalid_type(Unexpected::Map, &visitor)),
        }
    }

    fn deserialize_unit_struct<V>(self, _name: &'static str, visitor: V) -> Result<V::Value, Self::Error>
        where V: serde::de::Visitor<'de>
    {
        self.deserialize_unit(visitor)
    }

    fn deserialize_newtype_struct<V>(self, _name: &'static str, visitor: V) -> Result<V::Value, Self::Error>
        where V: serde::de::Visitor<'de>
    {
        visitor.visit_newtype_struct(self)
    }

    fn deserialize_seq<V>(self, visitor: V) -> Result<V::Value, Self::Error>
        where V: serde::de::Visitor<'de>
    {
        match *self.node.data().value() {
            Value::Null => Err(DeError::invalid_type(Unexpected::Unit, &visitor)),
            Value::Boolean(b) => Err(DeError::invalid_type(Unexpected::Bool(b), &visitor)),
            Value::Integer(i) => Err(DeError::invalid_type(Unexpected::Signed(i), &visitor)),
            Value::Float(f) => Err(DeError::invalid_type(Unexpected::Float(f), &visitor)),
            Value::String(ref s) => Err(DeError::invalid_type(Unexpected::Str(s), &visitor)),
            Value::Binary(ref b) => Err(DeError::invalid_type(Unexpected::Bytes(b), &visitor)),
            Value::Array(ref e) => visitor.visit_seq(Array::new(e)),
            Value::Object(_) => Err(DeError::invalid_type(Unexpected::Map, &visitor)),
        }
    }

    fn deserialize_tuple<V>(self, _len: usize, visitor: V) -> Result<V::Value, Self::Error>
        where V: serde::de::Visitor<'de>
    {
        self.deserialize_seq(visitor)
    }

    fn deserialize_tuple_struct<V>(self, _name: &'static str, len: usize, visitor: V) -> Result<V::Value, Self::Error>
        where V: serde::de::Visitor<'de>
    {
        self.deserialize_tuple(len, visitor)
    }

    fn deserialize_map<V>(self, visitor: V) -> Result<V::Value, Self::Error>
        where V: serde::de::Visitor<'de>
    {
        match *self.node.data().value() {
            Value::Null => Err(DeError::invalid_type(Unexpected::Unit, &visitor)),
            Value::Boolean(b) => Err(DeError::invalid_type(Unexpected::Bool(b), &visitor)),
            Value::Integer(i) => Err(DeError::invalid_type(Unexpected::Signed(i), &visitor)),
            Value::Float(f) => Err(DeError::invalid_type(Unexpected::Float(f), &visitor)),
            Value::String(ref s) => Err(DeError::invalid_type(Unexpected::Str(s), &visitor)),
            Value::Binary(ref b) => Err(DeError::invalid_type(Unexpected::Bytes(b), &visitor)),
            Value::Array(_) => Err(DeError::invalid_type(Unexpected::Seq, &visitor)),
            Value::Object(ref p) => visitor.visit_map(Object::new(p)),
        }
    }

    fn deserialize_struct<V>(self, _name: &'static str, _fields: &'static [&'static str], visitor: V) -> Result<V::Value, Self::Error>
        where V: serde::de::Visitor<'de>
    {
        self.deserialize_map(visitor)
    }

    fn deserialize_enum<V>(self, _name: &'static str, _variants: &'static [&'static str], visitor: V) -> Result<V::Value, Self::Error>
        where V: serde::de::Visitor<'de>
    {
        match *self.node.data().value() {
            Value::Null => Err(DeError::invalid_type(Unexpected::Unit, &visitor)),
            Value::Boolean(b) => Err(DeError::invalid_type(Unexpected::Bool(b), &visitor)),
            Value::Integer(i) => Err(DeError::invalid_type(Unexpected::Signed(i), &visitor)),
            Value::Float(f) => Err(DeError::invalid_type(Unexpected::Float(f), &visitor)),
            Value::String(ref s) => visitor.visit_enum(EnumString::new(s)),
            Value::Binary(ref b) => Err(DeError::invalid_type(Unexpected::Bytes(b), &visitor)),
            Value::Array(_) => Err(DeError::invalid_type(Unexpected::Seq, &visitor)),
            Value::Object(ref p) => visitor.visit_enum(EnumObject::new(p)),
        }
    }

    fn deserialize_identifier<V>(self, visitor: V) -> Result<V::Value, Self::Error>
        where V: serde::de::Visitor<'de>
    {
        match *self.node.data().value() {
            Value::Null => Err(DeError::invalid_type(Unexpected::Unit, &visitor)),
            Value::Boolean(b) => Err(DeError::invalid_type(Unexpected::Bool(b), &visitor)),
            Value::Integer(i) => Err(DeError::invalid_type(Unexpected::Signed(i), &visitor)),
            Value::Float(f) => Err(DeError::invalid_type(Unexpected::Float(f), &visitor)),
            Value::String(ref s) => visitor.visit_str(s),
            Value::Binary(ref b) => Err(DeError::invalid_type(Unexpected::Bytes(b), &visitor)),
            Value::Array(_) => Err(DeError::invalid_type(Unexpected::Seq, &visitor)),
            Value::Object(ref p) => {
                if p.len() > 0 {
                    visitor.visit_str(p.keys().next().unwrap())
                } else {
                    Err(Error::DeserializationError(line!())) //Object cannot be empty
                }
            }
        }
    }

    fn deserialize_ignored_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
        where V: serde::de::Visitor<'de>
    {
        self.deserialize_any(visitor)
    }
}


struct Array<'a> {
    iter: std::slice::Iter<'a, NodeRef>,
}

impl<'a> Array<'a> {
    fn new(elems: &'a Elements) -> Array<'a> {
        Array { iter: elems.iter() }
    }
}

impl<'a, 'de> serde::de::SeqAccess<'de> for Array<'a> {
    type Error = self::error::Error;

    fn next_element_seed<T>(&mut self, seed: T) -> self::error::Result<Option<T::Value>>
        where T: serde::de::DeserializeSeed<'de>
    {
        match self.iter.next() {
            Some(n) => {
                let mut de = NodeDeserializer::new(n);
                seed.deserialize(de).map(Some)
            }
            None => Ok(None)
        }
    }
}

struct Object<'a> {
    iter: kg_utils::collections::linked_hash_map::Iter<'a, Symbol, NodeRef>,
    value: Option<&'a NodeRef>,
}

impl<'a> Object<'a> {
    fn new(props: &'a Properties) -> Object<'a> {
        Object {
            iter: props.iter(),
            value: None,
        }
    }
}

impl<'a, 'de> serde::de::MapAccess<'de> for Object<'a> {
    type Error = self::error::Error;

    fn next_key_seed<K>(&mut self, seed: K) -> self::error::Result<Option<K::Value>>
        where K: serde::de::DeserializeSeed<'de>
    {
        match self.iter.next() {
            Some((k, v)) => {
                self.value = Some(v);
                seed.deserialize(k.as_ref().into_deserializer()).map(Some)
            }
            None => Ok(None)
        }

    }

    fn next_value_seed<V>(&mut self, seed: V) -> self::error::Result<V::Value>
        where V: serde::de::DeserializeSeed<'de>
    {
        seed.deserialize(NodeDeserializer::new(self.value.take().unwrap()))
    }
}


struct EnumString<'a> {
    variant: &'a str,
}

impl<'a> EnumString<'a> {
    fn new(variant: &'a str) -> Self {
        EnumString { variant }
    }
}

impl<'a, 'de> serde::de::EnumAccess<'de> for EnumString<'a> {
    type Error = Error;
    type Variant = Self;

    fn variant_seed<V>(self, seed: V) -> Result<(V::Value, Self::Variant), Self::Error>
        where V: serde::de::DeserializeSeed<'de>
    {
        let val = seed.deserialize(self.variant.into_deserializer())?;
        Ok((val, self))
    }
}

impl<'a, 'de> serde::de::VariantAccess<'de> for EnumString<'a> {
    type Error = Error;

    fn unit_variant(self) -> Result<(), Self::Error> {
        Ok(())
    }

    fn newtype_variant_seed<T>(self, _seed: T) -> Result<T::Value, Self::Error>
        where T: serde::de::DeserializeSeed<'de>
    {
        Err(Error::DeserializationError(line!())) //FIXME (jc) string enums cannot have any data
    }

    fn tuple_variant<V>(self, _len: usize, _visitor: V) -> Result<V::Value, Self::Error>
        where V: serde::de::Visitor<'de>
    {
        Err(Error::DeserializationError(line!())) //FIXME (jc) string enums cannot have any data
    }

    fn struct_variant<V>(self, _fields: &'static [&'static str], _visitor: V) -> Result<V::Value, Self::Error>
        where V: serde::de::Visitor<'de>
    {
        Err(Error::DeserializationError(line!())) //FIXME (jc) string enums cannot have any data
    }
}


struct EnumObject<'a> {
    props: &'a Properties,
}

impl<'a> EnumObject<'a> {
    fn new(props: &'a Properties) -> Self {
        EnumObject { props }
    }
}

impl<'a, 'de> serde::de::EnumAccess<'de> for EnumObject<'a> {
    type Error = Error;
    type Variant = EnumObjectData<'a>;

    fn variant_seed<V>(self, seed: V) -> Result<(V::Value, Self::Variant), Self::Error>
        where V: serde::de::DeserializeSeed<'de>
    {
        if self.props.len() == 1 {
            let (k, v) = self.props.iter().next().unwrap();
            let val = seed.deserialize(k.as_ref().into_deserializer())?;
            Ok((val, EnumObjectData::new(v)))
        } else {
            Err(Error::DeserializationError(line!())) //FIXME (jc)
        }
    }
}

struct EnumObjectData<'a> {
    node: &'a NodeRef,
}

impl<'a> EnumObjectData<'a> {
    fn new(node: &'a NodeRef) -> Self {
        EnumObjectData { node }
    }
}

impl<'a, 'de> serde::de::VariantAccess<'de> for EnumObjectData<'a> {
    type Error = Error;

    fn unit_variant(self) -> Result<(), Self::Error> {
        Err(Error::DeserializationError(line!())) //FIXME (jc) enum with object representation must have data
    }

    fn newtype_variant_seed<T>(self, seed: T) -> Result<T::Value, Self::Error>
        where T: serde::de::DeserializeSeed<'de>
    {
        seed.deserialize(NodeDeserializer::new(self.node))
    }

    fn tuple_variant<V>(self, _len: usize, visitor: V) -> Result<V::Value, Self::Error>
        where V: serde::de::Visitor<'de>
    {
        use serde::de::Deserializer;

        let de = NodeDeserializer::new(self.node);
        de.deserialize_seq(visitor)
    }

    fn struct_variant<V>(self, _fields: &'static [&'static str], visitor: V) -> Result<V::Value, Self::Error>
        where V: serde::de::Visitor<'de>
    {
        use serde::de::Deserializer;

        let de = NodeDeserializer::new(self.node);
        de.deserialize_map(visitor)
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn enum_internally_tagged() {
        #[derive(Debug, Serialize, Deserialize)]
        #[serde(rename_all = "kebab-case", tag = "kind")]
        enum TestEnum {
            Unit,
            Struct { a: u32, b: String },
        }

        let a = TestEnum::Unit;
        let n = ser::to_tree(&a).unwrap();
        println!("{}", n);
        let a: TestEnum = de::from_tree(&n).unwrap();
        println!("{:?}", a);

        let a = TestEnum::Struct { a:2, b: "aaa".to_string() };
        let n = ser::to_tree(&a).unwrap();
        println!("{}", n);
        let a: TestEnum = de::from_tree(&n).unwrap();
        println!("{:?}", a);
    }

    #[test]
    fn enum_externally_tagged() {
        #[derive(Debug, Serialize, Deserialize)]
        #[serde(rename_all = "kebab-case")]
        enum TestEnum {
            Unit,
            Newtype(u32),
            Tuple(u32, u32),
            Struct { a: u32, b: String },
        }

        let a = TestEnum::Unit;
        let n = ser::to_tree(&a).unwrap();
        println!("{}", n);
        let a: TestEnum = de::from_tree(&n).unwrap();
        println!("{:?}", a);

        let a = TestEnum::Newtype(1);
        let n = ser::to_tree(&a).unwrap();
        println!("{}", n);
        let a: TestEnum = de::from_tree(&n).unwrap();
        println!("{:?}", a);

        let a = TestEnum::Tuple(2, 3);
        let n = ser::to_tree(&a).unwrap();
        println!("{}", n);
        let a: TestEnum = de::from_tree(&n).unwrap();
        println!("{:?}", a);

        let a = TestEnum::Struct { a: 2, b: "aaa".to_string() };
        let n = ser::to_tree(&a).unwrap();
        println!("{}", n);
        let a: TestEnum = de::from_tree(&n).unwrap();
        println!("{:?}", a);
    }
}
