//! Whether a value holds only finite numbers, read through its `Serialize` impl.

use std::fmt;

use serde::Serialize;
use serde::ser;

/// Whether every `f32` and `f64` that `value` serializes is finite.
///
/// `serde_json` writes an infinite or NaN number as `null`, which a rule then
/// reads as a missing value, so the check runs on the value before it becomes
/// JSON. A `Serialize` impl that fails counts as finite here, because the
/// later conversion to JSON reports that failure itself.
pub(crate) fn all_finite<T: Serialize + ?Sized>(value: &T) -> bool {
    !matches!(value.serialize(Walk), Err(Found::NotFinite))
}

/// Stops the walk at the first number that is not finite.
#[derive(Debug)]
enum Found {
    NotFinite,
    Other,
}

impl fmt::Display for Found {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            Self::NotFinite => "a number is not finite",
            Self::Other => "the value failed to serialize",
        })
    }
}

impl std::error::Error for Found {}

impl ser::Error for Found {
    fn custom<M: fmt::Display>(_: M) -> Self {
        Self::Other
    }
}

/// Visits every value and keeps nothing but the answer.
struct Walk;

fn float(finite: bool) -> Result<(), Found> {
    if finite {
        Ok(())
    } else {
        Err(Found::NotFinite)
    }
}

macro_rules! leaves {
    ($($method:ident($ty:ty)),* $(,)?) => {$(
        fn $method(self, _: $ty) -> Result<(), Found> {
            Ok(())
        }
    )*};
}

impl ser::Serializer for Walk {
    type Ok = ();
    type Error = Found;
    type SerializeSeq = Self;
    type SerializeTuple = Self;
    type SerializeTupleStruct = Self;
    type SerializeTupleVariant = Self;
    type SerializeMap = Self;
    type SerializeStruct = Self;
    type SerializeStructVariant = Self;

