//! An [`Os`] a test scripts and reads, under the `testing` feature.
//!
//! ```
//! use std::path::PathBuf;
//!
//! use hypersig_desktop::testing::{DialogKind, FakeOs};
//!
//! # tokio::runtime::Builder::new_current_thread().build().unwrap().block_on(async {
//! let fake = FakeOs::new();
//! fake.answer_dialog([PathBuf::from("/notes/today.md")]);
//! let os = fake.os();
//!
//! // A handler given `os` asks for a file and gets the one the test chose.
//! assert_eq!(os.dialog().open_file().await, Some(PathBuf::from("/notes/today.md")));
//! os.clipboard().write_text("copied").await.unwrap();
//! assert_eq!(fake.clipboard_text().as_deref(), Some("copied"));
//! assert_eq!(fake.dialogs()[0].kind, DialogKind::OpenFile);
//! # });
//! ```

use std::collections::VecDeque;
use std::ffi::OsString;
use std::path::PathBuf;
use std::sync::{Arc, Mutex, PoisonError};

use hypersig::{SafeUrl, Url};

use crate::os::Os;

/// A fake system: the answers its dialogs give, and what the handlers did.
#[derive(Debug, Clone, Default)]
pub struct FakeOs {
    fake: Arc<Mutex<Fake>>,
}

/// What a [`FakeOs`] holds.
#[derive(Debug)]
pub(crate) struct Fake {
    pub(crate) answers: VecDeque<Option<Vec<PathBuf>>>,
    pub(crate) dialogs: Vec<ShownDialog>,
    pub(crate) clipboard: Option<String>,
    /// Each window opened, in order, with `None` once it is closed.
    pub(crate) windows: Vec<Option<Url>>,
    /// The index in `windows` of the window opened or focused last.
    pub(crate) focused: Option<usize>,
    pub(crate) external: Vec<SafeUrl>,
    pub(crate) launches: tokio::sync::broadcast::Sender<crate::Launch>,
    pub(crate) badge: u32,
    pub(crate) tray: Option<crate::Tray>,
    /// Each notification shown, with where to say it was clicked.
    pub(crate) notifications: Vec<(
        crate::Notification,
        Option<tokio::sync::oneshot::Sender<bool>>,
    )>,
}

impl Default for Fake {
    fn default() -> Self {
        Self {
            answers: VecDeque::new(),
            dialogs: Vec::new(),
            clipboard: None,
            windows: Vec::new(),
            focused: None,
            external: Vec::new(),
            launches: crate::handoff::channel(),
            badge: 0,
            tray: None,
            notifications: Vec::new(),
        }
    }
}

impl Fake {
    /// Records `dialog` and returns the next answer a test gave for it.
    ///
    /// # Panics
    ///
    /// Panics when the test gave no answer for this dialog, which fails the
    /// test rather than hiding a dialog it did not expect, and when a dialog
    /// that chooses one path is answered with another number of paths.
    pub(crate) fn answer_dialog(&mut self, dialog: ShownDialog) -> Option<Vec<PathBuf>> {
        let number = self.dialogs.len() + 1;
        let answer = self.answers.pop_front().unwrap_or_else(|| {
            panic!(
                "dialog {number} was shown, and the test gave FakeOs no answer for it with answer_dialog or cancel_dialog"
            )
        });
        if let Some(paths) = &answer
            && dialog.kind != DialogKind::OpenFiles
            && paths.len() != 1
        {
            panic!(
                "dialog {number} was shown with DialogKind::{:?}, which chooses one path, and the test answered it with {} paths",
                dialog.kind,
                paths.len()
            );
        }
        self.dialogs.push(dialog);
        answer
    }
}

/// A dialog a handler showed, as [`FakeOs::dialogs`] returns it.
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub struct ShownDialog {
    /// The method that showed it.
    pub kind: DialogKind,
    /// The title [`Dialog::title`](crate::Dialog::title) gave.
    pub title: Option<String>,
    /// Each name and extensions [`Dialog::filter`](crate::Dialog::filter) gave, in order.
    pub filters: Vec<(String, Vec<String>)>,
    /// The directory [`Dialog::directory`](crate::Dialog::directory) gave.
    pub directory: Option<PathBuf>,
    /// The name [`Dialog::file_name`](crate::Dialog::file_name) gave.
    pub file_name: Option<String>,
}

/// Which method of [`Dialog`](crate::Dialog) showed a dialog.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum DialogKind {
    /// [`Dialog::open_file`](crate::Dialog::open_file).
    OpenFile,
    /// [`Dialog::open_files`](crate::Dialog::open_files).
    OpenFiles,
    /// [`Dialog::pick_folder`](crate::Dialog::pick_folder).
    PickFolder,
    /// [`Dialog::save_file`](crate::Dialog::save_file).
    SaveFile,
}

