//! Evaluates expressions on the server, so the first HTML already shows the
//! values the browser runtime would compute.
//!
//! This mirrors `evaluate` in `runtime/src/eval.ts`. Where the two disagree,
//! the runtime wins: it re-evaluates every binding when the page loads, so a
//! difference shows for one frame and is then corrected. The known
//! differences are the last bit of `log10` and `pow`, which each side takes
//! from its platform's math library, and the spaces and commas of a date,
//! which the browser takes from its own version of the Unicode CLDR data.
//! `differential` compares the two on generated expressions.
//!
//! The server writes locale-dependent text for the locale the expression
//! names, never the visitor's, and a date in the zone the expression names.
//! It leaves out a date in the browser's zone, the browser's current time,
//! and a random number, which only the browser knows.

use std::borrow::Cow;
use std::cell::RefCell;

use serde_json::{Map, Number, Value};

use crate::route::{encode_uri_component, encode_uri_path, query_string};
use crate::tape::{Items, Ref, View};

/// What an expression evaluates to: a value read from the signals or from
/// the expression itself, or one the expression computed.
pub(crate) enum Val<'a> {
    Ref(Ref<'a>),
    Owned(Value),
}

impl<'b> From<&'b Val<'_>> for Ref<'b> {
    fn from(value: &'b Val<'_>) -> Self {
        match value {
            Val::Ref(read) => *read,
            Val::Owned(value) => Ref::Json(value),
        }
    }
}

impl Val<'_> {
    pub(crate) fn into_value(self) -> Value {
        match self {
            Val::Ref(read) => read.to_value(),
            Val::Owned(value) => value,
        }
    }

    /// The value as a `serde_json::Value`, copied only where it sits in a tape.
    fn to_json(&self) -> Cow<'_, Value> {
        match self {
            Val::Ref(Ref::Json(value)) => Cow::Borrowed(value),
            Val::Owned(value) => Cow::Borrowed(value),
            Val::Ref(read) => Cow::Owned(read.to_value()),
        }
    }
}

/// One enclosing list item: its value and its position in the list.
#[derive(Clone, Copy)]
pub(crate) struct Slot<'a> {
    value: Ref<'a>,
    index: usize,
}

impl<'a> Slot<'a> {
    /// The item `value` at position `index` of its list.
    pub(crate) fn new(value: Ref<'a>, index: usize) -> Self {
        Self { value, index }
    }
}

/// The signals an expression reads, and the item of each enclosing list.
pub(crate) struct Ctx<'a> {
    pub(crate) root: Ref<'a>,
    pub(crate) scope: Vec<Slot<'a>>,
    /// Whether the HTML being written is a copy of a template that the runtime
    /// takes over. A copy leaves out the `data-hs-*` attributes, because the
    /// runtime reads them from the template.
    pub(crate) copy: bool,
    /// The values `let_in` names, by depth.
    pub(crate) vars: Vec<Value>,
    /// Where an element puts its bound style declarations in place of a
    /// `style` attribute, for a page with a CSP nonce. Each entry is the body
    /// of one rule, and its index is the element's `data-hs-inline` value.
    pub(crate) rules: Option<&'a RefCell<Vec<String>>>,
}

impl<'a> Ctx<'a> {
    /// Runs `each` once per item of `items`, the list at `depth`, with a
    /// copy of this context standing on that item, and stops at the first
    /// `None`. The copy is built once and moved from item to item.
    pub(crate) fn for_each_item(
        &self,
        depth: usize,
        items: impl Iterator<Item = Ref<'a>>,
        mut each: impl FnMut(&Ctx<'a>, usize) -> Option<()>,
    ) -> Option<()> {
        let mut scope: Vec<Slot<'a>> = self.scope.iter().take(depth).copied().collect();
        scope.push(Slot {
            value: Ref::NULL,
            index: 0,
        });
        let mut item = Ctx {
            root: self.root,
            scope,
            copy: true,
            vars: self.vars.clone(),
            rules: self.rules,
        };
        for (index, value) in items.enumerate() {
            if let Some(last) = item.scope.last_mut() {
                *last = Slot { value, index };
            }
            each(&item, index)?;
        }
        Some(())
    }

    pub(crate) fn as_copy(&self) -> Ctx<'a> {
        Ctx {
            root: self.root,
            scope: self.scope.clone(),
            copy: true,
            vars: self.vars.clone(),
            rules: self.rules,
        }
    }

