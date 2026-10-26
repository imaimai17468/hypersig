//! Reads the query string of a page into its [`Search`](crate::Search) struct.
//!
//! axum's `Query` reads with `serde_urlencoded`, which cannot read a key that
//! appears more than once into a `Vec`. This reader groups the pairs by key
//! and hands each field of the struct its group: a `Vec` field takes every
//! value of its key, in order, and any other field takes the one value.

use serde::de::value::{Error, SeqDeserializer, StrDeserializer};
use serde::de::{
    self, DeserializeOwned, DeserializeSeed, Deserializer, IntoDeserializer, MapAccess, Visitor,
};

use crate::route::percent_escape;

/// Reads `query`, without its `?`, into `T`.
///
/// A field whose key is absent reads as an empty group, so a `Vec` is empty,
/// an `Option` is `None`, and any other field is missing. A key the struct
/// does not have is ignored, and so is a key that does not decode to UTF-8,
/// which no field's name can match.
pub(crate) fn parse_query<T: DeserializeOwned>(query: &str) -> Result<T, Error> {
    let pairs = query
        .split('&')
        .filter(|pair| !pair.is_empty())
        .filter_map(|pair| {
            let (name, value) = pair.split_once('=').unwrap_or((pair, ""));
            let name = decode(name).ok()?;
            Some(decode(value).map(|value| (name, value)))
        })
        .collect::<Result<Vec<(String, String)>, Error>>()?;
    T::deserialize(Query { pairs: &pairs })
}

/// Percent-decodes `text` as a form does, with `+` standing for a space.
///
/// # Errors
///
/// Returns an error where the bytes are not UTF-8, which a browser never
/// sends for text a visitor typed.
fn decode(text: &str) -> Result<String, Error> {
    let bytes = text.as_bytes();
    let mut out = Vec::with_capacity(bytes.len());
    let mut index = 0;
    while let Some(&byte) = bytes.get(index) {
        match (byte, percent_escape(bytes, index)) {
            (_, Some(escaped)) => {
                out.push(escaped);
                index += 3;
            }
            (b'+', None) => {
                out.push(b' ');
                index += 1;
            }
            (_, None) => {
                out.push(byte);
                index += 1;
            }
        }
    }
    String::from_utf8(out).map_err(|error| {
        de::Error::custom(format_args!("`{text}` does not decode to UTF-8: {error}"))
    })
}

struct Query<'q> {
    pairs: &'q [(String, String)],
}

impl<'de> Deserializer<'de> for Query<'_> {
    type Error = Error;

    fn deserialize_struct<V: Visitor<'de>>(
        self,
        _name: &'static str,
        fields: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Error> {
        visitor.visit_map(Fields {
            pairs: self.pairs,
            names: fields.iter(),
            current: "",
        })
    }

    fn deserialize_any<V: Visitor<'de>>(self, _visitor: V) -> Result<V::Value, Error> {
        Err(de::Error::custom(
            "search parameters are a struct with named fields",
        ))
    }

    serde::forward_to_deserialize_any! {
        bool i8 i16 i32 i64 i128 u8 u16 u32 u64 u128 f32 f64 char str string
        bytes byte_buf option unit unit_struct newtype_struct seq tuple
        tuple_struct map enum identifier ignored_any
    }
}

/// Each declared field of the struct, with the values of its key.
struct Fields<'q> {
    pairs: &'q [(String, String)],
    names: std::slice::Iter<'static, &'static str>,
    current: &'static str,
}

impl<'de> MapAccess<'de> for Fields<'_> {
    type Error = Error;

    fn next_key_seed<K: DeserializeSeed<'de>>(
        &mut self,
        seed: K,
    ) -> Result<Option<K::Value>, Error> {
        let Some(&field) = self.names.next() else {
            return Ok(None);
        };
        self.current = field;
        seed.deserialize(StrDeserializer::<Error>::new(field))
            .map(Some)
    }

    fn next_value_seed<V: DeserializeSeed<'de>>(&mut self, seed: V) -> Result<V::Value, Error> {
        let field = self.current;
        let values = self
            .pairs
            .iter()
            .filter(|(name, _)| name == field)
            .map(|(_, value)| value.as_str())
            .collect();
        seed.deserialize(Group { field, values })
    }
}

/// The values of one key, in the order the query string holds them.
struct Group<'q> {
    field: &'static str,
    values: Vec<&'q str>,
}

