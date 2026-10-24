//! Typed expressions and statements that the browser runtime evaluates.
//!
//! An [`Expr`] or [`Stmt`] is serialized into an HTML attribute as a JSON
//! array whose first element names the operation. The runtime in
//! `runtime/src/eval.ts` builds each array into a function once, with no `eval`.
//!
//! | Rust | JSON |
//! |---|---|
//! | `Page::count()` | `["sig","count"]` |
//! | `Page::panel().at(Panel::open())` | `["sig","panel.open"]` |
//! | `item.at(Todo::done())` inside a list | `["item",depth,"done"]` |
//! | `item.index()` | `["index",depth]` |
//! | literal `1`, `&value` | `["lit",1]` |
//! | `a + b`, `a - b`, `a * b`, `a / b`, `a % b` | `["add",a,b]`, `["sub",a,b]`, ... `["rem",a,b]` |
//! | `a.min(b)`, `a.max(b)` | `["min",a,b]`, `["max",a,b]` |
//! | `a.equals(b)`, `a.not_equals(b)` | `["eq",a,b]`, `["ne",a,b]` |
//! | `a.gt(b)`, `ge`, `lt`, `le` | `["gt",a,b]`, ... |
//! | `a.and(b)`, `a.or(b)`, `!a` | `["and",a,b]`, `["or",a,b]`, `["not",a]` |
//! | `if_else(c, a, b)` | `["if",c,a,b]` |
//! | `(a, b)`, `(a, b, c)` | `["tuple",a,b]`, `["tuple",a,b,c]` |
//! | `a.to_text()`, `a.to_fixed(n)` | `["str",a]`, `["fixed",a,n]` |
//! | `a.format(&f)` | `["format",a,digits,group,point,prefix,suffix]` |
//! | `a.floor()`, `ceil`, `round`, `log10`, `a.pow(b)` | `["floor",a]`, `["ceil",a]`, `["round",a]`, `["log10",a]`, `["pow",a,b]` |
//! | `a.concat(b)`, `trim`, `to_lowercase`, `to_uppercase` | `["concat",a,b]`, `["trim",a]`, `["lower",a]`, `["upper",a]` |
//! | `a.contains(b)`, `a.starts_with(b)`, `a.char_count()` | `["contains",a,b]`, `["starts",a,b]`, `["chars",a]` |
//! | `a.all_chars(c)`, `a.any_char(c)` | `["chars_all",a,class]`, `["chars_any",a,class]` |
//! | `n.plural(locale, \|category\| form)` | `["plural",n,locale,[zero,one,two,few,many,other]]` |
//! | `t.format_date(&f)` | `["date",t,locale,dateStyle,timeStyle,offset]`, with `null` for a style left out and for the browser's zone |
//! | `t.relative_to(now, locale)` | `["reltime",t,now,locale]` |
//! | `t.millis()`, `t.add_millis(n)`, `Expr::timestamp_now()` | `t`, `["add",t,n]`, `["now"]` |
//! | `list.len()`, `list.nth(i)` | `["len",a]`, `["nth",a,i]` |
//! | `Expr::random_below(n)`, `Expr::now()` | `["random",n]`, `["now"]` |
//! | `list.count(\|item\| p)` | `["count",place,depth,p]` |
//! | `list.sum_of(\|item\| e)`, `max_of`, `min_of` | `["sum_of",place,depth,e]`, `["max_of",place,depth,e]`, `["min_of",place,depth,e]` |
//! | `list.map(\|item\| e)` | `["map",place,depth,e]` |
//! | `opt.unwrap_or(d)`, `a.some()` | `["or_else",a,d]`, `a` |
//! | `rec.at(field)`, `rec.with(field, v)` | `["get",a,"path"]`, `["with",a,"path",v]` |
//! | `sig.set(e)` | `["set",place,e]` |
//! | `list.push(e)`, `list.remove_at(i)` | `["push",place,e]`, `["remove",place,i]` |
//! | `list.append(e)` | `["append",place,e]` |
//! | `list.retain(\|item\| p)`, `list.set_all(f, e)` | `["retain",place,depth,p]`, `["set_all",place,"path",e]` |
//! | `list.extend(n, \|i\| e)` | `["extend",place,n,depth,e]` |
//! | `list.update_where(\|item\| p, f, \|item\| e)` | `["update",place,depth,p,"path",e]` |
//! | `list.swap(i, j)` | `["swap",place,i,j]` |
//! | `list.sort_by(\|item\| k)`, `list.sort_by_desc(\|item\| k)` | `["sort",place,depth,k,false]`, `["sort",place,depth,k,true]` |
//! | `Stmt::when(c, s)`, `Stmt::if_else(c, s, t)` | `["if",c,s,t]` |
//! | `route.link_with(p)` for `"/users/{id}"` | `["concat",["lit","/users/"],["uri",p]]` |
//! | `route.link_with((p, q))` with search parameters | `["concat",path,["query",q]]` |
//! | `route.link_with(rest)` for `"/files/{*path}"` | `["concat",["lit","/files/"],["uri_path",rest]]` |
//! | `action.call()` | `["call",method,path,args,indicator,error]` |
//! | `action.call().latest()` | `["call",method,path,args,indicator,error,["latest"]]` |
//! | `stream.call().reconnect()` | `["call","GET",path,args,indicator,error,["reconnect"]]` |
//! | `upload.call_with(file_input, args)` | `["upload",path,args,indicator,error,name]` |
//! | `node_ref.focus()` | `["dom","focus",name]` |
//! | `file_input.clear()` | `["clear-files",name]` |
//! | `link.navigate()` | `["navigate",url]` |
//! | `link.push_url()`, `link.replace_url()` | `["history","push",url]`, `["history","replace",url]` |
//! | `s1.then(s2)` | `["seq",[s1,s2]]` |
//! | `a.let_in(\|v\| e)`, and `v` inside `e` | `["let",a,depth,e]`, `["var",depth]` |
//! | `a.let_in_stmt(\|v\| s)` | `["let",a,depth,s]` |
//! | `el.on_custom(event, \|detail\| s)`, and `detail` inside `s` | `[name,depth,s]`, one entry of `data-hs-on-custom`, and `["var",depth]` |
//! | `call.on_error(\|message\| s)`, `call.on_success(t)`, a call inside a `Part` | `["call",...,flags,{"error":[depth,s],"ok":t,"part":"field"}]` |
//!
//! A place is `"dotted.path"` for a page signal and `["item",depth,"path"]`
//! for a field of a list item.

