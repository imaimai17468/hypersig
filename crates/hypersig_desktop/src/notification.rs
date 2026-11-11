//! The system's notifications, which tell the person something while the
//! application's windows are in the background.

use notify_rust::NotificationResponse;
use tokio::sync::oneshot;

use crate::error::Error;

/// A notification for [`Os::notify`](crate::Os::notify): a title and an optional body.
#[derive(Debug, Clone, PartialEq, Eq)]
#[must_use = "a notification shows only when it is given to `Os::notify`"]
pub struct Notification {
    pub(crate) title: String,
    pub(crate) body: Option<String>,
}

impl Notification {
    /// A notification titled `title`, such as the sender of a message.
    pub fn new(title: impl Into<String>) -> Self {
        Self {
            title: title.into(),
            body: None,
        }
    }

    /// Writes `body` under the title, such as the message itself.
    pub fn body(mut self, body: impl Into<String>) -> Self {
        self.body = Some(body.into());
        self
    }

    /// The title the notification shows.
    #[must_use]
    pub fn title(&self) -> &str {
        &self.title
    }

    /// The text under the title, if it has one.
    #[must_use]
    pub fn text(&self) -> Option<&str> {
        self.body.as_deref()
    }
}

/// A notification the system showed, from [`Os::notify`](crate::Os::notify).
#[derive(Debug)]
pub struct Notified {
    pub(crate) clicked: oneshot::Receiver<bool>,
}

impl Notified {
    /// Whether the person clicked the notification, once they have clicked
    /// or dismissed it.
    ///
    /// It waits as long as the notification stays on screen or in the
    /// system's list of notifications, so a handler that needs it spawns a
    /// task rather than awaiting it before it answers. Until then, each
    /// notification shown keeps a thread of the process waiting for it.
    pub async fn clicked(self) -> bool {
        self.clicked.await.unwrap_or(false)
    }
}

/// Shows `notification` and sends whether it was clicked to `clicked`, from
/// a thread of its own, since the system's call waits for the person.
pub(crate) fn show(notification: &Notification) -> Result<Notified, Error> {
    let (clicked, heard) = oneshot::channel();
    let mut system = notify_rust::Notification::new();
    system.summary(&notification.title);
    if let Some(body) = &notification.body {
        system.body(body);
    }
    // The freedesktop servers report a click on the body as the action
    // named "default", and only when the notification offers it.
    #[cfg(target_os = "linux")]
    system.action("default", "Open");
    let handle = system.show().map_err(Error::notification)?;
    std::thread::Builder::new()
        .name("hypersig-desktop-notification".to_owned())
        .spawn(move || {
            // On Windows, `wait_for_action` reports a click on the body as
            // `"__closed"`; the typed response tells the two apart everywhere.
            let waited = handle.wait_for_response(|response: &NotificationResponse| {
                let _ = clicked.send(!matches!(response, NotificationResponse::Closed(_)));
            });
            if let Err(error) = waited {
                tracing::warn!(%error, "the notification's click was not heard");
            }
        })
        .map_err(Error::notification)?;
    Ok(Notified { clicked: heard })
}

/// Names `id` as the application the notifications come from, so macOS
/// shows them under the application installed with that identifier. A run
/// outside such a bundle keeps the default, and its notifications appear
/// under another application's name.
#[cfg(target_os = "macos")]
pub(crate) fn name_application(id: &str) {
    if let Err(error) = notify_rust::set_application(id) {
        tracing::debug!(%error, "notifications keep the default application, since no bundle has this identifier");
    }
}
