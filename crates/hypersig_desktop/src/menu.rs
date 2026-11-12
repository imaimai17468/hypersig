//! The menu bar: the menus the application builds with [`Menu`], and on
//! macOS the application, Edit, and Window menus around them.
//!
//! macOS sends Command-C, Command-V, and the other editing shortcuts to the
//! menu, not to the web view, so without the Edit menu a text field takes no
//! pasted text. Windows and Linux hand those keys to the web view themselves.

use crate::item::MenuItem;

/// A menu of the menu bar, for [`Os::menus`](crate::Os::menus).
#[derive(Debug, Clone)]
#[must_use = "a menu shows only when it is given to `Os::menus`"]
pub struct Menu {
    pub(crate) title: String,
    pub(crate) entries: Vec<Entry>,
}

/// One line of a [`Menu`].
#[derive(Debug, Clone)]
pub(crate) enum Entry {
    Item(MenuItem),
    Separator,
}

impl Menu {
    /// A menu titled `title`, such as "File", with no items yet.
    pub fn new(title: impl Into<String>) -> Self {
        Self {
            title: title.into(),
            entries: Vec::new(),
        }
    }

    /// Adds `item` below the entries added before it.
    pub fn item(mut self, item: &MenuItem) -> Self {
        self.entries.push(Entry::Item(item.clone()));
        self
    }

    /// Adds a line between the entries before it and those after it.
    pub fn separator(mut self) -> Self {
        self.entries.push(Entry::Separator);
        self
    }

    /// The title the menu shows in the menu bar.
    #[must_use]
    pub fn title(&self) -> &str {
        &self.title
    }

    /// The labels of the menu's items, in order, without the separators.
    #[must_use]
    pub fn labels(&self) -> Vec<&str> {
        self.items().map(|item| item.label.as_str()).collect()
    }

    /// The menu's items, in order.
    pub(crate) fn items(&self) -> impl Iterator<Item = &MenuItem> {
        self.entries.iter().filter_map(|entry| match entry {
            Entry::Item(item) => Some(item),
            Entry::Separator => None,
        })
    }
}

#[cfg(target_os = "macos")]
pub(crate) use system::QUIT;
#[cfg(any(target_os = "macos", target_os = "linux"))]
pub(crate) use system::bar;
#[cfg(target_os = "linux")]
pub(crate) use system::{attach, detach};

/// The menu bar as muda builds it, on the systems where this crate shows one.
#[cfg(any(target_os = "macos", target_os = "linux"))]
mod system {
    use muda::{MenuItem as SystemItem, PredefinedMenuItem, Submenu};

    use super::{Entry, Menu};
    use crate::error::Error;

    /// The id of the item that quits, which the event loop answers.
    #[cfg(target_os = "macos")]
    pub(crate) const QUIT: &str = "hypersig-desktop-quit";

    /// The menu bar holding `menus`, and on macOS the application and Edit
    /// menus before them and the Window menu after them. The menu bar lasts
    /// as long as the returned value.
    ///
    /// On macOS, quitting is an item of its own instead of the one macOS
    /// provides, which ends the process at once, so that it stops the server
    /// as closing the window does.
    pub(crate) fn bar(menus: &[Menu]) -> Result<muda::Menu, Error> {
        let bar = muda::Menu::new();
        #[cfg(target_os = "macos")]
        for leading in [application()?, edit()?] {
            bar.append(&leading).map_err(Error::window)?;
        }
        for menu in menus {
            bar.append(&submenu(menu)?).map_err(Error::window)?;
        }
        #[cfg(target_os = "macos")]
        bar.append(&window()?).map_err(Error::window)?;
        #[cfg(target_os = "macos")]
        bar.init_for_nsapp();
        Ok(bar)
    }

    /// Puts `bar` at the top of `window`, above the web view, unless it
    /// holds no menu.
    #[cfg(target_os = "linux")]
    pub(crate) fn attach(bar: &muda::Menu, window: &tao::window::Window) {
        use tao::platform::unix::WindowExtUnix as _;
        if bar.items().is_empty() {
            return;
        }
        if let Err(error) = bar.init_for_gtk_window(window.gtk_window(), window.default_vbox()) {
            tracing::error!(%error, "the menu bar was not put in a window");
        }
    }

    /// Takes `bar` out of `window`, where [`attach`] put it.
    #[cfg(target_os = "linux")]
    pub(crate) fn detach(bar: &muda::Menu, window: &tao::window::Window) {
        use tao::platform::unix::WindowExtUnix as _;
        if !bar.items().is_empty() {
            let _ = bar.remove_for_gtk_window(window.gtk_window());
        }
    }

    /// `menu` as a submenu of the bar.
    fn submenu(menu: &Menu) -> Result<Submenu, Error> {
        let submenu = Submenu::new(&menu.title, true);
        for entry in &menu.entries {
            match entry {
                Entry::Item(item) => submenu
                    .append(&SystemItem::with_id(
                        item.id.clone(),
                        &item.label,
                        true,
                        None,
                    ))
                    .map_err(Error::window)?,
                Entry::Separator => submenu
                    .append(&PredefinedMenuItem::separator())
                    .map_err(Error::window)?,
            }
        }
        Ok(submenu)
    }

    #[cfg(target_os = "macos")]
    fn application() -> Result<Submenu, Error> {
        let quit = SystemItem::with_id(
            QUIT,
            "Quit",
            true,
            Some(
                "CmdOrCtrl+Q"
                    .parse::<muda::accelerator::Accelerator>()
                    .expect("CmdOrCtrl+Q is an accelerator"),
            ),
        );
        Submenu::with_items(
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
        .map_err(Error::window)
    }

    #[cfg(target_os = "macos")]
    fn edit() -> Result<Submenu, Error> {
        Submenu::with_items(
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
        .map_err(Error::window)
    }

    #[cfg(target_os = "macos")]
    fn window() -> Result<Submenu, Error> {
        Submenu::with_items(
            "Window",
            true,
            &[
                &PredefinedMenuItem::minimize(None),
                &PredefinedMenuItem::close_window(None),
            ],
        )
        .map_err(Error::window)
    }
}

#[cfg(test)]
mod tests {
    use super::Menu;
    use crate::MenuItem;

    #[test]
    fn a_menu_keeps_its_items_in_order_without_the_separators() {
        let open = MenuItem::new("Open");
        let save = MenuItem::new("Save");
        let file = Menu::new("File").item(&open).separator().item(&save);
        assert_eq!(file.title(), "File");
        assert_eq!(file.labels(), ["Open", "Save"]);
    }
}
