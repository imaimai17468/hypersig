//! Evaluates expressions on the server, so the first HTML already shows the
//! values the browser runtime would compute.
//!
//! This mirrors `evaluate` in `runtime/src/eval.ts`. Where the two disagree,
//! the runtime wins: it re-evaluates every binding when the page loads, so a
//! difference shows for one frame and is then corrected. The known
//! differences are string ordering outside the Basic Multilingual Plane,
//! and the last bit of `log10` and `pow`, which each side takes from its
//! platform's math library.

use std::borrow::Cow;

use serde_json::{Map, Number, Value};

use crate::route::{encode_uri_component, encode_uri_path, query_string};

static NULL: Value = Value::Null;

/// The signals an expression reads, and the store path of each enclosing list item.
pub(crate) struct Ctx<'a> {
    pub(crate) root: &'a Value,
    pub(crate) scope: Vec<String>,
    /// Whether the HTML being written is a copy of a template that the runtime
    /// takes over. A copy leaves out the `data-hs-*` attributes, because the
    /// runtime reads them from the template.
    pub(crate) copy: bool,
    /// The values `let_in` names, by depth.
    pub(crate) vars: Vec<Value>,
}

impl<'a> Ctx<'a> {
    pub(crate) fn with_item(&self, depth: usize, path: String) -> Ctx<'a> {
        let mut scope: Vec<String> = self.scope.iter().take(depth).cloned().collect();
        scope.push(path);
        Ctx {
            root: self.root,
            scope,
            copy: true,
            vars: self.vars.clone(),
        }
    }

    pub(crate) fn as_copy(&self) -> Ctx<'a> {
        Ctx {
            root: self.root,
            scope: self.scope.clone(),
            copy: true,
            vars: self.vars.clone(),
        }
    }

    /// The store path a place names, as `resolve` in the runtime computes it.
    pub(crate) fn resolve(&self, place: &Value) -> Option<String> {
        match place {
            Value::String(path) => Some(path.clone()),
            Value::Array(parts) if parts.first().and_then(Value::as_str) == Some("item") => {
                let depth = usize::try_from(parts.get(1)?.as_u64()?).ok()?;
                let base = self.scope.get(depth)?;
                Some(join(base, parts.get(2)?.as_str()?))
            }
            _ => None,
        }
    }

    pub(crate) fn get(&self, path: &str) -> &'a Value {
        get_path(self.root, path).unwrap_or(&NULL)
    }
}

pub(crate) fn join(base: &str, tail: &str) -> String {
    match (base.is_empty(), tail.is_empty()) {
        (true, _) => tail.to_owned(),
        (_, true) => base.to_owned(),
        _ => format!("{base}.{tail}"),
    }
}

fn get_path<'v>(value: &'v Value, path: &str) -> Option<&'v Value> {
    if path.is_empty() {
        return Some(value);
    }
    path.split('.')
        .try_fold(value, |current, segment| match current {
            Value::Object(map) => map.get(segment),
            Value::Array(items) => items.get(segment.parse::<usize>().ok()?),
            _ => None,
        })
}

pub(crate) fn set_path(value: &Value, path: &str, next: Value) -> Value {
    if path.is_empty() {
        return next;
    }
    let (head, tail) = path.split_once('.').unwrap_or((path, ""));
    match value {
        Value::Array(items) => {
            let mut items = items.clone();
            if let Ok(index) = head.parse::<usize>()
                && index < items.len()
            {
                items[index] = set_path(&items[index], tail, next);
            }
            Value::Array(items)
        }
        Value::Object(map) => {
            let mut map = map.clone();
            let inner = set_path(map.get(head).unwrap_or(&Value::Null), tail, next);
            map.insert(head.to_owned(), inner);
            Value::Object(map)
        }
        _ => {
            let mut map = Map::new();
            map.insert(head.to_owned(), set_path(&Value::Null, tail, next));
            Value::Object(map)
        }
    }
}