use std::cell::Cell;
use std::fmt;
use std::marker::PhantomData;
use std::ops;

use serde::Serialize;
use serde_json::{Value, json};

use crate::Segment;
use crate::exact;
use crate::signal::Sig;

/// An expression over the signals of `S` that evaluates to a `T` in the browser.
pub struct Expr<S, T> {
    ast: Value,
    marker: PhantomData<fn() -> (S, T)>,
}

impl<S> Expr<S, i32> {
    /// A random integer from 0 up to but not including `bound`, drawn in the browser.
    ///
    /// The server leaves it out of the first HTML, because it cannot draw the
    /// number the browser will.
    #[must_use]
    pub fn random_below(bound: impl IntoExpr<S, i32>) -> Self {
        Self::from_json(json!(["random", bound.into_expr().ast]))
    }
}

impl<S> Expr<S, f64> {
    /// The browser's current time, in milliseconds since the Unix epoch.
    ///
    /// The server leaves it out of the first HTML, as it does a random number.
    #[must_use]
    pub fn now() -> Self {
        Self::from_json(json!(["now"]))
    }
}

impl<S, T> Expr<S, T> {
    pub(crate) fn from_json(ast: Value) -> Self {
        Self {
            ast,
            marker: PhantomData,
        }
    }

    /// A constant value, serialized to JSON once at render time.
    ///
    /// # Panics
    ///
    /// Panics if `value` fails to serialize to JSON, which only happens for a
    /// `Serialize` impl that returns an error, or if it holds an integer outside -2^53 to 2^53,
    /// which the browser would round.
    #[must_use]
    pub fn lit(value: &T) -> Self
    where
        T: Serialize,
    {
        let value = exact::to_value(value, "a literal does not serialize to JSON");
        Self::from_json(json!(["lit", value]))
    }

    fn binary<U>(op: &str, lhs: Self, rhs: Self) -> Expr<S, U> {
        Expr::from_json(Value::Array(vec![Value::from(op), lhs.ast, rhs.ast]))
    }

    fn unary<U>(op: &str, operand: Self) -> Expr<S, U> {
        Expr::from_json(Value::Array(vec![Value::from(op), operand.ast]))
    }

    fn retype<U>(self) -> Expr<S, U> {
        Expr::from_json(self.ast)
    }

    /// The field `field` of the struct this expression evaluates to.
    #[must_use]
    pub fn at<U>(self, field: Sig<T, U>) -> Expr<S, U> {
        Expr::from_json(json!(["get", self.ast, field.raw_path().dotted()]))
    }

    /// The expression `body` builds from this value, which is evaluated once however often `body` uses it.
    ///
    /// `body` receives an expression that stands for the value. Each use of
    /// a plain expression copies it into the attribute and evaluates it
    /// again, so a value that costs a pass over a list, such as
    /// [`Sig::max_of`], is better named once with `let_in`. The expression
    /// `body` receives means nothing outside `body`, and the type system
    /// does not stop one from being moved out.
    ///
    /// ```
    /// use hypersig::{Expr, Signals, if_else};
    /// use serde::{Deserialize, Serialize};
    ///
    /// #[derive(Signals, Serialize, Deserialize)]
    /// struct Scores {
    ///     scores: Vec<i32>,
    /// }
    ///
    /// let best = Scores::scores().max_of(|score| score.value()).unwrap_or(0);
    /// let _label: Expr<Scores, String> =
    ///     best.let_in(|best| if_else(best.clone().gt(90), "top", best.to_text()));
    /// ```
    #[must_use]
    pub fn let_in<U>(self, body: impl FnOnce(Expr<S, T>) -> Expr<S, U>) -> Expr<S, U> {
        in_let_scope(|depth| {
            let body = body(Expr::from_json(json!(["var", depth])));
            Expr::from_json(json!(["let", self.ast, depth, body.ast]))
        })
    }

