//! Modal dialogs, opened and closed by the browser's invoker commands.

use std::fmt;
use std::marker::PhantomData;

use hypersig::html::{El, Node, Role, attr, button as button_el, div, h2, p};
use hypersig::style::pattern::{hstack, stack};
use hypersig::style::value::{
    BorderStyle, Color, FontWeight, JustifyContent, LineHeight, Position, px, raw, rem,
};
use hypersig::style::{PseudoElement, Style};
use hypersig::{DialogRef, Stmt};

use crate::button::{ButtonSize, ButtonVariant, button};
use crate::id::check_name;
use crate::look::{self, icon};
use crate::token;

/// A modal dialog on a page whose signals are `S`.
///
/// Declare one as a `const` and use the same value for the dialog and for the
/// buttons that open and close it, so they agree on its id. The buttons carry
/// `commandfor` and `command`, and the browser opens the `<dialog>` with
/// `showModal()`: it makes the rest of the page inert, so that Tab moves
/// between the dialog's controls and the browser's own and never reaches the
/// page behind, closes on Escape, and returns focus to the button that
/// opened it. The
/// browser needs invoker commands for this, which Chrome 135, Firefox 144,
/// and Safari 26.2 ship.
///
/// The dialog is labeled by its title and described by its description
/// through ids derived from the name. Two dialogs with the same name on one
/// page share an id, and rendering the page panics.
///
/// ```
/// use hypersig::Signals;
/// use hypersig::html::p;
/// use hypersig_ui::{ButtonSize, ButtonVariant, Dialog, dialog_footer};
/// use serde::{Deserialize, Serialize};
///
/// #[derive(Signals, Serialize, Deserialize)]
/// struct Profile {}
///
/// const EDIT: Dialog<Profile> = Dialog::new("edit-profile");
///
/// let open = EDIT.trigger(ButtonVariant::Outline, ButtonSize::Default).text("Edit profile");
/// let dialog = EDIT.dialog(
///     "Edit profile",
///     "Changes are saved when you press Save.",
///     dialog_footer().child(EDIT.close(ButtonVariant::Default, ButtonSize::Default).text("Save")),
/// );
/// assert!(open.render().contains(r#"command="show-modal""#));
/// assert!(dialog.render().contains(r#"aria-labelledby="edit-profile-title""#));
/// ```
pub struct Dialog<S> {
    name: &'static str,
    marker: PhantomData<fn() -> S>,
}

impl<S> Dialog<S> {
    /// A dialog whose id is `name`.
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

    /// A button that opens the dialog.
    #[must_use]
    pub fn trigger(self, variant: ButtonVariant, size: ButtonSize) -> El<S> {
        button(variant, size)
            .attr(attr::ARIA_HASPOPUP, "dialog")
            .attr(attr::COMMANDFOR, self.name)
            .attr(attr::COMMAND, "show-modal")
    }

    /// A statement that opens the dialog, as its trigger button does.
    ///
    /// Use it where no button opens the dialog, such as a keyboard shortcut
    /// from [`Page::on_key`](hypersig::Page::on_key). Focus returns to the
    /// element that had it when the dialog closes.
    #[must_use]
    pub fn open(self) -> Stmt<S> {
        DialogRef::new(self.name).show_modal()
    }

    /// A statement that closes the dialog, as its close button does.
    #[must_use]
    pub fn dismiss(self) -> Stmt<S> {
        DialogRef::new(self.name).close()
    }

    /// A button that closes the dialog. Put it inside the dialog.
    #[must_use]
    pub fn close(self, variant: ButtonVariant, size: ButtonSize) -> El<S> {
        button(variant, size)
            .attr(attr::COMMANDFOR, self.name)
            .attr(attr::COMMAND, "close")
    }

