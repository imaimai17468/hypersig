//! Checkboxes, switches, and radio groups, drawn over native inputs.
//!
//! Each control is a native `<input>` the browser draws nothing for, laid
//! over a box drawn by the elements after it. The input keeps its role, its
//! keyboard handling, and its place in a form, and the drawn box follows its
//! state through `Style::peer`.

use hypersig::html::{El, div, fieldset, input, label, legend, span};
use hypersig::style::pattern::{hstack, stack};
use hypersig::style::value::{
    Appearance, BorderStyle, Color, Cursor, Display, Opacity, Position, TextAlign, px, rem,
};
use hypersig::style::{Condition, PseudoClass, Style};
use hypersig::{Form, Sig, Validate};
use serde::Serialize;

use crate::field::{Feedback, label_style};
use crate::look::{self, icon};
use crate::token;

/// The native input, covering the box drawn after it and invisible, so that
/// a click anywhere on the box reaches it.
fn hidden_input() -> Style {
    Style::new()
        .appearance(Appearance::None)
        .position(Position::Absolute)
        .inset(px(0))
        .margin(px(0))
        .opacity(Opacity::Number(0.0))
        .cursor(Cursor::Pointer)
        .peer()
        .on(
            PseudoClass::Disabled,
            Style::new().cursor(Cursor::NotAllowed),
        )
}

/// The box around the input and what draws it.
fn holder(size: hypersig::style::value::Length) -> Style {
    Style::new()
        .position(Position::Relative)
        .display(Display::InlineFlex)
        .flex_shrink(0.0)
        .width(size.clone())
        .height(size)
}

/// Dims what a disabled input draws.
fn peer_disabled() -> Style {
    Style::new().on(
        Condition::Peer(PseudoClass::Disabled),
        Style::new().opacity(Opacity::Number(0.5)),
    )
}

/// The `<label>` that holds a control and its text, so that the text names
/// the control and a click on it toggles the control.
fn row<S>(text: String) -> (El<S>, El<S>) {
    let wrapper = label().css(
        hstack(rem(0.5))
            .merge(label_style())
            .cursor(Cursor::Pointer),
    );
    (wrapper, span().text(text))
}

/// A checkbox labeled `text`, bound to `checked`.
///
/// It is a native `<input type="checkbox">` inside the `<label>` that holds
/// the text, so it needs no id, and it works on a field of a list item.
///
/// ```
/// use hypersig::Signals;
/// use hypersig_ui::checkbox;
/// use serde::{Deserialize, Serialize};
///
/// #[derive(Signals, Serialize, Deserialize)]
/// struct Terms {
///     accepted: bool,
/// }
///
/// let accept = checkbox::<Terms, _>("I accept the terms", Terms::accepted());
/// assert!(accept.render().contains(r#"type="checkbox""#));
/// ```
#[must_use]
pub fn checkbox<S, R>(text: impl Into<String>, checked: Sig<S, bool, R>) -> El<S> {
    let (wrapper, text) = row(text.into());
    wrapper.child(check_control(input(), checked)).child(text)
}

/// A checkbox labeled `text`, bound to `field` of `form`.
///
/// It is [`checkbox`] with the form's [`error`](Sig::error) for `field`
/// under it, tied to the input by `aria-describedby` and `aria-invalid`, and
/// it marks the field touched when the visitor leaves the checkbox.
///
/// ```
/// use hypersig::{Form, Rules, Sig, Signals, Validate};
/// use hypersig_ui::checkbox_in_form;
/// use serde::{Deserialize, Serialize};
///
/// #[derive(Signals, Serialize, Deserialize, Clone)]
/// struct Signup {
///     terms: bool,
/// }
///
/// impl Validate for Signup {
///     fn rules<S>(form: Sig<S, Self>, rules: &mut Rules<S, Self>) {
///         rules.check(Signup::terms(), form.at(Signup::terms()), "Accept the terms.");
///     }
/// }
///
/// #[derive(Signals, Serialize, Deserialize)]
/// struct Join {
///     signup: Form<Signup>,
/// }
///
/// let terms = checkbox_in_form("I accept the terms", Join::signup(), Signup::terms());
/// assert!(terms.render().contains(r#"aria-describedby="ui-signup-value-terms-error""#));
/// ```
#[must_use]
pub fn checkbox_in_form<S, T: Validate>(
    text: impl Into<String>,
    form: Sig<S, Form<T>>,
    field: Sig<T, bool>,
) -> El<S> {
    let feedback = Feedback::of(form, field);
    let control = feedback.describe(input()).on_blur(feedback.touch());
    let (wrapper, text) = row(text.into());
    with_message(
        wrapper
            .child(check_control(control, form.field(field)))
            .child(text),
        feedback,
    )
}

