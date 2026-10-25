//! Toasts: short notices stacked in a corner of the page.

use std::sync::atomic::{AtomicU32, Ordering};
use std::time::Duration;

use hypersig::html::{El, attr, div, each_keyed, li, ol, p, section};
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
/// The id tells toasts apart while the list changes, and nothing outside
/// this type sets it. On the server, [`Toast::new`] numbers each toast from
/// a counter of the process, counting down from -1, so toasts made in one
/// process have different ids until 2³¹ of them have been made. In the
/// browser, [`Toast::push`] and [`Toast::push_expr`] give a toast one more
/// than the highest id in the list, and 1 where no id in the list is above 0.
/// A server id is below 0 and a browser id above it, so a toast the server
/// appends with [`patch_append`](hypersig::Sig::patch_append) and a toast the
/// browser pushes never share an id. A clone keeps its id, so a list that
/// holds a toast and its clone repeats one; the runtime then tells the two
/// apart by position and writes a warning to the console.
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
/// // On the server, after the toasts the browser already shows.
/// let failed = App::toasts().patch_append(&[
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

/// How many toasts [`Toast::new`] has made, which gives the next one its id.
static MADE: AtomicU32 = AtomicU32::new(0);

impl Toast {
    /// A toast titled `title`, with an id no other toast of the process has.
    #[must_use]
    pub fn new(title: impl Into<String>) -> Self {
        Self {
            key: server_key(MADE.fetch_add(1, Ordering::Relaxed)),
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
    /// highest id in `toasts`, and by 1 where no id there is above 0.
    #[must_use]
    pub fn push_expr<S>(toasts: Sig<S, Vec<Self>>, toast: impl IntoExpr<S, Self>) -> Stmt<S> {
        toasts.push(toast.into_expr().with(Self::key(), browser_key(toasts)))
    }
}

/// The id of the toast [`Toast::new`] makes after `made` others: -1, then
/// -2, staying below 0 when `made` wraps past 2³¹.
fn server_key(made: u32) -> i32 {
    !(made & i32::MAX.cast_unsigned()).cast_signed()
}

/// The id the browser gives a toast it adds to `toasts`: above 0 and above
/// every id in the list.
fn browser_key<S>(toasts: Sig<S, Vec<Toast>>) -> Expr<S, i32> {
    toasts
        .max_of(|toast| toast.at(Toast::key()))
        .unwrap_or(0)
        .max(0)
        + 1
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
            .attr(attr::ARIA_LABEL, self.label)
            .css(region_style())
            .child(
                ol().attr(attr::ARIA_LIVE, "polite")
                    .css(stack(rem(0.5)))
                    .child(each_keyed(
                        self.toasts,
                        |toast| toast.at(Toast::key()),
                        move |toast| {
                            let tone = toast.at(Toast::tone());
                            let body = toast.at(Toast::body());
                            let mut item = li()
                                .attr(attr::ARIA_ATOMIC, "true")
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

#[cfg(test)]
mod tests {
    use hypersig::html::span;
    use hypersig::{Page, Signals};
    use serde::{Deserialize, Serialize};

    use super::{Toast, browser_key, server_key};

    #[derive(Signals, Serialize, Deserialize)]
    struct App {
        toasts: Vec<Toast>,
    }

    /// The id the browser gives the next toast it pushes onto `toasts`, as
    /// the first HTML evaluates it.
    fn next_in_browser(toasts: &[Toast]) -> i32 {
        let html = Page::new(
            "Toasts",
            App {
                toasts: toasts.to_vec(),
            },
        )
        .child(span().bind_text(browser_key(App::toasts()).to_text()))
        .render();
        let end = html.find("</span>").expect("the span is rendered");
        let start = html[..end].rfind('>').expect("the span has a start tag") + 1;
        html[start..end].parse().expect("the span holds a number")
    }

    #[test]
    fn server_and_browser_toasts_never_share_an_id() {
        // A page renders the toast its button pushes once, and the button
        // pushes it on every click without calling `Toast::new` again.
        let clicked = Toast::new("From the browser");
        let mut toasts = Vec::new();
        for round in 0..6 {
            // The server appends a toast, then the browser pushes one, and
            // every third round the browser's newest toast is dismissed.
            toasts.push(Toast::new("From the server"));
            let mut pushed = clicked.clone();
            pushed.key = next_in_browser(&toasts);
            assert!(
                toasts.iter().all(|toast| toast.key != pushed.key),
                "{} is already in the list",
                pushed.key,
            );
            toasts.push(pushed);
            if round % 3 == 2 {
                toasts.pop();
            }
        }
        let keys: Vec<i32> = toasts.iter().map(|toast| toast.key).collect();
        for (index, key) in keys.iter().enumerate() {
            assert!(
                !keys[index + 1..].contains(key),
                "{key} repeats in {keys:?}"
            );
        }
    }

    #[test]
    fn the_browser_starts_at_one_above_server_toasts() {
        assert_eq!(next_in_browser(&[]), 1);
        assert_eq!(next_in_browser(&[Toast::new("a"), Toast::new("b")]), 1);
    }

    #[test]
    fn server_ids_stay_below_zero() {
        assert_eq!(server_key(0), -1);
        assert_eq!(server_key(1), -2);
        assert_eq!(server_key(u32::MAX >> 1), i32::MIN);
        assert_eq!(server_key(1 << 31), -1);
        assert!(Toast::new("a").key < 0);
    }
}
