//! Evaluates expressions on the server, so the first HTML already shows the
//! values the browser runtime would compute.
//!
//! This mirrors `evaluate` in `runtime/src/eval.ts`. Where the two disagree,
//! the runtime wins: it re-evaluates every binding when the page loads, so a
//! difference shows for one frame and is then corrected. The known
//! differences are halfway rounding in `to_fixed`, numbers below `1e-6` or at
//! or above `1e21` in `to_text`, and string ordering outside the Basic
//! Multilingual Plane.

use std::borrow::Cow;

use serde_json::{Map, Number, Value};

static NULL: Value = Value::Null;

/// The signals an expression reads, and the store path of each enclosing list item.
pub(crate) struct Ctx<'a> {
    pub(crate) root: &'a Value,
    pub(crate) scope: Vec<String>,
    /// Whether the HTML being written is a copy of a template that the runtime
    /// takes over. A copy leaves out the `data-hs-*` attributes, because the
    /// runtime reads them from the template.
    pub(crate) copy: bool,
}

impl<'a> Ctx<'a> {
    pub(crate) fn with_item(&self, depth: usize, path: String) -> Ctx<'a> {
        let mut scope: Vec<String> = self.scope.iter().take(depth).cloned().collect();
        scope.push(path);
        Ctx {
            root: self.root,
            scope,
            copy: true,
        }
    }

    pub(crate) fn as_copy(&self) -> Ctx<'a> {
        Ctx {
            root: self.root,
            scope: self.scope.clone(),
            copy: true,
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
    } else if value.fract() == 0.0 && value.abs() < 1e21 {
        format!("{value:.0}").replace("-0", "0")
    } else {
        format!("{value}")
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
        "and" => Value::Bool(truthy(&*eval(1)?) && truthy(&*eval(2)?)),
        "or" => Value::Bool(truthy(&*eval(1)?) || truthy(&*eval(2)?)),
        "not" => Value::Bool(!truthy(&*eval(1)?)),
        "str" => Value::from(js_string(&*eval(1)?)),
        "fixed" => {
            let digits = usize::try_from(arg(2)?.as_u64()?).ok()?;
            Value::from(format!("{:.*}", digits, f(&*eval(1)?)?))
        }
        "with" => set_path(&*eval(1)?, arg(2)?.as_str()?, eval(3)?.into_owned()),
        "count" => {
            let path = ctx.resolve(arg(1)?)?;
            let depth = usize::try_from(arg(2)?.as_u64()?).ok()?;
            let len = ctx.get(&path).as_array().map_or(0, Vec::len);
            let mut count = 0_u64;
            for index in 0..len {
                let item = ctx.with_item(depth, join(&path, &index.to_string()));
                if truthy(&*evaluate(arg(3)?, &item)?) {
                    count += 1;
                }
            }
            Value::from(count)
        }
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
        "gt" | "ge" | "lt" | "le" => {
            let (a, b) = (eval(1)?, eval(2)?);
            let order = match (&*a, &*b) {
                (Value::String(a), Value::String(b)) => a.cmp(b),
                _ => f(&a)?.partial_cmp(&f(&b)?)?,
            };
            Value::Bool(match op {
                "gt" => order.is_gt(),
                "ge" => order.is_ge(),
                "lt" => order.is_lt(),
                _ => order.is_le(),
            })
        }
        "add" | "sub" | "mul" | "div" | "rem" | "min" | "max" => {
            let (a, b) = (f(&*eval(1)?)?, f(&*eval(2)?)?);
            number(match op {
                "add" => a + b,
                "sub" => a - b,
                "mul" => a * b,
                "div" => a / b,
                "rem" => a % b,
                "min" => a.min(b),
                _ => a.max(b),
            })
        }
        _ => return None,
    };
    Some(Cow::Owned(value))
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::{Ctx, evaluate, js_json, js_string};

    fn eval(ast: &serde_json::Value, root: &serde_json::Value) -> serde_json::Value {
        evaluate(
            ast,
            &Ctx {
                root,
                scope: Vec::new(),
                copy: false,
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
    fn writes_numbers_as_javascript_does() {
        assert_eq!(js_string(&json!(3.0)), "3");
        assert_eq!(js_string(&json!(0.5)), "0.5");
        assert_eq!(js_string(&json!(-0.0)), "0");
        assert_eq!(
            js_json(&json!({ "id": 1.0, "t": "a\"b" })),
            r#"{"id":1,"t":"a\"b"}"#
        );
    }
}