/// JavaScript's truthiness for a JSON value.
pub(crate) fn truthy(value: &Value) -> bool {
    match value {
        Value::Null => false,
        Value::Bool(b) => *b,
        Value::Number(n) => n.as_f64().is_some_and(|n| n != 0.0 && !n.is_nan()),
        Value::String(s) => !s.is_empty(),
        Value::Array(_) | Value::Object(_) => true,
    }
}

fn number(value: f64) -> Value {
    #[expect(
        clippy::cast_possible_truncation,
        reason = "only integral values inside i64's range reach the cast"
    )]
    if value.fract() == 0.0 && value.abs() < 9e15 {
        Value::from(value as i64)
    } else {
        Number::from_f64(value).map_or(Value::Null, Value::Number)
    }
}

fn js_number(value: f64) -> String {
    if value.is_nan() {
        "NaN".to_owned()
    } else if value.is_infinite() {
        if value > 0.0 { "Infinity" } else { "-Infinity" }.to_owned()
    } else if value != 0.0 && (value.abs() >= 1e21 || value.abs() < 1e-6) {
        // JavaScript writes these with an exponent, and a `+` before a
        // positive one. Both sides write the shortest digits that read back
        // as the same double.
        let text = format!("{value:e}");
        match text.split_once('e') {
            Some((digits, exponent)) if !exponent.starts_with('-') => {
                format!("{digits}e+{exponent}")
            }
            _ => text,
        }
    } else if value == 0.0 {
        "0".to_owned()
    } else {
        format!("{value}")
    }
}

/// `Math.round(value)` in JavaScript, which rounds a halfway value up.
fn js_round(value: f64) -> f64 {
    let floor = value.floor();
    // Subtracting the floor is exact, so the halfway test sees the true fraction.
    if value - floor >= 0.5 {
        floor + 1.0
    } else {
        floor
    }
}

/// `value.toFixed(digits)` in JavaScript, for `digits` up to 100.
///
/// Both round the exact value of the double to `digits` places, and differ
/// only on an exact halfway value, which Rust rounds to even and
/// JavaScript away from zero. A halfway value is `m / 2^(digits + 1)` for
/// an odd `m`, so it is found exactly by scaling by that power of two,
/// and the next double up rounds the way JavaScript does.
pub(crate) fn js_fixed(value: f64, digits: u8) -> String {
    if !value.is_finite() || value.abs() >= 1e21 {
        return js_number(value);
    }
    let magnitude = value.abs();
    // Doubling is exact, where `powi` may round.
    let scaled = (0..=digits).fold(magnitude, |scaled, _| scaled * 2.0);
    let halfway = scaled.fract() == 0.0 && (scaled / 2.0).fract() != 0.0;
    let rounded = if halfway {
        magnitude.next_up()
    } else {
        magnitude
    };
    let text = format!("{:.*}", usize::from(digits), rounded);
    // `(-0.001).toFixed(2)` is "-0.00", and `(-0).toFixed(2)` is "0.00".
    if value < 0.0 {
        format!("-{text}")
    } else {
        text
    }
}

/// `value` written as `format` in `runtime/src/eval.ts` writes it.
pub(crate) fn format_number(
    value: f64,
    digits: u8,
    group: &str,
    point: &str,
    prefix: &str,
    suffix: &str,
) -> String {
    if !value.is_finite() {
        return format!("{prefix}{}{suffix}", js_number(value));
    }
    let fixed = js_fixed(value.abs(), digits);
    let negative = value < 0.0 && fixed.bytes().any(|byte| matches!(byte, b'1'..=b'9'));
    let sign = if negative { "-" } else { "" };
    if fixed.contains('e') {
        return format!("{sign}{prefix}{fixed}{suffix}");
    }
    let (whole, fraction) = fixed.split_once('.').unwrap_or((&fixed, ""));
    let mut grouped = String::with_capacity(whole.len() + whole.len() / 3 * group.len());
    for (index, digit) in whole.chars().enumerate() {
        if index > 0 && (whole.len() - index) % 3 == 0 {
            grouped.push_str(group);
        }
        grouped.push(digit);
    }
    if fraction.is_empty() {
        format!("{sign}{prefix}{grouped}{suffix}")
    } else {
        format!("{sign}{prefix}{grouped}{point}{fraction}{suffix}")
    }
}