/// `control` with the message of `feedback` under it.
fn with_message<S>(control: El<S>, feedback: Feedback<S>) -> El<S> {
    div()
        .css(stack(rem(0.5)))
        .child(control)
        .child(feedback.message())
}

/// The box of a checkbox with `input` laid over it, bound to `checked`.
///
/// `input` comes from the caller, which names it where no `<label>` holds it.
pub(crate) fn check_control<S, R>(input: El<S>, checked: Sig<S, bool, R>) -> El<S> {
    check_mark(input.bind(checked))
}

/// `input` as a checkbox under a drawn box, for an input whose checked
/// state its caller binds.
pub(crate) fn check_mark<S>(input: El<S>) -> El<S> {
    let mark = span()
        .attr("aria-hidden", "true")
        .css(check_box().on(Condition::Peer(PseudoClass::Checked), check_box_checked()))
        .child(icon(icon::CHECK, rem(0.875)));
    span()
        .css(holder(rem(1)))
        .child(input.attr("type", "checkbox").css(hidden_input()))
        .child(mark)
}

/// The drawn box of an unchecked checkbox.
pub(crate) fn check_box() -> Style {
    Style::new()
        .display(Display::InlineFlex)
        .align_items(hypersig::style::value::AlignItems::Center)
        .justify_content(hypersig::style::value::JustifyContent::Center)
        .width(rem(1))
        .height(rem(1))
        .border_width(px(1))
        .border_style(BorderStyle::Solid)
        .border_color(token::input().var())
        .border_radius(px(4))
        .color(Color::TRANSPARENT)
        .merge(look::transition())
        .merge(look::peer_focus_ring())
        .merge(peer_disabled())
}

/// The colors the box takes once checked.
pub(crate) fn check_box_checked() -> Style {
    Style::new()
        .border_color(token::primary().var())
        .background_color(token::primary().var())
        .color(token::primary_foreground().var())
}

/// An on and off switch labeled `text`, bound to `on`.
///
/// It is a native checkbox with `role="switch"`, so a screen reader says
/// "switch, on" rather than "checkbox, checked", and Space toggles it.
///
/// ```
/// use hypersig::Signals;
/// use hypersig_ui::switch;
/// use serde::{Deserialize, Serialize};
///
/// #[derive(Signals, Serialize, Deserialize)]
/// struct Settings {
///     airplane: bool,
/// }
///
/// let airplane = switch::<Settings, _>("Airplane mode", Settings::airplane());
/// assert!(airplane.render().contains(r#"role="switch""#));
/// ```
#[must_use]
pub fn switch<S, R>(text: impl Into<String>, on: Sig<S, bool, R>) -> El<S> {
    switch_with(text.into(), on, input())
}

