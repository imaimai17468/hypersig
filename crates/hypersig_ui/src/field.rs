//! Text fields with a label, a description, and an error message.

use hypersig::html::{El, div, input, label, p, span, textarea};
use hypersig::style::pattern::stack;
use hypersig::style::value::{
    BorderStyle, Color, FontWeight, LineHeight, MinWidth, Resize, pct, px, rem,
};
use hypersig::style::{PseudoClass, PseudoElement, Style};
use hypersig::{Expr, IntoExpr, Sig, if_else};

use crate::look;
use crate::token;

/// The `type` of a single-line [`TextField`], which picks the on-screen
/// keyboard and what the browser checks.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum InputKind {
    /// `text`.
    Text,
    /// `email`.
    Email,
    /// `password`.
    Password,
    /// `search`.
    Search,
    /// `tel`.
    Tel,
    /// `url`.
    Url,
}

impl InputKind {
    fn name(self) -> &'static str {
        match self {
            Self::Text => "text",
            Self::Email => "email",
            Self::Password => "password",
            Self::Search => "search",
            Self::Tel => "tel",
            Self::Url => "url",
        }
    }
}

/// What a [`TextField`] holds, as `autocomplete` names it for the browser
/// and for WCAG 1.3.5.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum Autocomplete {
    /// `off`: the browser offers nothing.
    Off,
    /// `name`.
    Name,
    /// `email`.
    Email,
    /// `username`.
    Username,
    /// `current-password`.
    CurrentPassword,
    /// `new-password`.
    NewPassword,
    /// `one-time-code`.
    OneTimeCode,
    /// `organization`.
    Organization,
    /// `street-address`.
    StreetAddress,
    /// `postal-code`.
    PostalCode,
    /// `country-name`.
    Country,
    /// `tel`.
    Tel,
    /// `url`.
    Url,
}

impl Autocomplete {
    fn name(self) -> &'static str {
        match self {
            Self::Off => "off",
            Self::Name => "name",
            Self::Email => "email",
            Self::Username => "username",
            Self::CurrentPassword => "current-password",
            Self::NewPassword => "new-password",
            Self::OneTimeCode => "one-time-code",
            Self::Organization => "organization",
            Self::StreetAddress => "street-address",
            Self::PostalCode => "postal-code",
            Self::Country => "country-name",
            Self::Tel => "tel",
            Self::Url => "url",
        }
    }
}

/// A text field bound to a `String` signal, with its label and, where given,
/// a description and an error message tied to it.
///
/// The ids that tie them together are derived from the signal's path, so a
/// label always names its own field. `aria-describedby` points at the
/// description and the error, and `aria-invalid` is `true` while the error is
/// not empty. The error sits in an `aria-live="polite"` region, so a screen
/// reader reads a new message when it appears.
///
/// Two fields bound to the same signal on one page get the same ids, which
/// nothing checks.
///
/// ```
/// use hypersig::{Signals, if_else};
/// use hypersig_ui::{Autocomplete, InputKind, TextField};
/// use serde::{Deserialize, Serialize};
///
/// #[derive(Signals, Serialize, Deserialize)]
/// struct Signup {
///     email: String,
/// }
///
/// let email = Signup::email();
/// let field = TextField::new("Email", email)
///     .kind(InputKind::Email)
///     .autocomplete(Autocomplete::Email)
///     .description("We send the receipt here.")
///     .error(if_else(email.contains("@").or(email.is_empty()), "", "Enter an email address."))
///     .required();
/// let html = field.render().render();
/// assert!(html.contains(r#"for="ui-email-input""#));
/// assert!(html.contains(r#"aria-describedby="ui-email-description ui-email-error""#));
/// ```
#[derive(Debug)]
pub struct TextField<S> {
    label: String,
    value: Sig<S, String>,
    kind: InputKind,
    multiline: bool,
    autocomplete: Option<Autocomplete>,
    placeholder: Option<String>,
    description: Option<String>,
    error: Option<Expr<S, String>>,
    required: bool,
    disabled: Option<Expr<S, bool>>,
}

impl<S> TextField<S> {
    /// A single-line text field labeled `label`, bound to `value`.
    #[must_use]
    pub fn new(label: impl Into<String>, value: Sig<S, String>) -> Self {
        Self {
            label: label.into(),
            value,
            kind: InputKind::Text,
            multiline: false,
            autocomplete: None,
            placeholder: None,
            description: None,
            error: None,
            required: false,
            disabled: None,
        }
    }

    /// Sets the `type` of the input. A multiline field ignores it.
    #[must_use]
    pub fn kind(mut self, kind: InputKind) -> Self {
        self.kind = kind;
        self
    }

    /// Makes the field a `<textarea>` that the visitor can resize vertically.
    #[must_use]
    pub fn multiline(mut self) -> Self {
        self.multiline = true;
        self
    }

    /// Tells the browser what the field holds, so that it can fill it in.
    #[must_use]
    pub fn autocomplete(mut self, autocomplete: Autocomplete) -> Self {
        self.autocomplete = Some(autocomplete);
        self
    }

