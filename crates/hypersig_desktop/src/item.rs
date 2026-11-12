//! The items of the menus the application builds, in the menu bar and in
//! the tray, whose clicks reach the code that holds them.

use std::fmt;

use tokio::sync::broadcast;
use tray_icon::menu::MenuId;

/// An item of a menu, whose [`clicks`](Self::clicks) the application waits on.
///
/// It goes in a [`Menu`](crate::Menu) of the menu bar or in a
/// [`Tray`](crate::Tray)'s menu, and its clicks reach only the code that
/// holds it, since no item is named by a string.
#[derive(Clone)]
pub struct MenuItem {
    pub(crate) id: MenuId,
    pub(crate) label: String,
    pub(crate) clicked: broadcast::Sender<()>,
}

impl fmt::Debug for MenuItem {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("MenuItem")
            .field("label", &self.label)
            .finish_non_exhaustive()
    }
}

impl MenuItem {
    /// An item labeled `label`.
    pub fn new(label: impl Into<String>) -> Self {
        Self {
            id: MenuId::new(format!("hypersig-desktop-item-{}", next_id())),
            label: label.into(),
            clicked: broadcast::channel(16).0,
        }
    }

    /// The clicks on this item that come after this call.
    #[must_use]
    pub fn clicks(&self) -> Clicks {
        Clicks {
            receiver: self.clicked.subscribe(),
        }
    }
}

/// The clicks on a [`MenuItem`], from [`MenuItem::clicks`].
#[derive(Debug)]
pub struct Clicks {
    receiver: broadcast::Receiver<()>,
}

impl Clicks {
    /// Waits for the next click, and returns `false` once the item is gone.
    pub async fn next(&mut self) -> bool {
        loop {
            match self.receiver.recv().await {
                Ok(()) => return true,
                Err(broadcast::error::RecvError::Lagged(_)) => {}
                Err(broadcast::error::RecvError::Closed) => return false,
            }
        }
    }
}

/// A number no other item of this run has, for its menu id.
fn next_id() -> u64 {
    use std::sync::atomic::{AtomicU64, Ordering};
    static NEXT: AtomicU64 = AtomicU64::new(0);
    NEXT.fetch_add(1, Ordering::Relaxed)
}
