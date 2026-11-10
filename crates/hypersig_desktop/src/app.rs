//! The application: its identifier, its first window, and the router it serves.

use std::env;
use std::error::Error as StdError;
use std::path::PathBuf;

use axum::Router;

use crate::error::Error;

/// A desktop application, named by a reverse domain name.
///
/// The identifier decides where [`App::data_dir`] is, and so where the
/// application keeps its port, its lock, and its web view's data.
#[derive(Debug, Clone, Copy)]
pub struct App {
    pub(crate) id: &'static str,
    pub(crate) size: (u32, u32),
    pub(crate) min_size: Option<(u32, u32)>,
    pub(crate) resizable: bool,
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
        }
    }

    /// Opens the window `width` by `height` logical pixels large.
    #[must_use]
    pub const fn size(mut self, width: u32, height: u32) -> Self {
        self.size = (width, height);
        self
    }

    /// Keeps the window at least `width` by `height` logical pixels large.
    #[must_use]
    pub const fn min_size(mut self, width: u32, height: u32) -> Self {
        self.min_size = Some((width, height));
        self
    }

    /// Whether the person can resize the window, which they can by default.
    #[must_use]
    pub const fn resizable(mut self, resizable: bool) -> Self {
        self.resizable = resizable;
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
    use super::is_identifier;

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
}
