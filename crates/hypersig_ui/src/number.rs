//! Number fields with a label, a description, and an error message.

use std::fmt;

use hypersig::html::{Bindable, El, attr, input};
use hypersig::style::value::rem;
use hypersig::{Form, IntoExpr, Sig, Validate};

use crate::field::{Frame, field_style};

/// A number type a [`NumberField`] can be bound to: `i32` or `f64`.
///
/// The browser's number input and the runtime's binding read these two, so
/// the trait is sealed.
#[diagnostic::on_unimplemented(
    message = "a number field cannot be bound to a `{Self}` signal",
    note = "bind an `i32` or `f64` signal; text goes in a `TextField`"
)]
pub trait Numeric: Bindable + Copy + PartialOrd + fmt::Display + sealed::Sealed {}

impl Numeric for i32 {}
impl Numeric for f64 {}

mod sealed {
    pub trait Sealed {
        /// Zero, which a step must exceed.
        const ZERO: Self;
        /// The `step` a field without one gets, where the browser's own
        /// default of 1 would reject a value the type holds.
        const DEFAULT_STEP: Option<&'static str>;
        /// The `inputmode` of a field that takes no negative value.
        const NON_NEGATIVE_MODE: &'static str;
        /// Whether the value is a number, which an `f64` NaN is not.
        fn is_number(&self) -> bool;
    }

    impl Sealed for i32 {
        const ZERO: Self = 0;
        const DEFAULT_STEP: Option<&'static str> = None;
        const NON_NEGATIVE_MODE: &'static str = "numeric";
        fn is_number(&self) -> bool {
            true
        }
    }

    impl Sealed for f64 {
        const ZERO: Self = 0.0;
        const DEFAULT_STEP: Option<&'static str> = Some("any");
        const NON_NEGATIVE_MODE: &'static str = "decimal";
        fn is_number(&self) -> bool {
            !self.is_nan()
        }
    }
}

/// A number field bound to an `i32` or `f64` signal, with its label and,
/// where given, a description and an error message tied to it.
///
/// It is an `<input type="number">` with the ids, `aria-describedby`, and
/// `aria-invalid` of a [`TextField`](crate::TextField). `min`, `max`, and
/// `step` take the field's own type, so a bound cannot be text. A field
/// whose `min` is 0 or more asks for a keyboard of digits, which on a phone
/// has no minus sign. An `f64` field without a step takes any decimal.
///
/// ```
/// use hypersig::Signals;
/// use hypersig_ui::NumberField;
/// use serde::{Deserialize, Serialize};
///
/// #[derive(Signals, Serialize, Deserialize)]
/// struct Cart {
///     quantity: i32,
/// }
///
/// let quantity = NumberField::new("Quantity", Cart::quantity()).min(1).max(10);
/// let html = quantity.render().render();
/// assert!(html.contains(r#"type="number""#));
/// assert!(html.contains(r#"min="1""#));
/// assert!(html.contains(r#"inputmode="numeric""#));
/// assert!(html.contains(r#"for="ui-quantity-input""#));
/// ```
#[derive(Debug)]
pub struct NumberField<S, T> {
    value: Sig<S, T>,
    min: Option<T>,
    max: Option<T>,
    step: Option<T>,
    frame: Frame<S>,
}

impl<S, T: Numeric> NumberField<S, T> {
    /// A number field labeled `label`, bound to `value`.
    #[must_use]
    pub fn new(label: impl Into<String>, value: Sig<S, T>) -> Self {
        Self {
            value,
            min: None,
            max: None,
            step: None,
            frame: Frame::new(label.into()),
        }
    }

    /// A number field labeled `label`, bound to `field` of `form`.
    ///
    /// It shows [`error`](Sig::error) of the form for `field` and marks the
    /// field touched when the visitor leaves it, as
    /// [`TextField::in_form`](crate::TextField::in_form) does.
    #[must_use]
    pub fn in_form<V: Validate>(
        label: impl Into<String>,
        form: Sig<S, Form<V>>,
        field: Sig<V, T>,
    ) -> Self {
        let mut out = Self::new(label, form.field(field));
        out.frame = out.frame.in_form(form, field);
        out
    }