    /// A statement that runs `body` with this value, computed once when the statement runs.
    ///
    /// `body` receives an expression that reads the value as it was then.
    /// Statements that run later inside `body`, such as a call's
    /// [`on_error`](crate::Call::on_error), still read that value after the
    /// signals it came from have changed. As with [`let_in`](Expr::let_in),
    /// the expression means nothing outside `body`.
    ///
    /// ```
    /// use hypersig::{Action, Signals};
    /// use serde::{Deserialize, Serialize};
    ///
    /// #[derive(Signals, Serialize, Deserialize)]
    /// struct Notes {
    ///     next: i32,
    ///     failed: Vec<i32>,
    /// }
    ///
    /// const SAVE: Action<Notes, i32> = Action::post("/save");
    /// let next = Notes::next();
    /// let _save = next.let_in_stmt(|sent| {
    ///     SAVE.call_with(sent.clone())
    ///         .on_error(|_| Notes::failed().push(sent))
    ///         .then(next.set(next + 1))
    /// });
    /// ```
    #[must_use]
    pub fn let_in_stmt<B: IntoStmt<S>>(self, body: impl FnOnce(Expr<S, T>) -> B) -> Stmt<S> {
        in_let_scope(|depth| {
            let body = body(Expr::from_json(json!(["var", depth]))).into_stmt();
            Stmt::new(json!(["let", self.ast, depth, body.ast]))
        })
    }

    pub(crate) fn into_json(self) -> Value {
        self.ast
    }

    pub(crate) fn as_json(&self) -> &Value {
        &self.ast
    }

    pub(crate) fn to_json(&self) -> String {
        self.ast.to_string()
    }
}

thread_local! {
    static LET_DEPTH: Cell<u8> = const { Cell::new(0) };
}

/// Runs `body` with the depth of a new `let_in` scope, one deeper than the
/// enclosing one, so nested names do not collide.
pub(crate) fn in_let_scope<R>(body: impl FnOnce(u8) -> R) -> R {
    struct Restore(u8);
    impl Drop for Restore {
        fn drop(&mut self) {
            LET_DEPTH.set(self.0);
        }
    }
    let depth = LET_DEPTH.get();
    let _restore = Restore(depth);
    LET_DEPTH.set(
        depth
            .checked_add(1)
            .expect("let_in scopes nest fewer than 256 deep"),
    );
    body(depth)
}

impl<S, T> Clone for Expr<S, T> {
    fn clone(&self) -> Self {
        Self::from_json(self.ast.clone())
    }
}

impl<S, T> fmt::Debug for Expr<S, T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("Expr").field(&self.ast).finish()
    }
}

/// Conversion into an [`Expr`] over `S` of type `T`.
///
/// Implemented for [`Expr`] itself, for [`Sig`] and [`Item`](crate::Item), for literals of
/// `i32`, `i64`, `f64`, `bool`, `String`, and `&str`, and for a reference to any
/// `Serialize` value, so an operand can be any of them.
#[diagnostic::on_unimplemented(
    message = "`{Self}` is not an expression of type `{T}` on the page whose signals are `{S}`",
    note = "an expression must have the type the attribute expects, and use signals of the same page"
)]
pub trait IntoExpr<S, T> {
    /// Converts `self` into an expression.
    fn into_expr(self) -> Expr<S, T>;
}

impl<S, T> IntoExpr<S, T> for Expr<S, T> {
    fn into_expr(self) -> Expr<S, T> {
        self
    }
}

impl<S, T, R> IntoExpr<S, T> for Sig<S, T, R> {
    fn into_expr(self) -> Expr<S, T> {
        self.expr()
    }
}

impl<S, T: Serialize> IntoExpr<S, T> for &T {
    fn into_expr(self) -> Expr<S, T> {
        Expr::lit(self)
    }
}

macro_rules! literal {
    ($($ty:ty => $target:ty),* $(,)?) => {$(
        impl<S> IntoExpr<S, $target> for $ty {
            fn into_expr(self) -> Expr<S, $target> {
                let value = json!(self);
                exact::check(&value);
                Expr::from_json(json!(["lit", value]))
            }
        }
    )*};
}

literal!(i32 => i32, i64 => i64, f64 => f64, bool => bool, String => String, &str => String);

macro_rules! tuple {
    ($(($expr:ident, $value:ident, $ty:ident)),+) => {
        /// A tuple of expressions is an expression of the tuple of their values,
        /// such as the arguments of an [`Action`](crate::Action) call.
        impl<S, $($ty,)+ $($expr: IntoExpr<S, $ty>,)+> IntoExpr<S, ($($ty,)+)> for ($($expr,)+) {
            fn into_expr(self) -> Expr<S, ($($ty,)+)> {
                let ($($value,)+) = self;
                Expr::from_json(json!(["tuple", $($value.into_expr().ast),+]))
            }
        }
    };
}

tuple!((A, a, TA), (B, b, TB));
tuple!((A, a, TA), (B, b, TB), (C, c, TC));

/// The expression `then` while `cond` is true, and `otherwise` while it is false.
#[must_use]
pub fn if_else<S, T>(
    cond: impl IntoExpr<S, bool>,
    then: impl IntoExpr<S, T>,
    otherwise: impl IntoExpr<S, T>,
) -> Expr<S, T> {
    Expr::from_json(json!([
        "if",
        cond.into_expr().ast,
        then.into_expr().ast,
        otherwise.into_expr().ast
    ]))
}

/// A type the runtime does arithmetic on with the same result Rust would get.
///
/// `i32`, `i64`, and `f64` only. JavaScript numbers are 64-bit floats, so
/// integer division would come back with a different value, and only `f64`
/// divides. A result outside `i32`'s range fails to deserialize on the next
/// action and the action answers 400.
///
/// An `i64` is exact in the browser from -2^53 to 2^53
/// (9,007,199,254,740,992). An integer past that, sent by the server in a
/// page, a patch, or a literal, panics before it becomes JSON, because the
/// browser would round it to the nearest double and the server would read
/// back the rounded value. Arithmetic in the browser that leaves the range
/// is not checked and rounds the same way. Keep an amount of
/// money in minor units, such as cents, in an `i64`, and show it with
/// `(cents.to_f64() / 100.0).format(&format)`.
#[diagnostic::on_unimplemented(
    message = "the browser does not do arithmetic on `{Self}`",
    note = "arithmetic is supported on `i32`, `i64`, and `f64`, and division on `f64` only"
)]
pub trait Number: sealed::Sealed {}
impl Number for i32 {}
impl Number for i64 {}
impl Number for f64 {}

