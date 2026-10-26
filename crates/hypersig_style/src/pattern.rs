//! Layouts used often enough to name, each a function that returns a [`Style`].
//!
//! Each pattern is a starting point. Add to it with the property methods, or
//! merge it into another style with [`Style::merge`], and a later value for
//! the same property wins.
//!
//! ```
//! use hypersig_style::pattern::{center, hstack, stack};
//! use hypersig_style::value::{JustifyContent, rem};
//!
//! let toolbar = hstack(rem(0.5)).justify_content(JustifyContent::SpaceBetween);
//! let column = stack(rem(1));
//! let badge = center();
//! # let _ = (toolbar, column, badge);
//! ```

use crate::PseudoElement;
use crate::style::Style;
use crate::value::{
    AlignItems, BorderTopStyle, Color, Display, FlexDirection, FlexWrap, Gap, JustifyContent,
    Length, MarginInline, MaxWidth, Overflow, PaddingInline, Position, WhiteSpace, pct, px, quoted,
    raw,
};

/// Children in a column, `gap` apart.
#[must_use]
pub fn stack(gap: impl Into<Gap>) -> Style {
    Style::new()
        .display(Display::Flex)
        .flex_direction(FlexDirection::Column)
        .gap(gap)
}

/// Children in a column, centered across it, `gap` apart.
#[must_use]
pub fn vstack(gap: impl Into<Gap>) -> Style {
    stack(gap).align_items(AlignItems::Center)
}

/// Children in a row, centered on its line, `gap` apart.
#[must_use]
pub fn hstack(gap: impl Into<Gap>) -> Style {
    Style::new()
        .display(Display::Flex)
        .flex_direction(FlexDirection::Row)
        .align_items(AlignItems::Center)
        .gap(gap)
}

/// Children in rows that wrap onto new lines, `gap` apart.
#[must_use]
pub fn wrap(gap: impl Into<Gap>) -> Style {
    Style::new()
        .display(Display::Flex)
        .flex_wrap(FlexWrap::Wrap)
        .gap(gap)
}

/// Children centered on both axes.
#[must_use]
pub fn center() -> Style {
    Style::new()
        .display(Display::Flex)
        .align_items(AlignItems::Center)
        .justify_content(JustifyContent::Center)
}

/// An empty flex child that takes the free space, pushing its siblings apart.
#[must_use]
pub fn spacer() -> Style {
    Style::new().flex_grow(1).flex_shrink(1).flex_basis(px(0))
}

/// A grid of `columns` equal columns, `gap` apart.
///
/// Each column is `minmax(0, 1fr)`, so long content wraps inside its column
/// rather than widening it.
#[must_use]
pub fn grid(columns: u16, gap: impl Into<Gap>) -> Style {
    Style::new()
        .display(Display::Grid)
        .grid_template_columns(raw(format!("repeat({columns}, minmax(0, 1fr))")))
        .gap(gap)
}

/// A grid child spanning `columns` columns.
#[must_use]
pub fn grid_item(columns: u16) -> Style {
    Style::new().grid_column(raw(format!("span {columns} / span {columns}")))
}

/// A block no wider than `max`, centered, with `padding` on both sides.
#[must_use]
pub fn container(max: impl Into<MaxWidth>, padding: impl Into<PaddingInline>) -> Style {
    Style::new()
        .max_width(max)
        .margin_inline(MarginInline::Auto)
        .padding_inline(padding)
}

/// A `size` square that centers its children.
#[must_use]
pub fn square(size: Length) -> Style {
    center().width(size.clone()).height(size).flex_shrink(0)
}

/// A circle `size` across that centers its children.
#[must_use]
pub fn circle(size: Length) -> Style {
    square(size).border_radius(pct(50))
}

/// A box whose height follows its width at `ratio`.
///
/// `ratio` is width over height, and the box clips its content.
#[must_use]
pub fn aspect_ratio(ratio: f64) -> Style {
    Style::new()
        .position(Position::Relative)
        .aspect_ratio(ratio)
        .overflow(Overflow::Hidden)
}

/// A line across the box, `width` thick.
#[must_use]
pub fn divider(width: Length, color: Color) -> Style {
    Style::new()
        .border_top_width(width)
        .border_top_style(BorderTopStyle::Solid)
        .border_top_color(color)
}

/// Hidden from sight but read by screen readers.
///
/// It writes the rules that the `sr-only` utility of Tailwind CSS and Panda
/// CSS's `visuallyHidden` pattern write.
#[must_use]
pub fn visually_hidden() -> Style {
    Style::new()
        .position(Position::Absolute)
        .width(px(1))
        .height(px(1))
        .padding(px(0))
        .margin(px(-1))
        .overflow(Overflow::Hidden)
        .clip_path(raw("inset(50%)"))
        .white_space(WhiteSpace::Nowrap)
        .border_width(px(0))
}

/// Makes a link cover the nearest positioned ancestor, so a whole card is
/// clickable. Put [`aspect_ratio`] or `position: relative` on the card.
#[must_use]
pub fn link_overlay() -> Style {
    Style::new().pseudo(
        PseudoElement::Before,
        Style::new()
            .content(quoted(""))
            .position(Position::Absolute)
            .inset(px(0)),
    )
}

#[cfg(test)]
mod tests {
    use super::{circle, grid, link_overlay, stack};
    use crate::Sheet;
    use crate::value::rem;

    #[test]
    fn writes_the_declarations_of_a_pattern() {
        assert_eq!(
            stack(rem(1)).class_names().collect::<Vec<_>>(),
            ["display_flex", "flex-direction_column", "gap_1rem"]
        );
        assert!(
            circle(rem(2))
                .class_names()
                .any(|name| name == "border-radius_50%")
        );
        let mut sheet = Sheet::new();
        sheet.add(&grid(3, rem(1)));
        assert!(
            sheet
                .to_css()
                .contains("{grid-template-columns:repeat(3, minmax(0, 1fr))}")
        );
    }

    #[test]
    fn puts_the_overlay_on_a_pseudo_element() {
        let mut sheet = Sheet::new();
        sheet.add(&link_overlay());
        let css = sheet.to_css();
        assert!(css.contains("::before{content:\"\"}"));
        assert!(css.contains("::before{inset:0}"));
    }
}
