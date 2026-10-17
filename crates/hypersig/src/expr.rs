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
//! | `a.concat(b)`, `trim`, `to_lowercase`, `to_uppercase` | `["concat",a,b]`, `["trim",a]`, `["lower",a]`, `["upper",a]` |
//! | `a.contains(b)`, `a.starts_with(b)`, `a.char_count()` | `["contains",a,b]`, `["starts",a,b]`, `["chars",a]` |
//! | `list.len()`, `list.nth(i)` | `["len",a]`, `["nth",a,i]` |
//! | `Expr::random_below(n)`, `Expr::now()` | `["random",n]`, `["now"]` |
//! | `list.count(\|item\| p)` | `["count",place,depth,p]` |
//! | `opt.unwrap_or(d)`, `a.some()` | `["or_else",a,d]`, `a` |
//! | `rec.at(field)`, `rec.with(field, v)` | `["get",a,"path"]`, `["with",a,"path",v]` |
//! | `sig.set(e)` | `["set",place,e]` |
//! | `list.push(e)`, `list.remove_at(i)` | `["push",place,e]`, `["remove",place,i]` |
//! | `list.retain(\|item\| p)`, `list.set_all(f, e)` | `["retain",place,depth,p]`, `["set_all",place,"path",e]` |
//! | `list.extend(n, \|i\| e)` | `["extend",place,n,depth,e]` |
//! | `list.update_where(\|item\| p, f, \|item\| e)` | `["update",place,depth,p,"path",e]` |
//! | `list.swap(i, j)`, `list.sort_by(\|item\| k)` | `["swap",place,i,j]`, `["sort",place,depth,k]` |
//! | `Stmt::when(c, s)`, `Stmt::if_else(c, s, t)` | `["if",c,s,t]` |
//! | `route.link_with(p)` for `"/users/{id}"` | `["concat",["lit","/users/"],["uri",p]]` |
//! | `route.link_with((p, q))` with search parameters | `["concat",path,["query",q]]` |
//! | `route.link_with(rest)` for `"/files/{*path}"` | `["concat",["lit","/files/"],["uri_path",rest]]` |
//! | `action.call()` | `["call",method,path,args,indicator,error]` |
//! | `node_ref.focus()` | `["dom","focus",name]` |
//! | `link.navigate()` | `["navigate",url]` |
//! | `s1.then(s2)` | `["seq",[s1,s2]]` |
//!
//! A place is `"dotted.path"` for a page signal and `["item",depth,"path"]`
//! for a field of a list item.

use std::fmt;
use std::marker::PhantomData;
use std::ops;

use serde::Serialize;
use serde_json::{Value, json};

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
    /// `Serialize` impl that returns an error.
    #[must_use]
    pub fn lit(value: &T) -> Self
    where
        T: Serialize,
    {
        let value = serde_json::to_value(value).expect("a literal serializes to JSON");
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
/// `i32`, `f64`, `bool`, `String`, and `&str`, and for a reference to any
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
                Expr::from_json(json!(["lit", self]))
            }
        }
    )*};
}

literal!(i32 => i32, f64 => f64, bool => bool, String => String, &str => String);

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
/// `i32` and `f64` only. JavaScript numbers are 64-bit floats, so integer
/// division and 64-bit integers would come back with a different value. A
/// result outside `i32`'s range fails to deserialize on the next action and
/// the action answers 400.
#[diagnostic::on_unimplemented(
    message = "the browser does not do arithmetic on `{Self}`",
    note = "arithmetic is supported on `i32` and `f64`, and division on `f64` only"
)]
pub trait Number: sealed::Sealed {}
impl Number for i32 {}
impl Number for f64 {}

/// A type the runtime can order with `<` and friends.
///
/// The runtime orders strings by UTF-16 code unit and the server by code point, so
/// two strings holding characters outside the Basic Multilingual Plane can
/// order differently in the first HTML. The runtime's result replaces the
/// server's once the page loads.
pub trait Comparable: sealed::Sealed {}
impl Comparable for i32 {}
impl Comparable for f64 {}
impl Comparable for String {}

/// A type the runtime can turn into display text.
pub trait Textual: sealed::Sealed {}
impl Textual for i32 {}
impl Textual for f64 {}
impl Textual for bool {}

mod sealed {
    pub trait Sealed {}
    impl Sealed for i32 {}
    impl Sealed for f64 {}
    impl Sealed for bool {}
    impl Sealed for String {}
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
        }

        impl<S $(, $r)?> $ty<S, i32 $(, $r)?> {
            /// This integer as an `f64`, which holds every `i32` exactly.
            #[must_use]
            pub fn to_f64(self) -> Expr<S, f64> {
                self.into_expr().retype()
            }
        }

        impl<S $(, $r)?> $ty<S, f64 $(, $r)?> {
            /// This number as text with `digits` digits after the point.
            ///
            /// The runtime uses `Number.prototype.toFixed`, which can round a
            /// halfway value differently from Rust's `format!("{:.n}")`.
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
            /// This value as display text.
            ///
            /// The server writes a number below `1e-6` or at or above `1e21` in
            /// the first HTML differently from JavaScript's `String`. The page
            /// shows the server's text for one frame, until the runtime writes its own.
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

    #[test]
    fn conditional_statement() {
        let s: Stmt<Page> = Stmt::when(count().gt(9), count().set(0));
        assert_eq!(
            s.to_json(),
            r#"["if",["gt",["sig","count"],["lit",9]],["set","count",["lit",0]],["seq",[]]]"#
        );
    }
}
