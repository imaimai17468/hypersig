//! An [`Os`] a test scripts and reads, under the `testing` feature.
//!
//! ```
//! use std::path::PathBuf;
//!
//! use hypersig_desktop::testing::FakeOs;
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
//! # });
//! ```

use std::collections::VecDeque;
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
#[derive(Debug, Default)]
pub(crate) struct Fake {
    pub(crate) answers: VecDeque<Option<Vec<PathBuf>>>,
    pub(crate) asked: usize,
    pub(crate) clipboard: Option<String>,
    /// Each window opened, in order, with `None` once it is closed.
    pub(crate) windows: Vec<Option<Url>>,
    pub(crate) external: Vec<SafeUrl>,
}

impl Fake {
    /// The next answer a test gave for a dialog.
    ///
    /// # Panics
    ///
    /// Panics when the test gave no answer for this dialog, which fails the
    /// test rather than hiding a dialog it did not expect.
    pub(crate) fn answer_dialog(&mut self) -> Option<Vec<PathBuf>> {
        self.asked += 1;
        self.answers.pop_front().unwrap_or_else(|| {
            panic!(
                "dialog {} was shown, and the test gave FakeOs no answer for it with answer_dialog or cancel_dialog",
                self.asked
            )
        })
    }
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

    /// Makes the next dialog answer as if the person chose `paths`. Dialogs
    /// take the answers in the order they were given.
    pub fn answer_dialog(&self, paths: impl IntoIterator<Item = PathBuf>) {
        self.lock()
            .answers
            .push_back(Some(paths.into_iter().collect()));
    }

    /// Makes the next dialog answer as if the person cancelled it.
    pub fn cancel_dialog(&self) {
        self.lock().answers.push_back(None);
    }

    /// How many dialogs the handlers have shown.
    #[must_use]
    pub fn dialogs_shown(&self) -> usize {
        self.lock().asked
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

    /// The URLs the handlers opened in another program.
    #[must_use]
    pub fn opened_externally(&self) -> Vec<SafeUrl> {
        self.lock().external.clone()
    }

    fn lock(&self) -> std::sync::MutexGuard<'_, Fake> {
        self.fake.lock().unwrap_or_else(PoisonError::into_inner)
    }
}
