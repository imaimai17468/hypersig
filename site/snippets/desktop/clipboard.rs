# use hypersig::html::{button, div, p, textarea};
# use hypersig::{Action, Page, Patches, Signals};
# use hypersig_desktop::Os;
# use hypersig_desktop::testing::FakeOs;
# use serde::{Deserialize, Serialize};
#[derive(Signals, Serialize, Deserialize)]
struct Editor {
    text: String,
    status: String,
}

#[derive(Clone)]
struct AppState {
    os: Os,
}

hypersig::routes! {
    fn actions(Router<AppState>);
    action COPY: Action<Editor, String> = post "/copy" => copy;
    action PASTE: Action<Editor> = post "/paste" => paste;
}

fn editor() -> Page<Editor> {
    Page::new("Editor", Editor { text: String::new(), status: String::new() }).child(
        div()
            .child(textarea().bind(Editor::text()))
            .child(button().on_click(COPY.call_with(Editor::text())).text("Copy"))
            .child(button().on_click(PASTE.call()).text("Paste"))
            .child(p().bind_text(Editor::status())),
    )
}

/// Puts the text the page sent on the clipboard.
async fn copy(state: AppState, text: String) -> Patches<Editor> {
    match state.os.clipboard().write_text(text).await {
        Ok(()) => Editor::status().patch("Copied.".to_owned()).into(),
        Err(error) => Editor::status().patch(format!("Not copied: {error}")).into(),
    }
}

/// Replaces the page's text with the clipboard's.
async fn paste(state: AppState, (): ()) -> Patches<Editor> {
    match state.os.clipboard().read_text().await {
        Ok(Some(text)) => Editor::text().patch(text).into(),
        Ok(None) => Editor::status().patch("The clipboard holds no text.".to_owned()).into(),
        Err(error) => Editor::status().patch(format!("Not pasted: {error}")).into(),
    }
}
# fn main() {
# let _ = actions;
# tokio::runtime::Builder::new_current_thread().build().unwrap().block_on(async {
# let fake = FakeOs::new();
# let state = AppState { os: fake.os() };
# let page = editor();
# let mut browser = page.simulate();
# browser.apply(paste(state.clone(), ()).await);
# assert_eq!(browser.value_of(Editor::status()), "The clipboard holds no text.");
# browser.apply(copy(state.clone(), "hello".to_owned()).await);
# assert_eq!(fake.clipboard_text().as_deref(), Some("hello"));
# fake.set_clipboard_text("from another program");
# browser.apply(paste(state, ()).await);
# assert_eq!(browser.value_of(Editor::text()), "from another program");
# });
# }
