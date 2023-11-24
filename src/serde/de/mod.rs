//! Deserializing BESS into Rust structures.

use std::mem;

use serde::{de, Deserialize};

use crate::{Bess, Error, Result};

mod decode;

use self::decode::Decode;

impl TryFrom<&[u8]> for Bess {
    type Error = Error;

    #[inline]
    fn try_from(bytes: &[u8]) -> Result<Self> {
        self::from_bytes(bytes)
    }
}

/// Deserializes an instance of a `Bess` from bytes.
///
/// # Errors
///
/// Returns an error when the input is invalid.
pub fn from_bytes(bytes: &[u8]) -> Result<Bess> {
    let mut de = Deserializer::from_bytes(bytes);
    let bess = Bess::deserialize(&mut de)?;
    if de.input.is_empty() {
        Ok(bess)
    } else {
        todo!("trailing characters"); // FIXME
    }
}

/// A structure that deserializes BESS into Rust structures.
#[derive(Debug)]
struct Deserializer<'de> {
    input: &'de [u8],
}

impl<'de> Deserializer<'de> {
    /// Constructs a `Deserializer` from a byte array.
    #[must_use]
    pub fn from_bytes(input: &'de [u8]) -> Self {
        Self { input }
    }

    /// Pops a slice off the front of the input buffer.
    fn pop(&mut self, len: usize) -> &[u8] {
        let pop = &self.input[..len];
        self.input = &self.input[len..];
        pop
    }

    /// Pops an array reference off the front of the input buffer.
    fn pop_ref<const N: usize>(&mut self) -> &[u8; N] {
        assert!(N <= self.input.len());
        let (pop, rem) = self.input.split_at(N);
        let pop = pop.try_into().unwrap();
        self.input = rem;
        pop
    }
}

impl<'de> de::Deserializer<'de> for &mut Deserializer<'de> {
    type Error = Error;

    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        Err(Error::Unsupported)
    }

    fn deserialize_bool<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        let bytes = self.pop_ref::<{ mem::size_of::<u8>() }>();
        let value = match u8::from_le_bytes(*bytes) {
            0b0 => false,
            0b1 => true,
            _ => panic!(),
        };
        visitor.visit_bool(value)
    }

    fn deserialize_i8<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        visitor.visit_i8(i8::from_le_bytes(*self.pop_ref()))
    }

    fn deserialize_i16<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        visitor.visit_i16(i16::from_le_bytes(*self.pop_ref()))
    }

    fn deserialize_i32<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        visitor.visit_i32(i32::from_le_bytes(*self.pop_ref()))
    }

    fn deserialize_i64<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        visitor.visit_i64(i64::from_le_bytes(*self.pop_ref()))
    }

    fn deserialize_u8<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        visitor.visit_u8(u8::from_le_bytes(*self.pop_ref()))
    }

    fn deserialize_u16<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        visitor.visit_u16(u16::from_le_bytes(*self.pop_ref()))
    }

    fn deserialize_u32<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        visitor.visit_u32(u32::from_le_bytes(*self.pop_ref()))
    }

    fn deserialize_u64<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        visitor.visit_u64(u64::from_le_bytes(*self.pop_ref()))
    }

    fn deserialize_f32<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        visitor.visit_f32(f32::from_le_bytes(*self.pop_ref()))
    }

    fn deserialize_f64<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        visitor.visit_f64(f64::from_le_bytes(*self.pop_ref()))
    }

    fn deserialize_char<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        Err(Error::Unsupported)
    }

    fn deserialize_str<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        let bytes = self.input;
        let str = std::str::from_utf8(bytes).unwrap();
        visitor.visit_borrowed_str(str)
    }

    fn deserialize_string<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        self.deserialize_str(visitor)
    }

    fn deserialize_bytes<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        let bytes = self.input;
        visitor.visit_borrowed_bytes(bytes)
    }

    fn deserialize_byte_buf<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        self.deserialize_bytes(visitor)
    }

    fn deserialize_option<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        Err(Error::Unsupported)
    }

    fn deserialize_unit<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        visitor.visit_unit()
    }

    fn deserialize_unit_struct<V>(self, name: &'static str, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        visitor.visit_unit()
    }

    fn deserialize_newtype_struct<V>(self, name: &'static str, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        visitor.visit_newtype_struct(self)
    }

    fn deserialize_seq<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        self.deserialize_tuple(self.input.len(), visitor)
    }

    fn deserialize_tuple<V>(self, len: usize, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        let mut de = Deserializer::from_bytes(&self.input[..len]);
        visitor.visit_seq(&mut de)
    }

    fn deserialize_tuple_struct<V>(
        self,
        name: &'static str,
        len: usize,
        visitor: V,
    ) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        self.deserialize_tuple(len, visitor)
    }

    fn deserialize_map<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        self.deserialize_tuple(self.input.len(), visitor)
    }

    fn deserialize_struct<V>(
        self,
        name: &'static str,
        fields: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        self.deserialize_tuple(fields.len(), visitor)
    }

    fn deserialize_enum<V>(
        self,
        name: &'static str,
        variants: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        visitor.visit_enum(self)
    }

    fn deserialize_identifier<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        Err(Error::Unsupported)
    }

    fn deserialize_ignored_any<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        Err(Error::Unsupported)
    }
}

