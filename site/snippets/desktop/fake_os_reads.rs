# use hypersig::{Page, Route, SafeUrl, Signals};
# use hypersig_desktop::testing::FakeOs;
# use serde::{Deserialize, Serialize};
# #[derive(Signals, Serialize, Deserialize)]
# struct Note {}
# hypersig::routes! {
#     fn pages(Router<()>);
#     const NOTE: Route<u64> = "/notes/{id}" => note;
# }
# async fn note((): (), _id: u64) -> Page<Note> {
#     Page::new("Note", Note {})
# }
# const SOURCE: SafeUrl = SafeUrl::new("https://github.com/imaimai17468/hypersig");
# fn main() -> Result<(), hypersig_desktop::Error> {
# let _ = pages;
# tokio::runtime::Builder::new_current_thread().build().unwrap().block_on(async {
let fake = FakeOs::new();
let os = fake.os();

let window = os.windows().open(&NOTE.url(7)).await?;
assert_eq!(fake.open_windows(), [NOTE.url(7)]);
assert_eq!(fake.focused_window(), Some(NOTE.url(7)));
window.close()?;
assert_eq!(fake.open_windows(), []);

fake.set_clipboard_text("from another program");
assert_eq!(os.clipboard().read_text().await?.as_deref(), Some("from another program"));

os.open_external(&SOURCE)?;
assert_eq!(fake.opened_externally(), [SOURCE]);

// With the `updater` feature: a fake system finds no newer version.
assert!(os.updates().await?.is_none());
# Ok(())
# })
# }
