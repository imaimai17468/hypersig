//! Runs a hypersig application in a desktop window.
//!
//! The application's [`Router`](axum::Router) is served in the same process,
//! on `127.0.0.1`, and the window shows it in the operating system's web
//! view: `WKWebView` on macOS, `WebView2` on Windows, and `WebKitGTK` on Linux.
//! The pages, actions, and patches are the ones the application serves on the
//! web, so only `main` changes:
//!
//! ```no_run
//! use axum::Router;
//! use hypersig::{Page, Route, RouterExt, Signals};
//! use hypersig_desktop::App;
//! use serde::{Deserialize, Serialize};
//!
//! #[derive(Signals, Serialize, Deserialize)]
//! struct Home {}
//!
//! hypersig::routes! {
//!     fn pages(Router<()>);
//!     const INDEX: Route = "/" => index;
//! }
//!
//! async fn index((): (), (): ()) -> Page<Home> {
//!     Page::new("Home", Home {})
//! }
//!
//! const APP: App = App::new("dev.example.home");
//!
//! fn main() -> Result<(), hypersig_desktop::Error> {
//!     APP.size(800, 600).run(&INDEX.url(()), |_os| async {
//!         pages(Router::new()).hypersig_runtime()
//!     })
//! }
//! ```
//!
//! Every window shows a page of the same server, so two windows that open a
//! stream such as a [`StreamAction`](hypersig::StreamAction) fed by one
//! broadcast channel show the same state, as two browser tabs do. A handler
//! opens another window with [`Windows::open`], through the [`Os`] that
//! [`App::run`] gives the setup, and a page's `window.open`, or a link with
//! `target="_blank"`, opens one too. Closing the last window closes the
//! application, on macOS as well.
//!
//! The windows reach the server over HTTP/1.1, and a web view keeps at most
//! six connections to one host for all its windows together. A stream holds
//! its connection open, so with six windows that each hold a stream, a seventh
//! request waits until one closes; in `WKWebView`, the todo example's sixth
//! window left the first window's actions unanswered. Until the windows reach
//! the server without HTTP, an application keeps at most five streams open
//! across all its windows.
//!
//! A handler reaches the system through the same [`Os`]: [`Os::dialog`]
//! shows a file dialog, [`Os::clipboard`] reads and writes the clipboard's
//! text, and [`Os::open_external`] opens a URL in the person's browser, as a
//! window does with a link to another site. With the `testing` feature,
//! `testing::FakeOs` stands in for the system, so a test calls the handlers
//! with dialogs it answers and reads back what they did, and clicks through
//! the pages with [`Page::simulate`](hypersig::Page::simulate), without a
//! window. `examples/notes` is written and tested this way.
//!
//! With the `updater` feature, `App::updates` names where new versions are
//! announced and the key that signs them, and the `Updater` that `App::run`
//! then hands the setup finds a newer version, downloads it, checks its signature, and installs it, in the
//! format `cargo-packager` builds and signs packages in.
//!
//! The server answers only the application's own windows. It listens on
//! `127.0.0.1` alone, refuses a request whose `Host` is not that address, and
//! refuses one without the cookie the first window receives when it opens,
//! which the others share. No window leaves the application's origin.
//!
//! The server listens on the port it listened on the previous time the
//! application ran, so that the signals a page keeps in the browser with
//! [`Page::persist`](hypersig::Page::persist) are found again. When another
//! program holds that port, the server takes another one, and those signals
//! start over.
//!
//! A second copy of the application started while the first runs stops with
//! [`ErrorKind::AlreadyRunning`].
//!
//! `WKWebView`, which shows the window on macOS, does not send a `Secure`
//! cookie to `http://127.0.0.1`, and neither does Playwright's build of
//! `WebKit`, the engine of `WebKitGTK` on Linux. A session layer that marks
//! its cookie `Secure`, as `tower-sessions` does by default, signs no one in
//! inside the window, so the desktop build of such an application sets its
//! cookie without `Secure`. The checks above keep other programs off the
//! server.
//!
//! On macOS the menu bar holds the application, Edit, and Window menus,
//! because macOS sends the shortcuts for copying, pasting, closing, and
//! quitting to the menu rather than to the web view.
//!
//! # Smoke runs
//!
//! In a build with the `smoke` feature, for CI and benchmarks, and with
//! `HYPERSIG_DESKTOP_SMOKE` set, the application checks that its first
//! page is one hypersig rendered, writes to standard output how many
//! milliseconds after [`App::run`] the page was shown, and closes. It ends
//! with an error when the page is not hypersig's, as when the server refused
//! the window, or does not load within 10 seconds.
//!
//! With `HYPERSIG_DESKTOP_SMOKE_SCRIPT` set to the path of a JavaScript file,
//! the run then evaluates that file in the first page and in the page of
//! every other window, and closes once the first page sets
//! `window.hypersigSmoke` to `"pass"`. Anything else it sets, or nothing
//! within 20 seconds, ends the run with an error. An empty
//! `HYPERSIG_DESKTOP_SMOKE_SCRIPT` counts as unset.

mod app;
mod badge;
mod clipboard;
mod error;
#[cfg(target_os = "linux")]
mod framing;
mod guard;
mod handoff;
mod instance;
#[cfg(target_os = "macos")]
mod menu;
mod os;
mod port;
mod run;
mod smoke;
#[cfg(feature = "testing")]
pub mod testing;
#[cfg(feature = "updater")]
mod updates;
mod window;

pub use app::{App, IntoRouter};
pub use error::{Error, ErrorKind};
pub use handoff::{Launch, Launches};
pub use os::{Clipboard, Dialog, Os, Window, Windows};
#[cfg(feature = "updater")]
pub use updates::{Downloaded, Update, Updater, Updates};
