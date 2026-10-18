//! Modal dialogs, opened and closed by the browser's invoker commands.

use std::fmt;
use std::marker::PhantomData;

use hypersig::html::{El, Node, button as button_el, div, h2, p};
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
            .attr("aria-haspopup", "dialog")
            .attr("commandfor", self.name)
            .attr("command", "show-modal")
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
            .attr("commandfor", self.name)
            .attr("command", "close")
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
        self.frame(title.into(), description.into(), content.into())
            .attr("closedby", "any")
            .child(
                button_el()
                    .attr("type", "button")
                    .attr("aria-label", "Close")
                    .attr("commandfor", self.name)
                    .attr("command", "close")
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
        self.frame(title.into(), description.into(), content.into())
            .attr("role", "alertdialog")
    }

    fn frame(self, title: String, description: String, content: Node<S>) -> El<S> {
        let title_id = format!("{}-title", self.name);
        let description_id = format!("{}-description", self.name);
        DialogRef::new(self.name)
            .dialog()
            .id(self.name)
            .attr("aria-labelledby", title_id.clone())
            .attr("aria-describedby", description_id.clone())
            .css(dialog_style())
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

fn dialog_style() -> Style {
    // The browser hides a closed <dialog> with `display: none`, which a
    // `display` here would override, so the layout lives on the child. The
    // browser centers a modal dialog with `margin: auto`, which a reset that
    // zeroes every margin removes, so it is set again here.
    Style::new()
        .margin(raw("auto"))
        .width(raw("min(32rem, calc(100% - 2rem))"))
        .max_height(raw("calc(100% - 2rem)"))
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