/// `String(value)` in JavaScript.
pub(crate) fn js_string(value: &Value) -> String {
    match value {
        Value::Null => "null".to_owned(),
        Value::Bool(b) => b.to_string(),
        Value::Number(n) => js_number(n.as_f64().unwrap_or(f64::NAN)),
        Value::String(s) => s.clone(),
        Value::Array(items) => items
            .iter()
            .map(|item| match item {
                Value::Null => String::new(),
                other => js_string(other),
            })
            .collect::<Vec<_>>()
            .join(","),
        Value::Object(_) => "[object Object]".to_owned(),
    }
}

/// `JSON.stringify(value)` in JavaScript, which writes integral numbers without a fraction.
pub(crate) fn js_json(value: &Value) -> String {
    match value {
        Value::Number(n) => n.as_f64().map_or_else(|| "null".to_owned(), js_number),
        Value::Array(items) => format!(
            "[{}]",
            items.iter().map(js_json).collect::<Vec<_>>().join(",")
        ),
        Value::Object(map) => format!(
            "{{{}}}",
            map.iter()
                .map(|(key, value)| format!("{}:{}", Value::from(key.as_str()), js_json(value)))
                .collect::<Vec<_>>()
                .join(",")
        ),
        other => other.to_string(),
    }
}

/// Structural equality, comparing numbers by value.
pub(crate) fn equal(a: &Value, b: &Value) -> bool {
    match (a, b) {
        (Value::Number(x), Value::Number(y)) => x.as_f64() == y.as_f64(),
        (Value::Array(x), Value::Array(y)) => {
            x.len() == y.len() && x.iter().zip(y).all(|(x, y)| equal(x, y))
        }
        (Value::Object(x), Value::Object(y)) => {
            x.len() == y.len()
                && x.iter()
                    .all(|(key, value)| y.get(key).is_some_and(|other| equal(value, other)))
        }
        _ => a == b,
    }
}

fn f(value: &Value) -> Option<f64> {
    value.as_f64()
}

fn s(value: &Value) -> Option<&str> {
    value.as_str()
}

