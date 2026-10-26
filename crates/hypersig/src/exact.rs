//! The integers a value can hold on its way to the browser.
//!
//! The browser reads JSON numbers as 64-bit floats, which hold every integer
//! from -2^53 to 2^53 and round the ones past it to the nearest double. A
//! rounded value is read back by the server on the next action as if the
//! visitor had changed it, so a value the server sends is checked before it
//! becomes JSON, and one past the range panics. The type of such a value is
//! the application's choice, so the panic marks a bug in that choice, as a
//! `Serialize` impl that fails does: an identifier or an amount past 2^53
//! belongs in a `String`.

use serde::Serialize;
use serde_json::Value;

/// 2^53, the largest integer from which every smaller one is a double.
pub(crate) const LIMIT: u64 = 1 << 53;

/// Whether the browser reads `v` back as `v`.
pub(crate) fn holds_i64(v: i64) -> bool {
    v.unsigned_abs() <= LIMIT
}

/// Whether the browser reads `v` back as `v`.
pub(crate) fn holds_u64(v: u64) -> bool {
    v <= LIMIT
}

/// Whether the browser reads `v` back as `v`.
pub(crate) fn holds_i128(v: i128) -> bool {
    v.unsigned_abs() <= u128::from(LIMIT)
}

/// The message of the panic for `v`, which the browser would round.
pub(crate) fn message(v: impl std::fmt::Display) -> String {
    format!(
        "the integer {v} is outside -2^53 to 2^53, which the browser holds exactly; keep it in a String"
    )
}

/// `value` as JSON for the browser.
///
/// # Panics
///
/// Panics if `value` fails to serialize, saying `what` it is, or holds an
/// integer past 2^53 either way.
pub(crate) fn to_value(value: &(impl Serialize + ?Sized), what: &str) -> Value {
    let value = serde_json::to_value(value).unwrap_or_else(|error| panic!("{what}: {error}"));
    check(&value);
    value
}

/// Panics at the first integer in `value` that the browser would round.
pub(crate) fn check(value: &Value) {
    if let Some(error) = rounded(value) {
        panic!("{error}");
    }
}

/// The message for the first integer in `value` that the browser would
/// round, if one is there.
pub(crate) fn rounded(value: &Value) -> Option<String> {
    match value {
        Value::Number(n) => match (n.as_i64(), n.as_u64()) {
            (Some(v), _) if !holds_i64(v) => Some(message(v)),
            (None, Some(v)) if !holds_u64(v) => Some(message(v)),
            _ => None,
        },
        Value::Array(items) => items.iter().find_map(rounded),
        Value::Object(fields) => fields.values().find_map(rounded),
        Value::Null | Value::Bool(_) | Value::String(_) => None,
    }
}

#[cfg(feature = "axum")]
/// Reads `A` from the JSON in `json`, refusing an integer the browser would
/// round, which the page could not send back as it came.
///
/// The integers are checked as the parser hands them to `A`, so the body is
/// read once and no `Value` of it is built.
///
/// # Errors
///
/// Returns the error of `serde_json`, or one naming the integer.
pub(crate) fn from_slice<A: serde::de::DeserializeOwned>(
    json: &[u8],
) -> Result<A, serde_json::Error> {
    let mut deserializer = serde_json::Deserializer::from_slice(json);
    let value = A::deserialize(checked::Exact(&mut deserializer))?;
    // `serde_json::from_slice` refuses data after the value the same way.
    deserializer.end()?;
    Ok(value)
}

/// A deserializer that passes each value through and refuses an integer
/// past 2^53 where the parser hands it over.
///
/// Each wrapper forwards to the one it holds and wraps what that one hands
/// back, a visitor, a seed, or a sequence, map, or enum access, so the
/// integers of a nested value reach `Checked::visit_i64` and
/// `Checked::visit_u64` too. A float passes as it is, as it passed the
/// check of a `Value`, and `serde_json` parses an integer too large for a
/// `u64` into one.
#[cfg(feature = "axum")]
mod checked {
    use std::fmt;

    use serde::de::{
        DeserializeSeed, Deserializer, EnumAccess, Error, MapAccess, SeqAccess, VariantAccess,
        Visitor,
    };

    use super::{LIMIT, holds_i64, holds_i128, holds_u64, message};

    /// The deserializer `D`, whose integers are checked.
    pub(super) struct Exact<D>(pub(super) D);

    /// A visitor, a seed, or an access whose integers are checked.
    pub(super) struct Checked<T>(T);

    macro_rules! forward {
        ($($method:ident)*) => {$(
            fn $method<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value, D::Error> {
                self.0.$method(Checked(visitor))
            }
        )*};
    }

