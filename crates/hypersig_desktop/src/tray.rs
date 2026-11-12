//! The icon in the system tray, with a menu of items whose clicks reach the
//! application: the menu bar on macOS, the notification area on Windows,
//! and the `StatusNotifierItem` tray of the desktop on Linux.

use std::fmt;
use std::sync::Arc;

use tray_icon::menu::{Menu, MenuItem as SystemItem, PredefinedMenuItem};
use tray_icon::{Icon, TrayIcon, TrayIconBuilder};

use crate::error::Error;
use crate::item::MenuItem;

/// An icon in the system tray with a menu, for [`Os::tray`](crate::Os::tray).
///
/// While it is shown, closing the last window leaves the application
/// running, and its Quit item ends it.
#[derive(Clone)]
#[must_use = "a tray shows only when it is given to `Os::tray`"]
pub struct Tray {
    pub(crate) icon: Arc<[u8]>,
    pub(crate) quit: String,
    pub(crate) tooltip: Option<String>,
    pub(crate) items: Vec<MenuItem>,
}

impl fmt::Debug for Tray {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Tray")
            .field("quit", &self.quit)
            .field("tooltip", &self.tooltip)
            .field("items", &self.items)
            .finish_non_exhaustive()
    }
}

impl Tray {
    /// A tray icon drawn from `png`, whose menu ends with an item labeled
    /// `quit` that ends the application.
    pub fn new(png: impl Into<Arc<[u8]>>, quit: impl Into<String>) -> Self {
        Self {
            icon: png.into(),
            quit: quit.into(),
            tooltip: None,
            items: Vec::new(),
        }
    }

    /// Shows `text` while the pointer rests on the icon.
    pub fn tooltip(mut self, text: impl Into<String>) -> Self {
        self.tooltip = Some(text.into());
        self
    }

    /// Adds `item` to the menu, above the items added after it.
    pub fn item(mut self, item: &MenuItem) -> Self {
        self.items.push(item.clone());
        self
    }

    /// The labels of the menu's items, the Quit item last.
    #[must_use]
    pub fn labels(&self) -> Vec<&str> {
        self.items
            .iter()
            .map(|item| item.label.as_str())
            .chain(std::iter::once(self.quit.as_str()))
            .collect()
    }
}

/// An item of a [`Tray`]'s menu, the same type as a [`MenuItem`] of the menu bar.
pub type TrayItem = MenuItem;

/// The id of the Quit item of every tray.
pub(crate) const QUIT: &str = "hypersig-desktop-tray-quit";

/// Makes the tray icon `tray` describes. The icon lasts as long as the
/// returned value, and its menu's clicks arrive as `muda::MenuEvent`s.
///
/// # Errors
///
/// Fails when `png` is not a PNG, and when the system has no tray to put
/// the icon in.
pub(crate) fn show(tray: &Tray) -> Result<TrayIcon, Error> {
    let icon = decode(&tray.icon)?;
    let menu = Menu::new();
    for item in &tray.items {
        menu.append(&SystemItem::with_id(
            item.id.clone(),
            &item.label,
            true,
            None,
        ))
        .map_err(Error::tray)?;
    }
    if !tray.items.is_empty() {
        menu.append(&PredefinedMenuItem::separator())
            .map_err(Error::tray)?;
    }
    menu.append(&SystemItem::with_id(QUIT, &tray.quit, true, None))
        .map_err(Error::tray)?;
    let mut builder = TrayIconBuilder::new()
        .with_icon(icon)
        .with_menu(Box::new(menu));
    if let Some(tooltip) = &tray.tooltip {
        builder = builder.with_tooltip(tooltip);
    }
    builder.build().map_err(Error::tray)
}

/// The pixels of the PNG `bytes`, as the tray takes them.
fn decode(bytes: &[u8]) -> Result<Icon, Error> {
    let mut decoder = png::Decoder::new(std::io::Cursor::new(bytes));
    decoder.set_transformations(
        png::Transformations::normalize_to_color8() | png::Transformations::ALPHA,
    );
    let mut reader = decoder.read_info().map_err(Error::tray)?;
    let mut pixels = vec![
        0;
        reader
            .output_buffer_size()
            .ok_or_else(|| Error::tray("the PNG is too large"))?
    ];
    let frame = reader.next_frame(&mut pixels).map_err(Error::tray)?;
    pixels.truncate(frame.buffer_size());
    let rgba = match frame.color_type {
        png::ColorType::Rgba => pixels,
        png::ColorType::GrayscaleAlpha => pixels
            .as_chunks::<2>()
            .0
            .iter()
            .flat_map(|&[gray, alpha]| [gray, gray, gray, alpha])
            .collect(),
        other => {
            return Err(Error::tray(format!(
                "the PNG's pixels are {other:?}, not RGBA"
            )));
        }
    };
    Icon::from_rgba(rgba, frame.width, frame.height).map_err(Error::tray)
}

#[cfg(test)]
mod tests {
    use super::{Tray, TrayItem, decode};

    /// A 2 by 2 red PNG.
    fn red() -> Vec<u8> {
        let mut bytes = Vec::new();
        let mut encoder = png::Encoder::new(&mut bytes, 2, 2);
        encoder.set_color(png::ColorType::Rgba);
        encoder.set_depth(png::BitDepth::Eight);
        let mut writer = encoder
            .write_header()
            .unwrap_or_else(|error| panic!("{error}"));
        writer
            .write_image_data(&[255, 0, 0, 255].repeat(4))
            .unwrap_or_else(|error| panic!("{error}"));
        drop(writer);
        bytes
    }

    #[test]
    fn a_png_is_decoded_and_bytes_that_are_not_one_are_refused() {
        decode(&red()).unwrap_or_else(|error| panic!("{error}"));
        decode(b"not a png").map(|_| ()).unwrap_err();
    }

    #[test]
    fn the_quit_item_comes_last() {
        let show = TrayItem::new("Show");
        let tray = Tray::new(red(), "Quit").item(&show);
        assert_eq!(tray.labels(), ["Show", "Quit"]);
        assert_ne!(TrayItem::new("A").id, TrayItem::new("A").id);
    }
}