/// An on and off switch labeled `text`, bound to `field` of `form`.
///
/// It is [`switch`] with the form's [`error`](Sig::error) for `field` under
/// it, tied to the input by `aria-describedby` and `aria-invalid`, and it
/// marks the field touched when the visitor leaves the switch.
///
/// ```
/// use hypersig::{Form, Rules, Sig, Signals, Validate};
/// use hypersig_ui::switch_in_form;
/// use serde::{Deserialize, Serialize};
///
/// #[derive(Signals, Serialize, Deserialize, Clone)]
/// struct Settings {
///     backups: bool,
/// }
///
/// impl Validate for Settings {
///     fn rules<S>(form: Sig<S, Self>, rules: &mut Rules<S, Self>) {
///         rules.check(Settings::backups(), form.at(Settings::backups()), "Keep backups on.");
///     }
/// }
///
/// #[derive(Signals, Serialize, Deserialize)]
/// struct Page {
///     settings: Form<Settings>,
/// }
///
/// let backups = switch_in_form("Backups", Page::settings(), Settings::backups());
/// assert!(backups.render().contains(r#"role="switch""#));
/// ```
#[must_use]
pub fn switch_in_form<S, T: Validate>(
    text: impl Into<String>,
    form: Sig<S, Form<T>>,
    field: Sig<T, bool>,
) -> El<S> {
    let feedback = Feedback::of(form, field);
    let control = feedback.describe(input()).on_blur(feedback.touch());
    with_message(
        switch_with(text.into(), form.field(field), control),
        feedback,
    )
}

/// A switch labeled `text` with `input` as its checkbox, bound to `on`.
fn switch_with<S, R>(text: String, on: Sig<S, bool, R>, input: El<S>) -> El<S> {
    let (wrapper, text) = row(text);
    let track = span().attr("aria-hidden", "true").css(
        Style::new()
            .display(Display::Block)
            .width(rem(2))
            .height(rem(1.15))
            .border_width(px(1))
            .border_style(BorderStyle::Solid)
            .border_color(Color::TRANSPARENT)
            .border_radius(px(999))
            .background_color(token::input().var())
            .merge(look::transition())
            .on(
                Condition::Peer(PseudoClass::Checked),
                Style::new().background_color(token::primary().var()),
            )
            .merge(look::peer_focus_ring())
            .merge(peer_disabled()),
    );
    let thumb = span().attr("aria-hidden", "true").css(
        Style::new()
            .position(Position::Absolute)
            .top(rem(0.15))
            .left(rem(0.15))
            .width(rem(0.85))
            .height(rem(0.85))
            .border_radius(px(999))
            .background_color(token::background().var())
            .pointer_events(hypersig::style::value::PointerEvents::None)
            .merge(look::transition())
            .on(
                Condition::Peer(PseudoClass::Checked),
                Style::new().translate(hypersig::style::value::raw("0.85rem 0")),
            ),
    );
    wrapper
        .child(
            span()
                .css(
                    holder(rem(2))
                        .height(rem(1.15))
                        .text_align(TextAlign::Start),
                )
                .child(
                    input
                        .attr("type", "checkbox")
                        .attr("role", "switch")
                        .css(hidden_input())
                        .bind(on),
                )
                .child(track)
                .child(thumb),
        )
        .child(text)
}

/// A group of radio buttons with `title` in its `<legend>`, one per option,
/// that sets `selected` to the option's value.
///
/// The radios share a `name` derived from the signal's path, so the browser
/// gives the group one Tab stop and moves between its radios with the arrow
/// keys. The `<fieldset>` and `<legend>` name the group for a screen reader.
///
/// ```
/// use hypersig::Signals;
/// use hypersig_ui::radio_group;
/// use serde::{Deserialize, Serialize};
///
/// #[derive(Serialize, Deserialize, Clone, Copy, PartialEq)]
/// enum Plan {
///     Free,
///     Pro,
/// }
///
/// #[derive(Signals, Serialize, Deserialize)]
/// struct Billing {
///     plan: Plan,
/// }
///
/// let plans = radio_group("Plan", Billing::plan(), [(Plan::Free, "Free"), (Plan::Pro, "Pro")]);
/// assert!(plans.render().contains(r#"name="ui-plan-radio""#));
/// ```
#[must_use]
pub fn radio_group<S, T: Serialize + PartialEq, L: Into<String>>(
    title: impl Into<String>,
    selected: Sig<S, T>,
    options: impl IntoIterator<Item = (T, L)>,
) -> El<S> {
    radio_with(title.into(), selected, options, None)
}

