//! Typed expressions and statements that the browser runtime evaluates.
//!
//! An [`Expr`] or [`Stmt`] is serialized into an HTML attribute as a JSON
//! array whose first element names the operation. The runtime in
//! `runtime/src/eval.ts` interprets that array directly, with no `eval`:
//!
//! | Rust | JSON |
//! |---|---|
//! | `Counter::count()` | `["sig","count"]` |
//! | literal `1` | `["lit",1]` |
//! | `a + b` (numbers) | `["add",a,b]` |
//! | `a + b` (strings) | `["concat",a,b]` |
//! | `a - b`, `a * b`, `a / b` | `["sub",a,b]`, `["mul",a,b]`, `["div",a,b]` |
//! | `a.equals(b)`, `a.not_equals(b)` | `["eq",a,b]`, `["ne",a,b]` |
//! | `a.gt(b)`, `ge`, `lt`, `le` | `["gt",a,b]`, ... |
//! | `a.and(b)`, `a.or(b)`, `!a` | `["and",a,b]`, `["or",a,b]`, `["not",a]` |
//! | `a.to_text()` | `["str",a]` |
//! | `sig.set(e)` | `["set","path",e]` |
//! | `action.call()` | `["call","POST","/path",args]` |
//! | `s1.then(s2)` | `["seq",[s1,s2]]` |

use std::fmt;
use std::marker::PhantomData;
use std::ops;

use serde_json::{Value, json};

use crate::signal::Sig;

/// An expression over the signals of `S` that evaluates to a `T` in the browser.
pub struct Expr<S, T> {
    ast: Value,
    marker: PhantomData<fn() -> (S, T)>,
}

impl<S, T> Expr<S, T> {
    fn new(ast: Value) -> Self {
        Self {
            ast,
            marker: PhantomData,
        }
    }

    pub(crate) fn sig(path: &'static str) -> Self {
        Self::new(json!(["sig", path]))
    }

    fn binary<U>(op: &str, lhs: Self, rhs: Self) -> Expr<S, U> {
        Expr::new(Value::Array(vec![Value::from(op), lhs.ast, rhs.ast]))
    }

    fn unary<U>(op: &str, operand: Self) -> Expr<S, U> {
        Expr::new(Value::Array(vec![Value::from(op), operand.ast]))
    }

    pub(crate) fn to_json(&self) -> String {
        self.ast.to_string()
    }
}

impl<S, T> Clone for Expr<S, T> {
    fn clone(&self) -> Self {
        Self::new(self.ast.clone())
    }
}

impl<S, T> fmt::Debug for Expr<S, T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("Expr").field(&self.ast).finish()
    }
}

/// Conversion into an [`Expr`] over `S` of type `T`.
///
/// Implemented for [`Expr`] itself, for [`Sig`], and for literals of the
/// supported primitive types, so an operand can be any of them.
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

impl<S, T> IntoExpr<S, T> for Sig<S, T> {
    fn into_expr(self) -> Expr<S, T> {
        self.expr()
    }
}

macro_rules! literal {
    ($($ty:ty => $target:ty),* $(,)?) => {$(
        impl<S> IntoExpr<S, $target> for $ty {
            fn into_expr(self) -> Expr<S, $target> {
                Expr::new(json!(["lit", self]))
            }
        }
    )*};
}

literal!(i32 => i32, f64 => f64, bool => bool, String => String, &str => String);

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

/// A type the runtime can compare with `===`, `<`, and friends.
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

