//! A notepad that opens text files from disk, each in a window of its own.
//!
//! Run with `cargo run -p example-notes`, or `cargo run -p example-notes --
//! today.md` to open that file at the start. Started again while it runs,
//! it opens the files of that start in the copy already running. The first window lists the open
//! notes; "Open…" picks a file in the system's dialog and opens it in a new
//! window, where it is edited, saved, saved under another name, or copied to
//! the clipboard. A save reaches the list over a stream, so the list window
//! shows it without a reload.
//!
//! The tests at the end run the same handlers with
//! `hypersig_desktop::testing::FakeOs` in place of the system, and click
//! through the pages with `Page::simulate`, so none of them opens a window.

use std::fs;
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};

use axum::Router;
use futures::{Stream, StreamExt as _, stream};
use hypersig::html::{Stylesheet, button, div, each_keyed, h1, li, p, span, textarea, ul};
use hypersig::{Action, Page, Patches, Route, RouterExt, Signals, StreamAction};
use hypersig_desktop::{App, Launches, Os};
use serde::{Deserialize, Serialize};
use tokio::sync::broadcast;
use tokio_stream::wrappers::BroadcastStream;

/// The list window's signals.
#[derive(Signals, Serialize, Deserialize)]
struct NotesPage {
    notes: Vec<Note>,
    status: String,
}

/// One open note, as the list shows it.
#[derive(Signals, Serialize, Deserialize, Clone, Debug, PartialEq)]
struct Note {
    id: u64,
    name: String,
    saves: u32,
}

/// A note window's signals.
#[derive(Signals, Serialize, Deserialize)]
struct EditPage {
    id: u64,
    name: String,
    text: String,
    status: String,
}

hypersig::routes! {
    fn pages(Router<AppState>);
    const INDEX: Route = "/" => index;
    const NOTE: Route<u64> = "/notes/{id}" => note;
    action OPEN: Action<NotesPage> = post "/open" => open;
    action SHOW: Action<NotesPage, u64> = post "/show" => show;
    action SAVE: Action<EditPage, (u64, String)> = post "/save" => save;
    action SAVE_AS: Action<EditPage, (u64, String)> = post "/save-as" => save_as;
    action COPY: Action<EditPage, String> = post "/copy" => copy;
    stream UPDATES: StreamAction<NotesPage> = "/updates" => updates;
}

hypersig::node_ref!(const OPEN_BUTTON: NodeRef<NotesPage>);
hypersig::node_ref!(const TEXT: NodeRef<EditPage>);
hypersig::node_ref!(const SAVE_BUTTON: NodeRef<EditPage>);
hypersig::node_ref!(const SAVE_AS_BUTTON: NodeRef<EditPage>);
hypersig::node_ref!(const COPY_BUTTON: NodeRef<EditPage>);

/// A note's file and text, as the server keeps it.
struct Stored {
    note: Note,
    path: PathBuf,
}

#[derive(Default)]
struct Notes {
    next_id: u64,
    items: Vec<Stored>,
}

#[derive(Clone)]
struct AppState {
    notes: Arc<Mutex<Notes>>,
    /// Each note whose row changed, for the list's streams.
    changed: broadcast::Sender<Note>,
    os: Os,
}

impl AppState {
    fn new(os: Os) -> Self {
        let (changed, _) = broadcast::channel(16);
        Self {
            notes: Arc::default(),
            changed,
            os,
        }
    }

    fn rows(&self) -> Vec<Note> {
        let notes = self.notes.lock().expect("notes lock poisoned");
        notes
            .items
            .iter()
            .map(|stored| stored.note.clone())
            .collect()
    }

    fn path(&self, id: u64) -> Option<PathBuf> {
        let notes = self.notes.lock().expect("notes lock poisoned");
        let stored = notes.items.iter().find(|stored| stored.note.id == id)?;
        Some(stored.path.clone())
    }