/// A type the runtime can order with `<` and friends.
///
/// The runtime orders strings by UTF-16 code unit and the server by code point, so
/// two strings holding characters outside the Basic Multilingual Plane can
/// order differently in the first HTML. The runtime's result replaces the
/// server's once the page loads.
pub trait Comparable: sealed::Sealed {}
impl Comparable for i32 {}
impl Comparable for i64 {}
impl Comparable for f64 {}
impl Comparable for String {}
impl Comparable for crate::Timestamp {}

/// A type the runtime can turn into display text.
///
/// It is implemented for `f64`, `bool`, and every [`Segment`]: the
/// integers, `String`, and an enum with `#[derive(Segment)]`, whose text is
/// the string serde writes for the variant.
#[diagnostic::on_unimplemented(
    message = "the browser cannot write `{Self}` as text",
    note = "`to_text` takes a number, a `bool`, a `String`, or an enum with `#[derive(Segment)]`"
)]
pub trait Textual: sealed::TextSeal {}
impl<T: Segment> Textual for T {}
impl Textual for f64 {}
impl Textual for bool {}

mod sealed {
    use crate::Segment;

    pub trait Sealed {}
    impl Sealed for i32 {}
    impl Sealed for i64 {}
    impl Sealed for f64 {}
    impl Sealed for bool {}
    impl Sealed for String {}
    impl Sealed for crate::Timestamp {}

    pub trait TextSeal {}
    impl<T: Segment> TextSeal for T {}
    impl TextSeal for f64 {}
    impl TextSeal for bool {}
}

