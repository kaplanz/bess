//! Serializing Rust structures into BESS.

use std::mem;

use serde::{ser, Serialize};

use crate::{Bess, Error, Result};

mod encode;

use self::encode::Encode;

impl Bess {
    /// Serializes `self` as a byte vector.
    #[inline]
    #[must_use]
    pub fn to_bytes(self) -> Vec<u8> {
        self::to_bytes(&self)
    }
}

/// Serializes the given `Bess` structure as a byte vector.
#[must_use]
pub fn to_bytes(bess: &Bess) -> Vec<u8> {
    let mut ser = Serializer::default();
    bess.serialize(&mut ser).unwrap();
    ser.output
}

/// A structure for serializing Rust structures into BESS.
#[derive(Debug, Default)]
struct Serializer {
    output: Vec<u8>,
}

impl ser::Serializer for &mut Serializer {
    type Ok = ();
    type Error = Error;

    type SerializeSeq = Self;
    type SerializeTuple = Self;
    type SerializeTupleStruct = Self;
    type SerializeTupleVariant = Self;
    type SerializeMap = Self;
    type SerializeStruct = Self;
    type SerializeStructVariant = Self;

    fn serialize_bool(self, v: bool) -> Result<Self::Ok> {
        v.encode(&mut self.output).map(|_| ())
    }

    fn serialize_i8(self, v: i8) -> Result<Self::Ok> {
        v.encode(&mut self.output).map(|_| ())
    }

    fn serialize_i16(self, v: i16) -> Result<Self::Ok> {
        v.encode(&mut self.output).map(|_| ())
    }

    fn serialize_i32(self, v: i32) -> Result<Self::Ok> {
        v.encode(&mut self.output).map(|_| ())
    }

    fn serialize_i64(self, v: i64) -> Result<Self::Ok> {
        v.encode(&mut self.output).map(|_| ())
    }

    fn serialize_u8(self, v: u8) -> Result<Self::Ok> {
        v.encode(&mut self.output).map(|_| ())
    }

    fn serialize_u16(self, v: u16) -> Result<Self::Ok> {
        v.encode(&mut self.output).map(|_| ())
    }

    fn serialize_u32(self, v: u32) -> Result<Self::Ok> {
        v.encode(&mut self.output).map(|_| ())
    }

    fn serialize_u64(self, v: u64) -> Result<Self::Ok> {
        v.encode(&mut self.output).map(|_| ())
    }

    fn serialize_f32(self, v: f32) -> Result<Self::Ok> {
        v.encode(&mut self.output).map(|_| ())
    }

    fn serialize_f64(self, v: f64) -> Result<Self::Ok> {
        v.encode(&mut self.output).map(|_| ())
    }

    fn serialize_char(self, v: char) -> Result<Self::Ok> {
        Err(Error::Unsupported)
    }

    fn serialize_str(self, v: &str) -> Result<Self::Ok> {
        v.as_bytes().serialize(self)
    }

    fn serialize_bytes(self, v: &[u8]) -> Result<Self::Ok> {
        v.encode(&mut self.output).map(|_| ())
    }

    fn serialize_none(self) -> Result<Self::Ok> {
        Err(Error::Unsupported)
    }

    fn serialize_some<T: ?Sized>(self, value: &T) -> Result<Self::Ok>
    where
        T: serde::Serialize,
    {
        Err(Error::Unsupported)
    }

    fn serialize_unit(self) -> Result<Self::Ok> {
        Ok(())
    }

    fn serialize_unit_struct(self, name: &'static str) -> Result<Self::Ok> {
        Ok(())
    }

    fn serialize_unit_variant(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
    ) -> Result<Self::Ok> {
        if let Ok(idx) = u8::try_from(variant_index) {
            idx.serialize(self)
        } else if let Ok(idx) = u16::try_from(variant_index) {
            idx.serialize(self)
        } else {
            variant_index.serialize(self)
        }
    }