/// Defines the operator impls and comparison methods on both `Expr` and
/// `Sig`, so a signal can be used as an operand without converting it first.
macro_rules! operations {
    ($ty:ident) => {
        impl<S, T: Number, R: IntoExpr<S, T>> ops::Add<R> for $ty<S, T> {
            type Output = Expr<S, T>;
            fn add(self, rhs: R) -> Expr<S, T> {
                Expr::binary("add", self.into_expr(), rhs.into_expr())
            }
        }

        impl<S, T: Number, R: IntoExpr<S, T>> ops::Sub<R> for $ty<S, T> {
            type Output = Expr<S, T>;
            fn sub(self, rhs: R) -> Expr<S, T> {
                Expr::binary("sub", self.into_expr(), rhs.into_expr())
            }
        }

        impl<S, T: Number, R: IntoExpr<S, T>> ops::Mul<R> for $ty<S, T> {
            type Output = Expr<S, T>;
            fn mul(self, rhs: R) -> Expr<S, T> {
                Expr::binary("mul", self.into_expr(), rhs.into_expr())
            }
        }

        impl<S, R: IntoExpr<S, f64>> ops::Div<R> for $ty<S, f64> {
            type Output = Expr<S, f64>;
            fn div(self, rhs: R) -> Expr<S, f64> {
                Expr::binary("div", self.into_expr(), rhs.into_expr())
            }
        }

        impl<S> ops::Not for $ty<S, bool> {
            type Output = Expr<S, bool>;
            fn not(self) -> Expr<S, bool> {
                Expr::unary("not", self.into_expr())
            }
        }

        impl<S> $ty<S, String> {
            /// Appends `rhs` to this string.
            #[must_use]
            pub fn concat(self, rhs: impl IntoExpr<S, String>) -> Expr<S, String> {
                Expr::binary("concat", self.into_expr(), rhs.into_expr())
            }

            /// Whether this string is empty.
            #[must_use]
            pub fn is_empty(self) -> Expr<S, bool> {
                self.equals("")
            }
        }

        impl<S, T: Comparable> $ty<S, T> {
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

        impl<S, T: Comparable> $ty<S, T> {
            /// Whether this value equals `rhs`.
            #[must_use]
            pub fn equals(self, rhs: impl IntoExpr<S, T>) -> Expr<S, bool> {
                Expr::binary("eq", self.into_expr(), rhs.into_expr())
            }

            /// Whether this value differs from `rhs`.
            #[must_use]
            pub fn not_equals(self, rhs: impl IntoExpr<S, T>) -> Expr<S, bool> {
                Expr::binary("ne", self.into_expr(), rhs.into_expr())
            }
        }

        impl<S> $ty<S, bool> {
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

        impl<S, T: Textual> $ty<S, T> {
            /// This value as display text.
            #[must_use]
            pub fn to_text(self) -> Expr<S, String> {
                Expr::unary("str", self.into_expr())
            }
        }
    };
}

operations!(Expr);
operations!(Sig);

/// A statement over the signals of `S` that the browser runs on an event.
pub struct Stmt<S> {
    ast: Value,
    marker: PhantomData<fn() -> S>,
}

impl<S> Stmt<S> {
    fn new(ast: Value) -> Self {
        Self {
            ast,
            marker: PhantomData,
        }
    }

    pub(crate) fn set<T>(path: &'static str, value: Expr<S, T>) -> Self {
        Self::new(Value::Array(vec![
            Value::from("set"),
            Value::from(path),
            value.ast,
        ]))
    }

    pub(crate) fn call(method: &str, path: &str, args: Value) -> Self {
        Self::new(Value::Array(vec![
            Value::from("call"),
            Value::from(method),
            Value::from(path),
            args,
        ]))
    }

    /// Runs `self`, then `next`.
    #[must_use]
    pub fn then(self, next: Self) -> Self {
        let mut steps = match self.ast {
            Value::Array(mut items) if items.first() == Some(&json!("seq")) => match items.pop() {
                Some(Value::Array(steps)) => steps,
                _ => unreachable!("a seq statement always holds an array of steps"),
            },
            single => vec![single],
        };
        steps.push(next.ast);
        Self::new(json!(["seq", steps]))
    }

    pub(crate) fn to_json(&self) -> String {
        self.ast.to_string()
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
    use crate::_private::sig;
    use crate::{Expr, IntoExpr, Sig};

    struct Page;

    fn count() -> Sig<Page, i32> {
        sig("count")
    }

    fn name() -> Sig<Page, String> {
        sig("name")
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
}
