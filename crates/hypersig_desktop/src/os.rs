//! What the application's handlers ask of the operating system.

use std::fmt;
use std::path::PathBuf;
#[cfg(feature = "testing")]
use std::sync::{Arc, Mutex, PoisonError};

use hypersig::{SafeUrl, Url};
use tao::event_loop::EventLoopProxy;
use tao::window::WindowId;
use tokio::sync::{broadcast, oneshot};

use crate::clipboard;
use crate::error::Error;
use crate::handoff::{Launch, Launches};
use crate::message::{Answer, Message};
use crate::notification::{Notification, Notified};
use crate::run::Event;
use crate::shortcut::{Presses, Shortcut};
#[cfg(feature = "testing")]
use crate::testing::Fake;
use crate::tray::Tray;

/// The operating system's windows and services, for the application's handlers.
///
/// [`App::run`](crate::App::run) gives it to the setup closure, which puts it
/// in the router's state, so a handler reaches it as `state.os`. A test gives
/// the handlers `testing::FakeOs::os` instead, with the
/// `testing` feature.
#[derive(Clone)]
pub struct Os {
    inner: Inner,
    launches: broadcast::Sender<Launch>,
}

#[derive(Clone)]
enum Inner {
    System(EventLoopProxy<Event>),
    #[cfg(feature = "testing")]
    Fake(Arc<Mutex<Fake>>),
}

impl fmt::Debug for Os {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Os").finish_non_exhaustive()
    }
}

impl Os {
    pub(crate) fn new(proxy: EventLoopProxy<Event>, launches: broadcast::Sender<Launch>) -> Self {
        Self {
            inner: Inner::System(proxy),
            launches,
        }
    }

    #[cfg(feature = "testing")]
    pub(crate) fn fake(fake: Arc<Mutex<Fake>>) -> Self {
        let launches = lock(&fake).launches.clone();
        Self {
            inner: Inner::Fake(fake),
            launches,
        }
    }

    /// The launches of the application that start while it runs.
    ///
    /// A start of the application while this copy runs hands its command
    /// line and working directory over and exits, and the copy that runs
    /// brings a window forward. Each [`Launches`] gets the launches handed
    /// over after it was made, so a setup that takes one before it returns
    /// the router misses none.
    #[must_use]
    pub fn launches(&self) -> Launches {
        Launches::new(self.launches.subscribe())
    }

    /// Shows `notification` among the system's notifications.
    ///
    /// The returned [`Notified`] tells whether the person clicked it. On
    /// macOS, the notification comes from the application installed with
    /// the identifier [`App::new`](crate::App::new) names; a run outside
    /// such a bundle shows it under another application's name.
    ///
    /// # Errors
    ///
    /// Fails with [`ErrorKind::Notification`](crate::ErrorKind::Notification)
    /// when the system does not show it, as on Linux without a notification
    /// server.
    pub async fn notify(&self, notification: Notification) -> Result<Notified, Error> {
        match &self.inner {
            Inner::System(_) => {
                tokio::task::spawn_blocking(move || crate::notification::show(&notification))
                    .await
                    .map_err(Error::notification)?
            }
            #[cfg(feature = "testing")]
            Inner::Fake(fake) => {
                let (clicked, heard) = oneshot::channel();
                lock(fake).notifications.push((notification, Some(clicked)));
                Ok(Notified { clicked: heard })
            }
        }
    }

    /// Registers `shortcut` for the rest of the run, so the application
    /// hears it while another application has focus.
    ///
    /// Each call returns its own [`Presses`] of the presses and releases
    /// after it, and a shortcut asked for again is not registered twice. On
    /// Linux, the shortcut is grabbed from the X server, so a Wayland session
    /// delivers it only while an X11 window has focus, and none at all
    /// without Xwayland, where registering still returns `Ok`.
    ///
    /// # Errors
    ///
    /// Fails with [`ErrorKind::Shortcut`](crate::ErrorKind::Shortcut) when
    /// the system does not register it, as when another application holds
    /// it, and when the application is closing.
    pub async fn shortcut(&self, shortcut: Shortcut) -> Result<Presses, Error> {
        match &self.inner {
            Inner::System(proxy) => {
                let (answer, answered) = oneshot::channel();
                Os::send(proxy, Event::Shortcut { shortcut, answer })?;
                answered.await.map_err(|_closed| Error::closed())?
            }
            #[cfg(feature = "testing")]
            Inner::Fake(fake) => Ok(Presses::new(
                lock(fake)
                    .shortcuts
                    .entry(shortcut)
                    .or_insert_with(crate::shortcut::channel)
                    .subscribe(),
            )),
        }
    }

