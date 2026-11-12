//! The system's message boxes, which tell the person something or ask
//! them to choose, for [`Os::tell`](crate::Os::tell),
//! [`Os::confirm`](crate::Os::confirm), and [`Os::ask`](crate::Os::ask).

/// How serious a [`Message`] is, which picks the icon the system shows.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum Level {
    /// Something the person should know.
    Info,
    /// Something that may go wrong, such as losing unsaved changes.
    Warning,
    /// Something that went wrong.
    Error,
}

/// A message box's title and text, for [`Os::tell`](crate::Os::tell) and the others.
#[derive(Debug, Clone, PartialEq, Eq)]
#[must_use = "a message shows only when it is given to `Os::tell`, `Os::confirm`, or `Os::ask`"]
pub struct Message {
    level: Level,
    title: String,
    text: Option<String>,
}

impl Message {
    /// A message of `level` titled `title`, such as "Discard the changes?".
    pub fn new(level: Level, title: impl Into<String>) -> Self {
        Self {
            level,
            title: title.into(),
            text: None,
        }
    }

    /// Writes `text` under the title, such as what is lost.
    pub fn body(mut self, text: impl Into<String>) -> Self {
        self.text = Some(text.into());
        self
    }

    /// The level the message was made with.
    #[must_use]
    pub const fn level(&self) -> Level {
        self.level
    }

    /// The title the message shows.
    #[must_use]
    pub fn title(&self) -> &str {
        &self.title
    }

    /// The text under the title, if it has one.
    #[must_use]
    pub fn text(&self) -> Option<&str> {
        self.text.as_deref()
    }

    /// The system's message box, with `buttons`.
    pub(crate) fn system(self, buttons: rfd::MessageButtons) -> rfd::AsyncMessageDialog {
        let level = match self.level {
            Level::Info => rfd::MessageLevel::Info,
            Level::Warning => rfd::MessageLevel::Warning,
            Level::Error => rfd::MessageLevel::Error,
        };
        let dialog = rfd::AsyncMessageDialog::new()
            .set_level(level)
            .set_title(self.title)
            .set_buttons(buttons);
        match self.text {
            Some(text) => dialog.set_description(text),
            None => dialog,
        }
    }
}

/// The button the person chose in [`Os::ask`](crate::Os::ask)'s message box.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[expect(
    clippy::exhaustive_enums,
    reason = "the box has these three buttons, and closing it counts as Cancel"
)]
pub enum Answer {
    /// Yes, such as saving the changes.
    Yes,
    /// No, such as discarding them.
    No,
    /// Cancel, or the box closed without a choice.
    Cancel,
}

impl Answer {
    /// The answer the system's box gave.
    pub(crate) fn from_system(result: &rfd::MessageDialogResult) -> Self {
        match result {
            rfd::MessageDialogResult::Yes => Self::Yes,
            rfd::MessageDialogResult::No => Self::No,
            _ => Self::Cancel,
        }
    }
}
