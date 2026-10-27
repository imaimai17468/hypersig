//! Text that depends on a locale, which the browser writes with `Intl` and
//! the server writes the same way for the first HTML.

use serde_json::{Value, json};

use crate::expr::{Expr, IntoExpr, Number};
use crate::signal::{Scopes, Sig};

/// A locale that the browser's `Intl` writes text for, and the first HTML matches.
///
/// The server writes the same text for each of these locales, from rules
/// this crate holds, so a page shows no change when the runtime starts.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum IntlLocale {
    /// English, `en`.
    En,
    /// Japanese, `ja`.
    Ja,
}

impl IntlLocale {
    /// The BCP 47 tag the browser's `Intl` receives, such as `"en"`.
    #[must_use]
    pub const fn tag(self) -> &'static str {
        match self {
            Self::En => "en",
            Self::Ja => "ja",
        }
    }
}

/// A plural category of the Unicode CLDR, which `Intl.PluralRules` picks for a number.
///
/// English uses [`One`](Self::One) for 1 and -1 and [`Other`](Self::Other)
/// for every other number, and Japanese uses `Other` for every number. The
/// other categories serve locales this crate does not have yet.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PluralCategory {
    /// `zero`.
    Zero,
    /// `one`.
    One,
    /// `two`.
    Two,
    /// `few`.
    Few,
    /// `many`.
    Many,
    /// `other`.
    Other,
}

impl PluralCategory {
    /// Every category, in the order the wire format lists their forms.
    const ALL: [Self; 6] = [
        Self::Zero,
        Self::One,
        Self::Two,
        Self::Few,
        Self::Many,
        Self::Other,
    ];
}

/// The forms `form` gives for each category, in the order of [`PluralCategory::ALL`].
fn plural_forms<S, U, L: IntoExpr<S, U>>(
    form: impl Fn(PluralCategory) -> L,
    scopes: &mut Scopes,
) -> Value {
    Value::Array(
        PluralCategory::ALL
            .iter()
            .map(|category| form(*category).into_expr().take(scopes))
            .collect(),
    )
}

/// Defines the locale-aware methods on both `Expr` and `Sig`.
macro_rules! intl_operations {
    ($ty:ident $(, $r:ident)?) => {
        impl<S, T: Number $(, $r)?> $ty<S, T $(, $r)?> {
            /// The form that `form` gives for the plural category of this number in `locale`.
            ///
            /// `form` is called once per [`PluralCategory`], so a `match`
            /// on the category names a form for each one, and only the form
            /// for the number's category is evaluated.
            ///
            /// ```
            /// use hypersig::{IntlLocale, PluralCategory, Signals};
            /// use serde::{Deserialize, Serialize};
            ///
            /// #[derive(Signals, Serialize, Deserialize)]
            /// struct Cart {
            ///     count: i32,
            /// }
            ///
            /// let count = Cart::count();
            /// let _label = count.to_text().concat(count.plural(IntlLocale::En, |category| match category {
            ///     PluralCategory::One => " item",
            ///     _ => " items",
            /// }));
            /// ```
            #[must_use]
            pub fn plural<U, L: IntoExpr<S, U>>(
                &self,
                locale: IntlLocale,
                form: impl Fn(PluralCategory) -> L,
            ) -> Expr<S, U> {
                Expr::build(|scopes| json!([
                    "plural",
                    self.operand().take(scopes),
                    locale.tag(),
                    plural_forms(form, scopes)
                ]))
            }
        }
    };
}

intl_operations!(Expr);
intl_operations!(Sig, R);

/// The index of the plural category of `value` in `locale`, as `Intl.PluralRules.select` picks it.
#[expect(
    clippy::float_cmp,
    reason = "CLDR's English `one` holds for exactly 1, with no fraction"
)]
pub(crate) fn plural_index(locale: &str, value: f64) -> Option<usize> {
    match locale {
        "en" if value.abs() == 1.0 => Some(1),
        "en" | "ja" => Some(5),
        _ => None,
    }
}
