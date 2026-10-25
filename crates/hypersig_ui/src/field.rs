//! Text fields with a label, a description, and an error message.

use hypersig::html::{El, Id, attr, div, input, label, p, span, textarea};
use hypersig::style::pattern::stack;
use hypersig::style::value::{
    BorderStyle, Color, FontWeight, LineHeight, MinWidth, Resize, pct, px, rem,
};
use hypersig::style::{PseudoClass, PseudoElement, Style};
use hypersig::{Expr, Form, IntoExpr, Sig, Stmt, Validate, if_else};

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

/// The on-screen keyboard a [`TextField`] asks for, as `inputmode` names it.
///
/// It changes only the keyboard. What the browser accepts and checks stays
/// with the field's [`InputKind`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum InputMode {
    /// `none`: no on-screen keyboard, for a field with its own picker.
    None,
    /// `text`: the keyboard of the visitor's language.
    Text,
    /// `decimal`: digits and the decimal separator.
    Decimal,
    /// `numeric`: digits only, for a code or a card number.
    Numeric,
    /// `tel`: digits, `*`, and `#`.
    Tel,
    /// `search`: a keyboard whose enter key reads "Search".
    Search,
    /// `email`: a keyboard with `@`.
    Email,
    /// `url`: a keyboard with `/` and `.`.
    Url,
}

impl InputMode {
    fn name(self) -> &'static str {
        match self {
            Self::None => "none",
            Self::Text => "text",
            Self::Decimal => "decimal",
            Self::Numeric => "numeric",
            Self::Tel => "tel",
            Self::Search => "search",
            Self::Email => "email",
            Self::Url => "url",
        }
    }
}

/// What a [`TextField`] holds, as `autocomplete` names it for the browser
/// and for WCAG 1.3.5.
///
/// The variants are the WHATWG autofill field names a sign-up form and a
/// checkout ask for. A form that asks for a shipping and a billing address
/// on one page has no way here to mark which is which.
///
/// ```
/// use hypersig::Signals;
/// use hypersig_ui::{Autocomplete, TextField};
/// use serde::{Deserialize, Serialize};
///
/// #[derive(Signals, Serialize, Deserialize)]
/// struct Address {
///     city: String,
/// }
///
/// let city = TextField::new("City", Address::city()).autocomplete(Autocomplete::AddressLevel2);
/// assert!(city.render().render().contains(r#"autocomplete="address-level2""#));
/// ```
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
    /// `given-name`: the first name in Western order.
    GivenName,
    /// `additional-name`: a middle name.
    AdditionalName,
    /// `family-name`: the last name in Western order.
    FamilyName,
    /// `honorific-prefix`, such as "Dr.".
    HonorificPrefix,
    /// `nickname`.
    Nickname,
    /// `organization`.
    Organization,
    /// `organization-title`: a job title.
    OrganizationTitle,
    /// `street-address`, for a field that takes the whole street address.
    StreetAddress,
    /// `address-line1`.
    AddressLine1,
    /// `address-line2`.
    AddressLine2,
    /// `address-line3`.
    AddressLine3,
    /// `address-level1`: a state, a province, or a prefecture.
    AddressLevel1,
    /// `address-level2`: a city or a town.
    AddressLevel2,
    /// `address-level3`: a district within a city.
    AddressLevel3,
    /// `postal-code`.
    PostalCode,
    /// `country-name`.
    Country,
    /// `country`: an ISO 3166-1 alpha-2 country code.
    CountryCode,
    /// `cc-name`: the name on a payment card.
    CcName,
    /// `cc-number`.
    CcNumber,
    /// `cc-exp`: the expiry date of a payment card.
    CcExp,
    /// `cc-exp-month`.
    CcExpMonth,
    /// `cc-exp-year`.
    CcExpYear,
    /// `cc-csc`: the security code of a payment card.
    CcCsc,
    /// `bday`: a birthday.
    Bday,
    /// `bday-day`.
    BdayDay,
    /// `bday-month`.
    BdayMonth,
    /// `bday-year`.
    BdayYear,
    /// `sex`: a gender identity, as free text.
    Sex,
    /// `language`: a preferred language, as a BCP 47 tag.
    Language,
    /// `tel`.
    Tel,
    /// `tel-national`: a phone number without the country code.
    TelNational,
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
            Self::GivenName => "given-name",
            Self::AdditionalName => "additional-name",
            Self::FamilyName => "family-name",
            Self::HonorificPrefix => "honorific-prefix",
            Self::Nickname => "nickname",
            Self::Organization => "organization",
            Self::OrganizationTitle => "organization-title",
            Self::StreetAddress => "street-address",
            Self::AddressLine1 => "address-line1",
            Self::AddressLine2 => "address-line2",
            Self::AddressLine3 => "address-line3",
            Self::AddressLevel1 => "address-level1",
            Self::AddressLevel2 => "address-level2",
            Self::AddressLevel3 => "address-level3",
            Self::PostalCode => "postal-code",
            Self::Country => "country-name",
            Self::CountryCode => "country",
            Self::CcName => "cc-name",
            Self::CcNumber => "cc-number",
            Self::CcExp => "cc-exp",
            Self::CcExpMonth => "cc-exp-month",
            Self::CcExpYear => "cc-exp-year",
            Self::CcCsc => "cc-csc",
            Self::Bday => "bday",
            Self::BdayDay => "bday-day",
            Self::BdayMonth => "bday-month",
            Self::BdayYear => "bday-year",
            Self::Sex => "sex",
            Self::Language => "language",
            Self::Tel => "tel",
            Self::TelNational => "tel-national",
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
/// Two fields bound to the same signal on one page get the same ids, and
/// the second field's label and messages then point at the first field, so
/// bind each field to its own signal.
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
    value: Sig<S, String>,
    kind: InputKind,
    multiline: bool,
    autocomplete: Option<Autocomplete>,
    input_mode: Option<InputMode>,
    min_length: Option<u32>,
    max_length: Option<u32>,
    placeholder: Option<String>,
    frame: Frame<S>,
}

