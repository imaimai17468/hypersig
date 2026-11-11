# use std::fs;
# use std::path::PathBuf;
# use hypersig::{Patches, Signals};
# use hypersig_desktop::Os;
# use hypersig_desktop::testing::FakeOs;
# use serde::{Deserialize, Serialize};
# #[derive(Signals, Serialize, Deserialize)]
# struct Library {
#     status: String,
# }
# #[derive(Clone)]
# struct AppState {
#     os: Os,
# }
/// Asks for any number of images.
async fn import(state: AppState, (): ()) -> Patches<Library> {
    let Some(files) = state
        .os
        .dialog()
        .title("Import photos")
        .filter("Images", &["png", "jpg", "jpeg"])
        .open_files()
        .await
    else {
        return Patches::new();
    };
    Library::status().patch(format!("{} files chosen.", files.len())).into()
}

/// Asks for a folder, starting in `start`.
async fn choose_folder(state: AppState, start: PathBuf) -> Patches<Library> {
    match state.os.dialog().directory(start).pick_folder().await {
        Some(folder) => Library::status().patch(format!("Watching {}.", folder.display())).into(),
        None => Patches::new(),
    }
}

/// Asks where to save `text`, and writes it there.
async fn export(state: AppState, text: String) -> Patches<Library> {
    let Some(path) = state
        .os
        .dialog()
        .title("Export the list")
        .filter("Text", &["txt"])
        .file_name("photos.txt")
        .save_file()
        .await
    else {
        return Patches::new();
    };
    // The dialog chose the path; the file is written here.
    match fs::write(&path, text) {
        Ok(()) => Library::status().patch(format!("Saved {}.", path.display())).into(),
        Err(error) => Library::status().patch(format!("Not saved: {error}")).into(),
    }
}
# fn main() {
# tokio::runtime::Builder::new_current_thread().build().unwrap().block_on(async {
# let dir = std::env::temp_dir().join(format!("site-dialogs-{}", std::process::id()));
# fs::create_dir_all(&dir).unwrap();
# let fake = FakeOs::new();
# let state = AppState { os: fake.os() };
# fake.answer_dialog([dir.join("a.png"), dir.join("b.png")]);
# let _ = import(state.clone(), ()).await;
# fake.answer_dialog([dir.clone()]);
# let _ = choose_folder(state.clone(), dir.clone()).await;
# fake.answer_dialog([dir.join("photos.txt")]);
# let _ = export(state.clone(), "a.png\n".to_owned()).await;
# assert_eq!(fs::read_to_string(dir.join("photos.txt")).unwrap(), "a.png\n");
# fake.cancel_dialog();
# let _ = export(state, String::new()).await;
# assert_eq!(fake.dialogs().len(), 4);
# fs::remove_dir_all(dir).unwrap();
# });
# }