    impl<'de, D: Deserializer<'de>> Deserializer<'de> for Exact<D> {
        type Error = D::Error;

        forward! {
            deserialize_any deserialize_bool
            deserialize_i8 deserialize_i16 deserialize_i32 deserialize_i64 deserialize_i128
            deserialize_u8 deserialize_u16 deserialize_u32 deserialize_u64 deserialize_u128
            deserialize_f32 deserialize_f64 deserialize_char deserialize_str deserialize_string
            deserialize_bytes deserialize_byte_buf deserialize_option deserialize_unit
            deserialize_seq deserialize_map deserialize_identifier
        }

        // `serde_json` skips an ignored value without handing its integers
        // to a visitor, so the value is read as any value and dropped, as the
        // check of a whole `Value` read it.
        fn deserialize_ignored_any<V: Visitor<'de>>(
            self,
            visitor: V,
        ) -> Result<V::Value, D::Error> {
            self.0.deserialize_any(Checked(visitor))
        }

        fn deserialize_unit_struct<V: Visitor<'de>>(
            self,
            name: &'static str,
            visitor: V,
        ) -> Result<V::Value, D::Error> {
            self.0.deserialize_unit_struct(name, Checked(visitor))
        }

        fn deserialize_newtype_struct<V: Visitor<'de>>(
            self,
            name: &'static str,
            visitor: V,
        ) -> Result<V::Value, D::Error> {
            self.0.deserialize_newtype_struct(name, Checked(visitor))
        }

        fn deserialize_tuple<V: Visitor<'de>>(
            self,
            len: usize,
            visitor: V,
        ) -> Result<V::Value, D::Error> {
            self.0.deserialize_tuple(len, Checked(visitor))
        }

        fn deserialize_tuple_struct<V: Visitor<'de>>(
            self,
            name: &'static str,
            len: usize,
            visitor: V,
        ) -> Result<V::Value, D::Error> {
            self.0.deserialize_tuple_struct(name, len, Checked(visitor))
        }

        fn deserialize_struct<V: Visitor<'de>>(
            self,
            name: &'static str,
            fields: &'static [&'static str],
            visitor: V,
        ) -> Result<V::Value, D::Error> {
            self.0.deserialize_struct(name, fields, Checked(visitor))
        }

        fn deserialize_enum<V: Visitor<'de>>(
            self,
            name: &'static str,
            variants: &'static [&'static str],
            visitor: V,
        ) -> Result<V::Value, D::Error> {
            self.0.deserialize_enum(name, variants, Checked(visitor))
        }

        fn is_human_readable(&self) -> bool {
            self.0.is_human_readable()
        }
    }

    // Each method is forwarded, rather than left to the default that calls
    // a wider one, because the visitor inside may handle the narrow type
    // itself.
    macro_rules! pass {
        ($($method:ident: $ty:ty)*) => {$(
            fn $method<E: Error>(self, v: $ty) -> Result<V::Value, E> {
                self.0.$method(v)
            }
        )*};
    }

    impl<'de, V: Visitor<'de>> Visitor<'de> for Checked<V> {
        type Value = V::Value;

        fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
            self.0.expecting(formatter)
        }

        pass! {
            visit_bool: bool
            visit_i8: i8 visit_i16: i16 visit_i32: i32
            visit_u8: u8 visit_u16: u16 visit_u32: u32
            visit_f32: f32 visit_f64: f64 visit_char: char
            visit_str: &str visit_borrowed_str: &'de str visit_string: String
            visit_bytes: &[u8] visit_borrowed_bytes: &'de [u8] visit_byte_buf: Vec<u8>
        }

        fn visit_i64<E: Error>(self, v: i64) -> Result<V::Value, E> {
            if !holds_i64(v) {
                return Err(E::custom(message(v)));
            }
            self.0.visit_i64(v)
        }

        fn visit_u64<E: Error>(self, v: u64) -> Result<V::Value, E> {
            if !holds_u64(v) {
                return Err(E::custom(message(v)));
            }
            self.0.visit_u64(v)
        }

        fn visit_i128<E: Error>(self, v: i128) -> Result<V::Value, E> {
            if !holds_i128(v) {
                return Err(E::custom(message(v)));
            }
            self.0.visit_i128(v)
        }

        fn visit_u128<E: Error>(self, v: u128) -> Result<V::Value, E> {
            if v > u128::from(LIMIT) {
                return Err(E::custom(message(v)));
            }
            self.0.visit_u128(v)
        }

        fn visit_none<E: Error>(self) -> Result<V::Value, E> {
            self.0.visit_none()
        }

        fn visit_unit<E: Error>(self) -> Result<V::Value, E> {
            self.0.visit_unit()
        }

        fn visit_some<D: Deserializer<'de>>(self, deserializer: D) -> Result<V::Value, D::Error> {
            self.0.visit_some(Exact(deserializer))
        }

        fn visit_newtype_struct<D: Deserializer<'de>>(
            self,
            deserializer: D,
        ) -> Result<V::Value, D::Error> {
            self.0.visit_newtype_struct(Exact(deserializer))
        }

        fn visit_seq<A: SeqAccess<'de>>(self, seq: A) -> Result<V::Value, A::Error> {
            self.0.visit_seq(Checked(seq))
        }

        fn visit_map<A: MapAccess<'de>>(self, map: A) -> Result<V::Value, A::Error> {
            self.0.visit_map(Checked(map))
        }

        fn visit_enum<A: EnumAccess<'de>>(self, data: A) -> Result<V::Value, A::Error> {
            self.0.visit_enum(Checked(data))
        }
    }

    impl<'de, T: DeserializeSeed<'de>> DeserializeSeed<'de> for Checked<T> {
        type Value = T::Value;

        fn deserialize<D: Deserializer<'de>>(self, deserializer: D) -> Result<T::Value, D::Error> {
            self.0.deserialize(Exact(deserializer))
        }
    }

    impl<'de, A: SeqAccess<'de>> SeqAccess<'de> for Checked<A> {
        type Error = A::Error;

        fn next_element_seed<T: DeserializeSeed<'de>>(
            &mut self,
            seed: T,
        ) -> Result<Option<T::Value>, A::Error> {
            self.0.next_element_seed(Checked(seed))
        }

        fn size_hint(&self) -> Option<usize> {
            self.0.size_hint()
        }
    }

    impl<'de, A: MapAccess<'de>> MapAccess<'de> for Checked<A> {
        type Error = A::Error;

        fn next_key_seed<K: DeserializeSeed<'de>>(
            &mut self,
            seed: K,
        ) -> Result<Option<K::Value>, A::Error> {
            self.0.next_key_seed(Checked(seed))
        }

        fn next_value_seed<T: DeserializeSeed<'de>>(
            &mut self,
            seed: T,
        ) -> Result<T::Value, A::Error> {
            self.0.next_value_seed(Checked(seed))
        }

        fn size_hint(&self) -> Option<usize> {
            self.0.size_hint()
        }
    }

    impl<'de, A: EnumAccess<'de>> EnumAccess<'de> for Checked<A> {
        type Error = A::Error;
        type Variant = Checked<A::Variant>;

        fn variant_seed<T: DeserializeSeed<'de>>(
            self,
            seed: T,
        ) -> Result<(T::Value, Self::Variant), A::Error> {
            let (value, variant) = self.0.variant_seed(Checked(seed))?;
            Ok((value, Checked(variant)))
        }
    }

    impl<'de, A: VariantAccess<'de>> VariantAccess<'de> for Checked<A> {
        type Error = A::Error;

        fn unit_variant(self) -> Result<(), A::Error> {
            self.0.unit_variant()
        }

        fn newtype_variant_seed<T: DeserializeSeed<'de>>(
            self,
            seed: T,
        ) -> Result<T::Value, A::Error> {
            self.0.newtype_variant_seed(Checked(seed))
        }

        fn tuple_variant<V: Visitor<'de>>(
            self,
            len: usize,
            visitor: V,
        ) -> Result<V::Value, A::Error> {
            self.0.tuple_variant(len, Checked(visitor))
        }

        fn struct_variant<V: Visitor<'de>>(
            self,
            fields: &'static [&'static str],
            visitor: V,
        ) -> Result<V::Value, A::Error> {
            self.0.struct_variant(fields, Checked(visitor))
        }
    }
}

