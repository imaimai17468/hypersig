//! The window and the web view inside it.

use std::path::PathBuf;

use tao::dpi::LogicalSize;
use tao::event_loop::{EventLoop, EventLoopProxy};
use tao::window::{Window, WindowBuilder};
use wry::{NewWindowResponse, PageLoadEvent, WebContext, WebView, WebViewBuilder};

use crate::app::App;
use crate::error::Error;
use crate::run::Event;

/// Makes the window of `app`, hidden until its first page has loaded.
pub(crate) fn window(app: &App, event_loop: &EventLoop<Event>) -> Result<Window, Error> {
    let (width, height) = app.size;
    let mut builder = WindowBuilder::new()
        .with_visible(false)
        .with_inner_size(LogicalSize::new(width, height))
        .with_resizable(app.resizable);
    if let Some((width, height)) = app.min_size {
        builder = builder.with_min_inner_size(LogicalSize::new(width, height));
    }
    builder.build(event_loop).map_err(Error::window)
}

/// The data directory of the web view, under the application's own.
///
/// `WKWebView` takes no directory, and keeps its data per application in
/// `~/Library/WebKit` instead.
pub(crate) fn context(dir: PathBuf) -> WebContext {
    WebContext::new(Some(dir))
}

/// Makes the web view of `window`, which stays at `origin` and tells the
/// event loop of its title and of each page of `origin` that finishes
/// loading. It loads nothing until it is given a URL.
pub(crate) fn web_view(
    window: &Window,
    context: &mut WebContext,
    origin: String,
    proxy: &EventLoopProxy<Event>,
) -> Result<WebView, Error> {
    let titles = proxy.clone();
    let loads = proxy.clone();
    let loaded_origin = origin.clone();
    let builder = WebViewBuilder::new_with_web_context(context)
        .with_devtools(cfg!(debug_assertions))
        // Before the start URL, the web view may show `about:blank`.
        .with_navigation_handler(move |url| {
            let stays = url.starts_with(&origin) || url == "about:blank";
            if !stays {
                tracing::warn!(%url, "the window stays on the application's pages, so it did not open this URL");
            }
            stays
        })
        .with_new_window_req_handler(|url, _| {
            tracing::warn!(%url, "the application opens no second window yet, so it did not open this URL");
            NewWindowResponse::Deny
        })
        .with_document_title_changed_handler(move |title| {
            let _ = titles.send_event(Event::Title(title));
        })
        .with_on_page_load_handler(move |event, url| {
            if matches!(event, PageLoadEvent::Finished) && url.starts_with(&loaded_origin) {
                let _ = loads.send_event(Event::Loaded);
            }
        });
    #[cfg(not(target_os = "linux"))]
    let web_view = builder.build(window);
    #[cfg(target_os = "linux")]
    let web_view = {
        use tao::platform::unix::WindowExtUnix as _;
        use wry::WebViewBuilderExtUnix as _;
        builder.build_gtk(window.gtk_window())
    };
    web_view.map_err(Error::window)
}