    /// Sets the lowest value the browser accepts.
    ///
    /// # Panics
    ///
    /// Panics if `min` is above a maximum set before, or is not a number.
    #[must_use]
    pub fn min(mut self, min: T) -> Self {
        assert!(
            min.is_number(),
            "a number field's minimum is a number, not {min}"
        );
        if let Some(max) = self.max {
            assert!(
                min <= max,
                "a number field's minimum is not above its maximum, but {min} is above {max}"
            );
        }
        self.min = Some(min);
        self
    }

    /// Sets the highest value the browser accepts.
    ///
    /// # Panics
    ///
    /// Panics if `max` is below a minimum set before, or is not a number.
    #[must_use]
    pub fn max(mut self, max: T) -> Self {
        assert!(
            max.is_number(),
            "a number field's maximum is a number, not {max}"
        );
        if let Some(min) = self.min {
            assert!(
                min <= max,
                "a number field's maximum is not below its minimum, but {max} is below {min}"
            );
        }
        self.max = Some(max);
        self
    }

    /// Sets how far the arrow keys move the value, and which values the
    /// browser accepts: the minimum plus a whole number of steps.
    ///
    /// # Panics
    ///
    /// Panics if `step` is not above zero.
    #[must_use]
    pub fn step(mut self, step: T) -> Self {
        assert!(
            step > T::ZERO,
            "a number field's step is above zero, not {step}"
        );
        self.step = Some(step);
        self
    }

    /// Adds a line under the field that `aria-describedby` points at.
    #[must_use]
    pub fn description(mut self, text: impl Into<String>) -> Self {
        self.frame.description = Some(text.into());
        self
    }

    /// Shows `message` under the field and marks the field invalid while
    /// `message` is not empty.
    #[must_use]
    pub fn error(mut self, message: impl IntoExpr<S, String>) -> Self {
        self.frame.error = Some(message.into_expr());
        self
    }

    /// Marks the field `required`, which the label shows with an asterisk
    /// that a screen reader skips.
    #[must_use]
    pub fn required(mut self) -> Self {
        self.frame.required = true;
        self
    }

    /// Disables the field while `when` is true.
    #[must_use]
    pub fn disabled(mut self, when: impl IntoExpr<S, bool>) -> Self {
        self.frame.disabled = Some(when.into_expr());
        self
    }

    /// The label, the field, the description, and the error, in a `<div>`.
    #[must_use]
    pub fn render(self) -> El<S> {
        let mut control = input()
            .attr(attr::TYPE, "number")
            .css(field_style().height(rem(2.25)))
            .bind(self.value);
        if self.min.is_some_and(|min| min >= T::ZERO) {
            control = control.attr(attr::INPUTMODE, T::NON_NEGATIVE_MODE);
        }
        if let Some(min) = self.min {
            control = control.attr(attr::MIN, min.to_string());
        }
        if let Some(max) = self.max {
            control = control.attr(attr::MAX, max.to_string());
        }
        match (self.step, T::DEFAULT_STEP) {
            (Some(step), _) => control = control.attr(attr::STEP, step.to_string()),
            (None, Some(step)) => control = control.attr(attr::STEP, step),
            (None, None) => {}
        }
        self.frame.render(&crate::id::base(self.value), control)
    }
}

impl<S, T: Numeric> From<NumberField<S, T>> for El<S> {
    fn from(field: NumberField<S, T>) -> Self {
        field.render()
    }
}

impl<S, T: Numeric> From<NumberField<S, T>> for hypersig::html::Node<S> {
    fn from(field: NumberField<S, T>) -> Self {
        field.render().into()
    }
}
