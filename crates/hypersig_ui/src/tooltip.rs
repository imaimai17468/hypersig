//! Tooltips: a short description shown while a button is hovered or focused.

use std::fmt;
use std::marker::PhantomData;

use hypersig::NodeRef;
use hypersig::html::{El, Event, Role, attr, div};
use hypersig::style::Style;
use hypersig::style::value::{
    BackgroundClip, BorderStyle, Color, PositionTryFallbacks, px, raw, rem,
};

use crate::button::{ButtonSize, ButtonVariant, button, icon_button};
use crate::id::check_name;
use crate::token;

/// A tooltip on a page whose signals are `S`, describing the button it is
/// attached to.
///
/// Declare one as a `const` and use the same value for the button and the
/// tooltip, so they agree on the tooltip's id. It follows the WAI-ARIA
/// tooltip pattern: the tooltip has `role="tooltip"` and the button
/// `aria-describedby` pointing at it, so a screen reader reads the text
/// after the button's name. It opens while the pointer is on the button or
/// the tooltip, or the button has focus, and closes when both are left or a
/// key is pressed on the button.
///
/// The tooltip is a `popover="hint"`, which the browser closes on Escape
/// without closing an open menu. browser-compat-data 8.1.6 lists hint
/// popovers for Chrome 151 and Firefox 153, an older partial version for
/// Chrome 133 to 150 and Firefox 149 to 152, and Safari only in its
/// preview. A browser without them treats the value as `manual`, and there
/// Escape closes the tooltip only while focus is on its button.
///
/// The button carries the attributes the tooltip needs, so build it here
/// rather than adding a tooltip to a button made elsewhere. The tooltip
/// carries a node ref with its name, so two tooltips with one name on a page
/// make [`Page::render`](hypersig::Page::render) panic.
///
/// ```
/// use hypersig::Signals;
/// use hypersig_ui::{ButtonVariant, Tooltip};
/// use serde::{Deserialize, Serialize};
///
/// #[derive(Signals, Serialize, Deserialize)]
/// struct Editor {}
///
/// const BOLD: Tooltip<Editor> = Tooltip::new("bold-tip");
///
/// let button = BOLD.icon_trigger(ButtonVariant::Outline, "Bold").text("B");
/// let tip = BOLD.tooltip("Ctrl+B");
/// assert!(button.render().contains(r#"aria-describedby="bold-tip""#));
/// assert!(tip.render().contains(r#"role="tooltip""#));
/// ```
pub struct Tooltip<S> {
    name: &'static str,
    marker: PhantomData<fn() -> S>,
}

impl<S> Tooltip<S> {
    /// A tooltip whose id is `name`.
    ///
    /// # Panics
    ///
    /// Panics if `name` is empty or holds a character other than an ASCII
    /// letter, digit, `-`, or `_`. In a `const`, that is a compile error.
    #[must_use]
    pub const fn new(name: &'static str) -> Self {
        check_name(name);
        Self {
            name,
            marker: PhantomData,
        }
    }

    fn anchor(self) -> String {
        format!("--{}", self.name)
    }

    fn node_ref(self) -> NodeRef<S> {
        NodeRef::new(self.name)
    }

    /// A button described by the tooltip.
    #[must_use]
    pub fn trigger(self, variant: ButtonVariant, size: ButtonSize) -> El<S> {
        self.attach(button(variant, size))
    }

    /// An icon button named `label` and described by the tooltip.
    #[must_use]
    pub fn icon_trigger(self, variant: ButtonVariant, label: impl Into<String>) -> El<S> {
        self.attach(icon_button(variant, label))
    }

    fn attach(self, button: El<S>) -> El<S> {
        let tip = self.node_ref();
        button
            .attr(attr::ARIA_DESCRIBEDBY, self.name)
            .css(Style::new().anchor_name(raw(self.anchor())))
            .on(Event::MouseEnter, tip.show_popover())
            .on(Event::MouseLeave, tip.hide_popover())
            .on(Event::Focus, tip.show_popover())
            .on(Event::Blur, tip.hide_popover())
            // Any key closes the tooltip, Escape among them. `on_key` would
            // cancel Escape and keep a dialog around the button from closing.
            .on(Event::KeyDown, tip.hide_popover())
    }

    /// The tooltip, holding `text`.
    #[must_use]
    pub fn tooltip(self, text: impl Into<String>) -> El<S> {
        let tip = self.node_ref();
        div()
            .id(self.name)
            .attr(attr::POPOVER, "hint")
            .role(Role::Tooltip)
            .node_ref(tip)
            // The pointer moving from the button onto the tooltip leaves one
            // and enters the other in the same event, so the tooltip closes
            // and opens again before it is drawn.
            .on(Event::MouseEnter, tip.show_popover())
            .on(Event::MouseLeave, tip.hide_popover())
            .css(tooltip_style(self.anchor()))
            .text(text)
    }
}

impl<S> Clone for Tooltip<S> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<S> Copy for Tooltip<S> {}

impl<S> fmt::Debug for Tooltip<S> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("Tooltip").field(&self.name).finish()
    }
}

fn tooltip_style(anchor: String) -> Style {
    // The transparent border is the gap between the button and the visible
    // box. It belongs to the tooltip, so the pointer crossing it stays on one
    // of the two and the tooltip stays open.
    Style::new()
        .position_anchor(raw(anchor))
        .position_area(raw("block-start"))
        .position_try_fallbacks(PositionTryFallbacks::FlipBlock)
        .inset(raw("auto"))
        .margin(px(0))
        .max_width(rem(20))
        .padding_block(rem(0.375))
        .padding_inline(rem(0.75))
        .border_width(rem(0.25))
        .border_style(BorderStyle::Solid)
        .border_color(Color::TRANSPARENT)
        .background_clip(BackgroundClip::PaddingBox)
        .border_radius(rem(0.625))
        .background_color(token::primary().var())
        .color(token::primary_foreground().var())
        .font_size(rem(0.75))
        .line_height(hypersig::style::value::LineHeight::Length(rem(1)))
}
