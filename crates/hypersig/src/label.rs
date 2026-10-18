//! The display text of an enum's variant, from a mapping the compiler checks covers every variant.

use serde_json::{Value, json};

use crate::expr::{Expr, IntoExpr};
use crate::route::Variants;
use crate::signal::Sig;

/// The chain `label` builds: for each variant but the last, whether `value`
/// equals it, and the last variant's text where none does.
fn label_chain<S, T: Variants, L: IntoExpr<S, String>>(
    value: &Value,
    label: impl Fn(&T) -> L,
) -> Value {
    let (last, rest) = T::VARIANTS
        .split_last()
        .expect("`#[derive(Segment)]` rejects an enum without variants");
    rest.iter()
        .rev()
        .fold(label(last).into_expr().into_json(), |otherwise, variant| {
            json!([
                "if",
                ["eq", value, ["lit", variant.write()]],
                label(variant).into_expr().into_json(),
                otherwise
            ])
        })
}

/// Defines `label` on both `Expr` and `Sig`.
macro_rules! label_operations {
    ($ty:ident $(, $r:ident)?) => {
        impl<S, T: Variants $(, $r)?> $ty<S, T $(, $r)?> {
            /// The text `label` gives for this value's variant, with every variant covered.
            ///
            /// `label` is called once per variant when the page is built,
            /// so a `match` in it names a text for each variant, and leaving
            /// one out is a compile error. The browser and the first HTML
            /// pick the text of the variant the value holds.
            ///
            /// ```
            /// use hypersig::{Segment, Signals};
            /// use serde::{Deserialize, Serialize};
            ///
            /// #[derive(Segment, Serialize, Deserialize, PartialEq)]
            /// enum Status {
            ///     Open,
            ///     InReview,
            ///     Closed,
            /// }
            ///
            /// #[derive(Signals, Serialize, Deserialize)]
            /// struct Ticket {
            ///     status: Status,
            /// }
            ///
            /// let _text = Ticket::status().label(|status| match status {
            ///     Status::Open => "Open",
            ///     Status::InReview => "In review",
            ///     Status::Closed => "Closed",
            /// });
            /// ```
            ///
            /// # Panics
            ///
            /// Panics if `T` lists no variants, which `#[derive(Segment)]`
            /// never writes.
            #[must_use]
            pub fn label<L: IntoExpr<S, String>>(self, label: impl Fn(&T) -> L) -> Expr<S, String> {
                let value = self.into_expr();
                match value.as_json().get(0).and_then(Value::as_str) {
                    // Reading a signal again costs no more than reading a name.
                    Some("sig" | "item" | "lit" | "var") => {
                        Expr::from_json(label_chain::<S, T, L>(value.as_json(), label))
                    }
                    _ => value.let_in(|value| {
                        Expr::from_json(label_chain::<S, T, L>(value.as_json(), label))
                    }),
                }
            }
        }
    };
}

label_operations!(Expr);
label_operations!(Sig, R);

#[cfg(test)]
mod tests {
    use serde::{Deserialize, Serialize};
    use serde_json::json;

    use crate::_private::sig;
    use crate::ssr::{Ctx, evaluate};
    use crate::{Expr, Sig, if_else};

    struct Page;

    #[derive(crate::Segment, Serialize, Deserialize, PartialEq)]
    #[serde(rename_all = "kebab-case")]
    enum Status {
        Open,
        InReview,
        Closed,
    }

    fn status() -> Sig<Page, Status> {
        sig("status")
    }

    #[test]
    fn picks_the_text_of_each_variant_on_the_server() {
        let label = status().label(|status| match status {
            Status::Open => "Open",
            Status::InReview => "In review",
            Status::Closed => "Closed",
        });
        assert_eq!(
            label.as_json(),
            &json!([
                "if",
                ["eq", ["sig", "status"], ["lit", "open"]],
                ["lit", "Open"],
                [
                    "if",
                    ["eq", ["sig", "status"], ["lit", "in-review"]],
                    ["lit", "In review"],
                    ["lit", "Closed"]
                ]
            ])
        );
        for (value, shown) in [
            ("open", "Open"),
            ("in-review", "In review"),
            ("closed", "Closed"),
        ] {
            let root = json!({ "status": value });
            let ctx = Ctx {
                root: &root,
                scope: Vec::new(),
                copy: false,
                vars: Vec::new(),
                rules: None,
            };
            let text = evaluate(label.as_json(), &ctx).expect("evaluates");
            assert_eq!(*text, json!(shown));
        }
    }

    #[test]
    fn names_a_computed_value_once() {
        let flagged: Sig<Page, bool> = sig("flagged");
        let value: Expr<Page, Status> = if_else(flagged, &Status::Closed, status());
        let label = value.label(|status| {
            if *status == Status::Open {
                "Open"
            } else {
                "Not open"
            }
        });
        assert_eq!(label.as_json()[0], json!("let"));
    }
}