    /// The dialog, with `title`, `description`, `content`, and a close button
    /// in its corner.
    ///
    /// A click on the backdrop closes it as Escape does (`closedby="any"`),
    /// in a browser that supports `closedby`. Focus moves to the first
    /// control in `content` when it opens, and the corner button comes after
    /// `content` so that it is not that control.
    #[must_use]
    pub fn dialog(
        self,
        title: impl Into<String>,
        description: impl Into<String>,
        content: impl Into<Node<S>>,
    ) -> El<S> {
        self.closable(
            dialog_style(),
            title.into(),
            description.into(),
            content.into(),
        )
    }

    /// A sheet: a dialog along the `side` edge of the viewport, with
    /// `title`, `description`, `content`, and a close button in its corner.
    ///
    /// It is the same modal `<dialog>` as [`Dialog::dialog`], opened by
    /// [`Dialog::trigger`], with the same focus, Escape, and backdrop
    /// behavior. Only its place and size differ: it spans the full height of
    /// the viewport at the left or the right, and the full width at the top
    /// or the bottom.
    ///
    /// ```
    /// use hypersig::Signals;
    /// use hypersig::html::p;
    /// use hypersig_ui::{ButtonSize, ButtonVariant, Dialog, Side};
    /// use serde::{Deserialize, Serialize};
    ///
    /// #[derive(Signals, Serialize, Deserialize)]
    /// struct Shop {}
    ///
    /// const CART: Dialog<Shop> = Dialog::new("cart");
    ///
    /// let open = CART.trigger(ButtonVariant::Outline, ButtonSize::Default).text("Cart");
    /// let sheet = CART.sheet(Side::Right, "Cart", "Two items.", p().text("Tea, cups"));
    /// assert!(sheet.render().contains(r#"aria-labelledby="cart-title""#));
    /// # let _ = open;
    /// ```
    #[must_use]
    pub fn sheet(
        self,
        side: Side,
        title: impl Into<String>,
        description: impl Into<String>,
        content: impl Into<Node<S>>,
    ) -> El<S> {
        self.closable(
            sheet_style(side),
            title.into(),
            description.into(),
            content.into(),
        )
    }

    /// The dialog in `style`, closed by a click on the backdrop, with a close
    /// button in its corner after `content`.
    fn closable(self, style: Style, title: String, description: String, content: Node<S>) -> El<S> {
        self.frame(style, title, description, content)
            .attr(attr::CLOSEDBY, "any")
            .child(
                button_el()
                    .attr(attr::TYPE, "button")
                    .attr(attr::ARIA_LABEL, "Close")
                    .attr(attr::COMMANDFOR, self.name)
                    .attr(attr::COMMAND, "close")
                    .css(corner_style())
                    .child(icon(look::icon::X, rem(1))),
            )
    }

    /// An alert dialog, which asks the visitor to confirm or cancel before
    /// going on, with `title`, `description`, and `content`.
    ///
    /// It has `role="alertdialog"`, no corner button, and stays open on a
    /// click on the backdrop, so that the visitor answers with one of the
    /// buttons in `content`. Escape still cancels it.
    #[must_use]
    pub fn alert_dialog(
        self,
        title: impl Into<String>,
        description: impl Into<String>,
        content: impl Into<Node<S>>,
    ) -> El<S> {
        self.frame(
            dialog_style(),
            title.into(),
            description.into(),
            content.into(),
        )
        .role(Role::AlertDialog)
    }

    fn frame(self, style: Style, title: String, description: String, content: Node<S>) -> El<S> {
        let title_id = format!("{}-title", self.name);
        let description_id = format!("{}-description", self.name);
        DialogRef::new(self.name)
            .dialog()
            .id(self.name)
            .attr(attr::ARIA_LABELLEDBY, title_id.clone())
            .attr(attr::ARIA_DESCRIBEDBY, description_id.clone())
            .css(style)
            .child(
                div()
                    .css(stack(rem(1)))
                    .child(
                        div()
                            .css(stack(rem(0.5)))
                            .child(
                                h2().id(title_id)
                                    .css(
                                        Style::new()
                                            .font_size(rem(1.125))
                                            .line_height(LineHeight::Number(1.0))
                                            .font_weight(FontWeight::Number(600.0)),
                                    )
                                    .text(title),
                            )
                            .child(
                                p().id(description_id)
                                    .css(look::small_text().color(token::muted_foreground().var()))
                                    .text(description),
                            ),
                    )
                    .child(content),
            )
    }
}

