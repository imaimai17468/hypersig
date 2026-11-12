//! Where a window sits on the screen and how large it is, which
//! [`Window`](crate::Window)'s methods change.

use tao::dpi::{LogicalPosition, LogicalSize};
use tao::window::Fullscreen;

/// How a window fills the screen, for [`Window::place`](crate::Window::place).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[non_exhaustive]
pub enum Placement {
    /// Neither maximized, minimized, nor fullscreen, as a window opens.
    #[default]
    Normal,
    /// As large as the screen allows, beside the system's bars.
    Maximized,
    /// Hidden in the Dock, the taskbar, or the system's list of windows.
    Minimized,
    /// Over the whole screen, without the window's frame.
    Fullscreen,
}

/// Whether a window stays above the others, for [`Window::layer`](crate::Window::layer).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[non_exhaustive]
pub enum Layer {
    /// Among the other windows, as a window opens.
    #[default]
    Normal,
    /// Above the windows of every other application, even when it loses focus.
    AlwaysOnTop,
}

/// One change of a window's place, layer, position, or size.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Arrange {
    Place(Placement),
    Layer(Layer),
    /// The top-left corner, in logical pixels from the top left of the
    /// primary screen.
    Move(i32, i32),
    /// The size of the page's area, in logical pixels.
    Resize(u32, u32),
}

/// Makes `change` to `window`.
pub(crate) fn apply(window: &tao::window::Window, change: Arrange) {
    match change {
        Arrange::Place(placement) => place(window, placement),
        Arrange::Layer(layer) => window.set_always_on_top(layer == Layer::AlwaysOnTop),
        Arrange::Move(x, y) => window.set_outer_position(LogicalPosition::new(x, y)),
        Arrange::Resize(width, height) => window.set_inner_size(LogicalSize::new(width, height)),
    }
}

/// Puts `window` in `placement`, leaving the placements it is not in.
fn place(window: &tao::window::Window, placement: Placement) {
    if placement != Placement::Fullscreen {
        window.set_fullscreen(None);
    }
    window.set_minimized(placement == Placement::Minimized);
    window.set_maximized(placement == Placement::Maximized);
    if placement == Placement::Fullscreen {
        window.set_fullscreen(Some(Fullscreen::Borderless(None)));
    }
}