/// Defines the operator impls and methods on both `Expr` and `Sig`, so a
/// signal can be used as an operand without converting it first.
macro_rules! operations {
    ($ty:ident $(, $r:ident)?) => {
        impl<S, T: Number, $($r,)? Rhs: IntoExpr<S, T>> ops::Add<Rhs> for $ty<S, T $(, $r)?> {
            type Output = Expr<S, T>;
            fn add(self, rhs: Rhs) -> Expr<S, T> {
                Expr::binary("add", self.into_expr(), rhs.into_expr())
            }
        }

        impl<S, T: Number, $($r,)? Rhs: IntoExpr<S, T>> ops::Sub<Rhs> for $ty<S, T $(, $r)?> {
            type Output = Expr<S, T>;
            fn sub(self, rhs: Rhs) -> Expr<S, T> {
                Expr::binary("sub", self.into_expr(), rhs.into_expr())
            }
        }

        impl<S, T: Number, $($r,)? Rhs: IntoExpr<S, T>> ops::Mul<Rhs> for $ty<S, T $(, $r)?> {
            type Output = Expr<S, T>;
            fn mul(self, rhs: Rhs) -> Expr<S, T> {
                Expr::binary("mul", self.into_expr(), rhs.into_expr())
            }
        }

        // JavaScript's `%` truncates toward zero like Rust's.
        impl<S, T: Number, $($r,)? Rhs: IntoExpr<S, T>> ops::Rem<Rhs> for $ty<S, T $(, $r)?> {
            type Output = Expr<S, T>;
            fn rem(self, rhs: Rhs) -> Expr<S, T> {
                Expr::binary("rem", self.into_expr(), rhs.into_expr())
            }
        }

        impl<S, $($r,)? Rhs: IntoExpr<S, f64>> ops::Div<Rhs> for $ty<S, f64 $(, $r)?> {
            type Output = Expr<S, f64>;
            fn div(self, rhs: Rhs) -> Expr<S, f64> {
                Expr::binary("div", self.into_expr(), rhs.into_expr())
            }
        }

        impl<S $(, $r)?> ops::Not for $ty<S, bool $(, $r)?> {
            type Output = Expr<S, bool>;
            fn not(self) -> Expr<S, bool> {
                Expr::unary("not", self.into_expr())
            }
        }

        impl<S, T: Number $(, $r)?> $ty<S, T $(, $r)?> {
            /// The smaller of this value and `rhs`.
            #[must_use]
            pub fn min(self, rhs: impl IntoExpr<S, T>) -> Expr<S, T> {
                Expr::binary("min", self.into_expr(), rhs.into_expr())
            }

            /// The larger of this value and `rhs`.
            #[must_use]
            pub fn max(self, rhs: impl IntoExpr<S, T>) -> Expr<S, T> {
                Expr::binary("max", self.into_expr(), rhs.into_expr())
            }

            /// This number as text in `format`, with its digits grouped and its affixes.
            ///
            /// ```
            /// use hypersig::{NumberFormat, Signals};
            /// use serde::{Deserialize, Serialize};
            ///
            /// #[derive(Signals, Serialize, Deserialize)]
            /// struct Cart {
            ///     total: f64,
            /// }
            ///
            /// const YEN: NumberFormat = NumberFormat::new().prefix("¥");
            /// let _label = Cart::total().format(&YEN);
            /// ```
            #[must_use]
            pub fn format(self, format: &NumberFormat) -> Expr<S, String> {
                Expr::from_json(json!([
                    "format",
                    self.into_expr().ast,
                    format.digits,
                    format.group,
                    format.point,
                    format.prefix,
                    format.suffix
                ]))
            }

            /// This number as an `f64`, which holds every `i32` exactly.
            ///
            /// The browser keeps both as a JavaScript number, so the value
            /// is unchanged. Code generic over [`Number`] uses it to do
            /// division, which only `f64` has.
            #[must_use]
            pub fn to_f64(self) -> Expr<S, f64> {
                self.into_expr().retype()
            }
        }

        impl<S $(, $r)?> $ty<S, f64 $(, $r)?> {
            /// The largest whole number at or below this one.
            #[must_use]
            pub fn floor(self) -> Expr<S, f64> {
                Expr::unary("floor", self.into_expr())
            }

            /// The smallest whole number at or above this one.
            #[must_use]
            pub fn ceil(self) -> Expr<S, f64> {
                Expr::unary("ceil", self.into_expr())
            }

            /// The nearest whole number, with a halfway value rounded up.
            ///
            /// It is JavaScript's `Math.round`, so `-2.5` rounds to `-2`,
            /// where Rust's `f64::round` gives `-3`.
            #[must_use]
            pub fn round(self) -> Expr<S, f64> {
                Expr::unary("round", self.into_expr())
            }

            /// The base 10 logarithm of this number.
            ///
            /// The browser and the server each use their platform's math
            /// library, which can differ in the last bit of the result.
            #[must_use]
            pub fn log10(self) -> Expr<S, f64> {
                Expr::unary("log10", self.into_expr())
            }

            /// This number raised to the power `exponent`.
            ///
            /// The browser and the server each use their platform's math
            /// library, which can differ in the last bit of the result.
            #[must_use]
            pub fn pow(self, exponent: impl IntoExpr<S, f64>) -> Expr<S, f64> {
                Expr::binary("pow", self.into_expr(), exponent.into_expr())
            }

            /// This number as text with `digits` digits after the point.
            ///
            /// The runtime uses `Number.prototype.toFixed`, which rounds a
            /// halfway value away from zero where Rust's `format!("{:.n}")`
            /// rounds it to even. The first HTML follows the runtime.
            #[must_use]
            pub fn to_fixed(self, digits: u8) -> Expr<S, String> {
                Expr::from_json(json!(["fixed", self.into_expr().ast, digits]))
            }
        }

        impl<S $(, $r)?> $ty<S, String $(, $r)?> {
            /// This string with `rhs` appended.
            #[must_use]
            pub fn concat(self, rhs: impl IntoExpr<S, String>) -> Expr<S, String> {
                Expr::binary("concat", self.into_expr(), rhs.into_expr())
            }

            /// Whether this string is empty.
            #[must_use]
            pub fn is_empty(self) -> Expr<S, bool> {
                self.equals("")
            }

            /// The number of Unicode scalar values, as Rust's `chars().count()`.
            #[must_use]
            pub fn char_count(self) -> Expr<S, i32> {
                Expr::unary("chars", self.into_expr())
            }

            /// This string without leading and trailing whitespace.
            #[must_use]
            pub fn trim(self) -> Expr<S, String> {
                Expr::unary("trim", self.into_expr())
            }

            /// This string in lowercase.
            #[must_use]
            pub fn to_lowercase(self) -> Expr<S, String> {
                Expr::unary("lower", self.into_expr())
            }

            /// This string in uppercase.
            #[must_use]
            pub fn to_uppercase(self) -> Expr<S, String> {
                Expr::unary("upper", self.into_expr())
            }

            /// Whether `needle` occurs in this string.
            #[must_use]
            pub fn contains(self, needle: impl IntoExpr<S, String>) -> Expr<S, bool> {
                Expr::binary("contains", self.into_expr(), needle.into_expr())
            }

            /// Whether this string starts with `prefix`.
            #[must_use]
            pub fn starts_with(self, prefix: impl IntoExpr<S, String>) -> Expr<S, bool> {
                Expr::binary("starts", self.into_expr(), prefix.into_expr())
            }

            /// Whether every character of this string is in `class`, true for an empty string.
            ///
            /// ```
            /// use hypersig::{CharClass, Signals};
            /// use serde::{Deserialize, Serialize};
            ///
            /// #[derive(Signals, Serialize, Deserialize)]
            /// struct Signup {
            ///     zip: String,
            /// }
            ///
            /// let zip = Signup::zip();
            /// let _valid = zip.char_count().equals(7).and(zip.all_chars(CharClass::AsciiDigit));
            /// ```
            #[must_use]
            pub fn all_chars(self, class: CharClass) -> Expr<S, bool> {
                Expr::from_json(json!(["chars_all", self.into_expr().ast, class.name()]))
            }

            /// Whether some character of this string is in `class`, false for an empty string.
            #[must_use]
            pub fn any_char(self, class: CharClass) -> Expr<S, bool> {
                Expr::from_json(json!(["chars_any", self.into_expr().ast, class.name()]))
            }
        }

        impl<S, T $(, $r)?> $ty<S, Vec<T> $(, $r)?> {
            /// The number of items.
            #[must_use]
            pub fn len(self) -> Expr<S, i32> {
                Expr::unary("len", self.into_expr())
            }

            /// The item at `index`, or `None` past either end.
            #[must_use]
            pub fn nth(self, index: impl IntoExpr<S, i32>) -> Expr<S, Option<T>> {
                Expr::from_json(json!(["nth", self.into_expr().ast, index.into_expr().ast]))
            }

            /// Whether the list has no items.
            #[must_use]
            pub fn is_empty(self) -> Expr<S, bool> {
                self.len().equals(0)
            }
        }

        impl<S, T $(, $r)?> $ty<S, Option<T> $(, $r)?> {
            /// Whether this option holds a value.
            #[must_use]
            pub fn is_some(self) -> Expr<S, bool> {
                Expr::binary("ne", self.into_expr(), Expr::from_json(json!(["lit", null])))
            }

            /// Whether this option is `None`.
            #[must_use]
            pub fn is_none(self) -> Expr<S, bool> {
                Expr::binary("eq", self.into_expr(), Expr::from_json(json!(["lit", null])))
            }

            /// The held value, or `default` for `None`.
            #[must_use]
            pub fn unwrap_or(self, default: impl IntoExpr<S, T>) -> Expr<S, T> {
                Expr::from_json(json!(["or_else", self.into_expr().ast, default.into_expr().ast]))
            }
        }

        impl<S, T: Comparable $(, $r)?> $ty<S, T $(, $r)?> {
            /// Whether this value is strictly greater than `rhs`.
            #[must_use]
            pub fn gt(self, rhs: impl IntoExpr<S, T>) -> Expr<S, bool> {
                Expr::binary("gt", self.into_expr(), rhs.into_expr())
            }

            /// Whether this value is greater than or equal to `rhs`.
            #[must_use]
            pub fn ge(self, rhs: impl IntoExpr<S, T>) -> Expr<S, bool> {
                Expr::binary("ge", self.into_expr(), rhs.into_expr())
            }

            /// Whether this value is strictly less than `rhs`.
            #[must_use]
            pub fn lt(self, rhs: impl IntoExpr<S, T>) -> Expr<S, bool> {
                Expr::binary("lt", self.into_expr(), rhs.into_expr())
            }

            /// Whether this value is less than or equal to `rhs`.
            #[must_use]
            pub fn le(self, rhs: impl IntoExpr<S, T>) -> Expr<S, bool> {
                Expr::binary("le", self.into_expr(), rhs.into_expr())
            }
        }

        impl<S, T: PartialEq $(, $r)?> $ty<S, T $(, $r)?> {
            /// Whether this value equals `rhs`.
            ///
            /// The runtime compares the JSON forms structurally, which agrees
            /// with a derived `PartialEq` and may disagree with a hand-written one.
            #[must_use]
            pub fn equals(self, rhs: impl IntoExpr<S, T>) -> Expr<S, bool> {
                Expr::binary("eq", self.into_expr(), rhs.into_expr())
            }

            /// Whether this value differs from `rhs`, comparing JSON forms.
            #[must_use]
            pub fn not_equals(self, rhs: impl IntoExpr<S, T>) -> Expr<S, bool> {
                Expr::binary("ne", self.into_expr(), rhs.into_expr())
            }
        }

        impl<S $(, $r)?> $ty<S, bool $(, $r)?> {
            /// Whether both this value and `rhs` are true.
            #[must_use]
            pub fn and(self, rhs: impl IntoExpr<S, bool>) -> Expr<S, bool> {
                Expr::binary("and", self.into_expr(), rhs.into_expr())
            }

            /// Whether this value or `rhs` is true.
            #[must_use]
            pub fn or(self, rhs: impl IntoExpr<S, bool>) -> Expr<S, bool> {
                Expr::binary("or", self.into_expr(), rhs.into_expr())
            }
        }

        impl<S, T: Textual $(, $r)?> $ty<S, T $(, $r)?> {
            /// This value as display text, as JavaScript's `String` writes it.
            #[must_use]
            pub fn to_text(self) -> Expr<S, String> {
                Expr::unary("str", self.into_expr())
            }
        }

        impl<S, T $(, $r)?> $ty<S, T $(, $r)?> {
            /// This value wrapped in `Some`.
            #[must_use]
            pub fn some(self) -> Expr<S, Option<T>> {
                self.into_expr().retype()
            }

            /// A copy of this struct with `field` replaced by `value`.
            #[must_use]
            pub fn with<U>(self, field: Sig<T, U>, value: impl IntoExpr<S, U>) -> Expr<S, T> {
                Expr::from_json(json!([
                    "with",
                    self.into_expr().ast,
                    field.raw_path().dotted(),
                    value.into_expr().ast
                ]))
            }
        }
    };
}