    /// Shows `text` in the empty field.
    ///
    /// A placeholder disappears as the visitor types, so the label and the
    /// description carry what the visitor needs to keep.
    #[must_use]
    pub fn placeholder(mut self, text: impl Into<String>) -> Self {
        self.placeholder = Some(text.into());
        self
    }

    /// Adds a line under the field that `aria-describedby` points at.
    #[must_use]
    pub fn description(mut self, text: impl Into<String>) -> Self {
        self.description = Some(text.into());
        self
    }

    /// Shows `message` under the field and marks the field invalid while
    /// `message` is not empty.
    ///
    /// A signal that an action call fills with
    /// [`Call::error`](hypersig::Call::error) works here, as does an
    /// expression that checks the value in the browser.
    #[must_use]
    pub fn error(mut self, message: impl IntoExpr<S, String>) -> Self {
        self.error = Some(message.into_expr());
        self
    }

    /// Marks the field `required`, which the label shows with an asterisk
    /// that a screen reader skips, since it reads "required" from the field.
    #[must_use]
    pub fn required(mut self) -> Self {
        self.required = true;
        self
    }

    /// Disables the field while `when` is true.
    #[must_use]
    pub fn disabled(mut self, when: impl IntoExpr<S, bool>) -> Self {
        self.disabled = Some(when.into_expr());
        self
    }

    /// The label, the field, the description, and the error, in a `<div>`.
    #[must_use]
    pub fn render(self) -> El<S> {
        let base = crate::id::of(self.value, "");
        let field_id = format!("{base}input");
        let description_id = format!("{base}description");
        let error_id = format!("{base}error");

        let mut described = Vec::new();
        if self.description.is_some() {
            described.push(description_id.as_str());
        }
        if self.error.is_some() {
            described.push(error_id.as_str());
        }

        let mut control = if self.multiline {
            textarea().css(field_style().min_height(rem(4)).resize(Resize::Vertical))
        } else {
            input()
                .attr("type", self.kind.name())
                .css(field_style().height(rem(2.25)))
        }
        .id(field_id.clone())
        .bind(self.value);
        if !described.is_empty() {
            control = control.attr("aria-describedby", described.join(" "));
        }
        if let Some(autocomplete) = self.autocomplete {
            control = control.attr("autocomplete", autocomplete.name());
        }
        if let Some(placeholder) = self.placeholder {
            control = control.attr("placeholder", placeholder);
        }
        if self.required {
            control = control.attr("required", "");
        }
        if let Some(disabled) = self.disabled {
            control = control.attr_when("disabled", disabled);
        }
        if let Some(error) = &self.error {
            let invalid = !error.clone().is_empty();
            control = control
                .bind_attr("aria-invalid", if_else(invalid.clone(), "true", "false"))
                .css_when(
                    Style::new().border_color(token::destructive().var()),
                    invalid,
                );
        }

        let mut caption = label()
            .attr("for", field_id)
            .css(label_style())
            .text(self.label);
        if self.required {
            caption = caption.child(
                span()
                    .attr("aria-hidden", "true")
                    .css(Style::new().color(token::destructive().var()))
                    .text(" *"),
            );
        }

        div()
            .css(stack(rem(0.5)))
            .child(caption)
            .child(control)
            .children(self.description.map(|text| {
                p().id(description_id)
                    .css(look::small_text().color(token::muted_foreground().var()))
                    .text(text)
            }))
            .children(self.error.map(|error| {
                p().id(error_id)
                    .attr("aria-live", "polite")
                    .css(look::small_text().color(token::destructive().var()))
                    .bind_text(error)
            }))
    }
}

impl<S> From<TextField<S>> for El<S> {
    fn from(field: TextField<S>) -> Self {
        field.render()
    }
}

impl<S> From<TextField<S>> for hypersig::html::Node<S> {
    fn from(field: TextField<S>) -> Self {
        field.render().into()
    }
}

/// The style of a label above a control.
pub(crate) fn label_style() -> Style {
    look::small_text().font_weight(FontWeight::Number(500.0))
}

/// The box of a text field or a select.
pub(crate) fn field_style() -> Style {
    Style::new()
        .width(pct(100))
        .min_width(MinWidth::from(px(0)))
        .padding_block(rem(0.25))
        .padding_inline(rem(0.75))
        .border_width(px(1))
        .border_style(BorderStyle::Solid)
        .border_color(token::input().var())
        .border_radius(px(6))
        .background_color(Color::TRANSPARENT)
        .color(token::foreground().var())
        .font_size(rem(0.875))
        .line_height(LineHeight::Length(rem(1.25)))
        .box_shadow(hypersig::style::value::raw("0 1px 2px 0 rgb(0 0 0 / 0.05)"))
        .pseudo(
            PseudoElement::Placeholder,
            Style::new().color(token::muted_foreground().var()),
        )
        .merge(look::transition())
        .on(PseudoClass::FocusVisible, look::ring(px(0)))
        .merge(look::disabled())
}