    /// Keeps the file at `path` as a note, after checking that it reads,
    /// and sends its row to the list's streams.
    fn keep(&self, path: PathBuf) -> std::io::Result<Note> {
        fs::read_to_string(&path)?;
        let note = {
            let mut notes = self.notes.lock().expect("notes lock poisoned");
            notes.next_id += 1;
            let note = Note {
                id: notes.next_id,
                name: file_name(&path),
                saves: 0,
            };
            notes.items.push(Stored {
                note: note.clone(),
                path,
            });
            note
        };
        let _ = self.changed.send(note.clone());
        Ok(note)
    }

    /// Runs `change` on the note `id` and sends its row to the list's streams.
    fn update(&self, id: u64, change: impl FnOnce(&mut Stored)) -> Option<Note> {
        let row = {
            let mut notes = self.notes.lock().expect("notes lock poisoned");
            let stored = notes.items.iter_mut().find(|stored| stored.note.id == id)?;
            change(stored);
            stored.note.clone()
        };
        let _ = self.changed.send(row.clone());
        Some(row)
    }
}

mod class {
    use hypersig::html::Class;

    use super::STYLE;

    pub(crate) const ROW: Class = STYLE.class("row");
    pub(crate) const NAME: Class = STYLE.class("name");
    pub(crate) const STATUS: Class = STYLE.class("status");
    pub(crate) const TEXT: Class = STYLE.class("text");
}