impl FakeOs {
    /// A fake system with no answers, no clipboard text, and no windows.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// The [`Os`] to give the application's handlers.
    #[must_use]
    pub fn os(&self) -> Os {
        Os::fake(Arc::clone(&self.fake))
    }

    /// Makes the next dialog answer as if the person chose `paths`.
    ///
    /// Dialogs take the answers in the order they were given. A dialog
    /// other than [`DialogKind::OpenFiles`] panics unless `paths` holds
    /// exactly one path, since the person chooses one there.
    pub fn answer_dialog(&self, paths: impl IntoIterator<Item = PathBuf>) {
        self.lock()
            .answers
            .push_back(Some(paths.into_iter().collect()));
    }

    /// Makes the next dialog answer as if the person cancelled it.
    pub fn cancel_dialog(&self) {
        self.lock().answers.push_back(None);
    }

    /// The dialogs the handlers have shown and the test answered, in order.
    #[must_use]
    pub fn dialogs(&self) -> Vec<ShownDialog> {
        self.lock().dialogs.clone()
    }

    /// The text on the fake clipboard.
    #[must_use]
    pub fn clipboard_text(&self) -> Option<String> {
        self.lock().clipboard.clone()
    }

    /// Puts `text` on the fake clipboard, as another program would.
    pub fn set_clipboard_text(&self, text: impl Into<String>) {
        self.lock().clipboard = Some(text.into());
    }

    /// The pages of the windows the handlers opened and have not closed.
    #[must_use]
    pub fn open_windows(&self) -> Vec<Url> {
        self.lock().windows.iter().flatten().cloned().collect()
    }

    /// The page of the window opened or focused last, while it is open.
    #[must_use]
    pub fn focused_window(&self) -> Option<Url> {
        let fake = self.lock();
        fake.windows.get(fake.focused?)?.clone()
    }

    /// An [`Updater`](crate::Updater) that finds no newer version and makes
    /// no request, for the handlers' state in place of the one
    /// [`App::run`](crate::App::run) gives.
    #[cfg(feature = "updater")]
    #[must_use]
    pub fn updater(&self) -> crate::Updater {
        crate::Updater::fake()
    }

    /// Hands `args`, started in `dir`, to the [`Launches`](crate::Launches)
    /// taken from this fake's `Os`, as a second start of the application
    /// would.
    pub fn launch(&self, dir: impl Into<PathBuf>, args: impl IntoIterator<Item = OsString>) {
        let launch = crate::Launch::new(dir.into(), args.into_iter().collect());
        // With no `Launches` taken, nobody hears it, as in the application.
        let _ = self.lock().launches.send(launch);
    }

    /// The count the handlers last put on the application's icon, 0 when
    /// none is shown.
    #[must_use]
    pub fn badge(&self) -> u32 {
        self.lock().badge
    }

    /// The notifications the handlers showed, in order.
    #[must_use]
    pub fn notifications(&self) -> Vec<crate::Notification> {
        self.lock()
            .notifications
            .iter()
            .map(|(shown, _)| shown.clone())
            .collect()
    }

    /// Clicks the notification shown `index`th from 0, so its
    /// [`Notified::clicked`](crate::Notified::clicked) returns `true`. A
    /// notification the test does not click returns `false` once the
    /// `FakeOs` and every `Os` from it are dropped.
    ///
    /// # Panics
    ///
    /// Panics when no notification was shown at `index`, or it was clicked
    /// already.
    pub fn click_notification(&self, index: usize) {
        let mut fake = self.lock();
        let clicked = fake
            .notifications
            .get_mut(index)
            .and_then(|(_, clicked)| clicked.take())
            .unwrap_or_else(|| panic!("no notification {index} is waiting to be clicked"));
        let _ = clicked.send(true);
    }

    /// The tray the handlers showed last, if any.
    #[must_use]
    pub fn tray(&self) -> Option<crate::Tray> {
        self.lock().tray.clone()
    }

    /// Clicks `item` in the tray's menu, as the person would.
    ///
    /// # Panics
    ///
    /// Panics when the tray shown, if any, has no such item.
    pub fn click_tray_item(&self, item: &crate::TrayItem) {
        let shown = self
            .lock()
            .tray
            .as_ref()
            .is_some_and(|tray| tray.items.iter().any(|shown| shown.id == item.id));
        assert!(shown, "the tray shown has no item labeled {:?}", item.label);
        let _ = item.clicked.send(());
    }

    /// The URLs the handlers opened in another program.
    #[must_use]
    pub fn opened_externally(&self) -> Vec<SafeUrl> {
        self.lock().external.clone()
    }

