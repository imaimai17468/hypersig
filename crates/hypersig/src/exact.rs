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
/// # Errors
///
/// Returns the error of `serde_json`, or one naming the integer.
pub(crate) fn from_slice<A: serde::de::DeserializeOwned>(
    json: &[u8],
) -> Result<A, serde_json::Error> {
    let value: Value = serde_json::from_slice(json)?;
    if let Some(error) = rounded(&value) {
        return Err(serde::de::Error::custom(error));
    }
    serde_json::from_value(value)
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
