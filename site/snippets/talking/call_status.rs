use axum::http::StatusCode;
use hypersig::html::{Role, a, attr, button, div, p};
use hypersig::{Action, Patches, Signals};
use serde::{Deserialize, Serialize};

#[derive(Signals, Serialize, Deserialize)]
struct Notes {
    saving: bool,
    problem: String,
    failure: i32,
}

hypersig::routes! {
    fn pages(Router<()>);
    action SAVE: Action<Notes> = post "/save" => save;
}
# async fn save((): (), (): ()) -> Patches<Notes> { Patches::new() }

let save = SAVE
    .call()
    .indicator(Notes::saving())
    .error(Notes::problem())
    .status(Notes::failure());
let signed_out = Notes::failure().equals(i32::from(StatusCode::UNAUTHORIZED.as_u16()));
let page = div::<Notes>()
    .child(
        button()
            .attr_when(attr::DISABLED, Notes::saving())
            .on_click(save)
            .text("Save"),
    )
    .child(p().role(Role::Alert).bind_text(Notes::problem()))
    .child(a().show(signed_out).text("Sign in again"));
# let _ = (page, pages);
