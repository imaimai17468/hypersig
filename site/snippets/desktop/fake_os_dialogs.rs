# use std::path::PathBuf;
# use hypersig_desktop::testing::{DialogKind, FakeOs};
# tokio::runtime::Builder::new_current_thread().build().unwrap().block_on(async {
let fake = FakeOs::new();
let os = fake.os();
fake.answer_dialog([PathBuf::from("/notes/today.md")]);

// The handler's code: a dialog that saves a Markdown file.
let path = os.dialog().filter("Markdown", &["md"]).file_name("today.md").save_file().await;
assert_eq!(path, Some(PathBuf::from("/notes/today.md")));

let [shown] = &fake.dialogs()[..] else { panic!("one dialog was shown") };
assert_eq!(shown.kind, DialogKind::SaveFile);
assert_eq!(shown.filters, [("Markdown".to_owned(), vec!["md".to_owned()])]);
assert_eq!(shown.file_name.as_deref(), Some("today.md"));
# });
