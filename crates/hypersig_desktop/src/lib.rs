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
//!     APP.size(800, 600).run(&INDEX.url(()), async {
//!         pages(Router::new()).hypersig_runtime()
//!     })
//! }
//! ```
//!
//! The server answers only the application's own window. It listens on
//! `127.0.0.1` alone, refuses a request whose `Host` is not that address, and
//! refuses one without the cookie the window receives when it opens. The
//! window does not leave the application's origin.
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
//! # Measuring the start
//!
//! With `HYPERSIG_DESKTOP_SMOKE` set, the application closes once its first
//! page has loaded, after writing to standard output how many milliseconds
//! passed since [`App::run`] was called. It ends with an error when the page
//! does not load within 10 seconds.

mod app;
mod error;
mod guard;
mod instance;
#[cfg(target_os = "macos")]
mod menu;
mod port;
mod run;
mod window;

pub use app::{App, IntoRouter};
pub use error::{Error, ErrorKind};