/// A group of radio buttons with `title` in its `<legend>`, bound to `field`
/// of `form`.
///
/// It is [`radio_group`] with the form's [`error`](Sig::error) for `field`
/// under it. ARIA allows `aria-invalid` on a radio group but not on a plain
/// group, so the `<fieldset>` takes `role="radiogroup"`, is named by its
/// legend through `aria-labelledby`, and carries `aria-describedby` and
/// `aria-invalid`. The field is marked touched when focus leaves the group,
/// and not while the arrow keys move it between the radios.
///
/// ```
/// use hypersig::{Form, Rules, Sig, Signals, Validate};
/// use hypersig_ui::radio_group_in_form;
/// use serde::{Deserialize, Serialize};
///
/// #[derive(Serialize, Deserialize, Clone, Copy, PartialEq, Default)]
/// enum Plan {
///     #[default]
///     Unset,
///     Free,
///     Pro,
/// }
///
/// #[derive(Signals, Serialize, Deserialize, Clone)]
/// struct Signup {
///     plan: Plan,
/// }
///
/// impl Validate for Signup {
///     fn rules<S>(form: Sig<S, Self>, rules: &mut Rules<S, Self>) {
///         let chosen = !form.at(Signup::plan()).equals(&Plan::Unset);
///         rules.check(Signup::plan(), chosen, "Choose a plan.");
///     }
/// }
///
/// #[derive(Signals, Serialize, Deserialize)]
/// struct Join {
///     signup: Form<Signup>,
/// }
///
/// let plans = radio_group_in_form(
///     "Plan",
///     Join::signup(),
///     Signup::plan(),
///     [(Plan::Free, "Free"), (Plan::Pro, "Pro")],
/// );
/// let html = plans.render();
/// assert!(html.contains(r#"role="radiogroup""#));
/// assert!(html.contains(r#"aria-describedby="ui-signup-value-plan-error""#));
/// ```
#[must_use]
pub fn radio_group_in_form<S, T: Validate, U: Serialize + PartialEq, L: Into<String>>(
    title: impl Into<String>,
    form: Sig<S, Form<T>>,
    field: Sig<T, U>,
    options: impl IntoIterator<Item = (U, L)>,
) -> El<S> {
    radio_with(
        title.into(),
        form.field(field),
        options,
        Some(Feedback::of(form, field)),
    )
}

fn radio_with<S, T: Serialize + PartialEq, L: Into<String>>(
    title: String,
    selected: Sig<S, T>,
    options: impl IntoIterator<Item = (T, L)>,
    feedback: Option<Feedback<S>>,
) -> El<S> {
    let name = crate::id::of(selected, "radio");
    let radio = |value: T, text: L| {
        let (wrapper, text) = row(text.into());
        let dot = span().attr("aria-hidden", "true").css(
            Style::new()
                .display(Display::Block)
                .width(rem(1))
                .height(rem(1))
                .border_width(px(1))
                .border_style(BorderStyle::Solid)
                .border_color(token::input().var())
                .border_radius(px(999))
                .merge(look::transition())
                .on(
                    Condition::Peer(PseudoClass::Checked),
                    Style::new()
                        .border_color(token::primary().var())
                        .border_width(px(5)),
                )
                .merge(look::peer_focus_ring())
                .merge(peer_disabled()),
        );
        wrapper
            .child(
                span()
                    .css(holder(rem(1)))
                    .child(
                        input()
                            .attr("name", name.clone())
                            .css(hidden_input())
                            .bind_radio(selected, &value),
                    )
                    .child(dot),
            )
            .child(text)
    };
    let mut caption = legend()
        .css(label_style().padding(px(0)).margin_bottom(rem(0.75)))
        .text(title);
    let mut group = fieldset().css(
        stack(rem(0.75))
            .border_style(BorderStyle::None)
            .padding(px(0))
            .min_width(hypersig::style::value::MinWidth::from(px(0))),
    );
    if let Some(feedback) = &feedback {
        let legend_id = crate::id::of(selected, "legend");
        caption = caption.id(legend_id.clone());
        group = feedback
            .describe(group.attr("role", "radiogroup"))
            .attr("aria-labelledby", legend_id)
            .on_blur(feedback.touch());
    }
    group
        .child(caption)
        .children(options.into_iter().map(|(value, text)| radio(value, text)))
        .children(feedback.map(Feedback::message))
}