    /// The value a place names: a store path of a page signal, or
    /// `["item",depth,field]` for a field of an enclosing list item.
    pub(crate) fn lookup(&self, place: &Value) -> Option<Ref<'a>> {
        match place {
            Value::String(path) => Some(self.get(path)),
            Value::Array(parts) if parts.first().and_then(Value::as_str) == Some("item") => {
                let depth = usize::try_from(parts.get(1)?.as_u64()?).ok()?;
                let slot = self.slot(depth);
                Some(
                    slot.value
                        .get_path(parts.get(2)?.as_str()?)
                        .unwrap_or(Ref::NULL),
                )
            }
            _ => None,
        }
    }

    /// The enclosing list item at `depth`.
    ///
    /// # Panics
    ///
    /// Panics if no list at `depth` encloses the expression, which happens
    /// when an expression built from an item inside its closure is used
    /// outside it, where the browser finds no item either.
    fn slot(&self, depth: usize) -> Slot<'a> {
        *self.scope.get(depth).unwrap_or_else(|| {
            panic!(
                "an expression reads the item at depth {depth} of a list that does not \
                 enclose it: build it inside the closure that received the item"
            )
        })
    }

    pub(crate) fn get(&self, path: &str) -> Ref<'a> {
        self.root.get_path(path).unwrap_or(Ref::NULL)
    }
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
pub(crate) fn truthy<'b>(value: impl Into<Ref<'b>>) -> bool {
    match value.into().view() {
        View::Null => false,
        View::Bool(b) => b,
        View::Number(n) => n.as_f64().is_some_and(|n| n != 0.0 && !n.is_nan()),
        View::Str(s) => !s.is_empty(),
        View::Array | View::Object => true,
    }
}

/// `value` as JSON, or `None` for an infinity or NaN, which JSON cannot hold.
///
/// The browser keeps such a value as a number, so a binding that computes one
/// is left for the runtime, and a form rule that does fails on the server.
fn number(value: f64) -> Option<Value> {
    #[expect(
        clippy::cast_possible_truncation,
        reason = "only integral values inside i64's range reach the cast"
    )]
    if value.fract() == 0.0 && value.abs() < 9e15 {
        Some(Value::from(value as i64))
    } else {
        Number::from_f64(value).map(Value::Number)
    }
}

fn js_number(value: f64) -> String {
    let mut out = String::new();
    write_js_number(&mut out, value);
    out
}

/// Appends `String(value)` for a number, as `js_number` returns it.
fn write_js_number(out: &mut String, value: f64) {
    if value.fract() == 0.0 && value.abs() < 9_007_199_254_740_992.0 {
        // An integer below 2^53 is written with all its digits and no
        // fraction or exponent, as an `i64` is. This covers -0, which
        // JavaScript writes as "0".
        #[expect(
            clippy::cast_possible_truncation,
            reason = "the value is integral and inside i64's range"
        )]
        let integer = value as i64;
        write_into(out, format_args!("{integer}"));
    } else if value.is_nan() {
        out.push_str("NaN");
    } else if value.is_infinite() {
        out.push_str(if value > 0.0 { "Infinity" } else { "-Infinity" });
    } else if value.abs() >= 1e21 || value.abs() < 1e-6 {
        // JavaScript writes these with an exponent, and a `+` before a
        // positive one. Both sides write the shortest digits that read back
        // as the same double.
        let text = format!("{value:e}");
        match text.split_once('e') {
            Some((digits, exponent)) if !exponent.starts_with('-') => {
                write_into(out, format_args!("{digits}e+{exponent}"));
            }
            _ => out.push_str(&text),
        }
    } else {
        write_into(out, format_args!("{value}"));
    }
}

