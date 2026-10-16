//! Toasts: short notices stacked in a corner of the page.

use std::time::Duration;

use hypersig::html::{El, div, each_keyed, li, ol, p, section};
use hypersig::style::pattern::stack;
use hypersig::style::value::{
    AlignItems, BorderStyle, Display, FontWeight, Position, ZIndex, pct, px, raw, rem,
};
use hypersig::style::{Media, Style};
use hypersig::{Expr, IntoExpr, Sig, Signals, Stmt};
use serde::{Deserialize, Serialize};

use crate::button::{ButtonVariant, icon_button};
use crate::look::{self, icon};
use crate::token;

/// How a [`Toast`] is drawn.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub enum ToastVariant {
    /// The card colors.
    #[default]
    Default,
    /// The title in [`token::destructive`], for a failure.
    Destructive,
}

/// One notice in a [`Toaster`]: a title, an optional description, and an id.
///
/// The id tells toasts apart while the list changes, so two toasts in one
/// list need different ids, which nothing checks. The server adds a toast by
/// patching the list, and a statement in the browser with [`Toast::push`].
///
/// ```
/// use hypersig::Signals;
/// use hypersig_ui::{Toast, ToastVariant};
/// use serde::{Deserialize, Serialize};
///
/// #[derive(Signals, Serialize, Deserialize)]
/// struct App {
///     toasts: Vec<Toast>,
///     next: i32,
/// }
///
/// // On the server, in the list a patch sends.
/// let failed = Toast::new("Upload failed").variant(ToastVariant::Destructive).id(7);
///
/// // In the browser, numbered from a counter.
/// let next = App::next();
/// let saved = Toast::new("Saved")
///     .description("Your changes are live.")
///     .push(App::toasts(), next)
///     .then(next.set(next + 1));
/// # let _ = (failed, saved);
/// ```
#[derive(Signals, Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Toast {
    key: i32,
    title: String,
    body: String,
    tone: ToastVariant,
}

impl Toast {
    /// A toast titled `title`, with the id 0.
    #[must_use]
    pub fn new(title: impl Into<String>) -> Self {
        Self {
            key: 0,
            title: title.into(),
            body: String::new(),
            tone: ToastVariant::Default,
        }
    }

    /// Adds a line of text under the title.
    #[must_use]
    pub fn description(mut self, text: impl Into<String>) -> Self {
        self.body = text.into();
        self
    }

    /// Sets how the toast is drawn.
    #[must_use]
    pub fn variant(mut self, variant: ToastVariant) -> Self {
        self.tone = variant;
        self
    }

    /// Sets the id that tells this toast apart from the others in its list.
    #[must_use]
    pub fn id(mut self, id: i32) -> Self {
        self.key = id;
        self
    }

    /// A statement that adds this toast to `toasts` with the id `id`.
    #[must_use]
    pub fn push<S>(self, toasts: Sig<S, Vec<Self>>, id: impl IntoExpr<S, i32>) -> Stmt<S> {
        toasts.push(Expr::lit(&self).with(Self::key(), id))
    }
}

/// The corner of the page that shows the toasts in the list `toasts`.
///
/// It is a landmark region named "Notifications" holding an
/// `aria-live="polite"` list, so a screen reader reads a toast when it is
/// added, after what it is reading. Each toast has a button that removes it.
/// A toast stays until it is dismissed, unless [`Toaster::dismiss_after`]
/// sets a time, because a visitor who reads slowly or uses a magnifier can
/// miss a notice that leaves by itself (WCAG 2.2.1).
///
/// The region sits in the page rather than in the top layer, so an open modal
/// dialog covers it and makes it inert until the dialog closes.
///
/// ```
/// use hypersig::Signals;
/// use hypersig_ui::{Toast, Toaster};
/// use serde::{Deserialize, Serialize};
///
/// #[derive(Signals, Serialize, Deserialize)]
/// struct App {
///     toasts: Vec<Toast>,
/// }
///
/// let toaster = Toaster::new(App::toasts()).render();
/// let html = toaster.render();
/// assert!(html.contains(r#"aria-label="Notifications""#));
/// assert!(html.contains(r#"aria-live="polite""#));
/// ```
#[derive(Debug)]
pub struct Toaster<S> {
    toasts: Sig<S, Vec<Toast>>,
    label: String,
    dismiss_after: Option<Duration>,
}

