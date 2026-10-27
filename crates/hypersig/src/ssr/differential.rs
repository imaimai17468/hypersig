//! Writes expression trees, the signals they read, and the value
//! [`evaluate`] gives each to `runtime/test/fixtures/differential.json`,
//! which `runtime/test/differential.test.ts` evaluates again with
//! `runtime/src/eval.ts` and compares.
//!
//! The trees are drawn by a seeded generator and keep to the types the Rust
//! builder in `expr.rs` and `signal.rs` gives each operation, because a tree
//! no page can write, such as `["trim",3]`, says nothing about a page. Only
//! the trees the server evaluates (`Some`) go into the file.
//!
//! It sits inside the crate because `evaluate` is crate-private, and runs with
//! `cargo test -p hypersig --lib ssr::differential`. The test fails when the
//! file differs from what the generator writes now. Run it with
//! `HYPERSIG_FIXTURES=overwrite` to rewrite it, as
//! `crates/hypersig/tests/fixtures.rs` is run.

use std::collections::BTreeSet;
use std::fmt::Write as _;
use std::path::PathBuf;

use serde_json::{Value, json};

use super::{Ctx, Val, evaluate};
use crate::tape::Tape;

/// The seed of the generator, fixed so that the file changes only with the code.
const SEED: u64 = 0x6879_7065_7273_6967;

/// The number of signal sets, each read by `CASES_PER_GROUP` trees.
const GROUPS: usize = 60;

const CASES_PER_GROUP: usize = 200;

/// The most operations from a tree's root to a leaf.
const MAX_DEPTH: u32 = 3;

/// Every operation `evaluate` gives a value for. `random` and `now` are left
/// out, because the server leaves both to the browser.
const OPERATIONS: [&str; 57] = [
    "sig",
    "item",
    "lit",
    "if",
    "let",
    "var",
    "get",
    "or_else",
    "index",
    "tuple",
    "nth",
    "and",
    "or",
    "not",
    "str",
    "uri",
    "uri_path",
    "query",
    "fixed",
    "format",
    "with",
    "count",
    "sum_of",
    "max_of",
    "min_of",
    "map",
    "floor",
    "ceil",
    "round",
    "log10",
    "trim",
    "lower",
    "upper",
    "chars",
    "chars_all",
    "chars_any",
    "date",
    "reltime",
    "plural",
    "len",
    "eq",
    "ne",
    "concat",
    "contains",
    "starts",
    "gt",
    "ge",
    "lt",
    "le",
    "add",
    "sub",
    "mul",
    "div",
    "rem",
    "min",
    "max",
    "pow",
];

/// The operations that read a value without computing one, which
/// [`Gen::case`] never puts at the root.
const LEAVES: [&str; 5] = ["sig", "item", "lit", "var", "index"];

/// `SplitMix64`, which needs no dependency and gives the same numbers on every platform.
pub(crate) struct Rng(pub(crate) u64);

impl Rng {
    pub(crate) fn next(&mut self) -> u64 {
        self.0 = self.0.wrapping_add(0x9E37_79B9_7F4A_7C15);
        let mut z = self.0;
        z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
        z ^ (z >> 31)
    }

    pub(crate) fn below(&mut self, n: usize) -> usize {
        let n = u64::try_from(n).unwrap();
        usize::try_from(self.next() % n).unwrap()
    }

    pub(crate) fn chance(&mut self, percent: usize) -> bool {
        self.below(100) < percent
    }

    pub(crate) fn pick<'a, T>(&mut self, items: &'a [T]) -> &'a T {
        &items[self.below(items.len())]
    }

    /// A float drawn from a wide spread of magnitudes, with either sign.
    fn float(&mut self) -> f64 {
        let exponent = i32::try_from(self.below(40)).unwrap() - 20;
        let mantissa =
            f64::from(u32::try_from(self.next() >> 40).unwrap()) / f64::from(1_u32 << 24);
        let sign = if self.chance(50) { -1.0 } else { 1.0 };
        sign * mantissa * 10_f64.powi(exponent)
    }
}