pub(crate) fn write_into(out: &mut String, args: std::fmt::Arguments<'_>) {
    std::fmt::Write::write_fmt(out, args).expect("writing to a String does not fail");
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
/// an odd `m`, so it is found exactly by scaling by that power of two.
/// Its exact expansion to one more place ends in `5`, and that digit is
/// rounded up by hand, because the digits of the double above it differ
/// from the value's own once `digits` reaches past its last bit.
pub(crate) fn js_fixed(value: f64, digits: u8) -> String {
    if !value.is_finite() || value.abs() >= 1e21 {
        return js_number(value);
    }
    let magnitude = value.abs();
    // Doubling is exact, where `powi` may round.
    let scaled = (0..=digits).fold(magnitude, |scaled, _| scaled * 2.0);
    let halfway = scaled.fract() == 0.0 && (scaled / 2.0).fract() != 0.0;
    let text = if halfway {
        let mut longer = format!("{:.*}", usize::from(digits) + 1, magnitude);
        longer.pop();
        if digits == 0 {
            longer.pop();
        }
        round_up_last_digit(&longer)
    } else {
        format!("{:.*}", usize::from(digits), magnitude)
    };
    // `(-0.001).toFixed(2)` is "-0.00", and `(-0).toFixed(2)` is "0.00".
    if value < 0.0 {
        format!("-{text}")
    } else {
        text
    }
}

/// The decimal `text`, such as `"2.49"`, with one added to its last digit.
fn round_up_last_digit(text: &str) -> String {
    let mut digits = text.as_bytes().to_vec();
    for at in (0..digits.len()).rev() {
        match digits[at] {
            b'.' => {}
            b'9' => digits[at] = b'0',
            digit => {
                digits[at] = digit + 1;
                return String::from_utf8(digits).expect("ASCII digits");
            }
        }
    }
    digits.insert(0, b'1');
    String::from_utf8(digits).expect("ASCII digits")
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
pub(crate) fn js_string<'b>(value: impl Into<Ref<'b>>) -> String {
    let value = value.into();
    match value.view() {
        View::Null => "null".to_owned(),
        View::Bool(b) => b.to_string(),
        View::Number(n) => js_number(n.as_f64().unwrap_or(f64::NAN)),
        View::Str(s) => s.to_owned(),
        View::Array => value
            .items()
            .into_iter()
            .flatten()
            .map(|item| {
                if item.is_null() {
                    String::new()
                } else {
                    js_string(item)
                }
            })
            .collect::<Vec<_>>()
            .join(","),
        View::Object => "[object Object]".to_owned(),
    }
}

/// `String(value)` in JavaScript, borrowing the text of a string rather than copying it.
pub(crate) fn js_str(value: Val<'_>) -> Cow<'_, str> {
    match value {
        Val::Ref(read) => read
            .as_str()
            .map_or_else(|| Cow::Owned(js_string(read)), Cow::Borrowed),
        Val::Owned(Value::String(text)) => Cow::Owned(text),
        Val::Owned(other) => Cow::Owned(js_string(&other)),
    }
}

/// `JSON.stringify(value)` in JavaScript, which writes integral numbers without a fraction.
pub(crate) fn js_json<'b>(value: impl Into<Ref<'b>>) -> String {
    let mut out = String::new();
    write_js_json(&mut out, value);
    out
}

/// Appends `JSON.stringify(value)`, as `js_json` returns it.
pub(crate) fn write_js_json<'b>(out: &mut String, value: impl Into<Ref<'b>>) {
    let value = value.into();
    match value.view() {
        View::Number(n) => match n.as_f64() {
            Some(n) => write_js_number(out, n),
            None => out.push_str("null"),
        },
        View::Array => {
            out.push('[');
            for (index, item) in value.items().into_iter().flatten().enumerate() {
                if index > 0 {
                    out.push(',');
                }
                write_js_json(out, item);
            }
            out.push(']');
        }
        View::Object => {
            out.push('{');
            for (index, (key, value)) in value.entries().into_iter().flatten().enumerate() {
                if index > 0 {
                    out.push(',');
                }
                write_into(out, format_args!("{}:", Value::from(key)));
                write_js_json(out, value);
            }
            out.push('}');
        }
        View::Null => out.push_str("null"),
        View::Bool(b) => out.push_str(if b { "true" } else { "false" }),
        View::Str(s) => write_into(out, format_args!("{}", Value::from(s))),
    }
}

/// Structural equality, comparing numbers by value.
pub(crate) fn equal<'b, 'c>(a: impl Into<Ref<'b>>, b: impl Into<Ref<'c>>) -> bool {
    let (a, b) = (a.into(), b.into());
    match (a.view(), b.view()) {
        (View::Null, View::Null) => true,
        (View::Bool(x), View::Bool(y)) => x == y,
        (View::Number(x), View::Number(y)) => x.as_f64() == y.as_f64(),
        (View::Str(x), View::Str(y)) => x == y,
        (View::Array, View::Array) => {
            a.len() == b.len()
                && a.items()
                    .into_iter()
                    .flatten()
                    .zip(b.items().into_iter().flatten())
                    .all(|(x, y)| equal(x, y))
        }
        (View::Object, View::Object) => {
            a.len() == b.len()
                && a.entries()
                    .into_iter()
                    .flatten()
                    .all(|(key, value)| b.field(key).is_some_and(|other| equal(value, other)))
        }
        _ => false,
    }
}