impl<'de> de::EnumAccess<'de> for &mut Deserializer<'de> {
    type Error = Error;
    type Variant = Self;

    fn variant_seed<V>(self, seed: V) -> Result<(V::Value, Self::Variant)>
    where
        V: de::DeserializeSeed<'de>,
    {
        let idx = u32::deserialize(&mut *self)?;
        let val = seed.deserialize(de::IntoDeserializer::<Error>::into_deserializer(idx))?;
        Ok((val, self))
    }
}

impl<'de> de::SeqAccess<'de> for &mut Deserializer<'de> {
    type Error = Error;

    fn next_element_seed<T>(&mut self, seed: T) -> Result<Option<T::Value>>
    where
        T: de::DeserializeSeed<'de>,
    {
        todo!()
    }
}

impl<'de> de::VariantAccess<'de> for &mut Deserializer<'de> {
    type Error = Error;

    fn unit_variant(self) -> Result<()> {
        Ok(())
    }

    fn newtype_variant_seed<T>(self, seed: T) -> Result<T::Value>
    where
        T: de::DeserializeSeed<'de>,
    {
        seed.deserialize(self)
    }

    fn tuple_variant<V>(self, len: usize, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        de::Deserializer::deserialize_tuple(self, len, visitor)
    }

    fn struct_variant<V>(self, fields: &'static [&'static str], visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        de::Deserializer::deserialize_tuple(self, fields.len(), visitor)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::build::tests::{setup, BYTES};

    #[test]
    fn int_deserialize_works() {
        let test = &[0x34, 0x12];
        let expect: u16 = 0x1234;

        let mut de = Deserializer::from_bytes(test);
        let found = u16::deserialize(&mut de).unwrap();

        assert_eq!(found, expect);
    }

    #[test]
    fn bytes_deserialize_works() {
        let test: Vec<_> = [0x34_u8, 0x12].into_iter().cycle().take(0x200).collect();
        let expect = [0x1234_u16; 0x100];

        let mut de = Deserializer::from_bytes(&test);
        let found = Vec::<u16>::deserialize(&mut de).unwrap();

        assert_eq!(found, expect);
    }

    #[test]
    fn struct_serialize_works() {
        #[derive(Debug, Deserialize, PartialEq)]
        struct Test {
            int: u32,
            seq: Vec<String>,
        }

        let test = &[0x01, 0x00, 0x00, 0x00, b'a', b'b'];
        let expect = Test {
            int: 1,
            seq: vec!["a".to_string(), "b".to_string()],
        };

        let mut de = Deserializer::from_bytes(test);
        let found = Test::deserialize(&mut de).unwrap();

        assert_eq!(found, expect);
    }

    #[test]
    fn builder_serialize_works() {
        let test = Bess::try_from(BYTES).unwrap();
        let expect = setup();

        assert_eq!(test.to_bytes(), expect.to_bytes());
    }
}
