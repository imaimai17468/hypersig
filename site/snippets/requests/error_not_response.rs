use hypersig::{Action, Patches, Signals};
use serde::{Deserialize, Serialize};

#[derive(Signals, Serialize, Deserialize)]
struct Notes {
    count: i32,
}

// No `IntoResponse`: the router has no answer to send for it.
struct NotesFull;

// error[E0277]: `Result<Patches<Notes>, NotesFull>` is not a reply of an action on the page whose signals are `Notes`
hypersig::routes! {
    fn pages(Router<()>);
    action ADD: Action<Notes, i32> = post "/notes" => add;
}

async fn add((): (), count: i32) -> Result<Patches<Notes>, NotesFull> {
    let count = (count < 3).then_some(count + 1).ok_or(NotesFull)?;
    Ok(Notes::count().patch(count).into())
}
# let _ = pages;