fn f<'b>(value: impl Into<Ref<'b>>) -> Option<f64> {
    value.into().as_f64()
}

fn s<'b>(value: impl Into<Ref<'b>>) -> Option<&'b str> {
    value.into().as_str()
}

/// Evaluates `ast`. `None` means the server cannot compute it, and the
/// runtime fills it in.
#[expect(
    clippy::too_many_lines,
    reason = "one arm per operation, in the order `build` in runtime/src/eval.ts takes them"
)]
pub(crate) fn evaluate<'a>(ast: &'a Value, ctx: &Ctx<'a>) -> Option<Val<'a>> {
    let parts = ast.as_array()?;
    let op = parts.first()?.as_str()?;
    let arg = |i: usize| parts.get(i);
    let eval = |i: usize| evaluate(arg(i)?, ctx);
    let value = match op {
        "sig" => return Some(Val::Ref(ctx.get(arg(1)?.as_str()?))),
        "item" => return ctx.lookup(ast).map(Val::Ref),
        "lit" => return arg(1).map(|value| Val::Ref(Ref::Json(value))),
        "if" => return if truthy(&eval(1)?) { eval(2) } else { eval(3) },
        "let" => return let_in(parts, ctx).map(Val::Owned),
        "var" => {
            let depth = usize::try_from(arg(1)?.as_u64()?).ok()?;
            return ctx.vars.get(depth).cloned().map(Val::Owned);
        }
        "get" => {
            let tail = arg(2)?.as_str()?;
            return Some(match eval(1)? {
                Val::Ref(read) => Val::Ref(read.get_path(tail).unwrap_or(Ref::NULL)),
                Val::Owned(value) => Val::Owned(
                    Ref::Json(&value)
                        .get_path(tail)
                        .map_or(Value::Null, Ref::to_value),
                ),
            });
        }
        "or_else" => {
            let value = eval(1)?;
            return if Ref::from(&value).is_null() {
                eval(2)
            } else {
                Some(value)
            };
        }
        "index" => {
            let depth = usize::try_from(arg(1)?.as_u64()?).ok()?;
            Value::from(ctx.slot(depth).index)
        }
        "tuple" => Value::Array(
            parts[1..]
                .iter()
                .map(|part| evaluate(part, ctx).map(Val::into_value))
                .collect::<Option<_>>()?,
        ),
        "nth" => {
            let index = usize::try_from(Ref::from(&eval(2)?).as_i64()?).ok();
            // A list the server cannot evaluate leaves the item to the
            // browser. Only an index past the end reads as no item.
            let list = eval(1)?;
            let item = index.and_then(|index| match list {
                Val::Ref(list) => Some(Val::Ref(list.items()?.nth(index)?)),
                Val::Owned(list) => Some(Val::Owned(list.as_array()?.get(index)?.clone())),
            });
            return Some(item.unwrap_or(Val::Ref(Ref::NULL)));
        }
        "and" => Value::Bool(truthy(&eval(1)?) && truthy(&eval(2)?)),
        "or" => Value::Bool(truthy(&eval(1)?) || truthy(&eval(2)?)),
        "not" => Value::Bool(!truthy(&eval(1)?)),
        "str" => Value::from(js_string(&eval(1)?)),
        "uri" => Value::from(encode_uri_component(&js_string(&eval(1)?))),
        "uri_path" => Value::from(encode_uri_path(&js_string(&eval(1)?))),
        "query" => Value::from(query_string(&eval(1)?.to_json())),
        "fixed" => {
            let digits = u8::try_from(arg(2)?.as_u64()?).ok()?;
            Value::from(js_fixed(f(&eval(1)?)?, digits))
        }
        "format" => {
            let digits = u8::try_from(arg(2)?.as_u64()?).ok()?;
            Value::from(format_number(
                f(&eval(1)?)?,
                digits,
                arg(3)?.as_str()?,
                arg(4)?.as_str()?,
                arg(5)?.as_str()?,
                arg(6)?.as_str()?,
            ))
        }
        "with" => set_path(
            &eval(1)?.to_json(),
            arg(2)?.as_str()?,
            eval(3)?.into_value(),
        ),
        "count" | "sum_of" | "max_of" | "min_of" | "map" => fold(op, parts, ctx)?,
        "floor" => number(f(&eval(1)?)?.floor())?,
        "ceil" => number(f(&eval(1)?)?.ceil())?,
        "round" => number(js_round(f(&eval(1)?)?))?,
        "log10" => number(f(&eval(1)?)?.log10())?,
        "trim" => Value::from(s(&eval(1)?)?.trim_matches(js_whitespace)),
        "lower" => Value::from(s(&eval(1)?)?.to_lowercase()),
        "upper" => Value::from(s(&eval(1)?)?.to_uppercase()),
        "chars" => Value::from(s(&eval(1)?)?.chars().count()),
        "chars_all" | "chars_any" => test_chars(op, &eval(1)?, arg(2)?)?,
        "date" | "reltime" | "plural" => return localized(op, parts, ctx),
        "len" => Value::from(Ref::from(&eval(1)?).items().map_or(0, Iterator::count)),
        "eq" => Value::Bool(equal(&eval(1)?, &eval(2)?)),
        "ne" => Value::Bool(!equal(&eval(1)?, &eval(2)?)),
        "concat" => Value::from(format!("{}{}", s(&eval(1)?)?, s(&eval(2)?)?)),
        "contains" => Value::Bool(s(&eval(1)?)?.contains(s(&eval(2)?)?)),
        "starts" => Value::Bool(s(&eval(1)?)?.starts_with(s(&eval(2)?)?)),
        "gt" | "ge" | "lt" | "le" => compare(op, &eval(1)?, &eval(2)?)?,
        "add" | "sub" | "mul" | "div" | "rem" | "min" | "max" | "pow" => {
            arithmetic(op, f(&eval(1)?)?, f(&eval(2)?)?)?
        }
        _ => return None,
    };
    Some(Val::Owned(value))
}

