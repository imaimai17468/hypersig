//! The application: its identifier, its first window, and the router it serves.

use std::env;
use std::error::Error as StdError;
use std::path::PathBuf;

use axum::Router;

use crate::error::Error;

/// A desktop application, named by a reverse domain name.
///
/// The identifier decides where [`App::data_dir`] is, and so where the
/// application keeps its port and its lock. On Windows and Linux the web
/// views keep their cookies and `localStorage` there too; on macOS,
/// `WKWebView` keeps them in `~/Library/WebKit`.
///
/// `U` is `()`, or [`Updates`](crate::Updates) once
/// [`App::updates`] has named where new versions are found, and then
/// [`App::run`] hands the setup an [`Updater`](crate::Updater) too.
#[derive(Debug, Clone, Copy)]
pub struct App<U = ()> {
    pub(crate) id: &'static str,
    pub(crate) size: (u32, u32),
    pub(crate) min_size: Option<(u32, u32)>,
    pub(crate) resizable: bool,
    pub(crate) frame: crate::Frame,
    pub(crate) devices: crate::device::Devices,
    #[cfg_attr(
        not(feature = "updater"),
        expect(dead_code, reason = "only the `run` of `App<Updates>` reads it")
    )]
    pub(crate) updates: U,
}

impl App {
    /// An application named by `id`, such as `dev.example.notes`.
    ///
    /// # Panics
    ///
    /// Panics unless `id` is two or more parts joined by dots, each of ASCII
    /// letters, digits, and hyphens. In a `const`, that is a compile error.
    #[must_use]
    pub const fn new(id: &'static str) -> Self {
        assert!(
            is_identifier(id),
            "an application identifier is a reverse domain name such as `dev.example.notes`: two or more parts joined by dots, each of ASCII letters, digits, and hyphens"
        );
        Self {
            id,
            size: (1024, 768),
            min_size: None,
            resizable: true,
            frame: crate::Frame::System,
            devices: crate::device::Devices::NONE,
            updates: (),
        }
    }

    /// Finds the application's new versions where `updates` says, with the
    /// [`Updater`](crate::Updater) [`App::run`] hands the setup.
    #[cfg(feature = "updater")]
    #[must_use]
    pub const fn updates(self, updates: crate::Updates) -> App<crate::Updates> {
        App {
            id: self.id,
            size: self.size,
            min_size: self.min_size,
            resizable: self.resizable,
            frame: self.frame,
            devices: self.devices,
            updates,
        }
    }
}

impl<U> App<U> {
    /// Opens the window `width` by `height` logical pixels large.
    ///
    /// # Panics
    ///
    /// Panics when the size is narrower or shorter than
    /// [`min_size`](Self::min_size). In a `const`, that is a compile error.
    ///
    /// ```compile_fail,E0080
    /// use hypersig_desktop::App;
    ///
    /// const APP: App = App::new("dev.example.notes").min_size(800, 600).size(640, 480);
    /// # let _ = APP;
    /// ```
    #[must_use]
    pub const fn size(mut self, width: u32, height: u32) -> Self {
        self.size = (width, height);
        self.check_sizes();
        self
    }

    /// Keeps the window at least `width` by `height` logical pixels large.
    ///
    /// # Panics
    ///
    /// Panics when [`size`](Self::size) is narrower or shorter than this. In
    /// a `const`, that is a compile error.
    #[must_use]
    pub const fn min_size(mut self, width: u32, height: u32) -> Self {
        self.min_size = Some((width, height));
        self.check_sizes();
        self
    }

    /// Checks that the window opens no smaller than it may become.
    const fn check_sizes(&self) {
        if let Some((width, height)) = self.min_size {
            assert!(
                self.size.0 >= width && self.size.1 >= height,
                "the window's `size` is at least its `min_size` in both width and height"
            );
        }
    }

    /// Whether the person can resize the window, which they can by default.
    #[must_use]
    pub const fn resizable(mut self, resizable: bool) -> Self {
        self.resizable = resizable;
        self
    }

    /// Draws `frame` around each window, which is the system's by default.
    ///
    /// With [`Frame::None`](crate::Frame::None), the page draws the title
    /// bar: [`MovesWindow::moves_window`](crate::MovesWindow::moves_window)
    /// marks the part the person drags the window by, and
    /// [`Window::close`](crate::Window::close) and
    /// [`Window::place`](crate::Window::place) answer its buttons. On macOS
    /// the window then has no close, minimize, and zoom buttons of its own,
    /// and on Linux no shadow where the window manager draws it with the
    /// border.
    #[must_use]
    pub const fn frame(mut self, frame: crate::Frame) -> Self {
        self.frame = frame;
        self
    }

