//! The menu bar of macOS, which holds the shortcuts for editing text and for
//! closing and quitting.
//!
//! macOS sends Command-C, Command-V, and the other editing shortcuts to the
//! menu, not to the web view, so without these items a text field takes no
//! pasted text. Windows and Linux hand those keys to the web view themselves.

use muda::accelerator::Accelerator;
use muda::{Menu, MenuEvent, MenuItem, PredefinedMenuItem, Submenu};
use tao::event_loop::EventLoopProxy;

use crate::error::Error;
use crate::run::Event;

/// The id of the item that quits through [`Event::Quit`].
const QUIT: &str = "hypersig-desktop-quit";

/// Puts the application, Edit, and Window menus in the menu bar. The menu
/// lasts as long as the returned value.
///
/// Quitting is an item of its own instead of the one macOS provides, which
/// ends the process at once, so that it stops the server as closing the
/// window does.
pub(crate) fn install(proxy: EventLoopProxy<Event>) -> Result<Menu, Error> {
    let quit = MenuItem::with_id(
        QUIT,
        "Quit",
        true,
        Some(
            "CmdOrCtrl+Q"
                .parse::<Accelerator>()
                .expect("CmdOrCtrl+Q is an accelerator"),
        ),
    );
    let application = Submenu::with_items(
        "",
        true,
        &[
            &PredefinedMenuItem::hide(None),
            &PredefinedMenuItem::hide_others(None),
            &PredefinedMenuItem::show_all(None),
            &PredefinedMenuItem::separator(),
            &quit,
        ],
    )
    .map_err(Error::window)?;
    let edit = Submenu::with_items(
        "Edit",
        true,
        &[
            &PredefinedMenuItem::undo(None),
            &PredefinedMenuItem::redo(None),
            &PredefinedMenuItem::separator(),
            &PredefinedMenuItem::cut(None),
            &PredefinedMenuItem::copy(None),
            &PredefinedMenuItem::paste(None),
            &PredefinedMenuItem::select_all(None),
        ],
    )
    .map_err(Error::window)?;
    let window = Submenu::with_items(
        "Window",
        true,
        &[
            &PredefinedMenuItem::minimize(None),
            &PredefinedMenuItem::close_window(None),
        ],
    )
    .map_err(Error::window)?;
    let menu = Menu::with_items(&[&application, &edit, &window]).map_err(Error::window)?;
    menu.init_for_nsapp();
    MenuEvent::set_event_handler(Some(move |event: MenuEvent| {
        if event.id() == QUIT {
            let _ = proxy.send_event(Event::Quit);
        }
    }));
    Ok(menu)
}