/// `["chars_all",text,class]` and `["chars_any",text,class]`.
fn test_chars<'b>(op: &str, text: impl Into<Ref<'b>>, class: &Value) -> Option<Value> {
    let (text, class) = (s(text)?, char_class(class.as_str()?)?);
    Some(Value::Bool(if op == "chars_all" {
        text.chars().all(class)
    } else {
        text.chars().any(class)
    }))
}

/// The test for one character that a class name stands for, as `classes` in `runtime/src/eval.ts` holds it.
fn char_class(name: &str) -> Option<fn(char) -> bool> {
    Some(match name {
        "digit" => |c: char| c.is_ascii_digit(),
        "hex" => |c: char| c.is_ascii_hexdigit(),
        "alpha" => |c: char| c.is_ascii_alphabetic(),
        "alnum" => |c: char| c.is_ascii_alphanumeric(),
        "upper" => |c: char| c.is_ascii_uppercase(),
        "lower" => |c: char| c.is_ascii_lowercase(),
        "space" => |c: char| c.is_ascii_whitespace(),
        _ => return None,
    })
}

/// `["date",...]`, `["reltime",...]`, and `["plural",...]`, whose text depends on the locale they name.
fn localized<'a>(op: &str, parts: &'a [Value], ctx: &Ctx<'a>) -> Option<Val<'a>> {
    let arg = |i: usize| parts.get(i);
    let eval = |i: usize| evaluate(arg(i)?, ctx);
    let text = match op {
        "date" => {
            let offset = i64::from(i16::try_from(arg(5)?.as_i64()?).ok()?);
            let millis = Ref::from(&eval(1)?)
                .as_i64()?
                .checked_add(offset * 60_000)?;
            let (date, time) = (arg(3)?.as_str(), arg(4)?.as_str());
            crate::date::write_date(arg(2)?.as_str()?, date, time, millis)?
        }
        "reltime" => {
            let millis = f(&eval(1)?)? - f(&eval(2)?)?;
            crate::date::write_relative(arg(3)?.as_str()?, millis)?
        }
        _ => {
            let index = crate::intl::plural_index(arg(2)?.as_str()?, f(&eval(1)?)?)?;
            return evaluate(arg(3)?.as_array()?.get(index)?, ctx);
        }
    };
    Some(Val::Owned(Value::from(text)))
}