#[cfg(test)]
mod tests {
    use serde::{Deserialize, Serialize};
    use serde_json::json;

    use super::{LIMIT, check, holds_i64, holds_u64};
    use crate::{Page, Patches, Signals};

    #[derive(Signals, Serialize, Deserialize)]
    struct Ledger {
        total: i64,
        ids: Vec<u64>,
    }

    const PAST: u64 = (1 << 53) + 1;

    #[test]
    fn holds_every_integer_up_to_two_to_the_53() {
        let limit = i64::try_from(LIMIT).expect("2^53 fits in an i64");
        assert!(holds_i64(limit) && holds_i64(-limit) && holds_i64(0));
        assert!(!holds_i64(limit + 1) && !holds_i64(-limit - 1) && !holds_i64(i64::MIN));
        assert!(holds_u64(LIMIT) && !holds_u64(LIMIT + 1) && !holds_u64(u64::MAX));
    }

    #[test]
    fn passes_a_value_whose_integers_all_fit() {
        check(&json!({ "a": [1, -2, 9_007_199_254_740_992_u64], "b": { "c": 1.5e300 } }));
    }

    #[test]
    #[should_panic(expected = "the integer 9007199254740993 is outside -2^53 to 2^53")]
    fn finds_an_integer_past_the_range_deep_in_a_value() {
        check(&json!({ "a": [{ "b": [9_007_199_254_740_993_u64] }] }));
    }