/// Evaluates `ast`. `None` means the server cannot compute it, and the
/// runtime fills it in.
pub(crate) fn evaluate<'a>(ast: &'a Value, ctx: &Ctx<'a>) -> Option<Cow<'a, Value>> {
    let parts = ast.as_array()?;
    let op = parts.first()?.as_str()?;
    let arg = |i: usize| parts.get(i);
    let eval = |i: usize| evaluate(arg(i)?, ctx);
    let value = match op {
        "sig" => return Some(Cow::Borrowed(ctx.get(arg(1)?.as_str()?))),
        "item" => return Some(Cow::Borrowed(ctx.get(&ctx.resolve(ast)?))),
        "lit" => return arg(1).map(Cow::Borrowed),
        "if" => return if truthy(&*eval(1)?) { eval(2) } else { eval(3) },
        "let" => return let_in(parts, ctx).map(Cow::Owned),
        "var" => {
            let depth = usize::try_from(arg(1)?.as_u64()?).ok()?;
            return ctx.vars.get(depth).cloned().map(Cow::Owned);
        }
        "get" => {
            let tail = arg(2)?.as_str()?;
            return Some(match eval(1)? {
                Cow::Borrowed(value) => Cow::Borrowed(get_path(value, tail).unwrap_or(&NULL)),
                Cow::Owned(value) => {
                    Cow::Owned(get_path(&value, tail).cloned().unwrap_or(Value::Null))
                }
            });
        }
        "or_else" => {
            let value = eval(1)?;
            return if value.is_null() {
                eval(2)
            } else {
                Some(value)
            };
        }
        "index" => {
            let depth = usize::try_from(arg(1)?.as_u64()?).ok()?;
            let path = ctx.scope.get(depth)?;
            Value::from(path.rsplit('.').next()?.parse::<u64>().ok()?)
        }
        "tuple" => Value::Array(
            parts[1..]
                .iter()
                .map(|part| evaluate(part, ctx).map(Cow::into_owned))
                .collect::<Option<_>>()?,
        ),
        "nth" => {
            let index = usize::try_from(eval(2)?.as_i64()?).ok();
            index
                .and_then(|index| eval(1)?.as_array()?.get(index).cloned())
                .unwrap_or(Value::Null)
        }
        "and" => Value::Bool(truthy(&*eval(1)?) && truthy(&*eval(2)?)),
        "or" => Value::Bool(truthy(&*eval(1)?) || truthy(&*eval(2)?)),
        "not" => Value::Bool(!truthy(&*eval(1)?)),
        "str" => Value::from(js_string(&*eval(1)?)),
        "uri" => Value::from(encode_uri_component(&js_string(&*eval(1)?))),
        "uri_path" => Value::from(encode_uri_path(&js_string(&*eval(1)?))),
        "query" => Value::from(query_string(&*eval(1)?)),
        "fixed" => {
            let digits = u8::try_from(arg(2)?.as_u64()?).ok()?;
            Value::from(js_fixed(f(&*eval(1)?)?, digits))
        }
        "format" => {
            let digits = u8::try_from(arg(2)?.as_u64()?).ok()?;
            Value::from(format_number(
                f(&*eval(1)?)?,
                digits,
                arg(3)?.as_str()?,
                arg(4)?.as_str()?,
                arg(5)?.as_str()?,
                arg(6)?.as_str()?,
            ))
        }
        "with" => set_path(&*eval(1)?, arg(2)?.as_str()?, eval(3)?.into_owned()),
        "count" | "sum_of" | "max_of" | "min_of" | "map" => fold(op, parts, ctx)?,
        "floor" => number(f(&*eval(1)?)?.floor()),
        "ceil" => number(f(&*eval(1)?)?.ceil()),
        "round" => number(js_round(f(&*eval(1)?)?)),
        "log10" => number(f(&*eval(1)?)?.log10()),
        "trim" => Value::from(s(&*eval(1)?)?.trim()),
        "lower" => Value::from(s(&*eval(1)?)?.to_lowercase()),
        "upper" => Value::from(s(&*eval(1)?)?.to_uppercase()),
        "chars" => Value::from(s(&*eval(1)?)?.chars().count()),
        "len" => Value::from(eval(1)?.as_array().map_or(0, Vec::len)),
        "eq" => Value::Bool(equal(&*eval(1)?, &*eval(2)?)),
        "ne" => Value::Bool(!equal(&*eval(1)?, &*eval(2)?)),
        "concat" => Value::from(format!("{}{}", s(&*eval(1)?)?, s(&*eval(2)?)?)),
        "contains" => Value::Bool(s(&*eval(1)?)?.contains(s(&*eval(2)?)?)),
        "starts" => Value::Bool(s(&*eval(1)?)?.starts_with(s(&*eval(2)?)?)),
        "gt" | "ge" | "lt" | "le" => compare(op, &*eval(1)?, &*eval(2)?)?,
        "add" | "sub" | "mul" | "div" | "rem" | "min" | "max" | "pow" => {
            arithmetic(op, f(&*eval(1)?)?, f(&*eval(2)?)?)
        }
        _ => return None,
    };
    Some(Cow::Owned(value))
}

/// `["let",value,depth,body]`: `body` with `["var",depth]` standing for `value`.
fn let_in<'a>(parts: &'a [Value], ctx: &Ctx<'a>) -> Option<Value> {
    let depth = usize::try_from(parts.get(2)?.as_u64()?).ok()?;
    let mut vars: Vec<Value> = ctx.vars.iter().take(depth).cloned().collect();
    vars.push(evaluate(parts.get(1)?, ctx)?.into_owned());
    let inner = Ctx {
        root: ctx.root,
        scope: ctx.scope.clone(),
        copy: ctx.copy,
        vars,
    };
    Some(evaluate(parts.get(3)?, &inner)?.into_owned())
}

