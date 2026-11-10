//! Shows web/index.html in a window the size of the hypersig todo example's,
//! hidden until the page has loaded, then writes how long after the process
//! started that was and exits, as a hypersig_desktop smoke run does.

use std::time::Instant;

use tauri::Manager as _;
use tauri::webview::PageLoadEvent;

fn main() {
    let started = Instant::now();
    tauri::Builder::default()
        .on_page_load(move |webview, payload| {
            if matches!(payload.event(), PageLoadEvent::Finished) {
                let _ = webview.window().show();
                println!(
                    "tauri: first page shown {} ms after start",
                    started.elapsed().as_millis()
                );
                webview.app_handle().exit(0);
            }
        })
        .run(tauri::generate_context!())
        .expect("the Tauri app runs");
}