const STYLE: Stylesheet = Stylesheet::new(
    "
body { font-family: system-ui, sans-serif; margin: 1.5rem; }
ul { list-style: none; padding: 0; }
.row { display: flex; gap: .5rem; align-items: center; padding: .4rem 0; border-bottom: 1px solid #ddd; }
.name { flex: 1; }
.status { color: #555; min-height: 1.2em; }
.text { width: 100%; min-height: 60vh; font: 1rem/1.5 ui-monospace, monospace; box-sizing: border-box; }
button { font-size: 1rem; }
",
);

fn file_name(path: &Path) -> String {
    path.file_name().map_or_else(
        || path.display().to_string(),
        |name| name.to_string_lossy().into_owned(),
    )
}

async fn index(state: AppState, (): ()) -> Page<NotesPage> {
    let status = NotesPage::status();
    let initial = NotesPage {
        notes: state.rows(),
        status: String::new(),
    };
    Page::new("Notes", initial).style(STYLE).child(
        div()
            .on_load(UPDATES.call().reconnect())
            .child(h1().text("Notes"))
            .child(
                button()
                    .node_ref(OPEN_BUTTON)
                    .on_click(OPEN.call())
                    .text("Open…"),
            )
            .child(p().class(class::STATUS).bind_text(status))
            .child(ul().child(each_keyed(
                NotesPage::notes(),
                |note| note.at(Note::id()),
                |note| {
                    let saves = note.at(Note::saves());
                    li().class(class::ROW)
                        .child(span().class(class::NAME).bind_text(note.at(Note::name())))
                        .child(span().bind_text(saves.to_text().concat(" saves")))
                        .child(
                            button()
                                .on_click(SHOW.call_with(note.at(Note::id())))
                                .text("Edit"),
                        )
                },
            ))),
    )
}

async fn note(state: AppState, id: u64) -> Option<Page<EditPage>> {
    let path = state.path(id)?;
    let text = fs::read_to_string(&path).unwrap_or_default();
    let name = file_name(&path);
    let page = EditPage {
        id,
        name: name.clone(),
        text,
        status: String::new(),
    };
    let args = (EditPage::id(), EditPage::text());
    Some(
        Page::new(name, page).style(STYLE).child(
            div()
                .child(h1().bind_text(EditPage::name()))
                .child(
                    textarea()
                        .node_ref(TEXT)
                        .class(class::TEXT)
                        .bind(EditPage::text()),
                )
                .child(
                    div()
                        .class(class::ROW)
                        .child(
                            button()
                                .node_ref(SAVE_BUTTON)
                                .on_click(SAVE.call_with(args))
                                .text("Save"),
                        )
                        .child(
                            button()
                                .node_ref(SAVE_AS_BUTTON)
                                .on_click(SAVE_AS.call_with(args))
                                .text("Save as…"),
                        )
                        .child(
                            button()
                                .node_ref(COPY_BUTTON)
                                .on_click(COPY.call_with(EditPage::text()))
                                .text("Copy"),
                        ),
                )
                .child(p().class(class::STATUS).bind_text(EditPage::status())),
        ),
    )
}

/// Asks for a text file, keeps it as a note, and opens it in a window.
async fn open(state: AppState, (): ()) -> Patches<NotesPage> {
    let status = NotesPage::status();
    let chosen = state
        .os
        .dialog()
        .title("Open a note")
        .filter("Text", &["txt", "md"])
        .open_file()
        .await;
    let Some(path) = chosen else {
        return status.patch(String::new()).into();
    };
    let note = match state.keep(path.clone()) {
        Ok(note) => note,
        Err(error) => {
            return status
                .patch(format!("{} could not be read: {error}", path.display()))
                .into();
        }
    };
    match state.os.windows().open(&NOTE.url(note.id)).await {
        Ok(_) => status.patch(format!("Opened {}.", note.name)).into(),
        Err(error) => status
            .patch(format!("{} could not be shown: {error}", note.name))
            .into(),
    }
}

/// Opens the note `id` in another window.
async fn show(state: AppState, id: u64) -> Patches<NotesPage> {
    match state.os.windows().open(&NOTE.url(id)).await {
        Ok(_) => Patches::new(),
        Err(error) => NotesPage::status()
            .patch(format!("The note could not be shown: {error}"))
            .into(),
    }
}

async fn save(state: AppState, (id, text): (u64, String)) -> Patches<EditPage> {
    let status = EditPage::status();
    let Some(path) = state.path(id) else {
        return status
            .patch("This note is no longer open.".to_owned())
            .into();
    };
    if let Err(error) = fs::write(&path, &text) {
        return status.patch(format!("Not saved: {error}")).into();
    }
    state.update(id, |stored| stored.note.saves += 1);
    status.patch("Saved.".to_owned()).into()
}

/// Asks where to save the note, writes it there, and keeps that file.
async fn save_as(state: AppState, (id, text): (u64, String)) -> Patches<EditPage> {
    let status = EditPage::status();
    let name = state
        .path(id)
        .map(|path| file_name(&path))
        .unwrap_or_default();
    let Some(path) = state
        .os
        .dialog()
        .title("Save the note as")
        .file_name(name)
        .save_file()
        .await
    else {
        return status.patch(String::new()).into();
    };
    if let Err(error) = fs::write(&path, &text) {
        return status.patch(format!("Not saved: {error}")).into();
    }
    let name = file_name(&path);
    state.update(id, |stored| {
        stored.path = path;
        stored.note.name.clone_from(&name);
        stored.note.saves += 1;
    });
    EditPage::name()
        .patch(name)
        .and(status.patch("Saved.".to_owned()))
        .into()
}

async fn copy(state: AppState, text: String) -> Patches<EditPage> {
    let status = EditPage::status();
    match state.os.clipboard().write_text(text).await {
        Ok(()) => status.patch("Copied.".to_owned()).into(),
        Err(error) => status.patch(format!("Not copied: {error}")).into(),
    }
}

/// Sends the list once, then each note whose row changed.
fn updates(state: AppState, (): ()) -> impl Stream<Item = Patches<NotesPage>> {
    let changes = BroadcastStream::new(state.changed.subscribe());
    let notes = NotesPage::notes();
    let current: Patches<NotesPage> = notes.patch(state.rows()).into();
    stream::once(async { current }).chain(changes.map(move |change| {
        match change {
            Ok(row) => notes
                .patch_upsert(&[row], |note| note.at(Note::id()))
                .into(),
            Err(_lagged) => notes.patch(state.rows()).into(),
        }
    }))
}

fn app(state: AppState) -> Router {
    pages(Router::new()).hypersig_runtime().with_state(state)
}

/// Opens the note window of each of `notes`.
async fn show_all(os: &Os, notes: Vec<Note>) {
    // hypersig_desktop logs a window it could not open, and a closing
    // application opens none, so the first failure ends the loop.
    for note in notes {
        if os.windows().open(&NOTE.url(note.id)).await.is_err() {
            break;
        }
    }
}

/// Opens a window for each file that a later start of notes names, in the
/// copy already running.
async fn open_launched(state: AppState, mut launches: Launches) {
    while let Some(launch) = launches.next().await {
        let mut notes = Vec::new();
        for arg in launch.args() {
            // A file that cannot be read opens no window; the running copy
            // has no terminal to say so in.
            if let Ok(note) = state.keep(launch.current_dir().join(arg)) {
                notes.push(note);
            }
        }
        show_all(&state.os, notes).await;
    }
}

/// Opens the notes list, and a window for each file named on the command line.
fn main() -> Result<(), hypersig_desktop::Error> {
    const APP: App = App::new("dev.hypersig.example-notes");
    let files: Vec<PathBuf> = std::env::args_os().skip(1).map(PathBuf::from).collect();
    APP.size(560, 640).run(&INDEX.url(()), |os| async move {
        let state = AppState::new(os.clone());
        tokio::spawn(open_launched(state.clone(), os.launches()));
        let mut notes = Vec::new();
        for path in files {
            notes.push(state.keep(path)?);
        }
        show_all(&os, notes).await;
        Ok::<_, std::io::Error>(app(state))
    })
}

#[cfg(test)]
mod tests {
    use std::env;

    use hypersig_desktop::testing::FakeOs;

    use super::*;

    /// A directory of its own for each test, with `today.md` in it.
    fn folder(test: &str) -> PathBuf {
        let dir = env::temp_dir().join(format!("example-notes-{test}-{}", std::process::id()));
        fs::create_dir_all(&dir).unwrap();
        fs::write(dir.join("today.md"), "# Today\n").unwrap();
        dir
    }

    /// Opens `today.md` in `dir` through the list page's button.
    async fn opened(fake: &FakeOs, state: &AppState, dir: &std::path::Path) {
        fake.answer_dialog([dir.join("today.md")]);
        let page = index(state.clone(), ()).await;
        let mut browser = page.simulate();
        let effects = browser.click(OPEN_BUTTON);
        assert_eq!(effects.calls(&OPEN), [()]);
        browser.apply(open(state.clone(), ()).await);
        assert_eq!(browser.value_of(NotesPage::status()), "Opened today.md.");
    }

    #[tokio::test]
    async fn opening_a_file_shows_it_in_a_window_of_its_own() {
        let dir = folder("open");
        let fake = FakeOs::new();
        let state = AppState::new(fake.os());
        opened(&fake, &state, &dir).await;

        assert_eq!(fake.open_windows(), [NOTE.url(1)]);
        let page = note(state, 1).await.unwrap();
        let browser = page.simulate();
        assert_eq!(browser.value_of(EditPage::text()), "# Today\n");
        assert_eq!(browser.value_of(EditPage::name()), "today.md");
        fs::remove_dir_all(dir).unwrap();
    }

    #[tokio::test]
    async fn a_later_start_opens_its_files_here() {
        let dir = folder("launch");
        let fake = FakeOs::new();
        let state = AppState::new(fake.os());
        tokio::spawn(open_launched(state.clone(), fake.os().launches()));

        fake.launch(&dir, ["today.md".into()]);
        while fake.open_windows().is_empty() {
            tokio::task::yield_now().await;
        }
        assert_eq!(fake.open_windows(), [NOTE.url(1)]);
        assert_eq!(state.rows().len(), 1);
        fs::remove_dir_all(dir).unwrap();
    }

    #[tokio::test]
    async fn a_cancelled_dialog_opens_nothing() {
        let fake = FakeOs::new();
        let state = AppState::new(fake.os());
        fake.cancel_dialog();
        let page = index(state.clone(), ()).await;
        let mut browser = page.simulate();
        browser.click(OPEN_BUTTON);
        browser.apply(open(state.clone(), ()).await);
        assert_eq!(fake.open_windows(), []);
        assert_eq!(state.rows(), []);
    }

    #[tokio::test]
    async fn a_save_writes_the_file_and_reaches_the_list_window() {
        let dir = folder("save");
        let fake = FakeOs::new();
        let state = AppState::new(fake.os());
        opened(&fake, &state, &dir).await;

        // The list window, with the stream it opens on load.
        let list_page = index(state.clone(), ()).await;
        let mut list = list_page.simulate();
        let mut stream = Box::pin(updates(state.clone(), ()));
        list.apply(stream.next().await.unwrap());

        let page = note(state.clone(), 1).await.unwrap();
        let mut editor = page.simulate();
        editor.input(TEXT, "# Today\nWrote the notes example.\n");
        let effects = editor.click(SAVE_BUTTON);
        let [call] = effects.calls(&SAVE).try_into().unwrap();
        editor.apply(save(state.clone(), call).await);

        assert_eq!(editor.value_of(EditPage::status()), "Saved.");
        assert_eq!(
            fs::read_to_string(dir.join("today.md")).unwrap(),
            "# Today\nWrote the notes example.\n"
        );
        list.apply(stream.next().await.unwrap());
        let rows = list.value_of(NotesPage::notes());
        assert_eq!(rows[0].saves, 1);
        fs::remove_dir_all(dir).unwrap();
    }

    #[tokio::test]
    async fn save_as_renames_the_note_in_both_windows() {
        let dir = folder("save-as");
        let fake = FakeOs::new();
        let state = AppState::new(fake.os());
        opened(&fake, &state, &dir).await;
        let mut stream = Box::pin(updates(state.clone(), ()));
        let list_page = index(state.clone(), ()).await;
        let mut list = list_page.simulate();
        list.apply(stream.next().await.unwrap());

        fake.answer_dialog([dir.join("tomorrow.md")]);
        let page = note(state.clone(), 1).await.unwrap();
        let mut editor = page.simulate();
        let effects = editor.click(SAVE_AS_BUTTON);
        let [call] = effects.calls(&SAVE_AS).try_into().unwrap();
        editor.apply(save_as(state.clone(), call).await);

        assert_eq!(editor.value_of(EditPage::name()), "tomorrow.md");
        assert_eq!(
            fs::read_to_string(dir.join("tomorrow.md")).unwrap(),
            "# Today\n"
        );
        list.apply(stream.next().await.unwrap());
        assert_eq!(list.value_of(NotesPage::notes())[0].name, "tomorrow.md");
        fs::remove_dir_all(dir).unwrap();
    }

    #[tokio::test]
    async fn copy_puts_the_text_on_the_clipboard() {
        let dir = folder("copy");
        let fake = FakeOs::new();
        let state = AppState::new(fake.os());
        opened(&fake, &state, &dir).await;

        let page = note(state.clone(), 1).await.unwrap();
        let mut editor = page.simulate();
        let effects = editor.click(COPY_BUTTON);
        let [text] = effects.calls(&COPY).try_into().unwrap();
        editor.apply(copy(state, text).await);

        assert_eq!(fake.clipboard_text().as_deref(), Some("# Today\n"));
        assert_eq!(editor.value_of(EditPage::status()), "Copied.");
        fs::remove_dir_all(dir).unwrap();
    }
}