    /// Shows `tray` in the system tray, in place of the tray shown before.
    ///
    /// While a tray is shown, closing the last window leaves the
    /// application running, and the tray's Quit item ends it.
    ///
    /// # Errors
    ///
    /// Fails with [`ErrorKind::Tray`](crate::ErrorKind::Tray) when the
    /// tray's icon is not a PNG or the system has no tray, and when the
    /// application is closing.
    pub async fn tray(&self, tray: Tray) -> Result<(), Error> {
        match &self.inner {
            Inner::System(proxy) => {
                let (answer, answered) = oneshot::channel();
                Os::send(proxy, Event::Tray { tray, answer })?;
                answered.await.map_err(|_closed| Error::closed())?
            }
            #[cfg(feature = "testing")]
            Inner::Fake(fake) => {
                lock(fake).tray = Some(tray);
                Ok(())
            }
        }
    }

    /// Shows `count` on the application's icon, or clears it at 0.
    ///
    /// macOS writes the number on the Dock icon. Linux sets it on the
    /// launcher entry of `<program>.desktop`, the desktop file of the
    /// packages cargo-packager builds, which Ubuntu's dock shows. Windows
    /// puts a red dot on each window's taskbar button, since a button has
    /// no room for a number.
    ///
    /// # Errors
    ///
    /// Fails when the application is closing.
    pub fn badge(&self, count: u32) -> Result<(), Error> {
        match &self.inner {
            Inner::System(proxy) => Os::send(proxy, Event::Badge(count)),
            #[cfg(feature = "testing")]
            Inner::Fake(fake) => {
                lock(fake).badge = count;
                Ok(())
            }
        }
    }

    /// The application's windows.
    #[must_use]
    pub fn windows(&self) -> Windows<'_> {
        Windows { os: self }
    }

    /// A file dialog, to be given a title and filters before it is shown.
    pub fn dialog(&self) -> Dialog {
        Dialog {
            #[cfg(feature = "testing")]
            os: self.clone(),
            title: None,
            filters: Vec::new(),
            directory: None,
            file_name: None,
        }
    }

    /// Shows `message` in the system's message box, with an OK button,
    /// and waits for the person to close it.
    pub async fn tell(&self, message: Message) {
        match &self.inner {
            Inner::System(_) => {
                message.system(rfd::MessageButtons::Ok).show().await;
            }
            #[cfg(feature = "testing")]
            Inner::Fake(fake) => lock(fake).tell(message),
        }
    }

    /// Shows `message` with OK and Cancel buttons, and returns whether the
    /// person chose OK. Closing the box counts as Cancel.
    pub async fn confirm(&self, message: Message) -> bool {
        match &self.inner {
            Inner::System(_) => {
                let chosen = message.system(rfd::MessageButtons::OkCancel).show().await;
                chosen == rfd::MessageDialogResult::Ok
            }
            #[cfg(feature = "testing")]
            Inner::Fake(fake) => lock(fake).confirm(message),
        }
    }

    /// Shows `message` with Yes, No, and Cancel buttons, and returns the
    /// one the person chose. Closing the box counts as Cancel.
    pub async fn ask(&self, message: Message) -> Answer {
        match &self.inner {
            Inner::System(_) => {
                let chosen = message
                    .system(rfd::MessageButtons::YesNoCancel)
                    .show()
                    .await;
                Answer::from_system(&chosen)
            }
            #[cfg(feature = "testing")]
            Inner::Fake(fake) => lock(fake).ask(message),
        }
    }

    /// The system clipboard.
    #[must_use]
    pub fn clipboard(&self) -> Clipboard<'_> {
        Clipboard { os: self }
    }

    /// Opens `url` in the person's web browser, or their mail program for a
    /// `mailto:` URL.
    ///
    /// # Errors
    ///
    /// Fails with [`ErrorKind::NotExternal`](crate::ErrorKind::NotExternal)
    /// when `url` names no scheme, as `/notes` does, since the system would
    /// open such a path as a file, and fails when the system cannot open it.
    pub fn open_external(&self, url: &SafeUrl) -> Result<(), Error> {
        if !is_external(url.as_str()) {
            return Err(Error::not_external());
        }
        match &self.inner {
            Inner::System(_) => open::that_detached(url.as_str()).map_err(Error::from),
            #[cfg(feature = "testing")]
            Inner::Fake(fake) => {
                lock(fake).external.push(url.clone());
                Ok(())
            }
        }
    }

    /// Sends `event` to the event loop.
    fn send(proxy: &EventLoopProxy<Event>, event: Event) -> Result<(), Error> {
        proxy.send_event(event).map_err(|_closed| Error::closed())
    }
}