    fn serialize_newtype_struct<T: ?Sized>(self, name: &'static str, value: &T) -> Result<Self::Ok>
    where
        T: serde::Serialize,
    {
        value.serialize(self)
    }

    fn serialize_newtype_variant<T: ?Sized>(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
        value: &T,
    ) -> Result<Self::Ok>
    where
        T: serde::Serialize,
    {
        value.serialize(self)
    }

    fn serialize_seq(self, len: Option<usize>) -> Result<Self::SerializeSeq> {
        Ok(self)
    }

    fn serialize_tuple(self, len: usize) -> Result<Self::SerializeTuple> {
        Ok(self)
    }

    fn serialize_tuple_struct(
        self,
        name: &'static str,
        len: usize,
    ) -> Result<Self::SerializeTupleStruct> {
        Ok(self)
    }

    fn serialize_tuple_variant(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
        len: usize,
    ) -> Result<Self::SerializeTupleVariant> {
        Ok(self)
    }

    fn serialize_map(self, len: Option<usize>) -> Result<Self::SerializeMap> {
        Ok(self)
    }

    fn serialize_struct(self, name: &'static str, len: usize) -> Result<Self::SerializeStruct> {
        Ok(self)
    }

    fn serialize_struct_variant(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
        len: usize,
    ) -> Result<Self::SerializeStructVariant> {
        Ok(self)
    }
}

impl ser::SerializeSeq for &mut Serializer {
    type Ok = ();
    type Error = Error;

    fn serialize_element<T: ?Sized>(&mut self, value: &T) -> Result<()>
    where
        T: Serialize,
    {
        value.serialize(&mut **self)
    }

    fn end(self) -> Result<Self::Ok> {
        Ok(())
    }
}

impl ser::SerializeTuple for &mut Serializer {
    type Ok = ();
    type Error = Error;

    fn serialize_element<T: ?Sized>(&mut self, value: &T) -> Result<()>
    where
        T: Serialize,
    {
        value.serialize(&mut **self)
    }

    fn end(self) -> Result<Self::Ok> {
        Ok(())
    }
}

impl ser::SerializeTupleStruct for &mut Serializer {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T: ?Sized>(&mut self, value: &T) -> Result<()>
    where
        T: Serialize,
    {
        value.serialize(&mut **self)
    }

    fn end(self) -> Result<Self::Ok> {
        Ok(())
    }
}

impl ser::SerializeTupleVariant for &mut Serializer {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T: ?Sized>(&mut self, value: &T) -> Result<()>
    where
        T: Serialize,
    {
        value.serialize(&mut **self)
    }

    fn end(self) -> Result<Self::Ok> {
        Ok(())
    }
}

impl ser::SerializeMap for &mut Serializer {
    type Ok = ();
    type Error = Error;

    fn serialize_key<T: ?Sized>(&mut self, key: &T) -> Result<()>
    where
        T: Serialize,
    {
        Ok(())
    }

    fn serialize_value<T: ?Sized>(&mut self, value: &T) -> Result<()>
    where
        T: Serialize,
    {
        value.serialize(&mut **self)
    }

    fn end(self) -> Result<Self::Ok> {
        Ok(())
    }
}

impl ser::SerializeStruct for &mut Serializer {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T: ?Sized>(&mut self, key: &'static str, value: &T) -> Result<()>
    where
        T: Serialize,
    {
        value.serialize(&mut **self)
    }

    fn end(self) -> Result<Self::Ok> {
        Ok(())
    }
}

impl ser::SerializeStructVariant for &mut Serializer {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T: ?Sized>(&mut self, key: &'static str, value: &T) -> Result<()>
    where
        T: Serialize,
    {
        value.serialize(&mut **self)
    }

    fn end(self) -> Result<Self::Ok> {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::build::tests::{setup, BYTES};

    #[test]
    fn int_serialize_works() {
        let test: u16 = 0x1234;
        let expect = [0x34, 0x12];

        let mut ser = Serializer::default();
        test.serialize(&mut ser);
        let found = ser.output;

        assert_eq!(found, expect);
    }

    #[test]
    fn bytes_serialize_works() {
        let test = [0x1234_u16; 0x100];
        let expect: Vec<_> = [0x34_u8, 0x12].into_iter().cycle().take(0x200).collect();

        let mut ser = Serializer::default();
        test.serialize(&mut ser);
        let found = ser.output;

        assert_eq!(found, expect);
    }

    #[test]
    fn struct_serialize_works() {
        #[derive(Serialize)]
        struct Test {
            int: u32,
            seq: Vec<&'static str>,
        }

        let test = Test {
            int: 1,
            seq: vec!["a", "b"],
        };
        let expect = [0x01, 0x00, 0x00, 0x00, b'a', b'b'];

        let mut ser = Serializer::default();
        test.serialize(&mut ser);
        let found = ser.output;

        assert_eq!(found, expect);
    }

    #[test]
    fn builder_serialize_works() {
        let test = setup();
        let found = test.to_bytes();
        let expect = BYTES;

        assert_eq!(found, expect);
    }
}