/// The type of an item of a list.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub(crate) enum El {
    Num,
    Int,
    Str,
    Row,
}

impl El {
    pub(crate) const ALL: [El; 4] = [El::Num, El::Int, El::Str, El::Row];

    pub(crate) fn ty(self) -> Ty {
        match self {
            El::Num => Ty::Num,
            El::Int => Ty::Int,
            El::Str => Ty::Str,
            El::Row => Ty::Row,
        }
    }
}

/// The Rust type an expression has: `f64`, `i32`, `String`, `bool`,
/// `Timestamp`, a row struct, a search struct, a `Vec`, or an `Option`.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub(crate) enum Ty {
    Num,
    Int,
    Str,
    Bool,
    Ts,
    Row,
    Search,
    List(El),
    Opt(El),
}

pub(crate) const TYPES: [Ty; 17] = [
    Ty::Num,
    Ty::Int,
    Ty::Str,
    Ty::Bool,
    Ty::Ts,
    Ty::Row,
    Ty::Search,
    Ty::List(El::Num),
    Ty::List(El::Int),
    Ty::List(El::Str),
    Ty::List(El::Row),
    Ty::Opt(El::Num),
    Ty::Opt(El::Int),
    Ty::Opt(El::Str),
    Ty::Opt(El::Row),
    // The scalar types twice more, since most bindings are text or a flag.
    Ty::Str,
    Ty::Bool,
];

/// The page signals of each type, as dotted paths.
pub(crate) fn sig_paths(ty: Ty) -> &'static [&'static str] {
    match ty {
        Ty::Num => &["n0", "n1", "n2", "rec.v", "rec.sub.x", "q.n"],
        Ty::Int => &["i0", "i1", "rec.k", "q.page"],
        Ty::Str => &["s0", "s1", "s2", "rec.t"],
        Ty::Bool => &["b0", "b1", "rec.done", "q.on"],
        Ty::Ts => &["t0", "t1"],
        Ty::Row => &["rec"],
        Ty::Search => &["q"],
        Ty::List(El::Num) => &["ln"],
        Ty::List(El::Int) => &["li"],
        Ty::List(El::Str) => &["ls", "rec.tags", "q.tags"],
        Ty::List(El::Row) => &["rows"],
        Ty::Opt(El::Num) => &["on"],
        Ty::Opt(El::Int) => &["oi"],
        Ty::Opt(El::Str) => &["os", "q.q"],
        Ty::Opt(El::Row) => &[],
    }
}

/// The fields of a row of type `ty`, as dotted paths inside the row.
pub(crate) fn row_fields(ty: Ty) -> &'static [&'static str] {
    match ty {
        Ty::Num => &["v", "sub.x"],
        Ty::Int => &["k"],
        Ty::Str => &["t"],
        Ty::Bool => &["done"],
        Ty::List(El::Str) => &["tags"],
        _ => &[],
    }
}

/// The fields of the search struct and their types.
const SEARCH_FIELDS: [(&str, Ty); 5] = [
    ("q", Ty::Opt(El::Str)),
    ("page", Ty::Int),
    ("tags", Ty::List(El::Str)),
    ("on", Ty::Bool),
    ("n", Ty::Num),
];

const NUMBERS: [f64; 44] = [
    0.0,
    -0.0,
    0.5,
    1.5,
    2.5,
    -0.5,
    -1.5,
    -2.5,
    1.0,
    -1.0,
    3.0,
    7.0,
    -7.0,
    0.1,
    0.2,
    0.3,
    1.005,
    2.675,
    0.125,
    -0.001,
    999.5,
    1234.5,
    -1234.5,
    1_000_000.0,
    1e16,
    // 2^53, and the doubles on either side of it.
    9_007_199_254_740_992.0,
    9_007_199_254_740_994.0,
    9_007_199_254_740_990.0,
    123_456_789_012_345_680_000.0,
    1e21,
    -1.5e22,
    1e-6,
    1e-7,
    5e-324,
    2.2e-308,
    1e308,
    -1e308,
    0.000_001_5,
    4.35,
    1e20,
    0.499_999_999_999_999_94,
    100.0,
    -100.0,
    10.0,
];

