//! What can stop the application from starting or running.

use std::error::Error as StdError;
use std::fmt;
use std::io;

/// Why an operation of this crate failed.
///
/// [`App::run`](crate::App::run), [`App::data_dir`](crate::App::data_dir), and
/// the methods of [`Os`](crate::Os), [`Windows`](crate::Windows),
/// [`Window`](crate::Window), [`Clipboard`](crate::Clipboard), and the
/// updater return it.
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
    /// Another copy of the application runs and did not take this start's command line.
    AlreadyRunning,
    /// The future given to [`App::run`](crate::App::run) returned an error.
    Setup,
    /// The window or its web view could not be made or could not load.
    Window,
    /// A file, the port, or the server failed.
    Io,
    /// The application is closing, so it opens and changes no window.
    Closed,
    /// A check of a smoke run failed.
    Smoke,
    /// The system clipboard could not be read or written.
    Clipboard,
    /// The system did not show a notification.
    Notification,
    /// The tray icon could not be made, or its PNG could not be read.
    Tray,
    /// [`Os::open_external`](crate::Os::open_external) got a URL without a scheme.
    ///
    /// The system would open such a URL as a file.
    NotExternal,
    /// Finding, downloading, verifying, or installing a new version failed.
    Update,
    /// [`App::run`](crate::App::run) was called inside a Tokio runtime.
    ///
    /// `run` starts a runtime of its own, so `main` is not `#[tokio::main]`.
    InRuntime,
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

    pub(crate) fn in_runtime() -> Self {
        Self {
            kind: ErrorKind::InRuntime,
            source: None,
        }
    }

    pub(crate) fn closed() -> Self {
        Self {
            kind: ErrorKind::Closed,
            source: None,
        }
    }

    pub(crate) fn smoke(why: impl Into<Box<dyn StdError + Send + Sync>>) -> Self {
        Self {
            kind: ErrorKind::Smoke,
            source: Some(why.into()),
        }
    }

    pub(crate) fn clipboard(why: impl Into<Box<dyn StdError + Send + Sync>>) -> Self {
        Self {
            kind: ErrorKind::Clipboard,
            source: Some(why.into()),
        }
    }

    pub(crate) fn tray(why: impl Into<Box<dyn StdError + Send + Sync>>) -> Self {
        Self {
            kind: ErrorKind::Tray,
            source: Some(why.into()),
        }
    }

    pub(crate) fn notification(why: impl Into<Box<dyn StdError + Send + Sync>>) -> Self {
        Self {
            kind: ErrorKind::Notification,
            source: Some(why.into()),
        }
    }

    #[cfg(feature = "updater")]
    pub(crate) fn update(why: impl Into<Box<dyn StdError + Send + Sync>>) -> Self {
        Self {
            kind: ErrorKind::Update,
            source: Some(why.into()),
        }
    }

    pub(crate) fn not_external() -> Self {
        Self {
            kind: ErrorKind::NotExternal,
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
            ErrorKind::Closed => "the application is closing",
            ErrorKind::InRuntime => {
                "`App::run` was called inside a Tokio runtime; call it from a `main` without `#[tokio::main]`"
            }
            ErrorKind::Smoke => "the smoke run failed",
            ErrorKind::Clipboard => "the clipboard could not be used",
            ErrorKind::Notification => "the notification could not be shown",
            ErrorKind::Tray => "the tray icon could not be shown",
            ErrorKind::Update => "the new version could not be found or installed",
            ErrorKind::NotExternal => {
                "the URL names no scheme, so it is not opened in another program"
            }
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
