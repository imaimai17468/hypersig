//! The window's frame, and the parts of a page that move a window without one.

use hypersig::html::{Attr, El};

/// What the system draws around a window, for [`App::frame`](crate::App::frame).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[non_exhaustive]
pub enum Frame {
    /// The system's title bar and border, with its close, minimize, and
    /// maximize buttons.
    #[default]
    System,
    /// No title bar and no border. The page draws its own, marks where the
    /// person drags the window with [`MovesWindow::moves_window`], and closes
    /// or places the window through [`Window`](crate::Window).
    None,
}

/// The attribute [`MovesWindow::moves_window`] writes, which [`SCRIPT`] looks for.
const MOVES: Attr = hypersig::attr!("data-hypersig-moves-window");

/// What the page sends the application when the person presses a part that
/// moves the window.
pub(crate) const DRAG: &str = "drag";

/// What the page sends when the person double-clicks such a part.
pub(crate) const ZOOM: &str = "zoom";

/// The script every page of a window runs before its own, which asks the
/// application to move the window when the left button goes down on a part
/// [`MovesWindow::moves_window`] marked, and to maximize or restore it on a
/// double click there. A control inside that part, such as a button, keeps
/// its own clicks.
pub(crate) const SCRIPT: &str = r#"addEventListener("mousedown", (event) => {
  if (event.button !== 0 || !(event.target instanceof Element)) return;
  const part = event.target.closest("[data-hypersig-moves-window]");
  if (part === null) return;
  const control = event.target.closest("a, button, input, select, textarea, label, [contenteditable], [tabindex]");
  if (control !== null && part.contains(control)) return;
  event.preventDefault();
  window.ipc.postMessage(event.detail === 2 ? "zoom" : "drag");
});"#;

/// A part of a page that moves the window when the person drags it.
///
/// It is implemented for hypersig's [`El`] alone.
pub trait MovesWindow: Sized + sealed::Sealed {
    /// Makes the person's drag on this element move the window.
    ///
    /// It is for a window of [`Frame::None`], whose page draws its own title
    /// bar. A double click maximizes the window or restores it, as on the
    /// system's title bar. A link, a button, a form control, or an element
    /// with `tabindex` inside the element keeps its clicks, so the bar can
    /// hold the window's own buttons. In a browser, the attribute does
    /// nothing.
    ///
    /// ```
    /// use hypersig::html::{div, span};
    /// use hypersig_desktop::MovesWindow as _;
    ///
    /// let bar = div::<()>().moves_window().child(span().text("Notes"));
    /// assert!(bar.render().contains("data-hypersig-moves-window"));
    /// ```
    #[must_use]
    fn moves_window(self) -> Self;
}

mod sealed {
    /// Keeps [`MovesWindow`](super::MovesWindow) to the types this crate implements it for.
    pub trait Sealed {}

    impl<S> Sealed for hypersig::html::El<S> {}
}

impl<S> MovesWindow for El<S> {
    fn moves_window(self) -> Self {
        self.attr(MOVES, "")
    }
}

#[cfg(test)]
mod tests {
    use hypersig::html::div;

    use super::{DRAG, MovesWindow as _, SCRIPT, ZOOM};

    #[test]
    fn the_script_sends_the_messages_the_application_answers() {
        assert!(SCRIPT.contains(&format!("\"{ZOOM}\" : \"{DRAG}\"")));
    }

    #[test]
    fn the_script_looks_for_the_attribute_the_method_writes() {
        let html = div::<()>().moves_window().render();
        assert_eq!(html, r#"<div data-hypersig-moves-window=""></div>"#);
        assert!(SCRIPT.contains("[data-hypersig-moves-window]"));
    }
}