operations!(Expr);
operations!(Sig, R);

/// How [`format`](Expr::format) writes a number: decimals, separators, and affixes.
///
/// Build one in a `const` and share it between the pages that show the
/// same kind of number. The browser and the first HTML write the same text,
/// with no dependence on the visitor's locale:
///
/// 1. the number rounded to [`digits`](NumberFormat::digits) places, as
///    `Number.prototype.toFixed` rounds it, with halves away from zero;
/// 2. its whole part split into groups of three digits by
///    [`group`](NumberFormat::group), and its fraction after
///    [`point`](NumberFormat::point);
/// 3. a minus sign for a negative number that does not round to zero,
///    then [`prefix`](NumberFormat::prefix), the digits, and
///    [`suffix`](NumberFormat::suffix).
///
/// A number at or above `1e21` keeps JavaScript's exponent form without
/// groups, and a value that is not finite is written as `NaN` or
/// `Infinity` between the affixes.
///
/// ```
/// use hypersig::NumberFormat;
///
/// const DOLLARS: NumberFormat = NumberFormat::new().digits(2).prefix("$");
/// const PERCENT: NumberFormat = NumberFormat::new().digits(1).suffix("%");
/// const EURO: NumberFormat = NumberFormat::new().digits(2).group(".").point(",").suffix(" €");
/// assert_eq!(DOLLARS.write(-1234.5), "-$1,234.50");
/// assert_eq!(PERCENT.write(12.25), "12.3%");
/// assert_eq!(EURO.write(1234567.891), "1.234.567,89 €");
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct NumberFormat {
    digits: u8,
    group: &'static str,
    point: &'static str,
    prefix: &'static str,
    suffix: &'static str,
}