    fn lock(&self) -> std::sync::MutexGuard<'_, Fake> {
        self.fake.lock().unwrap_or_else(PoisonError::into_inner)
    }
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use axum::Router;
    use hypersig::Route;

    use super::{DialogKind, FakeOs, ShownDialog};

    fn block_on<F: Future>(future: F) -> F::Output {
        tokio::runtime::Builder::new_current_thread()
            .build()
            .unwrap_or_else(|error| panic!("no runtime: {error}"))
            .block_on(future)
    }

    #[test]
    fn a_dialog_is_recorded_with_its_kind_and_setup() {
        let fake = FakeOs::new();
        fake.answer_dialog([PathBuf::from("/notes/today.md")]);
        let saved = block_on(
            fake.os()
                .dialog()
                .title("Save the note")
                .filter("Markdown", &["md"])
                .directory("/notes")
                .file_name("today.md")
                .save_file(),
        );
        assert_eq!(saved, Some(PathBuf::from("/notes/today.md")));
        assert_eq!(
            fake.dialogs(),
            [ShownDialog {
                kind: DialogKind::SaveFile,
                title: Some("Save the note".to_owned()),
                filters: vec![("Markdown".to_owned(), vec!["md".to_owned()])],
                directory: Some(PathBuf::from("/notes")),
                file_name: Some("today.md".to_owned()),
            }]
        );
    }

    #[test]
    fn open_files_takes_an_empty_answer() {
        let fake = FakeOs::new();
        fake.answer_dialog([]);
        assert_eq!(block_on(fake.os().dialog().open_files()), Some(vec![]));
    }

    #[test]
    #[should_panic(
        expected = "dialog 1 was shown with DialogKind::OpenFile, which chooses one path, and the test answered it with 0 paths"
    )]
    fn a_dialog_of_one_path_refuses_an_empty_answer() {
        let fake = FakeOs::new();
        fake.answer_dialog([]);
        let _ = block_on(fake.os().dialog().open_file());
    }

    #[test]
    fn an_unanswered_dialog_is_not_recorded() {
        let fake = FakeOs::new();
        let os = fake.os();
        let shown = std::thread::spawn(move || block_on(os.dialog().pick_folder())).join();
        assert!(shown.is_err(), "the unanswered dialog panics");
        assert_eq!(fake.dialogs(), []);
    }

    #[derive(hypersig::Signals, serde::Serialize, serde::Deserialize)]
    struct NotePage {
        id: u64,
    }

    hypersig::routes! {
        fn pages(Router<()>);
        const NOTE: Route<u64> = "/notes/{id}" => note;
    }

    async fn note((): (), id: u64) -> hypersig::Page<NotePage> {
        hypersig::Page::new("Note", NotePage { id })
    }

    fn ok<T>(result: Result<T, crate::Error>) -> T {
        result.unwrap_or_else(|error| panic!("{error}"))
    }

    #[test]
    fn the_window_opened_or_focused_last_is_focused() {
        let _ = pages(Router::new());
        let fake = FakeOs::new();
        let os = fake.os();
        let first = ok(block_on(os.windows().open(&NOTE.url(1))));
        let second = ok(block_on(os.windows().open(&NOTE.url(2))));
        assert_eq!(fake.focused_window(), Some(NOTE.url(2)));
        ok(first.focus());
        assert_eq!(fake.focused_window(), Some(NOTE.url(1)));
        ok(first.close());
        assert_eq!(fake.focused_window(), None);
        ok(second.focus());
        ok(first.focus());
        assert_eq!(fake.focused_window(), Some(NOTE.url(2)));
    }

    #[test]
    fn the_badge_keeps_the_count_set_last() {
        let fake = FakeOs::new();
        assert_eq!(fake.badge(), 0);
        ok(fake.os().badge(3));
        ok(fake.os().badge(0));
        ok(fake.os().badge(5));
        assert_eq!(fake.badge(), 5);
    }

    #[test]
    fn a_notification_is_recorded_and_clicked_by_the_test() {
        let fake = FakeOs::new();
        let os = fake.os();
        let (clicked, left) = block_on(async {
            let first = ok(os
                .notify(crate::Notification::new("Ada").body("Lunch?"))
                .await);
            let second = ok(os.notify(crate::Notification::new("Grace")).await);
            fake.click_notification(0);
            (first, second)
        });
        assert_eq!(fake.notifications()[0].title(), "Ada");
        assert_eq!(fake.notifications()[0].text(), Some("Lunch?"));
        assert_eq!(fake.notifications()[1].text(), None);
        assert!(block_on(clicked.clicked()));
        drop((fake, os));
        assert!(!block_on(left.clicked()));
    }
}
