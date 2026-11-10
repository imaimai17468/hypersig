//! What the application's handlers ask of the operating system.

use std::fmt;

use hypersig::Url;
use tao::event_loop::EventLoopProxy;
use tao::window::WindowId;
use tokio::sync::oneshot;

use crate::error::Error;
use crate::run::Event;

/// The operating system's windows and services, for the application's handlers.
///
/// [`App::run`](crate::App::run) gives it to the setup closure, which puts it
/// in the router's state, so a handler reaches it as `state.os`.
#[derive(Clone)]
pub struct Os {
    proxy: EventLoopProxy<Event>,
}

impl fmt::Debug for Os {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Os").finish_non_exhaustive()
    }
}

impl Os {
    pub(crate) fn new(proxy: EventLoopProxy<Event>) -> Self {
        Self { proxy }
    }

    /// The application's windows.
    #[must_use]
    pub fn windows(&self) -> Windows<'_> {
        Windows { os: self }
    }

    /// Sends `event` to the event loop, which answers on the channel the
    /// event carries.
    fn send(&self, event: Event) -> Result<(), Error> {
        self.proxy
            .send_event(event)
            .map_err(|_closed| Error::closed())
    }
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
        let (answer, answered) = oneshot::channel();
        self.os.send(Event::Open {
            url: page.as_str().to_owned(),
            answer: Some(answer),
        })?;
        let id = answered.await.map_err(|_closed| Error::closed())??;
        Ok(Window {
            id,
            os: self.os.clone(),
        })
    }
}

/// A window [`Windows::open`] opened.
#[derive(Debug, Clone)]
pub struct Window {
    id: WindowId,
    os: Os,
}

impl Window {
    /// Closes the window, as its close button does.
    ///
    /// # Errors
    ///
    /// Fails when the application is closing.
    pub fn close(&self) -> Result<(), Error> {
        self.os.send(Event::Close(self.id))
    }

    /// Brings the window to the front and gives it the keyboard.
    ///
    /// # Errors
    ///
    /// Fails when the application is closing.
    pub fn focus(&self) -> Result<(), Error> {
        self.os.send(Event::Focus(self.id))
    }
}

impl PartialEq for Window {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Eq for Window {}