const INTS: [i32; 16] = [
    0,
    1,
    -1,
    2,
    3,
    5,
    -7,
    10,
    100,
    1000,
    12345,
    -12345,
    2_147_483_647,
    -2_147_483_648,
    999,
    4,
];

const STRINGS: [&str; 48] = [
    "",
    " ",
    "Tea",
    " Tea ",
    "日本語",
    "東京都",
    "ｱｲｳ",
    "😀",
    "a😀b",
    "👨‍👩‍👧",
    "\u{feff}x\u{85}",
    "\t\n x \r",
    "\u{3000}全角\u{3000}",
    "\u{200b}z",
    "\u{180e}y",
    "\u{1680}w",
    "\u{2028}v\u{2029}",
    "\u{a0}nb\u{a0}",
    "\u{b}vt\u{c}",
    "ß",
    "İ",
    "ﬁ",
    "ΟΔΟΣ",
    "ა",
    "ǅ",
    "a/b",
    "/a",
    "//x",
    "a//b/",
    "a b&c=d",
    "100%",
    "Ab 12\u{ff11}",
    "0123",
    "aF9",
    "\u{ff61}",
    "\u{1f600}",
    "e\u{301}",
    "é",
    "x",
    "A",
    "a",
    "Z",
    "-1",
    "3",
    "null",
    "?#",
    "\u{10ffff}",
    "\u{e000}",
];

const TIMESTAMPS: [i64; 12] = [
    1_767_625_445_000,
    0,
    -1,
    -86_400_000,
    253_402_300_799_999,
    253_402_300_800_000,
    -62_135_596_800_000,
    -30_610_224_000_001,
    951_782_400_000,
    1_700_000_000_123,
    4_102_444_800_000,
    1_234_567_890_000,
];

/// Differences between two timestamps, for `reltime`.
const DELTAS: [i64; 16] = [
    0,
    -1,
    1,
    -500,
    500,
    999,
    -1000,
    45_000,
    59_999,
    60_000,
    -3_600_000,
    86_400_000,
    -259_200_000,
    2_592_000_000,
    86_400_000_000,
    -63_072_000_000,
];

const CLASSES: [&str; 7] = ["digit", "hex", "alpha", "alnum", "upper", "lower", "space"];
const LOCALES: [&str; 2] = ["en", "ja"];
const STYLES: [Option<&str>; 5] = [
    None,
    Some("full"),
    Some("long"),
    Some("medium"),
    Some("short"),
];
const OFFSETS: [Option<i64>; 7] = [
    Some(0),
    Some(540),
    Some(-300),
    Some(330),
    Some(-720),
    Some(840),
    None,
];
const DIGITS: [u8; 8] = [0, 1, 2, 3, 5, 10, 20, 100];
const GROUP_SEPARATORS: [&str; 5] = [",", ".", "", " ", "'"];
const POINTS: [&str; 2] = [".", ","];
const PREFIXES: [&str; 3] = ["", "$", "¥"];
const SUFFIXES: [&str; 3] = ["", " €", "%"];

pub(crate) struct Gen {
    pub(crate) rng: Rng,
    /// The type of the item of each enclosing list, by depth.
    pub(crate) items: Vec<El>,
    /// The type of each `let` value, by depth.
    pub(crate) vars: Vec<Ty>,
}

impl Gen {
    fn number(&mut self) -> f64 {
        if self.rng.chance(20) {
            self.rng.float()
        } else {
            *self.rng.pick(&NUMBERS)
        }
    }