impl<'q> Group<'q> {
    fn one(self) -> Result<Part<'q>, Error> {
        match self.values.as_slice() {
            [one] => Ok(Part(one)),
            [] => Err(de::Error::missing_field(self.field)),
            _ => Err(de::Error::duplicate_field(self.field)),
        }
    }
}

macro_rules! forward_to_one {
    ($($method:ident),* $(,)?) => {$(
        fn $method<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value, Error> {
            self.one()?.$method(visitor)
        }
    )*};
}

impl<'de> Deserializer<'de> for Group<'_> {
    type Error = Error;

    fn deserialize_seq<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value, Error> {
        visitor.visit_seq(SeqDeserializer::new(self.values.into_iter().map(Part)))
    }

    fn deserialize_option<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value, Error> {
        if self.values.is_empty() {
            visitor.visit_none()
        } else {
            visitor.visit_some(self.one()?)
        }
    }

    fn deserialize_newtype_struct<V: Visitor<'de>>(
        self,
        _name: &'static str,
        visitor: V,
    ) -> Result<V::Value, Error> {
        visitor.visit_newtype_struct(self)
    }

    fn deserialize_enum<V: Visitor<'de>>(
        self,
        name: &'static str,
        variants: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Error> {
        self.one()?.deserialize_enum(name, variants, visitor)
    }

    fn deserialize_unit_struct<V: Visitor<'de>>(
        self,
        name: &'static str,
        visitor: V,
    ) -> Result<V::Value, Error> {
        self.one()?.deserialize_unit_struct(name, visitor)
    }

    fn deserialize_tuple<V: Visitor<'de>>(self, len: usize, visitor: V) -> Result<V::Value, Error> {
        self.one()?.deserialize_tuple(len, visitor)
    }

    fn deserialize_tuple_struct<V: Visitor<'de>>(
        self,
        name: &'static str,
        len: usize,
        visitor: V,
    ) -> Result<V::Value, Error> {
        self.one()?.deserialize_tuple_struct(name, len, visitor)
    }

    fn deserialize_struct<V: Visitor<'de>>(
        self,
        name: &'static str,
        fields: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Error> {
        self.one()?.deserialize_struct(name, fields, visitor)
    }

    forward_to_one!(
        deserialize_any,
        deserialize_bool,
        deserialize_i8,
        deserialize_i16,
        deserialize_i32,
        deserialize_i64,
        deserialize_i128,
        deserialize_u8,
        deserialize_u16,
        deserialize_u32,
        deserialize_u64,
        deserialize_u128,
        deserialize_f32,
        deserialize_f64,
        deserialize_char,
        deserialize_str,
        deserialize_string,
        deserialize_bytes,
        deserialize_byte_buf,
        deserialize_unit,
        deserialize_map,
        deserialize_identifier,
        deserialize_ignored_any,
    );
}

/// One value of the query string, parsed into the type the field asks for.
struct Part<'q>(&'q str);

impl IntoDeserializer<'_, Error> for Part<'_> {
    type Deserializer = Self;

    fn into_deserializer(self) -> Self {
        self
    }
}

macro_rules! parse_float_as {
    ($($method:ident => $visit:ident),* $(,)?) => {$(
        fn $method<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value, Error> {
            match self.0.parse() {
                // A signal that is not a finite number is written as `null`
                // in JSON, so the page would not hold what the URL says.
                Ok(value) if !f64::from(value).is_finite() => {
                    Err(de::Error::custom(format_args!("`{}` is not a finite number", self.0)))
                }
                Ok(value) => visitor.$visit(value),
                Err(error) => Err(de::Error::custom(format_args!("`{}`: {error}", self.0))),
            }
        }
    )*};
}

macro_rules! parse_as {
    ($($method:ident => $visit:ident),* $(,)?) => {$(
        fn $method<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value, Error> {
            match self.0.parse() {
                Ok(value) => visitor.$visit(value),
                Err(error) => Err(de::Error::custom(format_args!("`{}`: {error}", self.0))),
            }
        }
    )*};
}