/// `["let",value,depth,body]`: `body` with `["var",depth]` standing for `value`.
fn let_in<'a>(parts: &'a [Value], ctx: &Ctx<'a>) -> Option<Value> {
    let depth = usize::try_from(parts.get(2)?.as_u64()?).ok()?;
    let mut vars: Vec<Value> = ctx.vars.iter().take(depth).cloned().collect();
    vars.push(evaluate(parts.get(1)?, ctx)?.into_value());
    let inner = Ctx {
        root: ctx.root,
        scope: ctx.scope.clone(),
        copy: ctx.copy,
        vars,
        rules: ctx.rules,
    };
    Some(evaluate(parts.get(3)?, &inner)?.into_value())
}

/// `["count",place,depth,e]` and the other operations that evaluate `e`
/// once per item of the list at `place`.
fn fold<'a>(op: &str, parts: &'a [Value], ctx: &Ctx<'a>) -> Option<Value> {
    let list = ctx.lookup(parts.get(1)?)?;
    let depth = usize::try_from(parts.get(2)?.as_u64()?).ok()?;
    let each = parts.get(3)?;
    let items = list.items().unwrap_or(Items::Json([].iter()));
    let mut count = 0_u64;
    let mut folded: Option<f64> = None;
    let mut mapped = Vec::new();
    ctx.for_each_item(depth, items, |item, _| {
        let value = evaluate(each, item)?;
        if op == "map" {
            mapped.push(value.into_value());
            return Some(());
        }
        if op == "count" {
            count += u64::from(truthy(&value));
            return Some(());
        }
        let value = f(&value)?;
        folded = Some(match (op, folded) {
            (_, None) => value,
            ("sum_of", Some(sum)) => sum + value,
            ("max_of", Some(max)) => max.max(value),
            (_, Some(min)) => min.min(value),
        });
        Some(())
    })?;
    Some(match (op, folded) {
        ("map", _) => Value::Array(mapped),
        ("count", _) => Value::from(count),
        ("sum_of", None) => Value::from(0),
        (_, Some(folded)) => number(folded)?,
        (_, None) => Value::Null,
    })
}

/// Whether JavaScript's `String.prototype.trim` removes `c`: the Unicode
/// `White_Space` characters `char::is_whitespace` tests, apart from U+0085,
/// and U+FEFF as well.
fn js_whitespace(c: char) -> bool {
    (c.is_whitespace() && c != '\u{85}') || c == '\u{feff}'
}

/// `a < b` and friends: strings in UTF-16 code unit order, as JavaScript
/// compares them, and anything else as numbers.
fn compare<'b, 'c>(op: &str, a: impl Into<Ref<'b>>, b: impl Into<Ref<'c>>) -> Option<Value> {
    let (a, b) = (a.into(), b.into());
    let order = match (a.view(), b.view()) {
        (View::Str(a), View::Str(b)) => a.encode_utf16().cmp(b.encode_utf16()),
        _ => f(a)?.partial_cmp(&f(b)?)?,
    };
    Some(Value::Bool(match op {
        "gt" => order.is_gt(),
        "ge" => order.is_ge(),
        "lt" => order.is_lt(),
        _ => order.is_le(),
    }))
}