    pub(crate) fn int(&mut self) -> i32 {
        if self.rng.chance(20) {
            i32::try_from(self.rng.below(2001)).unwrap() - 1000
        } else {
            *self.rng.pick(&INTS)
        }
    }

    fn string(&mut self) -> String {
        let pieces = if self.rng.chance(30) { 2 } else { 1 };
        (0..pieces).map(|_| *self.rng.pick(&STRINGS)).collect()
    }

    pub(crate) fn value(&mut self, ty: Ty) -> Value {
        match ty {
            Ty::Num => json!(self.number()),
            Ty::Int => json!(self.int()),
            Ty::Str => json!(self.string()),
            Ty::Bool => json!(self.rng.chance(50)),
            Ty::Ts => json!(*self.rng.pick(&TIMESTAMPS)),
            Ty::Row => json!({
                "v": self.number(),
                "k": self.int(),
                "t": self.string(),
                "done": self.rng.chance(50),
                "tags": self.value(Ty::List(El::Str)),
                "sub": { "x": self.number() },
            }),
            Ty::Search => {
                let mut map = serde_json::Map::new();
                for (name, ty) in SEARCH_FIELDS {
                    map.insert(name.to_owned(), self.value(ty));
                }
                Value::Object(map)
            }
            Ty::List(el) => {
                let len = self.rng.below(5);
                Value::Array((0..len).map(|_| self.value(el.ty())).collect())
            }
            Ty::Opt(el) => {
                if self.rng.chance(40) {
                    Value::Null
                } else {
                    self.value(el.ty())
                }
            }
        }
    }

    /// The page's signals, with `t1` a chosen distance from `t0` so that
    /// `reltime` sees every unit.
    pub(crate) fn signals(&mut self) -> Value {
        let t0 = *self.rng.pick(&TIMESTAMPS);
        let t1 = t0 + *self.rng.pick(&DELTAS);
        json!({
            "n0": self.value(Ty::Num),
            "n1": self.value(Ty::Num),
            "n2": self.value(Ty::Num),
            "i0": self.value(Ty::Int),
            "i1": self.value(Ty::Int),
            "s0": self.value(Ty::Str),
            "s1": self.value(Ty::Str),
            "s2": self.value(Ty::Str),
            "b0": self.value(Ty::Bool),
            "b1": self.value(Ty::Bool),
            "t0": t0,
            "t1": t1,
            "on": self.value(Ty::Opt(El::Num)),
            "oi": self.value(Ty::Opt(El::Int)),
            "os": self.value(Ty::Opt(El::Str)),
            "ln": self.value(Ty::List(El::Num)),
            "li": self.value(Ty::List(El::Int)),
            "ls": self.value(Ty::List(El::Str)),
            "rows": self.value(Ty::List(El::Row)),
            "rec": self.value(Ty::Row),
            "q": self.value(Ty::Search),
        })
    }

    fn lit(&mut self, ty: Ty) -> Value {
        json!(["lit", self.value(ty)])
    }

    /// A tree with no operation inside it: a literal, a signal, a field of
    /// an enclosing item, a `let` value, or an item's position.
    fn leaf(&mut self, ty: Ty) -> Value {
        let scalar = matches!(ty, Ty::Num | Ty::Int | Ty::Str | Ty::Bool | Ty::Ts);
        // A literal list or struct is long, and a page reads most of them from signals.
        let mut options = if scalar || sig_paths(ty).is_empty() || self.rng.chance(25) {
            vec![self.lit(ty)]
        } else {
            Vec::new()
        };
        for path in sig_paths(ty) {
            options.push(json!(["sig", path]));
        }
        for (depth, el) in self.items.iter().enumerate() {
            if *el == El::Row {
                for field in row_fields(ty) {
                    options.push(json!(["item", depth, field]));
                }
            }
            if el.ty() == ty {
                options.push(json!(["item", depth, ""]));
            }
        }
        for (depth, var) in self.vars.iter().enumerate() {
            if *var == ty {
                options.push(json!(["var", depth]));
            }
        }
        if ty == Ty::Int && !self.items.is_empty() {
            options.push(json!(["index", self.rng.below(self.items.len())]));
        }
        let at = self.rng.below(options.len());
        options.swap_remove(at)
    }