impl<S> TextField<S> {
    /// A single-line text field labeled `label`, bound to `value`.
    #[must_use]
    pub fn new(label: impl Into<String>, value: Sig<S, String>) -> Self {
        Self {
            value,
            kind: InputKind::Text,
            multiline: false,
            autocomplete: None,
            input_mode: None,
            min_length: None,
            max_length: None,
            placeholder: None,
            frame: Frame::new(label.into()),
        }
    }

    /// A single-line text field labeled `label`, bound to `field` of `form`.
    ///
    /// It shows [`error`](Sig::error) of the form for `field` and marks the
    /// field touched when the visitor leaves it, so a rule's message waits
    /// until the visitor has finished typing.
    ///
    /// ```
    /// use hypersig::{Form, Rules, Sig, Signals, Validate};
    /// use hypersig_ui::TextField;
    /// use serde::{Deserialize, Serialize};
    ///
    /// #[derive(Signals, Serialize, Deserialize, Clone)]
    /// struct Signup {
    ///     email: String,
    /// }
    ///
    /// impl Validate for Signup {
    ///     fn rules<S>(form: Sig<S, Self>, rules: &mut Rules<S, Self>) {
    ///         let email = form.at(Signup::email());
    ///         rules.check(Signup::email(), email.contains("@"), "Enter an email address.");
    ///     }
    /// }
    ///
    /// #[derive(Signals, Serialize, Deserialize)]
    /// struct Join {
    ///     signup: Form<Signup>,
    /// }
    ///
    /// let html = TextField::in_form("Email", Join::signup(), Signup::email()).render().render();
    /// assert!(html.contains("signup.touched.email"));
    /// ```
    #[must_use]
    pub fn in_form<T: Validate>(
        label: impl Into<String>,
        form: Sig<S, Form<T>>,
        field: Sig<T, String>,
    ) -> Self {
        let mut out = Self::new(label, form.field(field));
        out.frame = out.frame.in_form(form, field);
        out
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

    /// Asks for the on-screen keyboard `mode`, as `inputmode` does.
    ///
    /// ```
    /// use hypersig::Signals;
    /// use hypersig_ui::{Autocomplete, InputMode, TextField};
    /// use serde::{Deserialize, Serialize};
    ///
    /// #[derive(Signals, Serialize, Deserialize)]
    /// struct Login {
    ///     code: String,
    /// }
    ///
    /// let code = TextField::new("Code", Login::code())
    ///     .input_mode(InputMode::Numeric)
    ///     .autocomplete(Autocomplete::OneTimeCode)
    ///     .max_length(6);
    /// let html = code.render().render();
    /// assert!(html.contains(r#"inputmode="numeric""#));
    /// assert!(html.contains(r#"maxlength="6""#));
    /// ```
    #[must_use]
    pub fn input_mode(mut self, mode: InputMode) -> Self {
        self.input_mode = Some(mode);
        self
    }

    /// Stops the visitor from typing more than `count` characters, as
    /// `maxlength` does.
    #[must_use]
    pub fn max_length(mut self, count: u32) -> Self {
        self.max_length = Some(count);
        self
    }

    /// Makes the browser refuse to submit fewer than `count` characters, as
    /// `minlength` does.
    ///
    /// An empty field passes unless it is [`required`](Self::required).
    #[must_use]
    pub fn min_length(mut self, count: u32) -> Self {
        self.min_length = Some(count);
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
        self.frame.description = Some(text.into());
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
        self.frame.error = Some(message.into_expr());
        self
    }

    /// Marks the field `required`, which the label shows with an asterisk
    /// that a screen reader skips, since it reads "required" from the field.
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
        let mut control = if self.multiline {
            textarea().css(field_style().min_height(rem(4)).resize(Resize::Vertical))
        } else {
            input()
                .attr(attr::TYPE, self.kind.name())
                .css(field_style().height(rem(2.25)))
        }
        .bind(self.value);
        if let Some(autocomplete) = self.autocomplete {
            control = control.attr(attr::AUTOCOMPLETE, autocomplete.name());
        }
        if let Some(mode) = self.input_mode {
            control = control.attr(attr::INPUTMODE, mode.name());
        }
        if let Some(count) = self.min_length {
            control = control.attr(attr::MINLENGTH, count.to_string());
        }
        if let Some(count) = self.max_length {
            control = control.attr(attr::MAXLENGTH, count.to_string());
        }
        if let Some(placeholder) = self.placeholder {
            control = control.attr(attr::PLACEHOLDER, placeholder);
        }
        self.frame.render(&crate::id::base(self.value), control)
    }
}

