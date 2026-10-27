//! Date fields with a label, a description, and an error message.

use hypersig::html::{Bindable, El, attr, input};
use hypersig::style::value::rem;
use hypersig::{CalendarDate, Form, IntoExpr, Sig, Validate};

use crate::field::{Frame, field_style};

/// A date type a [`DateField`] can be bound to: `CalendarDate` or `Option<CalendarDate>`.
///
/// An `Option` field starts empty for `None`, and emptying it sets `None`.
/// A plain `CalendarDate` keeps its date when the visitor empties the field.
#[diagnostic::on_unimplemented(
    message = "a date field cannot be bound to a `{Self}` signal",
    note = "bind a `CalendarDate` or `Option<CalendarDate>` signal; text goes in a `TextField`"
)]
pub trait Dated: Bindable + sealed::Sealed {}

impl Dated for CalendarDate {}
impl Dated for Option<CalendarDate> {}

mod sealed {
    use hypersig::CalendarDate;

    pub trait Sealed {}
    impl Sealed for CalendarDate {}
    impl Sealed for Option<CalendarDate> {}
}

/// A date field bound to a `CalendarDate` or `Option<CalendarDate>` signal, with its label.
///
/// Where given, a description and an error message are tied to it.
///
/// It is an `<input type="date">` with the ids, `aria-describedby`, and
/// `aria-invalid` of a [`TextField`](crate::TextField). The browser draws
/// its own date picker. `min` and
/// `max` take a `CalendarDate`, so a bound cannot be text.
///
/// ```
/// use hypersig::{CalendarDate, Signals};
/// use hypersig_ui::DateField;
/// use serde::{Deserialize, Serialize};
///
/// #[derive(Signals, Serialize, Deserialize)]
/// struct Booking {
///     arrival: Option<CalendarDate>,
/// }
///
/// let arrival = DateField::new("Arrival", Booking::arrival())
///     .min(CalendarDate::new(2026, 1, 1))
///     .max(CalendarDate::new(2026, 12, 31));
/// let html = arrival.render().render();
/// assert!(html.contains(r#"type="date""#));
/// assert!(html.contains(r#"min="2026-01-01""#));
/// assert!(html.contains(r#"for="ui-arrival-input""#));
/// ```
#[derive(Debug)]
pub struct DateField<S, T> {
    value: Sig<S, T>,
    min: Option<CalendarDate>,
    max: Option<CalendarDate>,
    frame: Frame<S>,
}

impl<S, T: Dated> DateField<S, T> {
    /// A date field labeled `label`, bound to `value`.
    #[must_use]
    pub fn new(label: impl Into<String>, value: Sig<S, T>) -> Self {
        Self {
            value,
            min: None,
            max: None,
            frame: Frame::new(label.into()),
        }
    }

    /// A date field labeled `label`, bound to `field` of `form`.
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

    /// Sets the earliest date the browser accepts.
    ///
    /// # Panics
    ///
    /// Panics if `min` is after a maximum set before.
    #[must_use]
    pub fn min(mut self, min: CalendarDate) -> Self {
        if let Some(max) = self.max {
            assert!(
                min <= max,
                "a date field's earliest date is not after its latest, but {min} is after {max}"
            );
        }
        self.min = Some(min);
        self
    }

    /// Sets the latest date the browser accepts.
    ///
    /// # Panics
    ///
    /// Panics if `max` is before a minimum set before.
    #[must_use]
    pub fn max(mut self, max: CalendarDate) -> Self {
        if let Some(min) = self.min {
            assert!(
                min <= max,
                "a date field's latest date is not before its earliest, but {max} is before {min}"
            );
        }
        self.max = Some(max);
        self
    }

    /// Adds a line under the field that `aria-describedby` points at.
    #[must_use]
    pub fn description(mut self, text: impl Into<String>) -> Self {
        self.frame.description = Some(text.into());
        self
    }

    /// Shows `message` under the field.
    ///
    /// It marks the field invalid while `message` is not empty.
    #[must_use]
    pub fn error(mut self, message: impl IntoExpr<S, String>) -> Self {
        self.frame.error = Some(message.into_expr());
        self
    }

    /// Marks the field `required`.
    ///
    /// The label shows it with an asterisk that a screen reader skips.
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
            .attr(attr::TYPE, "date")
            .css(field_style().height(rem(2.25)))
            .bind(self.value);
        if let Some(min) = self.min {
            control = control.attr(attr::MIN, min.to_string());
        }
        if let Some(max) = self.max {
            control = control.attr(attr::MAX, max.to_string());
        }
        self.frame.render(&crate::id::base(self.value), control)
    }
}

impl<S, T: Dated> From<DateField<S, T>> for El<S> {
    fn from(field: DateField<S, T>) -> Self {
        field.render()
    }
}

impl<S, T: Dated> From<DateField<S, T>> for hypersig::html::Node<S> {
    fn from(field: DateField<S, T>) -> Self {
        field.render().into()
    }
}