    /// The tree of a list place a fold reads, and the type of its items.
    pub(crate) fn place(&mut self) -> (Value, El) {
        let mut options: Vec<(Value, El)> = vec![
            (json!("rows"), El::Row),
            (json!("ln"), El::Num),
            (json!("li"), El::Int),
            (json!("ls"), El::Str),
            (json!("rec.tags"), El::Str),
            (json!("q.tags"), El::Str),
        ];
        for (depth, el) in self.items.iter().enumerate() {
            if *el == El::Row {
                options.push((json!(["item", depth, "tags"]), El::Str));
            }
        }
        let at = self.rng.below(options.len());
        options.swap_remove(at)
    }

    fn fold(&mut self, op: &str, body: Ty, budget: u32) -> Value {
        let (place, el) = self.place();
        let depth = self.items.len();
        self.items.push(el);
        let body = self.expr(body, budget);
        self.items.pop();
        json!([op, place, depth, body])
    }

    /// A fold whose place holds items of type `el`, for `map` into a list of `el`.
    fn map(&mut self, el: El, budget: u32) -> Value {
        self.fold("map", el.ty(), budget)
    }

    fn number_ty(&mut self) -> Ty {
        if self.rng.chance(50) {
            Ty::Num
        } else {
            Ty::Int
        }
    }

    fn unary(&mut self, op: &str, ty: Ty, budget: u32) -> Value {
        json!([op, self.expr(ty, budget)])
    }

    fn binary(&mut self, op: &str, a: Ty, b: Ty, budget: u32) -> Value {
        json!([op, self.expr(a, budget), self.expr(b, budget)])
    }

    /// An expression of type `ty` with at most `budget` operations from its
    /// root to a leaf.
    pub(crate) fn expr(&mut self, ty: Ty, budget: u32) -> Value {
        if budget == 0 || self.rng.chance(20) {
            return self.leaf(ty);
        }
        self.operation(ty, budget)
    }

    /// An expression of type `ty` whose root is an operation.
    fn operation(&mut self, ty: Ty, budget: u32) -> Value {
        let b = budget - 1;
        match self.rng.below(14) {
            0 => {
                return json!([
                    "if",
                    self.expr(Ty::Bool, b),
                    self.expr(ty, b),
                    self.expr(ty, b)
                ]);
            }
            1 => {
                let value_ty = *self.rng.pick(&TYPES);
                let value = self.expr(value_ty, b);
                let depth = self.vars.len();
                self.vars.push(value_ty);
                let body = self.expr(ty, b);
                self.vars.pop();
                return json!(["let", value, depth, body]);
            }
            _ => {}
        }
        match ty {
            Ty::Num => self.num(b),
            Ty::Int => self.int_expr(b),
            Ty::Str => self.str_expr(b),
            Ty::Bool => self.bool_expr(b),
            Ty::Ts => match self.rng.below(2) {
                0 => self.binary("add", Ty::Ts, Ty::Int, b),
                _ => self.leaf(Ty::Ts),
            },
            Ty::Row => match self.rng.below(3) {
                0 => {
                    let field_ty =
                        *self
                            .rng
                            .pick(&[Ty::Num, Ty::Int, Ty::Str, Ty::Bool, Ty::List(El::Str)]);
                    let field = *self.rng.pick(row_fields(field_ty));
                    json!(["with", self.expr(Ty::Row, b), field, self.expr(field_ty, b)])
                }
                1 => self.binary("or_else", Ty::Opt(El::Row), Ty::Row, b),
                _ => self.leaf(Ty::Row),
            },
            Ty::Search => {
                let (field, field_ty) = *self.rng.pick(&SEARCH_FIELDS);
                json!([
                    "with",
                    self.expr(Ty::Search, b),
                    field,
                    self.expr(field_ty, b)
                ])
            }
            Ty::List(el) => match self.rng.below(3) {
                0 => self.map(el, b),
                1 if el == El::Str => json!(["get", self.expr(Ty::Row, b), "tags"]),
                _ => self.leaf(ty),
            },
            Ty::Opt(el) => match self.rng.below(4) {
                0 => self.binary("nth", Ty::List(el), Ty::Int, b),
                1 if matches!(el, El::Num | El::Int) => {
                    let op = *self.rng.pick(&["max_of", "min_of"]);
                    self.fold(op, el.ty(), b)
                }
                // `Expr::some`, which writes the value's own tree.
                2 if el != El::Row => self.expr(el.ty(), b),
                _ => self.leaf(ty),
            },
        }
    }