impl NumberFormat {
    /// Whole numbers in groups of three split by `,`, with no affixes.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            digits: 0,
            group: ",",
            point: ".",
            prefix: "",
            suffix: "",
        }
    }

    /// Writes `digits` digits after the point, 0 until set.
    ///
    /// # Panics
    ///
    /// Panics if `digits` is above 100, the most `toFixed` takes. In a
    /// `const`, that is a compile error.
    #[must_use]
    pub const fn digits(mut self, digits: u8) -> Self {
        assert!(digits <= 100, "toFixed takes at most 100 digits");
        self.digits = digits;
        self
    }

    /// Splits the whole part into groups of three with `separator`, `,` until set.
    ///
    /// An empty separator leaves the digits ungrouped.
    #[must_use]
    pub const fn group(mut self, separator: &'static str) -> Self {
        self.group = separator;
        self
    }

    /// Writes `point` between the whole part and the fraction, `.` until set.
    #[must_use]
    pub const fn point(mut self, point: &'static str) -> Self {
        self.point = point;
        self
    }

    /// Writes `prefix` before the digits and after any minus sign.
    #[must_use]
    pub const fn prefix(mut self, prefix: &'static str) -> Self {
        self.prefix = prefix;
        self
    }

    /// Writes `suffix` after the digits.
    #[must_use]
    pub const fn suffix(mut self, suffix: &'static str) -> Self {
        self.suffix = suffix;
        self
    }

    /// `value` written in this format, as the browser writes it.
    #[must_use]
    pub fn write(&self, value: f64) -> String {
        crate::ssr::format_number(
            value,
            self.digits,
            self.group,
            self.point,
            self.prefix,
            self.suffix,
        )
    }
}

impl Default for NumberFormat {
    fn default() -> Self {
        Self::new()
    }
}

/// A set of ASCII characters that [`all_chars`](Expr::all_chars) and [`any_char`](Expr::any_char) test for.
///
/// Each class holds what the `char` method of the same name accepts, such
/// as [`char::is_ascii_digit`]. A character outside ASCII is in no class,
/// so the browser and the server agree whatever Unicode version each has.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum CharClass {
    /// `0` to `9`.
    AsciiDigit,
    /// `0` to `9`, `a` to `f`, and `A` to `F`.
    AsciiHexdigit,
    /// `a` to `z` and `A` to `Z`.
    AsciiAlphabetic,
    /// `a` to `z`, `A` to `Z`, and `0` to `9`.
    AsciiAlphanumeric,
    /// `A` to `Z`.
    AsciiUppercase,
    /// `a` to `z`.
    AsciiLowercase,
    /// Space, tab, line feed, form feed, and carriage return.
    AsciiWhitespace,
}

impl CharClass {
    fn name(self) -> &'static str {
        match self {
            Self::AsciiDigit => "digit",
            Self::AsciiHexdigit => "hex",
            Self::AsciiAlphabetic => "alpha",
            Self::AsciiAlphanumeric => "alnum",
            Self::AsciiUppercase => "upper",
            Self::AsciiLowercase => "lower",
            Self::AsciiWhitespace => "space",
        }
    }
}

/// A statement over the signals of `S` that the browser runs on an event.
pub struct Stmt<S> {
    ast: Value,
    marker: PhantomData<fn() -> S>,
}

impl<S> Stmt<S> {
    pub(crate) fn new(ast: Value) -> Self {
        Self {
            ast,
            marker: PhantomData,
        }
    }

    pub(crate) fn set<T>(place: Value, value: Expr<S, T>) -> Self {
        Self::new(Value::Array(vec![Value::from("set"), place, value.ast]))
    }

    pub(crate) fn list(op: &str, place: Value, operand: Value) -> Self {
        Self::new(Value::Array(vec![Value::from(op), place, operand]))
    }

    /// A statement that does nothing.
    #[must_use]
    pub fn none() -> Self {
        Self::new(json!(["seq", []]))
    }

    /// Runs `then` only while `cond` is true.
    #[must_use]
    pub fn when(cond: impl IntoExpr<S, bool>, then: impl IntoStmt<S>) -> Self {
        Self::if_else(cond, then, Self::none())
    }

    /// Runs `then` while `cond` is true and `otherwise` while it is false.
    #[must_use]
    pub fn if_else(
        cond: impl IntoExpr<S, bool>,
        then: impl IntoStmt<S>,
        otherwise: impl IntoStmt<S>,
    ) -> Self {
        Self::new(json!([
            "if",
            cond.into_expr().ast,
            then.into_stmt().ast,
            otherwise.into_stmt().ast
        ]))
    }

    /// Runs `self`, then `next`.
    #[must_use]
    pub fn then(self, next: impl IntoStmt<S>) -> Self {
        let mut steps = match self.ast {
            Value::Array(mut items) if items.first() == Some(&json!("seq")) => match items.pop() {
                Some(Value::Array(steps)) => steps,
                _ => unreachable!("a seq statement always holds an array of steps"),
            },
            single => vec![single],
        };
        steps.push(next.into_stmt().ast);
        Self::new(json!(["seq", steps]))
    }

