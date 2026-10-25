//! Menu buttons: a button that opens a menu of actions in a popover.

use std::fmt;
use std::marker::PhantomData;

use hypersig::html::{El, Id, Key, Orientation, Role, attr, button as button_el, div};
use hypersig::style::value::{
    AlignItems, BorderStyle, Color, Cursor, Display, PositionTryFallbacks, TextAlign, px, raw, rem,
};
use hypersig::style::{PseudoClass, Style};
use hypersig::{Expr, IntoExpr, IntoStmt, NodeRef, Stmt};

use crate::button::{ButtonSize, ButtonVariant, button};
use crate::id::check_name;
use crate::look;
use crate::token;

/// A menu of actions on a page whose signals are `S`, opened by a button.
///
/// Declare one as a `const` and use the same value for the button and the
/// menu, so they agree on the menu's id. It follows the WAI-ARIA menu button
/// pattern: the button has `aria-haspopup="menu"`, the menu `role="menu"` and
/// each action `role="menuitem"`. The menu is a `popover`, so the browser
/// puts it above the page, closes it on Escape or on a click outside, and
/// returns focus to the button. When it opens, focus moves to the first
/// action. The arrow keys, Home, and End move between actions, an action
/// runs its statement and closes the menu, and Tab closes it.
///
/// The menu is placed under the button with CSS anchor positioning and flips
/// above it where there is no room below. The menu carries a node ref with
/// its name, so two menus with one name on a page make
/// [`Page::render`](hypersig::Page::render) panic.
///
/// ```
/// use hypersig::Signals;
/// use hypersig_ui::{ButtonSize, ButtonVariant, Menu, MenuItem};
/// use serde::{Deserialize, Serialize};
///
/// #[derive(Signals, Serialize, Deserialize)]
/// struct Doc {
///     archived: bool,
/// }
///
/// const ACTIONS: Menu<Doc> = Menu::new("doc-actions");
///
/// let open = ACTIONS.trigger(ButtonVariant::Outline, ButtonSize::Default).text("Actions");
/// let menu = ACTIONS.menu([
///     MenuItem::new("Archive", Doc::archived().set(true)),
///     MenuItem::new("Delete", Doc::archived().set(false)).destructive(),
/// ]);
/// assert!(open.render().contains(r#"aria-haspopup="menu""#));
/// assert!(menu.render().contains(r#"role="menuitem""#));
/// ```
pub struct Menu<S> {
    name: &'static str,
    marker: PhantomData<fn() -> S>,
}

impl<S> Menu<S> {
    /// A menu whose id is `name`.
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

    fn id(self) -> Id {
        Id::new(self.name)
    }

    fn trigger_id(self) -> Id {
        self.id().with_key("trigger")
    }

    fn anchor(self) -> String {
        format!("--{}", self.name)
    }

    /// The button that opens and closes the menu.
    #[must_use]
    pub fn trigger(self, variant: ButtonVariant, size: ButtonSize) -> El<S> {
        button(variant, size)
            .id(self.trigger_id())
            .attr(attr::ARIA_HASPOPUP, "menu")
            .command_for(&self.id())
            .attr(attr::COMMAND, "toggle-popover")
            .css(Style::new().anchor_name(raw(self.anchor())))
    }