    #[cfg(feature = "axum")]
    mod reading {
        use serde::Deserialize;

        use crate::exact::from_slice;

        #[derive(Debug, PartialEq, Deserialize)]
        enum Entry {
            Credit(u64),
            Move { from: i64, to: Option<i64> },
            Pair(i64, i64),
            Empty,
        }

        #[derive(Debug, PartialEq, Deserialize)]
        #[serde(untagged)]
        enum Either {
            Number(f64),
            Text(String),
        }

        #[derive(Debug, PartialEq, Deserialize)]
        struct Wrapped(Vec<i128>);

        fn refused<A: serde::de::DeserializeOwned + std::fmt::Debug>(json: &str) -> String {
            from_slice::<A>(json.as_bytes())
                .expect_err("an integer past 2^53")
                .to_string()
        }

        #[test]
        fn reads_integers_up_to_two_to_the_53_and_any_float() {
            let read: (Vec<Entry>, Either, Wrapped, f64) = from_slice(
                br#"[[{"Credit":9007199254740992},{"Move":{"from":-9007199254740992,"to":null}},{"Pair":[1,2]},"Empty"],1.5,[3],1e300]"#,
            )
            .expect("every integer fits");
            assert_eq!(read.0[0], Entry::Credit(1 << 53));
            assert_eq!(read.1, Either::Number(1.5));
            assert_eq!(read.2, Wrapped(vec![3]));
        }

        #[test]
        fn refuses_an_integer_past_two_to_the_53_wherever_it_sits() {
            #[derive(Debug, Deserialize)]
            struct Named {
                #[expect(dead_code, reason = "only the read is tested")]
                a: i32,
            }
            let past = "the integer 9007199254740993 is outside";
            assert!(refused::<Vec<Entry>>(r#"[{"Credit":9007199254740993}]"#).contains(past));
            assert!(
                refused::<Entry>(r#"{"Move":{"from":0,"to":-9007199254740993}}"#)
                    .contains("the integer -9007199254740993 is outside")
            );
            assert!(refused::<Entry>(r#"{"Pair":[0,9007199254740993]}"#).contains(past));
            assert!(refused::<Either>("9007199254740993").contains(past));
            assert!(refused::<Wrapped>("[9007199254740993]").contains(past));
            assert!(refused::<f64>("9007199254740993").contains(past));
            // A field the type does not name is checked as well.
            assert!(refused::<Named>(r#"{"a":1,"b":[9007199254740993]}"#).contains(past));
        }

        #[test]
        fn refuses_data_after_the_value() {
            let error = from_slice::<i32>(b"1 2").expect_err("trailing data");
            assert!(
                error.to_string().starts_with("trailing characters"),
                "{error}"
            );
        }
    }

    #[test]
    fn renders_and_patches_integers_up_to_two_to_the_53() {
        let limit = i64::try_from(LIMIT).expect("2^53 fits in an i64");
        let html = Page::new(
            "Ledger",
            Ledger {
                total: -limit,
                ids: vec![LIMIT],
            },
        )
        .render();
        assert!(html.contains("-9007199254740992") && html.contains("9007199254740992"));
        let _: Patches<Ledger> = Ledger::total().patch(limit).into();
        let _: Patches<Ledger> = Ledger::ids().patch_append(&[LIMIT]).into();
    }

    #[test]
    #[should_panic(expected = "the integer 9007199254740993 is outside")]
    fn a_page_whose_signal_the_browser_would_round_does_not_render() {
        let _ = Page::new(
            "Ledger",
            Ledger {
                total: 0,
                ids: vec![PAST],
            },
        )
        .render();
    }

    #[test]
    #[should_panic(expected = "the integer -9007199254740993 is outside")]
    fn a_patch_the_browser_would_round_is_not_built() {
        let _ = Ledger::total().patch(-i64::try_from(PAST).expect("fits"));
    }

    #[test]
    #[should_panic(expected = "the integer 9007199254740993 is outside")]
    fn a_list_item_the_browser_would_round_is_not_sent() {
        let _ = Ledger::ids().patch_append(&[PAST]);
    }

    #[test]
    #[should_panic(expected = "the integer 9007199254740993 is outside")]
    fn a_literal_the_browser_would_round_is_not_written() {
        let _ = crate::IntoExpr::<Ledger, i64>::into_expr(i64::try_from(PAST).expect("fits"));
    }
}
