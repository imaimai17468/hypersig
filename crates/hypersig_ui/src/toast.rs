//! Toasts: short notices stacked in a corner of the page.

use std::sync::atomic::{AtomicI32, Ordering};
use std::time::Duration;

use hypersig::html::{El, div, each_keyed, li, ol, p, section};
use hypersig::style::pattern::stack;
use hypersig::style::value::{
    AlignItems, BorderStyle, Display, FontWeight, Position, ZIndex, pct, px, raw, rem,
};
use hypersig::style::{Media, Style};
use hypersig::{IntoExpr, Sig, Signals, Stmt};
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
/// The id tells toasts apart while the list changes, and nothing outside
/// this type sets it. On the server, [`Toast::new`] numbers each toast from
/// a counter of the process, so toasts made in one process have different
/// ids until 2³² of them have been made. In the browser, [`Toast::push`] and
/// [`Toast::push_expr`] give a toast one more than the highest id in the
/// list. A clone keeps its id, so a list that holds a toast and its clone
/// repeats one; the runtime then tells the two apart by position and writes
/// a warning to the console.
///
/// ```
/// use hypersig::{Expr, Signals};
/// use hypersig_ui::{Toast, ToastVariant};
/// use serde::{Deserialize, Serialize};
///
/// #[derive(Signals, Serialize, Deserialize)]
/// struct App {
///     toasts: Vec<Toast>,
///     name: String,
/// }
///
/// // On the server, in the list a patch sends.
/// let failed = App::toasts().patch(vec![
///     Toast::new("Upload failed").variant(ToastVariant::Destructive),
/// ]);
///
/// // In the browser, with fixed text.
/// let saved = Toast::new("Saved").description("Your changes are live.").push(App::toasts());
///
/// // In the browser, with text from a signal.
/// let renamed = Toast::push_expr(
///     App::toasts(),
///     Expr::lit(&Toast::new("Renamed")).with(Toast::body(), App::name().concat(" is the new name.")),
/// );
/// # let _ = (failed, saved, renamed);
/// ```
#[derive(Signals, Serialize, Deserialize, Debug, Clone, PartialEq)]
#[non_exhaustive]
pub struct Toast {
    key: i32,
    /// The title, in bold.
    pub title: String,
    /// The line under the title, hidden while it is empty.
    pub body: String,
    /// How the toast is drawn.
    pub tone: ToastVariant,
}

/// The id of the next toast [`Toast::new`] makes.
static NEXT_KEY: AtomicI32 = AtomicI32::new(1);

impl Toast {
    /// A toast titled `title`, with an id no other toast of the process has.
    #[must_use]
    pub fn new(title: impl Into<String>) -> Self {
        Self {
            key: NEXT_KEY.fetch_add(1, Ordering::Relaxed),
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

    /// Sets the look of the toast.
    #[must_use]
    pub fn variant(mut self, variant: ToastVariant) -> Self {
        self.tone = variant;
        self
    }

    /// A statement that adds this toast to `toasts` in the browser.
    #[must_use]
    pub fn push<S>(self, toasts: Sig<S, Vec<Self>>) -> Stmt<S> {
        Self::push_expr(toasts, &self)
    }

    /// A statement that adds `toast`, which the browser evaluates, to `toasts`.
    ///
    /// Build `toast` from a literal toast with [`Expr::with`](hypersig::Expr::with) on
    /// [`Toast::title`], [`Toast::body`], or [`Toast::tone`], so that its
    /// text can come from a signal. Its id is replaced by one more than the
    /// highest id in `toasts`.
    #[must_use]
    pub fn push_expr<S>(toasts: Sig<S, Vec<Self>>, toast: impl IntoExpr<S, Self>) -> Stmt<S> {
        let next = toasts.max_of(|toast| toast.at(Self::key())).unwrap_or(0) + 1;
        toasts.push(toast.into_expr().with(Self::key(), next))
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
    /// A region that shows `toasts`, newest last.
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