    /// The menu, holding `items` in order.
    #[must_use]
    pub fn menu(self, items: impl IntoIterator<Item = MenuItem<S>>) -> El<S> {
        let node_ref = NodeRef::new(self.name);
        div()
            .id(self.id())
            .attr(attr::POPOVER, "auto")
            .role(Role::Menu)
            .aria_labelledby(&[&self.trigger_id()])
            .node_ref(node_ref)
            .roving_focus(Orientation::Vertical)
            // Tab and Shift+Tab both leave the menu, which closes it.
            .on_key(Key::Tab, node_ref.hide_popover())
            .on_key(Key::Tab.shift(), node_ref.hide_popover())
            .css(menu_style(self.anchor()))
            .children(items.into_iter().enumerate().map(|(index, item)| {
                let mut action = button_el()
                    .attr(attr::TYPE, "button")
                    .role(Role::MenuItem)
                    .attr(attr::TABINDEX, "-1")
                    .css(item_style(item.destructive))
                    // The menu closes from the statement rather than from a
                    // `command`, because a statement that disables this item
                    // makes the browser skip a disabled button's command.
                    .on_click(item.run.then(node_ref.hide_popover()))
                    .text(item.label);
                // The browser focuses an element with `autofocus` inside a
                // popover as the popover opens.
                if index == 0 {
                    action = action.attr(attr::AUTOFOCUS, "");
                }
                if let Some(disabled) = item.disabled {
                    action = action.disabled(disabled);
                }
                action
            }))
    }
}

impl<S> Clone for Menu<S> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<S> Copy for Menu<S> {}

impl<S> fmt::Debug for Menu<S> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("Menu").field(&self.name).finish()
    }
}

/// One action in a [`Menu`].
#[derive(Debug)]
pub struct MenuItem<S> {
    label: String,
    run: Stmt<S>,
    destructive: bool,
    disabled: Option<Expr<S, bool>>,
}

impl<S> MenuItem<S> {
    /// An action labeled `label` that runs `run` when chosen.
    #[must_use]
    pub fn new(label: impl Into<String>, run: impl IntoStmt<S>) -> Self {
        Self {
            label: label.into(),
            run: run.into_stmt(),
            destructive: false,
            disabled: None,
        }
    }

    /// Draws the action in [`token::destructive`]. It is for an action that deletes.
    #[must_use]
    pub fn destructive(mut self) -> Self {
        self.destructive = true;
        self
    }

    /// Disables the action while `when` is true, which the arrow keys then
    /// skip.
    #[must_use]
    pub fn disabled(mut self, when: impl IntoExpr<S, bool>) -> Self {
        self.disabled = Some(when.into_expr());
        self
    }
}

fn menu_style(anchor: String) -> Style {
    // A popover is centered in the viewport by the browser's `inset: 0` and
    // `margin: auto`, which the anchor position replaces.
    Style::new()
        .position_anchor(raw(anchor))
        .position_area(raw("block-end span-inline-end"))
        .position_try_fallbacks(PositionTryFallbacks::FlipBlock)
        .inset(raw("auto"))
        .margin(px(0))
        .margin_top(rem(0.25))
        .min_width(rem(8))
        .padding(rem(0.25))
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

fn item_style(destructive: bool) -> Style {
    let text = if destructive {
        token::destructive().var()
    } else {
        token::popover_foreground().var()
    };
    Style::new()
        .display(Display::Flex)
        .align_items(AlignItems::Center)
        .gap(rem(0.5))
        .width(hypersig::style::value::pct(100))
        .padding_block(rem(0.375))
        .padding_inline(rem(0.5))
        .border_style(BorderStyle::None)
        .border_radius(px(4))
        .background_color(Color::TRANSPARENT)
        .color(text)
        .text_align(TextAlign::Start)
        .cursor(Cursor::Pointer)
        .merge(look::small_text())
        .on(
            PseudoClass::Hover,
            Style::new().background_color(token::accent().var()),
        )
        .on(
            PseudoClass::Focus,
            Style::new().background_color(token::accent().var()),
        )
        .on(PseudoClass::FocusVisible, look::ring(px(-2)))
        .merge(look::disabled())
}

#[cfg(test)]
mod tests {
    use hypersig::html::div;

    use super::Menu;

    struct Page;

    const ACTIONS: Menu<Page> = Menu::new("actions");

    #[test]
    #[should_panic(expected = "`actions` where the runtime can reach both")]
    fn two_menus_with_one_name_panic_when_rendered() {
        let _ = div::<Page>()
            .child(ACTIONS.menu([]))
            .child(ACTIONS.menu([]))
            .render();
    }
}