/// `["count",place,depth,e]` and the other operations that evaluate `e`
/// once per item of the list at `place`.
fn fold<'a>(op: &str, parts: &'a [Value], ctx: &Ctx<'a>) -> Option<Value> {
    let path = ctx.resolve(parts.get(1)?)?;
    let depth = usize::try_from(parts.get(2)?.as_u64()?).ok()?;
    let each = parts.get(3)?;
    let len = ctx.get(&path).as_array().map_or(0, Vec::len);
    let mut count = 0_u64;
    let mut folded: Option<f64> = None;
    let mut mapped = Vec::new();
    for index in 0..len {
        let item = ctx.with_item(depth, join(&path, &index.to_string()));
        let value = evaluate(each, &item)?;
        if op == "map" {
            mapped.push(value.into_owned());
            continue;
        }
        if op == "count" {
            count += u64::from(truthy(&value));
            continue;
        }
        let value = f(&value)?;
        folded = Some(match (op, folded) {
            (_, None) => value,
            ("sum_of", Some(sum)) => sum + value,
            ("max_of", Some(max)) => max.max(value),
            (_, Some(min)) => min.min(value),
        });
    }
    Some(match (op, folded) {
        ("map", _) => Value::Array(mapped),
        ("count", _) => Value::from(count),
        ("sum_of", None) => Value::from(0),
        (_, folded) => folded.map_or(Value::Null, number),
    })
}

/// `a < b` and friends: strings in code point order, anything else as numbers.
fn compare(op: &str, a: &Value, b: &Value) -> Option<Value> {
    let order = match (a, b) {
        (Value::String(a), Value::String(b)) => a.cmp(b),
        _ => f(a)?.partial_cmp(&f(b)?)?,
    };
    Some(Value::Bool(match op {
        "gt" => order.is_gt(),
        "ge" => order.is_ge(),
        "lt" => order.is_lt(),
        _ => order.is_le(),
    }))
}