impl<S> Toaster<S> {
    /// A toaster that shows `toasts`, newest last.
    #[must_use]
    pub fn new(toasts: Sig<S, Vec<Toast>>) -> Self {
        Self {
            toasts,
            label: "Notifications".to_owned(),
            dismiss_after: None,
        }
    }

    /// Names the region for assistive technology, in place of
    /// "Notifications".
    #[must_use]
    pub fn label(mut self, label: impl Into<String>) -> Self {
        self.label = label.into();
        self
    }

    /// Removes each toast `delay` after it appears.
    ///
    /// The time runs while the pointer or focus is on the toast, so give it
    /// long enough to read the longest toast twice.
    #[must_use]
    pub fn dismiss_after(mut self, delay: Duration) -> Self {
        self.dismiss_after = Some(delay);
        self
    }

    /// The region and its list.
    #[must_use]
    pub fn render(self) -> El<S> {
        let dismiss_after = self.dismiss_after;
        section()
            .attr("aria-label", self.label)
            .css(region_style())
            .child(
                ol().attr("aria-live", "polite")
                    .css(stack(rem(0.5)))
                    .child(each_keyed(
                        self.toasts,
                        |toast| toast.at(Toast::key()),
                        move |toast| {
                            let tone = toast.at(Toast::tone());
                            let body = toast.at(Toast::body());
                            let mut item = li()
                                .attr("aria-atomic", "true")
                                .css(toast_style())
                                .child(
                                    div()
                                        .css(stack(rem(0.25)).flex_grow(1.0))
                                        .child(
                                            p().css(
                                                look::small_text()
                                                    .font_weight(FontWeight::Number(600.0)),
                                            )
                                            .css_when(
                                                Style::new().color(token::destructive().var()),
                                                tone.equals(&ToastVariant::Destructive),
                                            )
                                            .bind_text(toast.at(Toast::title())),
                                        )
                                        .child(
                                            p().css(
                                                look::small_text()
                                                    .color(token::muted_foreground().var()),
                                            )
                                            .show(!body.is_empty())
                                            .bind_text(body),
                                        ),
                                )
                                .child(
                                    icon_button(ButtonVariant::Ghost, "Dismiss")
                                        .css(Style::new().width(rem(1.75)).height(rem(1.75)))
                                        .on_click(toast.remove())
                                        .child(icon(look::icon::X, rem(1))),
                                );
                            if let Some(delay) = dismiss_after {
                                item = item.on_timeout(delay, toast.remove());
                            }
                            item
                        },
                    )),
            )
    }
}

impl<S> From<Toaster<S>> for El<S> {
    fn from(toaster: Toaster<S>) -> Self {
        toaster.render()
    }
}

impl<S> From<Toaster<S>> for hypersig::html::Node<S> {
    fn from(toaster: Toaster<S>) -> Self {
        toaster.render().into()
    }
}

fn region_style() -> Style {
    Style::new()
        .position(Position::Fixed)
        .right(rem(1))
        .bottom(rem(1))
        .left(rem(1))
        .z_index(ZIndex::Integer(100))
        .pointer_events(hypersig::style::value::PointerEvents::None)
        .on(Media::Sm, Style::new().left(raw("auto")).width(rem(22)))
}

fn toast_style() -> Style {
    Style::new()
        .display(Display::Flex)
        .align_items(AlignItems::Start)
        .gap(rem(0.75))
        .width(pct(100))
        .padding(rem(1))
        .border_width(px(1))
        .border_style(BorderStyle::Solid)
        .border_color(token::border().var())
        .border_radius(token::radius().var())
        .background_color(token::popover().var())
        .color(token::popover_foreground().var())
        .box_shadow(raw(
            "0 10px 15px -3px rgb(0 0 0 / 0.1), 0 4px 6px -4px rgb(0 0 0 / 0.1)",
        ))
        .pointer_events(hypersig::style::value::PointerEvents::Auto)
}
