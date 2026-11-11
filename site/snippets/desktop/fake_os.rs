# use std::fs;
# use hypersig::html::{button, div, p, pre};
# use hypersig::{Action, Page, Patches, Route, Signals};
# use hypersig_desktop::Os;
# use hypersig_desktop::testing::FakeOs;
# use serde::{Deserialize, Serialize};
# #[derive(Signals, Serialize, Deserialize)]
# struct Viewer {
#     text: String,
#     status: String,
# }
# #[derive(Clone)]
# struct AppState {
#     os: Os,
# }
# hypersig::routes! {
#     fn pages(Router<AppState>);
#     const INDEX: Route = "/" => index;
#     action OPEN: Action<Viewer> = post "/open" => open;
# }
hypersig::node_ref!(const OPEN_BUTTON: NodeRef<Viewer>);

async fn index(_: AppState, (): ()) -> Page<Viewer> {
    let initial = Viewer { text: String::new(), status: String::new() };
    Page::new("Viewer", initial).child(
        div()
            .child(button().node_ref(OPEN_BUTTON).on_click(OPEN.call()).text("Open…"))
            .child(p().bind_text(Viewer::status()))
            .child(pre().bind_text(Viewer::text())),
    )
}
# async fn open(state: AppState, (): ()) -> Patches<Viewer> {
#     let chosen = state.os.dialog().title("Open a note").filter("Text", &["txt", "md"]).open_file().await;
#     let Some(path) = chosen else {
#         return Patches::new();
#     };
#     match fs::read_to_string(&path) {
#         Ok(text) => Viewer::text().patch(text).and(Viewer::status().patch(path.display().to_string())).into(),
#         Err(error) => Viewer::status().patch(format!("Not read: {error}")).into(),
#     }
# }
# fn main() {
# let _ = pages;
# tokio::runtime::Builder::new_current_thread().build().unwrap().block_on(async {
# let dir = std::env::temp_dir().join(format!("site-fake-os-{}", std::process::id()));
# fs::create_dir_all(&dir).unwrap();
# let note = dir.join("today.md");
# fs::write(&note, "# Today\n").unwrap();

// The body of a `#[tokio::test] async fn`, with `note` a file the test wrote.
let fake = FakeOs::new();
fake.answer_dialog([note.clone()]);
let state = AppState { os: fake.os() };

let page = index(state.clone(), ()).await;
let mut browser = page.simulate();
let effects = browser.click(OPEN_BUTTON);
assert_eq!(effects.calls(&OPEN), [()]);
browser.apply(open(state.clone(), ()).await);
assert_eq!(browser.value_of(Viewer::text()), "# Today\n");

// A second click, and the person cancels: the text stays.
fake.cancel_dialog();
browser.click(OPEN_BUTTON);
browser.apply(open(state, ()).await);
assert_eq!(browser.value_of(Viewer::text()), "# Today\n");
assert_eq!(fake.dialogs().len(), 2);
# fs::remove_dir_all(dir).unwrap();
# });
# }
