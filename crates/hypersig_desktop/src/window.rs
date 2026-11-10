//! The windows and the web view inside each.

use std::path::PathBuf;

use tao::dpi::LogicalSize;
use tao::event_loop::{EventLoopProxy, EventLoopWindowTarget};
use tao::window::{Window, WindowBuilder};
use wry::{NewWindowResponse, PageLoadEvent, WebContext, WebView, WebViewBuilder};

use crate::app::App;
use crate::error::Error;
use crate::run::Event;

/// Makes a window the size `app` asks for, hidden until its page has loaded.
pub(crate) fn window(app: &App, target: &EventLoopWindowTarget<Event>) -> Result<Window, Error> {
    let (width, height) = app.size;
    // The page's title replaces the empty one once the page has it.
    let mut builder = WindowBuilder::new()
        .with_title("")
        .with_visible(false)
        .with_inner_size(LogicalSize::new(width, height))
        .with_resizable(app.resizable);
    if let Some((width, height)) = app.min_size {
        builder = builder.with_min_inner_size(LogicalSize::new(width, height));
    }
    builder.build(target).map_err(Error::window)
}

/// The data directory of the web views, under the application's own. Every
/// window's web view is made with this one, so they share their cookies and
/// `localStorage`.
///
/// `WKWebView` takes no directory, and its web views share the data it keeps
/// per application in `~/Library/WebKit` instead.
pub(crate) fn context(dir: PathBuf) -> WebContext {
    WebContext::new(Some(dir))
}

/// Makes the web view of `window`, which stays at `origin` and tells the
/// event loop of its title, of each page of `origin` that finishes loading,
/// of `about:blank` once the closing window has left for it, and of a page
/// of `origin` it asks to open in a new window. It loads nothing until it is
/// given a URL.
pub(crate) fn web_view(
    window: &Window,
    context: &mut WebContext,
    origin: &str,
    proxy: &EventLoopProxy<Event>,
) -> Result<WebView, Error> {
    let id = window.id();
    let titles = proxy.clone();
    let loads = proxy.clone();
    let opens = proxy.clone();
    let navigation_origin = origin.to_owned();
    let load_origin = origin.to_owned();
    let open_origin = origin.to_owned();
    let builder = WebViewBuilder::new_with_web_context(context)
        .with_devtools(cfg!(debug_assertions))
        // Before its first URL, the web view may show `about:blank`.
        .with_navigation_handler(move |url| {
            let stays = url.starts_with(&navigation_origin) || url == "about:blank";
            if !stays {
                tracing::warn!(%url, "the window stays on the application's pages, so it did not open this URL");
            }
            stays
        })
        // A link with `target="_blank"` and `window.open` ask for this.
        .with_new_window_req_handler(move |url, _| {
            if url.starts_with(&open_origin) {
                let _ = opens.send_event(Event::Open { url, answer: None });
            } else {
                tracing::warn!(%url, "the window stays on the application's pages, so it did not open this URL");
            }
            NewWindowResponse::Deny
        })
        .with_document_title_changed_handler(move |title| {
            let _ = titles.send_event(Event::Title(id, title));
        })
        .with_on_page_load_handler(move |event, url| {
            if !matches!(event, PageLoadEvent::Finished) {
                return;
            }
            if url.starts_with(&load_origin) {
                let _ = loads.send_event(Event::Loaded(id));
            } else if url == "about:blank" {
                let _ = loads.send_event(Event::Left(id));
            }
        });
    #[cfg(not(target_os = "linux"))]
    let web_view = builder.build(window);
    // tao puts a box in the GTK window, which holds one child, so the web
    // view goes in that box.
    #[cfg(target_os = "linux")]
    let web_view = {
        use tao::platform::unix::WindowExtUnix as _;
        use wry::WebViewBuilderExtUnix as _;
        let Some(vbox) = window.default_vbox() else {
            return Err(Error::window(
                "the window has no box for the web view to go in",
            ));
        };
        builder.build_gtk(vbox)
    };
    web_view.map_err(Error::window)
}
