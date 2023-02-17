use std::io::Read;

use serde::Deserialize;
use serde::de::{
    self, DeserializeSeed, SeqAccess, Visitor
};

use super::error::{Error, Result};

pub struct Deserializer<R> where R: Read {
    reader: R,
}

impl<R> Deserializer<R> where R: Read {
    pub fn from_reader(reader: R) -> Self {
        Deserializer { reader }
    }
}

pub fn from_reader<'a, T, R>(reader: R) -> Result<T>
where
    T: Deserialize<'a>,
    R: Read
{
    let mut deserializer = Deserializer::from_reader(reader);
    T::deserialize(&mut deserializer)
}

impl<R> Deserializer<R> where R: Read {
    fn parse_i32(&mut self) -> Result<i32> {
        let mut buf = [0; 4];
        self.reader.read_exact(&mut buf).map_err(|e| Error::InputOutput(e))?;
        Ok(i32::from_le_bytes(buf))
    }

    fn parse_u32(&mut self) -> Result<u32> {
        let mut buf = [0; 4];
        self.reader.read_exact(&mut buf).map_err(|e| Error::InputOutput(e))?;
        Ok(u32::from_le_bytes(buf))
    }

    fn parse_f32(&mut self) -> Result<f32> {
        let mut buf = [0; 4];
        self.reader.read_exact(&mut buf).map_err(|e| Error::InputOutput(e))?;
        Ok(f32::from_le_bytes(buf))
    }

    fn parse_bool(&mut self) -> Result<bool> {
        let int = self.parse_i32()?;
        Ok(int != 0)
    }

    // Requires the string to be valid UTF-8, which might be a bit too strict.
    fn parse_string(&mut self) -> Result<String> {
        let length = self.parse_i32()? as usize;
        let mut buf = String::with_capacity(length);
        self.reader.by_ref().take(length as u64).read_to_string(&mut buf).map_err(Error::InputOutput)?;
        Ok(buf)
    }
}

impl<'de, 'a, R> de::Deserializer<'de> for &'a mut Deserializer<R>
where
    R: Read
{
    type Error = Error;

    fn deserialize_any<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        unimplemented!("deserialize_any not supported");
    }

    fn deserialize_bool<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_bool(self.parse_bool()?)
    }

    fn deserialize_i8<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_i8(self.parse_i32()? as i8)
    }

    fn deserialize_i16<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_i16(self.parse_i32()? as i16)
    }

    fn deserialize_i32<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_i32(self.parse_i32()?)
    }

    fn deserialize_i64<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_i64(self.parse_i32()?.into())
    }

    fn deserialize_u8<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_u8(self.parse_u32()? as u8)
    }

    fn deserialize_u16<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_u16(self.parse_u32()? as u16)
    }

    fn deserialize_u32<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_u32(self.parse_u32()?)
    }

    fn deserialize_u64<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_u64(self.parse_u32()?.into())
    }

    fn deserialize_f32<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_f32(self.parse_f32()?)
    }

    fn deserialize_f64<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_f64(self.parse_f32()?.into())
    }

    fn deserialize_char<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        let string = self.parse_string()?;
        if string.len() == 1 {
            visitor.visit_char(string.chars().next().unwrap())
        } else {
            Err(Error::Message("more than one character found for char".to_owned()))
        }
    }

    fn deserialize_str<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        //visitor.visit_borrowed_str(&self.parse_string()?)
        visitor.visit_string(self.parse_string()?)
    }

    fn deserialize_string<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        self.deserialize_str(visitor)
    }

    fn deserialize_bytes<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_byte_buf<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_option<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        if self.parse_i32()? == 0 {
            visitor.visit_none()
        } else {
            visitor.visit_some(self)
        }
    }

    fn deserialize_unit<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_unit()
    }

    fn deserialize_unit_struct<V>(
        self,
        _name: &'static str,
        visitor: V,
    ) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        self.deserialize_unit(visitor)
    }

    fn deserialize_newtype_struct<V>(
        self,
        _name: &'static str,
        visitor: V,
    ) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_newtype_struct(self)
    }

    fn deserialize_seq<V>(mut self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_seq(EndlessAccess::new(&mut self))
    }

    fn deserialize_tuple<V>(self, _len: usize, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        self.deserialize_seq(visitor)
    }

    fn deserialize_tuple_struct<V>(
        self,
        _name: &'static str,
        _len: usize,
        visitor: V,
    ) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        self.deserialize_seq(visitor)
    }

    fn deserialize_map<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        self.deserialize_seq(visitor)
    }

    fn deserialize_struct<V>(
        self,
        _name: &'static str,
        _fields: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        self.deserialize_seq(visitor)
    }

    fn deserialize_enum<V>(
        self,
        _name: &'static str,
        _variants: &'static [&'static str],
        _visitor: V,
    ) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        unimplemented!();
    }

    fn deserialize_identifier<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        self.deserialize_i32(visitor)
    }

    fn deserialize_ignored_any<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        self.deserialize_any(visitor)
    }
}

struct EndlessAccess<'a, R: Read> {
    de: &'a mut Deserializer<R>,
}

impl<'a, R: Read> EndlessAccess<'a, R> {
    fn new(de: &'a mut Deserializer<R>) -> Self {
        EndlessAccess {
            de
        }
    }
}

impl<'de, 'a, R: Read> SeqAccess<'de> for EndlessAccess<'a, R> {
    type Error = Error;

    fn next_element_seed<T>(&mut self, seed: T) -> Result<Option<T::Value>>
    where
        T: DeserializeSeed<'de>,
    {
        seed.deserialize(&mut *self.de).map(Some)
    }
}