fn arithmetic(op: &str, a: f64, b: f64) -> Option<Value> {
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
pub(crate) mod differential;

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::{Ctx, evaluate, format_number, js_fixed, js_json, js_round, js_string};

    fn eval(ast: &serde_json::Value, root: &serde_json::Value) -> serde_json::Value {
        evaluate(
            ast,
            &Ctx {
                root: root.into(),
                scope: Vec::new(),
                copy: false,
                vars: Vec::new(),
                rules: None,
            },
        )
        .expect("evaluates")
        .into_value()
    }

    #[test]
    fn trims_and_orders_strings_as_javascript_does() {
        let root = json!({});
        assert_eq!(
            eval(&json!(["trim", ["lit", "\u{feff} a\u{85}"]]), &root),
            json!("a\u{85}")
        );
        // U+FF61 sorts after the surrogate pair that holds U+1F600 in UTF-16,
        // and before U+1F600 in code point order.
        assert_eq!(
            eval(
                &json!(["gt", ["lit", "\u{ff61}"], ["lit", "\u{1f600}"]]),
                &root
            ),
            json!(true)
        );
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
    fn tests_characters_by_ascii_class_as_the_runtime_does() {
        let root = json!({});
        let test =
            |op: &str, text: &str, class: &str| eval(&json!([op, ["lit", text], class]), &root);
        assert_eq!(test("chars_all", "0123", "digit"), json!(true));
        assert_eq!(test("chars_all", "", "digit"), json!(true));
        assert_eq!(test("chars_any", "", "digit"), json!(false));
        assert_eq!(test("chars_all", "\u{ff11}", "digit"), json!(false));
        assert_eq!(test("chars_all", "aF9", "hex"), json!(true));
        assert_eq!(test("chars_any", "a\u{b}b", "space"), json!(false));
        assert_eq!(test("chars_any", "a\u{c}b", "space"), json!(true));
        assert_eq!(test("chars_all", "aZ", "alpha"), json!(true));
        assert_eq!(test("chars_any", "😀", "alnum"), json!(false));
    }

    #[test]
    fn picks_the_plural_form_as_plural_rules_does() {
        let root = json!({});
        let forms = json!([
            ["lit", "z"],
            ["lit", "1"],
            ["lit", "2"],
            ["lit", "f"],
            ["lit", "m"],
            ["lit", "o"]
        ]);
        let plural =
            |locale: &str, n: f64| eval(&json!(["plural", ["lit", n], locale, forms]), &root);
        assert_eq!(plural("en", 1.0), json!("1"));
        assert_eq!(plural("en", -1.0), json!("1"));
        assert_eq!(plural("en", 0.0), json!("o"));
        assert_eq!(plural("en", 1.5), json!("o"));
        assert_eq!(plural("en", 2.0), json!("o"));
        assert_eq!(plural("ja", 1.0), json!("o"));
    }

    #[test]
    fn rounds_as_math_round_does() {
        assert_eq!(js_round(2.5), 3.0);
        assert_eq!(js_round(-2.5), -2.0);
        assert_eq!(js_round(-2.6), -3.0);
        assert_eq!(js_round(0.499_999_999_999_999_94), 0.0);
    }

    #[test]
    fn leaves_a_result_that_is_not_finite_to_the_browser() {
        let root = json!({ "rows": [{ "v": 1e308 }] });
        let ctx = Ctx {
            root: (&root).into(),
            scope: Vec::new(),
            copy: false,
            vars: Vec::new(),
            rules: None,
        };
        for ast in [
            json!(["div", ["lit", 1], ["lit", 0]]),
            json!(["mul", ["lit", 1e308], ["lit", 100]]),
            json!(["or_else", ["div", ["lit", 0], ["lit", 0]], ["lit", 0]]),
            json!([
                "or_else",
                [
                    "nth",
                    ["map", "rows", 0, ["mul", ["item", 0, "v"], ["lit", 100]]],
                    ["lit", 0]
                ],
                ["lit", 0]
            ]),
        ] {
            assert!(evaluate(&ast, &ctx).is_none(), "{ast}");
        }
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
        assert_eq!(js_fixed(9.5, 0), "10");
        assert_eq!(js_fixed(99.95, 1), "100.0");
    }

    #[test]
    fn rounds_a_halfway_value_up_past_its_last_bit_as_to_fixed_does() {
        // `toFixed(100)` in V8 writes the exact value rounded up at the last place.
        assert_eq!(
            js_fixed(-1.778_429_746_627_808e-15, 100),
            "-0.0000000000000017784297466278078070749163046679492164720405664862790118263546901289373636245727539063"
        );
    }

    #[test]
    #[should_panic(expected = "reads the item at depth 0 of a list that does not enclose it")]
    fn an_item_read_outside_every_list_panics() {
        let _ = eval(&json!(["item", 0, "v"]), &json!({}));
    }

    #[test]
    #[should_panic(expected = "reads the item at depth 1 of a list that does not enclose it")]
    fn an_index_read_outside_its_list_panics() {
        let _ = eval(
            &json!(["count", "rows", 0, ["eq", ["index", 1], ["lit", 0]]]),
            &json!({ "rows": [1] }),
        );
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
        assert_eq!(js_string(&json!(-42)), "-42");
        assert_eq!(
            js_string(&json!(9_007_199_254_740_991_i64)),
            "9007199254740991"
        );
        assert_eq!(
            js_string(&json!(9_007_199_254_740_993_i64)),
            "9007199254740992"
        );
        assert_eq!(js_json(&json!([1, -2.5, [3]])), "[1,-2.5,[3]]");
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