/// Whether `url` is one the system opens in another program: an absolute
/// `http`, `https`, or `mailto` URL. A [`SafeUrl`] has one of these schemes
/// or none.
pub(crate) fn is_external(url: &str) -> bool {
    ["http://", "https://", "mailto:"].iter().any(|scheme| {
        url.get(..scheme.len())
            .is_some_and(|start| start.eq_ignore_ascii_case(scheme))
    })
}

#[cfg(feature = "testing")]
fn lock(fake: &Mutex<Fake>) -> std::sync::MutexGuard<'_, Fake> {
    fake.lock().unwrap_or_else(PoisonError::into_inner)
}

/// The application's windows, from [`Os::windows`].
#[derive(Debug, Clone, Copy)]
pub struct Windows<'a> {
    os: &'a Os,
}

impl Windows<'_> {
    /// Opens `page` in a new window, shown once the page has loaded.
    ///
    /// # Errors
    ///
    /// Fails when the window or its web view cannot be made, and when the
    /// application is closing.
    pub async fn open(&self, page: &Url) -> Result<Window, Error> {
        let key = match &self.os.inner {
            Inner::System(proxy) => {
                let (answer, answered) = oneshot::channel();
                Os::send(
                    proxy,
                    Event::Open {
                        url: page.as_str().to_owned(),
                        answer: Some(answer),
                    },
                )?;
                Key::System(answered.await.map_err(|_closed| Error::closed())??)
            }
            #[cfg(feature = "testing")]
            Inner::Fake(fake) => {
                let mut fake = lock(fake);
                fake.windows.push(Some(page.clone()));
                let index = fake.windows.len() - 1;
                // A system window comes to the front when it is shown.
                fake.focused = Some(index);
                Key::Fake(index)
            }
        };
        Ok(Window {
            key,
            os: self.os.clone(),
        })
    }
}

/// A window [`Windows::open`] opened.
#[derive(Debug, Clone)]
pub struct Window {
    key: Key,
    os: Os,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Key {
    System(WindowId),
    #[cfg(feature = "testing")]
    Fake(usize),
}

impl Window {
    /// Closes the window, as its close button does.
    ///
    /// On a window that is already closed, it does nothing and returns
    /// `Ok(())`.
    ///
    /// # Errors
    ///
    /// Fails when the application is closing.
    pub fn close(&self) -> Result<(), Error> {
        match (&self.os.inner, self.key) {
            (Inner::System(proxy), Key::System(id)) => Os::send(proxy, Event::Close(id)),
            #[cfg(feature = "testing")]
            (Inner::Fake(fake), Key::Fake(index)) => {
                if let Some(window) = lock(fake).windows.get_mut(index) {
                    *window = None;
                }
                Ok(())
            }
            #[cfg(feature = "testing")]
            _ => Ok(()),
        }
    }

    /// Brings the window to the front and gives it the keyboard.
    ///
    /// On a window that is already closed, it does nothing and returns
    /// `Ok(())`.
    ///
    /// # Errors
    ///
    /// Fails when the application is closing.
    pub fn focus(&self) -> Result<(), Error> {
        match (&self.os.inner, self.key) {
            (Inner::System(proxy), Key::System(id)) => Os::send(proxy, Event::Focus(id)),
            #[cfg(feature = "testing")]
            (Inner::Fake(fake), Key::Fake(index)) => {
                let mut fake = lock(fake);
                if fake.windows.get(index).is_some_and(Option::is_some) {
                    fake.focused = Some(index);
                }
                Ok(())
            }
            #[cfg(feature = "testing")]
            _ => Ok(()),
        }
    }
}

impl PartialEq for Window {
    fn eq(&self, other: &Self) -> bool {
        self.key == other.key
    }
}

impl Eq for Window {}

/// A file dialog, from [`Os::dialog`], shown by one of its async methods.
#[derive(Debug, Clone)]
#[must_use = "a dialog shows only when one of its async methods is awaited"]
pub struct Dialog {
    /// Only a fake system answers in place of the system's own dialog.
    #[cfg(feature = "testing")]
    os: Os,
    title: Option<String>,
    filters: Vec<(String, Vec<String>)>,
    directory: Option<PathBuf>,
    file_name: Option<String>,
}

impl Dialog {
    /// Titles the dialog's window.
    pub fn title(mut self, title: impl Into<String>) -> Self {
        self.title = Some(title.into());
        self
    }

    /// Offers the files whose extension is one of `extensions`, as `name`.
    pub fn filter(mut self, name: impl Into<String>, extensions: &[&str]) -> Self {
        self.filters.push((
            name.into(),
            extensions
                .iter()
                .map(|&extension| extension.to_owned())
                .collect(),
        ));
        self
    }

    /// Opens the dialog in `directory`.
    pub fn directory(mut self, directory: impl Into<PathBuf>) -> Self {
        self.directory = Some(directory.into());
        self
    }