    fn num(&mut self, b: u32) -> Value {
        match self.rng.below(15) {
            0..=2 => {
                let op = *self.rng.pick(&["add", "sub", "mul", "rem", "min", "max"]);
                self.binary(op, Ty::Num, Ty::Num, b)
            }
            3 => self.binary("div", Ty::Num, Ty::Num, b),
            4 => self.binary("pow", Ty::Num, Ty::Num, b),
            5 | 6 => {
                let op = *self.rng.pick(&["floor", "ceil", "round", "log10"]);
                self.unary(op, Ty::Num, b)
            }
            // `to_f64` and `Timestamp::millis`, which write their operand's tree.
            7 => self.expr(Ty::Int, b),
            8 => self.expr(Ty::Ts, b),
            9 => self.fold("sum_of", Ty::Num, b),
            10 => self.binary("or_else", Ty::Opt(El::Num), Ty::Num, b),
            11 => {
                let field = *self.rng.pick(row_fields(Ty::Num));
                json!(["get", self.expr(Ty::Row, b), field])
            }
            _ => self.leaf(Ty::Num),
        }
    }

    fn int_expr(&mut self, b: u32) -> Value {
        match self.rng.below(12) {
            0..=2 => {
                let op = *self.rng.pick(&["add", "sub", "mul", "rem", "min", "max"]);
                self.binary(op, Ty::Int, Ty::Int, b)
            }
            3 => {
                let el = *self.rng.pick(&El::ALL);
                self.unary("len", Ty::List(el), b)
            }
            4 => self.unary("chars", Ty::Str, b),
            5 => self.fold("count", Ty::Bool, b),
            6 => self.fold("sum_of", Ty::Int, b),
            7 => self.binary("or_else", Ty::Opt(El::Int), Ty::Int, b),
            8 => json!(["get", self.expr(Ty::Row, b), "k"]),
            _ => self.leaf(Ty::Int),
        }
    }