    leaves!(
        serialize_bool(bool),
        serialize_i8(i8),
        serialize_i16(i16),
        serialize_i32(i32),
        serialize_i64(i64),
        serialize_i128(i128),
        serialize_u8(u8),
        serialize_u16(u16),
        serialize_u32(u32),
        serialize_u64(u64),
        serialize_u128(u128),
        serialize_char(char),
        serialize_str(&str),
        serialize_bytes(&[u8]),
        serialize_unit_struct(&'static str),
    );

    fn serialize_f32(self, v: f32) -> Result<(), Found> {
        float(v.is_finite())
    }

    fn serialize_f64(self, v: f64) -> Result<(), Found> {
        float(v.is_finite())
    }

    fn serialize_none(self) -> Result<(), Found> {
        Ok(())
    }

    fn serialize_some<V: Serialize + ?Sized>(self, value: &V) -> Result<(), Found> {
        value.serialize(self)
    }

    fn serialize_unit(self) -> Result<(), Found> {
        Ok(())
    }

    fn serialize_unit_variant(self, _: &'static str, _: u32, _: &'static str) -> Result<(), Found> {
        Ok(())
    }

    fn serialize_newtype_struct<V: Serialize + ?Sized>(
        self,
        _: &'static str,
        value: &V,
    ) -> Result<(), Found> {
        value.serialize(self)
    }

    fn serialize_newtype_variant<V: Serialize + ?Sized>(
        self,
        _: &'static str,
        _: u32,
        _: &'static str,
        value: &V,
    ) -> Result<(), Found> {
        value.serialize(self)
    }

    fn serialize_seq(self, _: Option<usize>) -> Result<Self, Found> {
        Ok(self)
    }

    fn serialize_tuple(self, _: usize) -> Result<Self, Found> {
        Ok(self)
    }

    fn serialize_tuple_struct(self, _: &'static str, _: usize) -> Result<Self, Found> {
        Ok(self)
    }

    fn serialize_tuple_variant(
        self,
        _: &'static str,
        _: u32,
        _: &'static str,
        _: usize,
    ) -> Result<Self, Found> {
        Ok(self)
    }

    fn serialize_map(self, _: Option<usize>) -> Result<Self, Found> {
        Ok(self)
    }

    fn serialize_struct(self, _: &'static str, _: usize) -> Result<Self, Found> {
        Ok(self)
    }

    fn serialize_struct_variant(
        self,
        _: &'static str,
        _: u32,
        _: &'static str,
        _: usize,
    ) -> Result<Self, Found> {
        Ok(self)
    }
}

impl ser::SerializeSeq for Walk {
    type Ok = ();
    type Error = Found;
    fn serialize_element<V: Serialize + ?Sized>(&mut self, value: &V) -> Result<(), Found> {
        value.serialize(Walk)
    }
    fn end(self) -> Result<(), Found> {
        Ok(())
    }
}

impl ser::SerializeTuple for Walk {
    type Ok = ();
    type Error = Found;
    fn serialize_element<V: Serialize + ?Sized>(&mut self, value: &V) -> Result<(), Found> {
        value.serialize(Walk)
    }
    fn end(self) -> Result<(), Found> {
        Ok(())
    }
}

impl ser::SerializeTupleStruct for Walk {
    type Ok = ();
    type Error = Found;
    fn serialize_field<V: Serialize + ?Sized>(&mut self, value: &V) -> Result<(), Found> {
        value.serialize(Walk)
    }
    fn end(self) -> Result<(), Found> {
        Ok(())
    }
}

impl ser::SerializeTupleVariant for Walk {
    type Ok = ();
    type Error = Found;
    fn serialize_field<V: Serialize + ?Sized>(&mut self, value: &V) -> Result<(), Found> {
        value.serialize(Walk)
    }
    fn end(self) -> Result<(), Found> {
        Ok(())
    }
}

impl ser::SerializeMap for Walk {
    type Ok = ();
    type Error = Found;
    fn serialize_key<V: Serialize + ?Sized>(&mut self, key: &V) -> Result<(), Found> {
        key.serialize(Walk)
    }
    fn serialize_value<V: Serialize + ?Sized>(&mut self, value: &V) -> Result<(), Found> {
        value.serialize(Walk)
    }
    fn end(self) -> Result<(), Found> {
        Ok(())
    }
}

impl ser::SerializeStruct for Walk {
    type Ok = ();
    type Error = Found;
    fn serialize_field<V: Serialize + ?Sized>(
        &mut self,
        _: &'static str,
        value: &V,
    ) -> Result<(), Found> {
        value.serialize(Walk)
    }
    fn end(self) -> Result<(), Found> {
        Ok(())
    }
}

impl ser::SerializeStructVariant for Walk {
    type Ok = ();
    type Error = Found;
    fn serialize_field<V: Serialize + ?Sized>(
        &mut self,
        _: &'static str,
        value: &V,
    ) -> Result<(), Found> {
        value.serialize(Walk)
    }
    fn end(self) -> Result<(), Found> {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeMap;

    use super::all_finite;

    #[derive(serde::Serialize)]
    struct Order {
        price: f64,
        lines: Vec<(String, f32)>,
        note: Option<f64>,
    }

    #[test]
    fn finds_a_number_that_is_not_finite_anywhere_in_the_value() {
        let order = |price, line, note| Order {
            price,
            lines: vec![("a".to_owned(), line)],
            note,
        };
        assert!(all_finite(&order(1.5, 2.0, None)));
        assert!(all_finite(&order(f64::MAX, f32::MIN, Some(0.0))));
        assert!(!all_finite(&order(f64::INFINITY, 2.0, None)));
        assert!(!all_finite(&order(1.5, f32::NAN, None)));
        assert!(!all_finite(&order(1.5, 2.0, Some(f64::NEG_INFINITY))));
        assert!(!all_finite(&BTreeMap::from([("k", f64::NAN)])));
    }
}