fn arithmetic(op: &str, a: f64, b: f64) -> Value {
    number(match op {
        "add" => a + b,
        "sub" => a - b,
        "mul" => a * b,
        "div" => a / b,
        "rem" => a % b,
        "min" => a.min(b),
        "pow" => a.powf(b),
        _ => a.max(b),
    })
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::{Ctx, evaluate, format_number, js_fixed, js_json, js_round, js_string};

    fn eval(ast: &serde_json::Value, root: &serde_json::Value) -> serde_json::Value {
        evaluate(
            ast,
            &Ctx {
                root,
                scope: Vec::new(),
                copy: false,
                vars: Vec::new(),
            },
        )
        .expect("evaluates")
        .into_owned()
    }

    #[test]
    fn matches_the_runtime_on_the_operators_eval_ts_tests() {
        let root =
            json!({ "count": 3, "name": " Tea ", "panel": { "open": true }, "filter": "Active" });
        assert_eq!(
            eval(
                &json!(["concat", ["sig", "name"], ["str", ["sig", "count"]]]),
                &root
            ),
            json!(" Tea 3")
        );
        assert_eq!(
            eval(&json!(["rem", ["lit", -7], ["lit", 3]]), &root),
            json!(-1)
        );
        assert_eq!(
            eval(&json!(["lower", ["trim", ["sig", "name"]]]), &root),
            json!("tea")
        );
        assert_eq!(eval(&json!(["chars", ["lit", "a😀"]]), &root), json!(2));
        assert_eq!(
            eval(&json!(["tuple", ["sig", "count"], ["lit", "x"]]), &root),
            json!([3, "x"])
        );
        assert_eq!(
            eval(&json!(["fixed", ["lit", 2.5], 2]), &root),
            json!("2.50")
        );
        assert_eq!(eval(&json!(["sig", "panel.open"]), &root), json!(true));
        let todos = json!({ "todos": [{ "done": true }, { "done": false }, { "done": false }] });
        assert_eq!(
            eval(
                &json!(["count", "todos", 0, ["not", ["item", 0, "done"]]]),
                &todos
            ),
            json!(2)
        );
    }

    #[test]
    fn names_a_value_once_for_its_body() {
        let root = json!({ "rows": [{ "v": 3 }, { "v": 7 }] });
        let best = json!(["max_of", "rows", 0, ["item", 0, "v"]]);
        let doubled = json!(["let", best, 0, ["add", ["var", 0], ["var", 0]]]);
        assert_eq!(eval(&doubled, &root), json!(14));
        let nested = json!([
            "let",
            ["lit", 2],
            0,
            ["let", ["lit", 5], 1, ["sub", ["var", 1], ["var", 0]]]
        ]);
        assert_eq!(eval(&nested, &root), json!(3));
    }

    #[test]
    fn rounds_as_math_round_does() {
        assert_eq!(js_round(2.5), 3.0);
        assert_eq!(js_round(-2.5), -2.0);
        assert_eq!(js_round(-2.6), -3.0);
        assert_eq!(js_round(0.499_999_999_999_999_94), 0.0);
    }

    #[test]
    fn folds_a_list_into_a_sum_a_largest_and_a_smallest() {
        let root = json!({ "rows": [{ "v": 3 }, { "v": -1.5 }, { "v": 7 }], "none": [] });
        let fold = |op: &str, list: &str| eval(&json!([op, list, 0, ["item", 0, "v"]]), &root);
        assert_eq!(fold("sum_of", "rows"), json!(8.5));
        assert_eq!(fold("max_of", "rows"), json!(7));
        assert_eq!(fold("min_of", "rows"), json!(-1.5));
        assert_eq!(fold("sum_of", "none"), json!(0));
        assert_eq!(fold("max_of", "none"), json!(null));
        assert_eq!(fold("map", "rows"), json!([3, -1.5, 7]));
        assert_eq!(eval(&json!(["floor", ["lit", -2.5]]), &root), json!(-3));
        assert_eq!(eval(&json!(["ceil", ["lit", 2.1]]), &root), json!(3));
        assert_eq!(eval(&json!(["round", ["lit", 2.5]]), &root), json!(3));
    }

    #[test]
    fn rounds_halfway_values_away_from_zero_as_to_fixed_does() {
        assert_eq!(js_fixed(2.5, 0), "3");
        assert_eq!(js_fixed(0.125, 2), "0.13");
        assert_eq!(js_fixed(-0.125, 2), "-0.13");
        // 1.005 is stored just below the halfway value, so both round down.
        assert_eq!(js_fixed(1.005, 2), "1.00");
        assert_eq!(js_fixed(-0.001, 2), "-0.00");
        assert_eq!(js_fixed(-0.0, 2), "0.00");
        assert_eq!(js_fixed(1e21, 2), "1e+21");
    }

    #[test]
    fn formats_numbers_as_the_runtime_does() {
        let plain = |value| format_number(value, 0, ",", ".", "", "");
        assert_eq!(plain(0.0), "0");
        assert_eq!(plain(999.0), "999");
        assert_eq!(plain(1000.0), "1,000");
        assert_eq!(plain(-1_234_567.0), "-1,234,567");
        assert_eq!(plain(-0.4), "0");
        assert_eq!(format_number(-1234.5, 2, ",", ".", "$", ""), "-$1,234.50");
        assert_eq!(
            format_number(1_234_567.891, 2, ".", ",", "", " €"),
            "1.234.567,89 €"
        );
        assert_eq!(format_number(12345.0, 0, "", ".", "", "%"), "12345%");
        assert_eq!(format_number(f64::NAN, 0, ",", ".", "¥", ""), "¥NaN");
        assert_eq!(format_number(2e21, 0, ",", ".", "", ""), "2e+21");
    }

    #[test]
    fn writes_numbers_as_javascript_does() {
        assert_eq!(js_string(&json!(3.0)), "3");
        assert_eq!(js_string(&json!(0.5)), "0.5");
        assert_eq!(js_string(&json!(-0.0)), "0");
        assert_eq!(js_string(&json!(1e21)), "1e+21");
        assert_eq!(js_string(&json!(-1.5e22)), "-1.5e+22");
        assert_eq!(js_string(&json!(1e-7)), "1e-7");
        assert_eq!(js_string(&json!(0.000_001)), "0.000001");
        assert_eq!(
            js_string(&json!(123_456_789_012_345_680_000.0)),
            "123456789012345680000"
        );
        assert_eq!(
            js_json(&json!({ "id": 1.0, "t": "a\"b" })),
            r#"{"id":1,"t":"a\"b"}"#
        );
    }
}
