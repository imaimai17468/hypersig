# use axum::http::StatusCode;
use hypersig::html::{Role, button, div, p};
use hypersig::{Action, Patches, Signals};
use serde::{Deserialize, Serialize};

#[derive(Signals, Serialize, Deserialize)]
struct Notes {
    problem: String,
    failure: i32,
}

hypersig::routes! {
    fn actions(Router<()>);
    action SAVE: Action<Notes> = post "/save" => save;
}

async fn save((): (), (): ()) -> Result<Patches<Notes>, (StatusCode, &'static str)> {
    Err((StatusCode::CONFLICT, "Someone else saved these notes first."))
}

let conflict = Notes::failure().equals(i32::from(StatusCode::CONFLICT.as_u16()));
let page = div::<Notes>()
    .child(
        button()
            .on_click(SAVE.call().error(Notes::problem()).status(Notes::failure()))
            .text("Save"),
    )
    .child(p().role(Role::Alert).bind_text(Notes::problem()))
    .child(p().show(conflict).text("Reload the page to see their version."));
# let _ = (page, actions);