    fn str_expr(&mut self, b: u32) -> Value {
        match self.rng.below(20) {
            0..=2 => self.binary("concat", Ty::Str, Ty::Str, b),
            3 | 4 => {
                let op = *self.rng.pick(&["trim", "lower", "upper"]);
                self.unary(op, Ty::Str, b)
            }
            5 | 6 => {
                let ty = *self.rng.pick(&[Ty::Num, Ty::Int, Ty::Bool, Ty::Str]);
                self.unary("str", ty, b)
            }
            7 => json!(["fixed", self.expr(Ty::Num, b), *self.rng.pick(&DIGITS)]),
            8 => {
                let ty = self.number_ty();
                json!([
                    "format",
                    self.expr(ty, b),
                    *self.rng.pick(&DIGITS[..6]),
                    *self.rng.pick(&GROUP_SEPARATORS),
                    *self.rng.pick(&POINTS),
                    *self.rng.pick(&PREFIXES),
                    *self.rng.pick(&SUFFIXES)
                ])
            }
            9 => {
                let ty = *self.rng.pick(&[Ty::Str, Ty::Int]);
                self.unary("uri", ty, b)
            }
            10 => self.unary("uri_path", Ty::Str, b),
            11 => self.unary("query", Ty::Search, b),
            12 => {
                let (date, time) = loop {
                    let pair = (*self.rng.pick(&STYLES), *self.rng.pick(&STYLES));
                    if pair != (None, None) {
                        break pair;
                    }
                };
                json!([
                    "date",
                    self.expr(Ty::Ts, b),
                    *self.rng.pick(&LOCALES),
                    date,
                    time,
                    *self.rng.pick(&OFFSETS)
                ])
            }
            13 => json!([
                "reltime",
                self.expr(Ty::Ts, b),
                self.expr(Ty::Ts, b),
                *self.rng.pick(&LOCALES)
            ]),
            14 => {
                let ty = self.number_ty();
                let value = self.expr(ty, b);
                let forms: Vec<Value> = (0..6).map(|_| self.leaf(Ty::Str)).collect();
                json!(["plural", value, *self.rng.pick(&LOCALES), forms])
            }
            15 => self.binary("or_else", Ty::Opt(El::Str), Ty::Str, b),
            16 => json!(["get", self.expr(Ty::Row, b), "t"]),
            _ => self.leaf(Ty::Str),
        }
    }

    fn bool_expr(&mut self, b: u32) -> Value {
        match self.rng.below(14) {
            0 => self.unary("not", Ty::Bool, b),
            1 => {
                let op = *self.rng.pick(&["and", "or"]);
                self.binary(op, Ty::Bool, Ty::Bool, b)
            }
            2..=4 => {
                let op = *self.rng.pick(&["eq", "ne"]);
                let ty = *self.rng.pick(&TYPES);
                // `is_some` and `is_none` compare with a literal `null`.
                if matches!(ty, Ty::Opt(_)) && self.rng.chance(40) {
                    return json!([op, self.expr(ty, b), ["lit", null]]);
                }
                self.binary(op, ty, ty, b)
            }
            5..=7 => {
                let op = *self.rng.pick(&["gt", "ge", "lt", "le"]);
                let ty = *self.rng.pick(&[Ty::Num, Ty::Int, Ty::Str, Ty::Ts]);
                self.binary(op, ty, ty, b)
            }
            8 => {
                let op = *self.rng.pick(&["contains", "starts"]);
                self.binary(op, Ty::Str, Ty::Str, b)
            }
            9 => {
                let op = *self.rng.pick(&["chars_all", "chars_any"]);
                json!([op, self.expr(Ty::Str, b), *self.rng.pick(&CLASSES)])
            }
            10 => json!(["get", self.expr(Ty::Row, b), "done"]),
            _ => self.leaf(Ty::Bool),
        }
    }

    /// The tree of one case: an expression of a random type, or a tuple of
    /// two or three, as an action's arguments are written.
    fn case(&mut self) -> Value {
        let depth = 1 + u32::try_from(self.rng.below(usize::try_from(MAX_DEPTH).unwrap())).unwrap();
        if self.rng.chance(4) {
            let len = 2 + self.rng.below(2);
            let mut parts = vec![json!("tuple")];
            for _ in 0..len {
                let ty = *self.rng.pick(&TYPES);
                parts.push(self.expr(ty, depth - 1));
            }
            return Value::Array(parts);
        }
        let ty = *self.rng.pick(&TYPES);
        self.operation(ty, depth)
    }
}

/// The names of the operations `ast` holds, apart from those inside a literal's value.
pub(crate) fn operations(ast: &Value, out: &mut BTreeSet<String>) {
    let Value::Array(parts) = ast else { return };
    if let Some(Value::String(op)) = parts.first() {
        out.insert(op.clone());
        if op == "lit" {
            return;
        }
    }
    for part in parts {
        operations(part, out);
    }
}

