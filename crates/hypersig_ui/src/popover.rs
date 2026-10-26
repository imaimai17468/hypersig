//! Popovers: a panel of content that a button opens beside itself.

use std::fmt;

use hypersig::NodeRef;
use hypersig::html::{El, Id, Role, attr, div};
use hypersig::style::Style;
use hypersig::style::value::{BorderStyle, PositionTryFallbacks, px, raw, rem};

use crate::button::{ButtonSize, ButtonVariant, button};
use crate::id::anchor;
use crate::token;

/// A panel on a page whose signals are `S`, opened by a button beside it.
///
/// Declare one as a `const` from a [`NodeRef`] that
/// [`node_ref!`](hypersig::node_ref) declares, and use the same value for
/// the button and the panel, so they agree on the panel's id, which is the
/// node ref's name. The panel is a `popover="auto"` with `role="dialog"`,
/// named by its button, and the button opens and closes it through invoker
/// commands with `aria-haspopup="dialog"`. The browser puts the panel above
/// the page, closes it on Escape or on a click outside, and returns focus to
/// the button when focus was inside it. Focus stays on the button as the
/// panel opens, so place the panel right after its button, where Tab moves
/// from the button into it.
///
/// The panel is placed under the button with CSS anchor positioning and
/// flips above it where there is no room below. The panel carries the node
/// ref, so placing one popover twice on a page makes
/// [`Page::render`](hypersig::Page::render) panic. Close it from a statement
/// with [`NodeRef::hide_popover`] on the same node ref.
///
/// ```
/// use hypersig::html::p;
/// use hypersig::{NodeRef, Signals};
/// use hypersig_ui::{ButtonSize, ButtonVariant, Popover};
/// use serde::{Deserialize, Serialize};
///
/// #[derive(Signals, Serialize, Deserialize)]
/// struct Profile {}
///
/// hypersig::node_ref!(const DETAILS: NodeRef<Profile>);
/// const INFO: Popover<Profile> = Popover::new(DETAILS);
///
/// # fn main() {
/// let open = INFO.trigger(ButtonVariant::Outline, ButtonSize::Default).text("Details");
/// let panel = INFO.popover().child(p().text("Joined in 2024."));
/// assert!(open.render().contains(r#"command="toggle-popover""#));
/// assert!(panel.render().contains(r#"popover="auto""#));
/// # }
/// ```
pub struct Popover<S> {
    panel: NodeRef<S>,
}

impl<S> Popover<S> {
    /// A popover that `panel` names, whose id is the name of `panel`.
    #[must_use]
    pub const fn new(panel: NodeRef<S>) -> Self {
        Self { panel }
    }

    fn id(self) -> Id {
        self.panel.id()
    }

    fn trigger_id(self) -> Id {
        self.id().with_key("trigger")
    }

    fn anchor(self) -> String {
        anchor(&self.id())
    }

    /// The button that opens and closes the panel.
    #[must_use]
    pub fn trigger(self, variant: ButtonVariant, size: ButtonSize) -> El<S> {
        button(variant, size)
            .id(self.trigger_id())
            .attr(attr::ARIA_HASPOPUP, "dialog")
            .command_for(&self.id())
            .attr(attr::COMMAND, "toggle-popover")
            .css(Style::new().anchor_name(raw(self.anchor())))
    }

    /// The panel, empty, for the content added with [`El::child`].
    #[must_use]
    pub fn popover(self) -> El<S> {
        div()
            .id(self.id())
            .attr(attr::POPOVER, "auto")
            .role(Role::Dialog)
            .aria_labelledby(&[&self.trigger_id()])
            .node_ref(self.panel)
            .css(panel_style(self.anchor()))
    }
}

impl<S> Clone for Popover<S> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<S> Copy for Popover<S> {}

impl<S> fmt::Debug for Popover<S> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("Popover").field(&self.panel).finish()
    }
}

fn panel_style(anchor: String) -> Style {
    // A popover is centered in the viewport by the browser's `inset: 0` and
    // `margin: auto`, which the anchor position replaces.
    Style::new()
        .position_anchor(raw(anchor))
        .position_area(raw("block-end span-inline-end"))
        .position_try_fallbacks(PositionTryFallbacks::FlipBlock)
        .inset(raw("auto"))
        .margin(px(0))
        .margin_top(rem(0.25))
        .width(rem(18))
        .max_width(raw("calc(100vw - 2rem)"))
        .padding(rem(1))
        .border_width(px(1))
        .border_style(BorderStyle::Solid)
        .border_color(token::border().var())
        .border_radius(px(6))
        .background_color(token::popover().var())
        .color(token::popover_foreground().var())
        .box_shadow(raw(
            "0 4px 6px -1px rgb(0 0 0 / 0.1), 0 2px 4px -2px rgb(0 0 0 / 0.1)",
        ))
}

#[cfg(test)]
mod tests {
    use hypersig::html::div;

    use super::Popover;

    struct Page;

    hypersig::node_ref!(const PANEL: NodeRef<Page>);
    const INFO: Popover<Page> = Popover::new(PANEL);

    #[test]
    fn the_panel_is_named_by_its_button() {
        let trigger = INFO
            .trigger(crate::ButtonVariant::Outline, crate::ButtonSize::Default)
            .render();
        let panel = INFO.popover().render();
        let id = "hypersig_ui::popover::tests::PANEL";
        assert!(trigger.contains(&format!(r#"id="{id}-trigger""#)));
        assert!(trigger.contains(&format!(r#"commandfor="{id}""#)));
        assert!(panel.contains(&format!(r#"aria-labelledby="{id}-trigger""#)));
        assert!(panel.contains(r#"role="dialog""#));
    }

    #[test]
    #[should_panic(
        expected = "`hypersig_ui::popover::tests::PANEL` where the runtime can reach both"
    )]
    fn one_popover_placed_twice_panics_when_rendered() {
        let _ = div::<Page>()
            .child(INFO.popover())
            .child(INFO.popover())
            .render();
    }
}
