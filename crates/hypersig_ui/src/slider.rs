//! Sliders: a labeled native range input drawn as a track and a thumb.

use hypersig::html::{El, attr, div, input, label, span};
use hypersig::style::pattern::stack;
use hypersig::style::value::{
    AlignItems, BorderRadius, BorderStyle, Cursor, Display, JustifyContent, Opacity, PointerEvents,
    Position, pct, px, rem,
};
use hypersig::style::{Condition, PseudoClass, Style};
use hypersig::{Expr, IntoExpr, Sig};

use crate::field::label_style;
use crate::look;
use crate::token;

/// A slider labeled `label` that sets the `i32` signal `value`.
///
/// It is a native `<input type="range">`, so the browser gives it
/// `role="slider"`, the arrow keys, Page Up, Page Down, Home, and End, and
/// dragging with a pointer or a finger. The input is transparent and lies
/// over a track and a thumb drawn from the value, so the slider looks the
/// same in every browser. The value is shown beside the label, hidden from
/// assistive technology, which reads it from the input.
///
/// The thumb is drawn 1rem wide. The browser's own transparent thumb, which
/// the pointer actually drags, has the width the browser gives it, so the
/// drawn thumb can sit a few pixels off the pointer near either end.
///
/// ```
/// use hypersig::Signals;
/// use hypersig_ui::Slider;
/// use serde::{Deserialize, Serialize};
///
/// #[derive(Signals, Serialize, Deserialize)]
/// struct Player {
///     volume: i32,
/// }
///
/// let volume = Slider::new("Volume", Player::volume()).range(0, 100).step(5);
/// let html = volume.render().render();
/// assert!(html.contains(r#"type="range""#));
/// assert!(html.contains(r#"max="100""#));
/// ```
#[derive(Debug)]
pub struct Slider<S> {
    label: String,
    value: Sig<S, i32>,
    min: i32,
    max: i32,
    step: i32,
    value_text: Option<Expr<S, String>>,
    disabled: Option<Expr<S, bool>>,
}

impl<S> Slider<S> {
    /// A slider from 0 to 100 in steps of 1, bound to `value`.
    ///
    /// `label` names it.
    #[must_use]
    pub fn new(label: impl Into<String>, value: Sig<S, i32>) -> Self {
        Self {
            label: label.into(),
            value,
            min: 0,
            max: 100,
            step: 1,
            value_text: None,
            disabled: None,
        }
    }

    /// Sets the lowest and the highest value.
    ///
    /// # Panics
    ///
    /// Panics if `min` is not below `max`.
    #[must_use]
    pub fn range(mut self, min: i32, max: i32) -> Self {
        assert!(
            min < max,
            "a slider's minimum is below its maximum, not {min} and {max}"
        );
        self.min = min;
        self.max = max;
        self
    }

    /// Sets how far one arrow key press moves the value.
    ///
    /// # Panics
    ///
    /// Panics if `step` is not positive.
    #[must_use]
    pub fn step(mut self, step: i32) -> Self {
        assert!(step > 0, "a slider's step is positive, not {step}");
        self.step = step;
        self
    }

    /// Shows `text` beside the label and gives it to assistive technology.
    ///
    /// It goes in `aria-valuetext`, for a value that needs a unit, such as "40
    /// %".
    #[must_use]
    pub fn value_text(mut self, text: impl IntoExpr<S, String>) -> Self {
        self.value_text = Some(text.into_expr());
        self
    }

    /// Disables the slider while `when` is true, and dims it.
    #[must_use]
    pub fn disabled(mut self, when: impl IntoExpr<S, bool>) -> Self {
        self.disabled = Some(when.into_expr());
        self
    }