    /// Lets the pages use `device` through `getUserMedia`.
    ///
    /// The web view grants a page only the devices named here, without
    /// asking the person, and refuses the others. The system still asks the
    /// person once whether the application may use the device. On macOS, the
    /// web view offers no `navigator.mediaDevices` unless the bundle's
    /// `Info.plist` has `NSMicrophoneUsageDescription` for the microphone and
    /// `NSCameraUsageDescription` for the camera, which a run outside a
    /// bundle lacks.
    ///
    /// `WebView2`, on Windows, keeps a grant in the web view's data, so a
    /// later version that no longer names the device is still granted it
    /// until that data is removed.
    #[must_use]
    pub const fn allow(mut self, device: crate::Device) -> Self {
        self.devices = self.devices.with(device);
        self
    }

    /// The directory where this application keeps its files.
    ///
    /// It is `~/Library/Application Support/<id>` on macOS,
    /// `%LOCALAPPDATA%\<id>` on Windows, and `$XDG_DATA_HOME/<id>` or
    /// `~/.local/share/<id>` elsewhere. [`App::run`] creates it before it
    /// polls the future that builds the router, so that future can open a
    /// file inside it.
    ///
    /// # Errors
    ///
    /// Fails with [`ErrorKind::NoDataDir`](crate::ErrorKind::NoDataDir) when
    /// the environment variable the directory is found from is unset or not
    /// an absolute path.
    pub fn data_dir(&self) -> Result<PathBuf, Error> {
        base_dir()
            .map(|base| base.join(self.id))
            .ok_or_else(Error::no_data_dir)
    }
}

/// What the future given to [`App::run`] returns: a router, or why there is none.
pub trait IntoRouter {
    /// The router to serve, or why there is none.
    ///
    /// # Errors
    ///
    /// Fails with the error the application's setup returned.
    fn into_router(self) -> Result<Router, Box<dyn StdError + Send + Sync>>;
}

impl IntoRouter for Router {
    fn into_router(self) -> Result<Router, Box<dyn StdError + Send + Sync>> {
        Ok(self)
    }
}

impl<E: Into<Box<dyn StdError + Send + Sync>>> IntoRouter for Result<Router, E> {
    fn into_router(self) -> Result<Router, Box<dyn StdError + Send + Sync>> {
        self.map_err(Into::into)
    }
}

/// Whether `id` is two or more non-empty parts joined by dots, each of ASCII
/// letters, digits, and hyphens, which every file system accepts as a
/// directory name.
const fn is_identifier(id: &str) -> bool {
    let bytes = id.as_bytes();
    let mut parts = 1;
    let mut part_len = 0;
    let mut i = 0;
    while i < bytes.len() {
        let byte = bytes[i];
        if byte == b'.' {
            if part_len == 0 {
                return false;
            }
            parts += 1;
            part_len = 0;
        } else if byte.is_ascii_alphanumeric() || byte == b'-' {
            part_len += 1;
        } else {
            return false;
        }
        i += 1;
    }
    parts >= 2 && part_len > 0
}

/// The directory each application's own directory is made in.
#[cfg(target_os = "macos")]
fn base_dir() -> Option<PathBuf> {
    absolute_var("HOME").map(|home| home.join("Library").join("Application Support"))
}

/// The directory each application's own directory is made in.
#[cfg(windows)]
fn base_dir() -> Option<PathBuf> {
    absolute_var("LOCALAPPDATA")
}

/// The directory each application's own directory is made in.
#[cfg(not(any(target_os = "macos", windows)))]
fn base_dir() -> Option<PathBuf> {
    absolute_var("XDG_DATA_HOME")
        .or_else(|| absolute_var("HOME").map(|home| home.join(".local").join("share")))
}

/// The value of the environment variable `name`, when it is an absolute path.
fn absolute_var(name: &str) -> Option<PathBuf> {
    env::var_os(name)
        .map(PathBuf::from)
        .filter(|path| path.is_absolute())
}

#[cfg(test)]
mod tests {
    use super::{App, is_identifier};

    #[test]
    fn a_reverse_domain_name_is_an_identifier() {
        assert!(is_identifier("dev.example.notes"));
        assert!(is_identifier("com.my-company.app2"));
    }

    #[test]
    fn other_text_is_not() {
        for id in [
            "",
            "notes",
            ".notes",
            "notes.",
            "dev..notes",
            "dev.ex ample",
            "dev/notes",
            "dev.ノート",
        ] {
            assert!(!is_identifier(id), "{id}");
        }
    }

    #[test]
    fn a_size_at_least_the_minimum_is_kept() {
        let app = App::new("dev.example.notes")
            .min_size(400, 300)
            .size(400, 500);
        assert_eq!((app.size, app.min_size), ((400, 500), Some((400, 300))));
    }

    #[test]
    #[should_panic(expected = "at least its `min_size`")]
    fn a_minimum_wider_than_the_size_panics() {
        let _ = App::new("dev.example.notes")
            .size(640, 480)
            .min_size(800, 300);
    }
}
