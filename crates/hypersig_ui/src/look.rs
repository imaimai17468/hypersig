//! Style parts that several components share, and the icons they draw.

use hypersig::html::{El, attr, el};
use hypersig::style::value::{
    Cursor, Length, LineHeight, Opacity, OutlineStyle, PointerEvents, TransitionDuration, ms, px,
    raw, rem,
};
use hypersig::style::{Condition, Media, PseudoClass, Style};

use crate::token;

/// An outline in [`token::ring`] while the element has keyboard focus.
///
/// An outline rather than a `box-shadow`, so that forced-colors mode, which
/// drops shadows, still draws it.
pub(crate) fn focus_ring() -> Style {
    Style::new().on(PseudoClass::FocusVisible, ring(px(2)))
}

/// The same outline, shown while the earlier sibling marked with
/// `Style::peer` has keyboard focus. A control drawn by a sibling of a hidden
/// native input uses it.
pub(crate) fn peer_focus_ring() -> Style {
    Style::new().on(Condition::Peer(PseudoClass::FocusVisible), ring(px(2)))
}

/// A solid outline in [`token::ring`], `offset` away from the border box.
pub(crate) fn ring(offset: Length) -> Style {
    Style::new()
        .outline_width(px(2))
        .outline_style(OutlineStyle::Solid)
        .outline_color(token::ring().var())
        .outline_offset(offset)
}

/// Half opacity and a not-allowed cursor while the element is disabled.
pub(crate) fn disabled() -> Style {
    Style::new().on(
        PseudoClass::Disabled,
        Style::new()
            .opacity(Opacity::Number(0.5))
            .cursor(Cursor::NotAllowed),
    )
}

/// Color changes fade over 150 ms, and are instant for a visitor who asks
/// for reduced motion.
pub(crate) fn transition() -> Style {
    Style::new()
        .transition_property(raw(
            "color, background-color, border-color, opacity, translate",
        ))
        .transition_duration(ms(150))
        .on(
            Media::ReducedMotion,
            Style::new().transition_duration(TransitionDuration::from(ms(0))),
        )
}

/// The small text of a label, a menu item, and a button.
pub(crate) fn small_text() -> Style {
    Style::new()
        .font_size(rem(0.875))
        .line_height(LineHeight::Length(rem(1.25)))
}

/// The paths of the icons the components draw, from Lucide (ISC license),
/// on a 24 by 24 grid.
pub(crate) mod icon {
    pub(crate) const ARROW_DOWN: &[&str] = &["M12 5v14", "m19 12-7 7-7-7"];
    pub(crate) const ARROW_UP: &[&str] = &["m5 12 7-7 7 7", "M12 19V5"];
    pub(crate) const ARROW_UP_DOWN: &[&str] =
        &["m21 16-4 4-4-4", "M17 20V4", "m3 8 4-4 4 4", "M7 4v16"];
    pub(crate) const CHECK: &[&str] = &["M20 6 9 17l-5-5"];
    pub(crate) const CHEVRON_DOWN: &[&str] = &["m6 9 6 6 6-6"];
    pub(crate) const CHEVRON_LEFT: &[&str] = &["m15 18-6-6 6-6"];
    pub(crate) const CHEVRON_RIGHT: &[&str] = &["m9 18 6-6-6-6"];
    pub(crate) const MINUS: &[&str] = &["M5 12h14"];
    pub(crate) const X: &[&str] = &["M18 6 6 18", "m6 6 12 12"];
}

/// An icon drawn in the current text color, hidden from assistive
/// technology because the element around it carries the name.
pub(crate) fn icon<S>(paths: &[&str], size: Length) -> El<S> {
    el("svg")
        .attr(hypersig::attr!("xmlns"), "http://www.w3.org/2000/svg")
        .attr(attr::VIEW_BOX, "0 0 24 24")
        .attr(attr::FILL, "none")
        .attr(attr::STROKE, "currentColor")
        .attr(attr::STROKE_WIDTH, "2")
        .attr(attr::STROKE_LINECAP, "round")
        .attr(attr::STROKE_LINEJOIN, "round")
        .attr(attr::ARIA_HIDDEN, "true")
        .attr(attr::FOCUSABLE, "false")
        .css(
            Style::new()
                .width(size.clone())
                .height(size)
                .flex_shrink(0.0)
                .pointer_events(PointerEvents::None),
        )
        .children(paths.iter().map(|d| el("path").attr(attr::D, *d)))
}