impl<S> Clone for Dialog<S> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<S> Copy for Dialog<S> {}

impl<S> fmt::Debug for Dialog<S> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("Dialog").field(&self.name).finish()
    }
}

/// The row of buttons at the bottom of a dialog, aligned to the end.
#[must_use]
pub fn dialog_footer<S>() -> El<S> {
    div().css(
        hstack(rem(0.5))
            .justify_content(JustifyContent::FlexEnd)
            .flex_wrap(hypersig::style::value::FlexWrap::Wrap),
    )
}

/// The edge of the viewport a [`Dialog::sheet`] sits along.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum Side {
    /// The top edge, the full width of the viewport.
    Top,
    /// The right edge, the full height of the viewport.
    Right,
    /// The bottom edge, the full width of the viewport.
    Bottom,
    /// The left edge, the full height of the viewport.
    Left,
}

fn dialog_style() -> Style {
    // The browser hides a closed <dialog> with `display: none`, which a
    // `display` here would override, so the layout lives on the child. The
    // browser centers a modal dialog with `margin: auto`, which a reset that
    // zeroes every margin removes, so it is set again here.
    Style::new()
        .margin(raw("auto"))
        .width(raw("min(32rem, calc(100% - 2rem))"))
        .max_height(raw("calc(100% - 2rem)"))
        .merge(panel_style())
}

fn sheet_style(side: Side) -> Style {
    // A modal dialog sits in the top layer at `position: fixed; inset: 0`,
    // so the margin on the side away from `side` pushes it to that edge. The
    // browser's own `max-width` and `max-height` keep it off the edges, so
    // they are lifted.
    let (margin, width, height) = match side {
        Side::Top => ("0 0 auto 0", "100%", "auto"),
        Side::Right => ("0 0 0 auto", "min(24rem, calc(100% - 3rem))", "100%"),
        Side::Bottom => ("auto 0 0 0", "100%", "auto"),
        Side::Left => ("0 auto 0 0", "min(24rem, calc(100% - 3rem))", "100%"),
    };
    Style::new()
        .margin(raw(margin))
        .width(raw(width))
        .height(raw(height))
        .max_width(raw("100%"))
        .max_height(raw("100%"))
        .overflow_y(hypersig::style::value::OverflowY::Auto)
        .merge(panel_style())
}

/// The padding, border, colors, and backdrop a dialog and a sheet share.
fn panel_style() -> Style {
    Style::new()
        .padding(rem(1.5))
        .border_width(px(1))
        .border_style(BorderStyle::Solid)
        .border_color(token::border().var())
        .border_radius(token::radius().var())
        .background_color(token::background().var())
        .color(token::foreground().var())
        .box_shadow(raw(
            "0 10px 15px -3px rgb(0 0 0 / 0.1), 0 4px 6px -4px rgb(0 0 0 / 0.1)",
        ))
        .pseudo(
            PseudoElement::Backdrop,
            Style::new().background_color(Color::rgb(0, 0, 0).opacity(50)),
        )
}

fn corner_style() -> Style {
    Style::new()
        .position(Position::Absolute)
        .top(rem(1))
        .right(rem(1))
        .display(hypersig::style::value::Display::InlineFlex)
        .padding(px(2))
        .border_style(BorderStyle::None)
        .border_radius(px(4))
        .background_color(Color::TRANSPARENT)
        .color(token::muted_foreground().var())
        .cursor(hypersig::style::value::Cursor::Pointer)
        .merge(look::transition())
        .merge(look::focus_ring())
        .on(
            hypersig::style::PseudoClass::Hover,
            Style::new().color(token::foreground().var()),
        )
}