/// The label, the description, and the error around a text or number
/// field, and the attributes of the control that tie them to it.
#[derive(Debug)]
pub(crate) struct Frame<S> {
    label: String,
    pub(crate) description: Option<String>,
    pub(crate) error: Option<Expr<S, String>>,
    pub(crate) required: bool,
    pub(crate) disabled: Option<Expr<S, bool>>,
    on_blur: Option<Stmt<S>>,
}

impl<S> Frame<S> {
    pub(crate) fn new(label: String) -> Self {
        Self {
            label,
            description: None,
            error: None,
            required: false,
            disabled: None,
            on_blur: None,
        }
    }

    /// The frame with the form's message for `field`, and the statement that
    /// marks `field` touched when the visitor leaves the control.
    pub(crate) fn in_form<T: Validate, U>(self, form: Sig<S, Form<T>>, field: Sig<T, U>) -> Self {
        Self {
            error: Some(form.error(field)),
            on_blur: Some(form.touch(field)),
            ..self
        }
    }

    /// `control` with its id and its ties to the label, the description, and
    /// the error, in a `<div>` with them. The ids are `base` with a key.
    pub(crate) fn render(self, base: &Id, control: El<S>) -> El<S> {
        let field_id = base.with_key("input");
        let description_id = base.with_key("description");
        let error_id = base.with_key("error");

        let mut described = Vec::new();
        if self.description.is_some() {
            described.push(&description_id);
        }
        if self.error.is_some() {
            described.push(&error_id);
        }

        let mut control = control.id(field_id.clone());
        if !described.is_empty() {
            control = control.aria_describedby(&described);
        }
        if self.required {
            control = control.attr(attr::REQUIRED, "");
        }
        if let Some(disabled) = self.disabled {
            control = control.disabled(disabled);
        }
        if let Some(touch) = self.on_blur {
            control = control.on_blur(touch);
        }
        if let Some(error) = &self.error {
            let invalid = !error.is_empty();
            control = control
                .bind_attr(
                    attr::ARIA_INVALID,
                    if_else(invalid.clone(), "true", "false"),
                )
                .css_when(
                    Style::new().border_color(token::destructive().var()),
                    invalid,
                );
        }

        let mut caption = label()
            .label_for(&field_id)
            .css(label_style())
            .text(self.label);
        if self.required {
            caption = caption.child(
                span()
                    .attr(attr::ARIA_HIDDEN, "true")
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
                    .attr(attr::ARIA_LIVE, "polite")
                    .css(look::small_text().color(token::destructive().var()))
                    .bind_text(error)
            }))
    }
}

/// What a control bound to a field of a [`Form`] adds: the form's message
/// for the field, under the control and tied to it, and the statement that
/// marks the field touched.
pub(crate) struct Feedback<S> {
    error: Expr<S, String>,
    touch: Stmt<S>,
    error_id: Id,
}

impl<S> Feedback<S> {
    /// The message and the touch for `field` of `form`.
    pub(crate) fn of<T: Validate, U>(form: Sig<S, Form<T>>, field: Sig<T, U>) -> Self {
        Self {
            error: form.error(field),
            touch: form.touch(field),
            error_id: crate::id::of(form.field(field), "error"),
        }
    }

    /// The statement that marks the field touched.
    pub(crate) fn touch(&self) -> Stmt<S> {
        self.touch.clone()
    }

    /// Whether the message is not empty.
    pub(crate) fn invalid(&self) -> Expr<S, bool> {
        !self.error.is_empty()
    }

    /// `control` with `aria-describedby` pointing at the message, and
    /// `aria-invalid` true while the message is not empty.
    pub(crate) fn describe(&self, control: El<S>) -> El<S> {
        control
            .aria_describedby(&[&self.error_id])
            .bind_attr(attr::ARIA_INVALID, if_else(self.invalid(), "true", "false"))
    }

    /// The message, in an `aria-live="polite"` region as [`TextField`] shows it.
    pub(crate) fn message(self) -> El<S> {
        p().id(self.error_id)
            .attr(attr::ARIA_LIVE, "polite")
            .css(look::small_text().color(token::destructive().var()))
            .bind_text(self.error)
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
