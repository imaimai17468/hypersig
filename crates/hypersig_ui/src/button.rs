//! Buttons, and the style that makes a link look like one.

use hypersig::html::{El, attr, button as button_el};
use hypersig::style::value::{
    AlignItems, BorderStyle, Color, Cursor, Display, FontWeight, JustifyContent,
    TextDecorationLine, UserSelect, WhiteSpace, px, raw, rem,
};
use hypersig::style::{PseudoClass, Style};

use crate::look;
use crate::token;

/// The look of a button.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum ButtonVariant {
    /// Filled with [`token::primary`], for the main action.
    Default,
    /// Filled with [`token::destructive`], for an action that deletes.
    Destructive,
    /// A border on the background.
    Outline,
    /// Filled with [`token::secondary`].
    Secondary,
    /// No fill until the pointer is over it.
    Ghost,
    /// Underlined text.
    Link,
}

/// The height and padding of a button.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum ButtonSize {
    /// 2.25rem high.
    Default,
    /// 2rem high.
    Sm,
    /// 2.5rem high.
    Lg,
}

/// The style of a button of `variant` and `size`.
///
/// It is for an element that is not a `<button>`, such as a link that leads to
/// a form.
///
/// ```
/// use hypersig::html::a;
/// use hypersig::{Page, Route, Signals};
/// use hypersig_ui::{ButtonSize, ButtonVariant, button_style};
/// use serde::{Deserialize, Serialize};
///
/// #[derive(Signals, Serialize, Deserialize)]
/// struct App {}
///
/// # async fn signup((): (), (): ()) -> Option<Page<App>> { None }
/// hypersig::routes! {
///     fn pages(Router<()>);
///     const SIGNUP: Route = "/signup" => signup;
/// }
///
/// let link = a::<App>()
///     .href(SIGNUP.link())
///     .css(button_style(ButtonVariant::Default, ButtonSize::Lg))
///     .text("Sign up");
/// # let _ = (link, pages);
/// ```
#[must_use]
pub fn button_style(variant: ButtonVariant, size: ButtonSize) -> Style {
    let height = match size {
        ButtonSize::Default => rem(2.25),
        ButtonSize::Sm => rem(2),
        ButtonSize::Lg => rem(2.5),
    };
    let padding = match size {
        ButtonSize::Default => rem(1),
        ButtonSize::Sm => rem(0.75),
        ButtonSize::Lg => rem(1.5),
    };
    base()
        .height(height)
        .padding_inline(padding)
        .merge(fill(variant))
}

/// The parts every button shares, whatever its variant and size.
fn base() -> Style {
    Style::new()
        .display(Display::InlineFlex)
        .align_items(AlignItems::Center)
        .justify_content(JustifyContent::Center)
        .gap(rem(0.5))
        .white_space(WhiteSpace::Nowrap)
        .border_width(px(1))
        .border_style(BorderStyle::Solid)
        .border_color(Color::TRANSPARENT)
        .border_radius(raw(format!("calc({} - 2px)", token::radius().var())))
        .font_weight(FontWeight::Number(500.0))
        .cursor(Cursor::Pointer)
        .user_select(UserSelect::None)
        .merge(look::small_text())
        .merge(look::transition())
        .merge(look::focus_ring())
        .merge(look::disabled())
}

fn fill(variant: ButtonVariant) -> Style {
    let solid = |background: Color, text: Color, hover: u8| {
        Style::new()
            .background_color(background.clone())
            .color(text)
            .on(
                PseudoClass::Hover,
                Style::new().background_color(background.opacity(hover)),
            )
    };
    match variant {
        ButtonVariant::Default => solid(
            token::primary().var(),
            token::primary_foreground().var(),
            90,
        ),
        ButtonVariant::Destructive => solid(
            token::destructive().var(),
            token::destructive_foreground().var(),
            90,
        ),
        ButtonVariant::Secondary => solid(
            token::secondary().var(),
            token::secondary_foreground().var(),
            80,
        ),
        ButtonVariant::Outline => Style::new()
            .border_color(token::border().var())
            .background_color(token::background().var())
            .color(token::foreground().var())
            .on(PseudoClass::Hover, hover_accent()),
        ButtonVariant::Ghost => Style::new()
            .background_color(Color::TRANSPARENT)
            .color(token::foreground().var())
            .on(PseudoClass::Hover, hover_accent()),
        ButtonVariant::Link => Style::new()
            .background_color(Color::TRANSPARENT)
            .color(token::primary().var())
            .text_underline_offset(px(4))
            .on(
                PseudoClass::Hover,
                Style::new().text_decoration_line(TextDecorationLine::Underline),
            ),
    }
}

fn hover_accent() -> Style {
    Style::new()
        .background_color(token::accent().var())
        .color(token::accent_foreground().var())
}

/// A `<button type="button">` of `variant` and `size`.
///
/// The type is `button`, so that a button inside a form does not submit it.
/// The one that should is a [`submit_button`]. Disable either with
/// [`El::disabled`], which dims it.
///
/// ```
/// use hypersig::Signals;
/// use hypersig_ui::{ButtonSize, ButtonVariant, button};
/// use serde::{Deserialize, Serialize};
///
/// #[derive(Signals, Serialize, Deserialize)]
/// struct Counter {
///     count: i32,
/// }
///
/// let count = Counter::count();
/// let plus = button::<Counter>(ButtonVariant::Outline, ButtonSize::Sm)
///     .on_click(count.set(count + 1))
///     .disabled(count.ge(10))
///     .text("Add one");
/// assert!(plus.render().contains(r#"type="button""#));
/// ```
#[must_use]
pub fn button<S>(variant: ButtonVariant, size: ButtonSize) -> El<S> {
    button_el()
        .attr(attr::TYPE, "button")
        .css(button_style(variant, size))
}

/// A `<button type="submit">` of `variant` and `size`, which submits the
/// form around it.
///
/// ```
/// use hypersig::html::form;
/// use hypersig::Signals;
/// use hypersig_ui::{ButtonSize, ButtonVariant, submit_button};
/// use serde::{Deserialize, Serialize};
///
/// #[derive(Signals, Serialize, Deserialize)]
/// struct Login {
///     sent: bool,
/// }
///
/// let login = form::<Login>()
///     .on_submit(Login::sent().set(true))
///     .child(submit_button(ButtonVariant::Default, ButtonSize::Default).text("Log in"));
/// assert!(login.render().contains(r#"type="submit""#));
/// ```
#[must_use]
pub fn submit_button<S>(variant: ButtonVariant, size: ButtonSize) -> El<S> {
    button_el()
        .attr(attr::TYPE, "submit")
        .css(button_style(variant, size))
}

/// A square button that shows only an icon, named by `label`.
///
/// The label is the accessible name, which a button whose only child is an
/// icon has no other way to get, so this constructor takes it.
///
/// ```
/// use hypersig::Signals;
/// use hypersig_ui::{ButtonVariant, icon_button};
/// use serde::{Deserialize, Serialize};
///
/// #[derive(Signals, Serialize, Deserialize)]
/// struct App {}
///
/// let more = icon_button::<App>(ButtonVariant::Ghost, "More actions").text("…");
/// assert!(more.render().contains(r#"aria-label="More actions""#));
/// ```
#[must_use]
pub fn icon_button<S>(variant: ButtonVariant, label: impl Into<String>) -> El<S> {
    button_el()
        .attr(attr::TYPE, "button")
        .attr(attr::ARIA_LABEL, label)
        .css(
            base()
                .width(rem(2.25))
                .height(rem(2.25))
                .padding(px(0))
                .merge(fill(variant)),
        )
}