impl<'de> Deserializer<'de> for Part<'_> {
    type Error = Error;

    fn deserialize_any<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value, Error> {
        visitor.visit_str(self.0)
    }

    parse_as!(
        deserialize_bool => visit_bool,
        deserialize_i8 => visit_i8,
        deserialize_i16 => visit_i16,
        deserialize_i32 => visit_i32,
        deserialize_u8 => visit_u8,
        deserialize_u16 => visit_u16,
        deserialize_u32 => visit_u32,
        deserialize_i64 => visit_i64,
        deserialize_i128 => visit_i128,
        deserialize_u64 => visit_u64,
        deserialize_u128 => visit_u128,
        deserialize_char => visit_char,
    );

    parse_float_as!(
        deserialize_f32 => visit_f32,
        deserialize_f64 => visit_f64,
    );

    fn deserialize_option<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value, Error> {
        visitor.visit_some(self)
    }

    fn deserialize_newtype_struct<V: Visitor<'de>>(
        self,
        _name: &'static str,
        visitor: V,
    ) -> Result<V::Value, Error> {
        visitor.visit_newtype_struct(self)
    }

    fn deserialize_enum<V: Visitor<'de>>(
        self,
        _name: &'static str,
        _variants: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Error> {
        visitor.visit_enum(StrDeserializer::<Error>::new(self.0))
    }

    serde::forward_to_deserialize_any! {
        str string bytes byte_buf unit unit_struct seq tuple tuple_struct map
        struct identifier ignored_any
    }
}

#[cfg(test)]
mod tests {
    use serde::Deserialize;

    use super::{decode, parse_query};

    #[derive(Deserialize, Debug, PartialEq)]
    #[serde(rename_all = "lowercase")]
    enum Color {
        Red,
        Blue,
    }

    #[derive(Deserialize, Debug, PartialEq)]
    struct Filter {
        q: String,
        page: Option<i32>,
        open: bool,
        color: Vec<Color>,
        size: Vec<i64>,
    }

    #[test]
    fn reads_repeated_keys_into_a_list_and_absent_ones_as_empty() {
        let filter: Filter =
            parse_query("color=red&q=a%20b+c&open=true&color=blue&utm=x").expect("parses");
        assert_eq!(
            filter,
            Filter {
                q: "a b c".to_owned(),
                page: None,
                open: true,
                color: vec![Color::Red, Color::Blue],
                size: Vec::new(),
            }
        );
        let filter: Filter = parse_query("q=&page=3&open=false&size=12").expect("parses");
        assert_eq!(
            (filter.q.as_str(), filter.page, filter.size),
            ("", Some(3), vec![12])
        );
    }

    #[test]
    fn rejects_a_missing_field_a_repeated_one_value_and_a_bad_value() {
        let missing = parse_query::<Filter>("open=true").expect_err("q is missing");
        assert_eq!(missing.to_string(), "missing field `q`");
        let twice = parse_query::<Filter>("q=a&q=b&open=true").expect_err("q is repeated");
        assert_eq!(twice.to_string(), "duplicate field `q`");
        parse_query::<Filter>("q=a&open=yes").expect_err("rejected");
        parse_query::<Filter>("q=a&open=true&color=green").expect_err("rejected");
        parse_query::<Filter>("q=a&open=true&size=1.5").expect_err("rejected");
    }

    #[derive(Deserialize, Debug)]
    struct Price {
        max: f64,
    }

    #[test]
    fn ignores_a_key_that_is_not_utf_8_and_refuses_such_a_value() {
        assert_eq!(
            parse_query::<Price>("%FF=1&max=2")
                .expect("key ignored")
                .max,
            2.0
        );
        let error = parse_query::<Price>("max=%FF").expect_err("value refused");
        assert!(
            error.to_string().contains("does not decode to UTF-8"),
            "{error}"
        );
    }

    #[test]
    fn rejects_a_number_that_is_not_finite() {
        assert_eq!(parse_query::<Price>("max=1.5").expect("finite").max, 1.5);
        for max in ["NaN", "inf", "-inf", "infinity"] {
            let error = parse_query::<Price>(&format!("max={max}")).expect_err(max);
            assert!(
                error.to_string().contains("is not a finite number"),
                "{error}"
            );
        }
    }

    #[test]
    fn reads_an_integer_past_two_to_the_53_exactly() {
        let filter: Filter =
            parse_query("q=a&open=true&size=9007199254740993").expect("a u64 past 2^53");
        assert_eq!(filter.size, vec![(1 << 53) + 1]);
    }

    #[test]
    fn decodes_percent_escapes_and_plus_signs() {
        assert_eq!(decode("%E3%81%82+%2B%zz%4").expect("UTF-8"), "あ +%zz%4");
    }
}