/// `text` read as JSON, with the value of each case that uses `pow` or
/// `log10` left out. Rust takes both from the platform's math library, and
/// glibc and macOS round the last bit of `log10` differently, so the file
/// written on one would fail the comparison on the other. `null` stands for
/// a file that does not parse.
fn portable(text: &str) -> Value {
    let mut file: Value = serde_json::from_str(text).unwrap_or(Value::Null);
    let groups = file.get_mut("groups").and_then(Value::as_array_mut);
    for group in groups.into_iter().flatten() {
        let cases = group.get_mut("cases").and_then(Value::as_array_mut);
        for case in cases.into_iter().flatten() {
            let mut used = BTreeSet::new();
            operations(&case[0], &mut used);
            if used.contains("pow") || used.contains("log10") {
                case[1] = Value::Null;
            }
        }
    }
    file
}

#[test]
#[cfg_attr(
    miri,
    ignore = "it reads and writes a fixture file, which miri's isolation refuses, and evaluates 12,000 trees"
)]
fn writes_what_the_server_evaluates_for_the_runtime_to_compare() {
    let mut generator = Gen {
        rng: Rng(SEED),
        items: Vec::new(),
        vars: Vec::new(),
    };
    let mut covered = BTreeSet::new();
    // Each operation is the root of at most `quota` cases, so a rare one,
    // such as `date`, is drawn about as often as `concat`.
    let roots = OPERATIONS.len() - LEAVES.len();
    let quota = (GROUPS * CASES_PER_GROUP).div_ceil(roots);
    let mut at_root = std::collections::BTreeMap::<String, usize>::new();
    let (mut evaluated, mut unevaluated) = (0_usize, 0_usize);
    let mut groups = Vec::new();
    for _ in 0..GROUPS {
        let signals = generator.signals();
        let tape = Tape::new(&signals).unwrap();
        let mut cases = Vec::new();
        while cases.len() < CASES_PER_GROUP {
            let ast = generator.case();
            let op = ast[0].as_str().unwrap().to_owned();
            if LEAVES.contains(&op.as_str()) {
                continue;
            }
            let roots = at_root.entry(op).or_insert(0);
            if *roots >= quota {
                continue;
            }
            let ctx = Ctx {
                root: tape.root(),
                scope: Vec::new(),
                copy: false,
                vars: Vec::new(),
                rules: None,
            };
            match evaluate(&ast, &ctx).map(Val::into_value) {
                Some(value) => {
                    *roots += 1;
                    operations(&ast, &mut covered);
                    cases.push(serde_json::to_string(&json!([ast, value])).unwrap());
                }
                None => unevaluated += 1,
            }
        }
        evaluated += cases.len();
        groups.push((signals, cases));
    }
    let missing: Vec<&str> = OPERATIONS
        .iter()
        .copied()
        .filter(|op| !covered.contains(*op))
        .collect();
    assert!(missing.is_empty(), "no evaluated case uses {missing:?}");

    let mut text = String::new();
    writeln!(
        text,
        "{{\"seed\":{SEED},\"evaluated\":{evaluated},\"unevaluated\":{unevaluated},\"groups\":["
    )
    .unwrap();
    for (at, (signals, cases)) in groups.iter().enumerate() {
        if at > 0 {
            text.push_str(",\n");
        }
        writeln!(text, "{{\"signals\":{signals},\"cases\":[").unwrap();
        text.push_str(&cases.join(",\n"));
        text.push_str("\n]}");
    }
    text.push_str("\n]}\n");

    let path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../runtime/test/fixtures/differential.json");
    if std::env::var("HYPERSIG_FIXTURES").as_deref() == Ok("overwrite") {
        std::fs::write(&path, text).expect("write fixture");
        return;
    }
    let on_disk = std::fs::read_to_string(&path).unwrap_or_default();
    assert!(
        portable(&on_disk) == portable(&text),
        "{} is out of date; rerun with HYPERSIG_FIXTURES=overwrite",
        path.display()
    );
}