    /// Fills in `name` as the file to save.
    pub fn file_name(mut self, name: impl Into<String>) -> Self {
        self.file_name = Some(name.into());
        self
    }

    /// The file the person chose, or `None` when they cancelled.
    pub async fn open_file(self) -> Option<PathBuf> {
        #[cfg(feature = "testing")]
        if let Some(paths) = self.fake_answer(crate::testing::DialogKind::OpenFile) {
            return paths?.into_iter().next();
        }
        self.system()
            .pick_file()
            .await
            .map(|file| file.path().to_owned())
    }

    /// The files the person chose, or `None` when they cancelled.
    pub async fn open_files(self) -> Option<Vec<PathBuf>> {
        #[cfg(feature = "testing")]
        if let Some(paths) = self.fake_answer(crate::testing::DialogKind::OpenFiles) {
            return paths;
        }
        self.system()
            .pick_files()
            .await
            .map(|files| files.iter().map(|file| file.path().to_owned()).collect())
    }

    /// The folder the person chose, or `None` when they cancelled.
    pub async fn pick_folder(self) -> Option<PathBuf> {
        #[cfg(feature = "testing")]
        if let Some(paths) = self.fake_answer(crate::testing::DialogKind::PickFolder) {
            return paths?.into_iter().next();
        }
        self.system()
            .pick_folder()
            .await
            .map(|folder| folder.path().to_owned())
    }

    /// Where the person chose to save, or `None` when they cancelled. The
    /// file is not written.
    pub async fn save_file(self) -> Option<PathBuf> {
        #[cfg(feature = "testing")]
        if let Some(paths) = self.fake_answer(crate::testing::DialogKind::SaveFile) {
            return paths?.into_iter().next();
        }
        self.system()
            .save_file()
            .await
            .map(|file| file.path().to_owned())
    }

    /// The answer a test gave a fake system for this dialog: the paths, or
    /// `None` for a cancelled dialog. The outer `None` means the system's
    /// own dialog answers.
    #[cfg(feature = "testing")]
    #[expect(
        clippy::option_option,
        reason = "the outer layer is who answers and the inner one is the answer"
    )]
    fn fake_answer(&self, kind: crate::testing::DialogKind) -> Option<Option<Vec<PathBuf>>> {
        match &self.os.inner {
            Inner::System(_) => None,
            Inner::Fake(fake) => Some(lock(fake).answer_dialog(crate::testing::ShownDialog {
                kind,
                title: self.title.clone(),
                filters: self.filters.clone(),
                directory: self.directory.clone(),
                file_name: self.file_name.clone(),
            })),
        }
    }

    fn system(self) -> rfd::AsyncFileDialog {
        let mut dialog = rfd::AsyncFileDialog::new();
        if let Some(title) = self.title {
            dialog = dialog.set_title(title);
        }
        for (name, extensions) in self.filters {
            dialog = dialog.add_filter(name, &extensions);
        }
        if let Some(directory) = self.directory {
            dialog = dialog.set_directory(directory);
        }
        if let Some(name) = self.file_name {
            dialog = dialog.set_file_name(name);
        }
        dialog
    }
}

/// The system clipboard, from [`Os::clipboard`].
#[derive(Debug, Clone, Copy)]
pub struct Clipboard<'a> {
    os: &'a Os,
}

impl Clipboard<'_> {
    /// The text on the clipboard, or `None` when it holds no text.
    ///
    /// # Errors
    ///
    /// Fails when the system clipboard cannot be read.
    pub async fn read_text(&self) -> Result<Option<String>, Error> {
        match &self.os.inner {
            Inner::System(_) => clipboard::read().await,
            #[cfg(feature = "testing")]
            Inner::Fake(fake) => Ok(lock(fake).clipboard.clone()),
        }
    }

    /// Puts `text` on the clipboard.
    ///
    /// # Errors
    ///
    /// Fails when the system clipboard cannot be written.
    pub async fn write_text(&self, text: impl Into<String>) -> Result<(), Error> {
        match &self.os.inner {
            Inner::System(_) => clipboard::write(text.into()).await,
            #[cfg(feature = "testing")]
            Inner::Fake(fake) => {
                lock(fake).clipboard = Some(text.into());
                Ok(())
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::is_external;

    #[test]
    fn only_an_absolute_url_is_external() {
        for url in [
            "https://example.com",
            "HTTP://example.com/a",
            "mailto:a@example.com",
        ] {
            assert!(is_external(url), "{url}");
        }
        for url in ["/notes", "notes", "//example.com", "file:///etc", ""] {
            assert!(!is_external(url), "{url}");
        }
    }
}