    pub(crate) fn into_json(self) -> Value {
        self.ast
    }

    pub(crate) fn as_json(&self) -> &Value {
        &self.ast
    }

    pub(crate) fn to_json(&self) -> String {
        self.ast.to_string()
    }
}

/// Conversion into a [`Stmt`] over `S`.
///
/// Implemented for [`Stmt`] and for [`Call`](crate::Call), so a handler can
/// take either.
#[diagnostic::on_unimplemented(
    message = "`{Self}` is not a statement on the page whose signals are `{S}`",
    note = "a handler runs statements built from the signals, actions, and node refs of its own page"
)]
pub trait IntoStmt<S> {
    /// Converts `self` into a statement.
    fn into_stmt(self) -> Stmt<S>;
}

impl<S> IntoStmt<S> for Stmt<S> {
    fn into_stmt(self) -> Stmt<S> {
        self
    }
}

impl<S> Clone for Stmt<S> {
    fn clone(&self) -> Self {
        Self::new(self.ast.clone())
    }
}

impl<S> fmt::Debug for Stmt<S> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("Stmt").field(&self.ast).finish()
    }
}

#[cfg(test)]
mod tests {
    use serde::{Deserialize, Serialize};

    use crate::_private::sig;
    use crate::{Expr, IntoExpr, Sig, Stmt, if_else};

    struct Page;

    #[derive(Serialize, Deserialize, PartialEq)]
    struct Todo {
        title: String,
        done: bool,
    }

    #[derive(Serialize, PartialEq)]
    enum Filter {
        Active,
    }

    fn count() -> Sig<Page, i32> {
        sig("count")
    }

    fn name() -> Sig<Page, String> {
        sig("name")
    }

    fn todos() -> Sig<Page, Vec<Todo>> {
        sig("todos")
    }

    fn filter() -> Sig<Page, Filter> {
        sig("filter")
    }

    fn done() -> Sig<Todo, bool> {
        sig("done")
    }

    fn title() -> Sig<Todo, String> {
        sig("title")
    }

    #[test]
    fn arithmetic_and_comparison() {
        let e = (count() + 1).gt(count() * 2);
        assert_eq!(
            e.to_json(),
            r#"["gt",["add",["sig","count"],["lit",1]],["mul",["sig","count"],["lit",2]]]"#
        );
    }

    #[test]
    fn string_concat_and_text() {
        let e: Expr<Page, String> = name().concat(" has ").concat(count().to_text());
        assert_eq!(
            e.to_json(),
            r#"["concat",["concat",["sig","name"],["lit"," has "]],["str",["sig","count"]]]"#
        );
    }

    #[test]
    fn then_flattens_into_one_seq() {
        let s = count()
            .set(0)
            .then(name().set(""))
            .then(count().set(count() + 1));
        assert_eq!(
            s.to_json(),
            r#"["seq",[["set","count",["lit",0]],["set","name",["lit",""]],["set","count",["add",["sig","count"],["lit",1]]]]]"#
        );
    }

    #[test]
    fn not_and_or() {
        let e = (!count().gt(0)).or(name().is_empty().into_expr());
        assert_eq!(
            e.to_json(),
            r#"["or",["not",["gt",["sig","count"],["lit",0]]],["eq",["sig","name"],["lit",""]]]"#
        );
    }

    #[test]
    fn enum_literal_and_ternary() {
        let e = if_else(filter().equals(&Filter::Active), "active", "all");
        assert_eq!(
            e.to_json(),
            r#"["if",["eq",["sig","filter"],["lit","Active"]],["lit","active"],["lit","all"]]"#
        );
    }

    #[test]
    fn list_predicates_bind_the_item_depth() {
        let left = todos().count(|todo| !todo.at(done()));
        assert_eq!(
            left.to_json(),
            r#"["count","todos",0,["not",["item",0,"done"]]]"#
        );
        let push = todos().push(
            Expr::lit(&Todo {
                title: String::new(),
                done: false,
            })
            .with(title(), name().trim()),
        );
        assert_eq!(
            push.to_json(),
            r#"["push","todos",["with",["lit",{"done":false,"title":""}],"title",["trim",["sig","name"]]]]"#
        );
    }

    #[derive(crate::Segment, Serialize, Deserialize)]
    #[serde(rename_all = "kebab-case")]
    enum Status {
        InReview,
    }

    #[test]
    fn writes_a_segment_enum_and_an_i64_as_text() {
        let status: Sig<Page, Status> = sig("status");
        let cents: Sig<Page, i64> = sig("cents");
        let e = status.to_text().concat((cents + 1).to_text());
        assert_eq!(
            e.to_json(),
            r#"["concat",["str",["sig","status"]],["str",["add",["sig","cents"],["lit",1]]]]"#
        );
        let literal: Expr<Page, Status> = Expr::lit(&Status::InReview);
        assert_eq!(
            literal.to_text().to_json(),
            r#"["str",["lit","in-review"]]"#
        );
    }

    #[test]
    fn conditional_statement() {
        let s: Stmt<Page> = Stmt::when(count().gt(9), count().set(0));
        assert_eq!(
            s.to_json(),
            r#"["if",["gt",["sig","count"],["lit",9]],["set","count",["lit",0]],["seq",[]]]"#
        );
    }
}
