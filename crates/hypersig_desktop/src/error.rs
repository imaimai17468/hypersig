//! What can stop the application from starting or running.

use std::error::Error as StdError;
use std::fmt;
use std::io;

/// Why [`App::run`](crate::App::run) or [`App::data_dir`](crate::App::data_dir) failed.
pub struct Error {
    kind: ErrorKind,
    source: Option<Box<dyn StdError + Send + Sync>>,
}

/// The kind of an [`Error`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum ErrorKind {
    /// The environment does not say where the application's directory is.
    NoDataDir,
    /// Another copy of the application holds its directory's lock.
    AlreadyRunning,
    /// The future given to [`App::run`](crate::App::run) returned an error.
    Setup,
    /// The window or its web view could not be made or could not load.
    Window,
    /// A file, the port, or the server failed.
    Io,
}

impl Error {
    /// What went wrong.
    #[must_use]
    pub fn kind(&self) -> ErrorKind {
        self.kind
    }

    pub(crate) fn no_data_dir() -> Self {
        Self {
            kind: ErrorKind::NoDataDir,
            source: None,
        }
    }

    pub(crate) fn already_running() -> Self {
        Self {
            kind: ErrorKind::AlreadyRunning,
            source: None,
        }
    }

    pub(crate) fn setup(source: Box<dyn StdError + Send + Sync>) -> Self {
        Self {
            kind: ErrorKind::Setup,
            source: Some(source),
        }
    }

    pub(crate) fn window(source: impl Into<Box<dyn StdError + Send + Sync>>) -> Self {
        Self {
            kind: ErrorKind::Window,
            source: Some(source.into()),
        }
    }
}

impl From<io::Error> for Error {
    fn from(source: io::Error) -> Self {
        Self {
            kind: ErrorKind::Io,
            source: Some(source.into()),
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let what = match self.kind {
            ErrorKind::NoDataDir => {
                "no directory for the application's files: HOME, LOCALAPPDATA, or XDG_DATA_HOME is unset or not an absolute path"
            }
            ErrorKind::AlreadyRunning => "another copy of this application is running",
            ErrorKind::Setup => "the application's setup failed",
            ErrorKind::Window => "the window could not be shown",
            ErrorKind::Io => "the application's files or server failed",
        };
        match &self.source {
            Some(source) => write!(f, "{what}: {source}"),
            None => f.write_str(what),
        }
    }
}

/// The message, as [`Display`](fmt::Display) writes it, because `main`
/// prints the `Debug` form of the error it returns.
impl fmt::Debug for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(self, f)
    }
}

impl StdError for Error {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        Some(self.source.as_deref()?)
    }
}