    /// The label, the shown value, and the slider, in a `<div>`.
    #[must_use]
    pub fn render(self) -> El<S> {
        let id = crate::id::of(self.value, "slider");
        let span_of_range = f64::from(self.max) - f64::from(self.min);
        // The share of the track left of the thumb, from 0 to 1.
        let share = ((self.value.to_f64() - f64::from(self.min)) / span_of_range)
            .max(0.0)
            .min(1.0);
        let shown = self
            .value_text
            .clone()
            .unwrap_or_else(|| self.value.to_text());

        let mut control = input()
            .attr(attr::TYPE, "range")
            .id(id.clone())
            .attr(attr::MIN, self.min.to_string())
            .attr(attr::MAX, self.max.to_string())
            .attr(attr::STEP, self.step.to_string())
            .bind(self.value)
            .css(input_style());
        if let Some(text) = self.value_text {
            control = control.bind_attr(attr::ARIA_VALUETEXT, text);
        }
        let mut out = div().css(stack(rem(0.75)));
        if let Some(when) = self.disabled {
            control = control.disabled(when.clone());
            // The input is transparent, so the dimming that marks a disabled
            // control goes on the drawn track and thumb around it.
            out = out.css_when(Style::new().opacity(Opacity::Number(0.5)), when);
        }

        out.child(
            div()
                .css(
                    Style::new()
                        .display(Display::Flex)
                        .align_items(AlignItems::Center)
                        .justify_content(JustifyContent::SpaceBetween),
                )
                .child(label().label_for(&id).css(label_style()).text(self.label))
                .child(
                    span()
                        .attr(attr::ARIA_HIDDEN, "true")
                        .css(look::small_text().color(token::muted_foreground().var()))
                        .bind_text(shown),
                ),
        )
        .child(
            div()
                .css(
                    Style::new()
                        .position(Position::Relative)
                        .height(rem(1))
                        .width(pct(100)),
                )
                .child(control)
                .child(
                    div()
                        .css(track_style())
                        .child(div().css(fill_style()).bind_style(
                            hypersig::style::Property::Width,
                            (&share * 100.0).to_fixed(3).concat("%"),
                        )),
                )
                .child(
                    div()
                        .css(thumb_style())
                        // The thumb's left edge travels the track minus
                        // its own width, as the browser's thumb does.
                        .bind_style(
                            hypersig::style::Property::Left,
                            Expr::text("calc((100% - 1rem) * ")
                                .concat(share.to_fixed(4))
                                .concat(")"),
                        ),
                ),
        )
    }
}

/// The transparent native input over the drawn slider, which takes the
/// pointer and the keyboard.
fn input_style() -> Style {
    Style::new()
        .position(Position::Absolute)
        .inset(px(0))
        .width(pct(100))
        .height(pct(100))
        .margin(px(0))
        .opacity(Opacity::Number(0.0))
        .cursor(Cursor::Pointer)
        .on(
            PseudoClass::Disabled,
            Style::new().cursor(Cursor::NotAllowed),
        )
        .peer()
}

fn track_style() -> Style {
    Style::new()
        .position(Position::Absolute)
        .left(px(0))
        .right(px(0))
        .top(rem(0.3125))
        .height(rem(0.375))
        .border_radius(BorderRadius::from(px(999)))
        .background_color(token::muted().var())
        .pointer_events(PointerEvents::None)
}

fn fill_style() -> Style {
    Style::new()
        .height(pct(100))
        .border_radius(BorderRadius::from(px(999)))
        .background_color(token::primary().var())
}

fn thumb_style() -> Style {
    Style::new()
        .position(Position::Absolute)
        .top(px(0))
        .width(rem(1))
        .height(rem(1))
        .border_width(px(1))
        .border_style(BorderStyle::Solid)
        .border_color(token::primary().var())
        .border_radius(BorderRadius::from(px(999)))
        .background_color(token::background().var())
        .box_shadow(hypersig::style::value::raw("0 1px 2px 0 rgb(0 0 0 / 0.05)"))
        .pointer_events(PointerEvents::None)
        .merge(look::peer_focus_ring())
        .on(
            Condition::Peer(PseudoClass::Disabled),
            Style::new().opacity(Opacity::Number(0.5)),
        )
}

impl<S> From<Slider<S>> for El<S> {
    fn from(slider: Slider<S>) -> Self {
        slider.render()
    }
}

impl<S> From<Slider<S>> for hypersig::html::Node<S> {
    fn from(slider: Slider<S>) -> Self {
        slider.render().into()
    }
}
