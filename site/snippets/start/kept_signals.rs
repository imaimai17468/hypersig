# use axum::Router;
# use hypersig::Signals;
# use serde::{Deserialize, Serialize};
use hypersig::html::{button, div, each, input, li, ul};
use hypersig::{Action, Page, Patches, Route};

#[derive(Signals, Serialize, Deserialize)]
struct Notes {
    draft: String,
    notes: Vec<String>,
}

hypersig::routes! {
    fn pages(Router<()>);
    const INDEX: Route = "/" => index;
    action ADD: Action<Notes, String> = post "/notes" => add;
}

async fn index((): (), (): ()) -> Page<Notes> {
    let draft = Notes::draft();
    let empty = Notes { draft: String::new(), notes: Vec::new() };
    Page::new("Notes", empty).child(
        div()
            .child(input().bind(draft))
            .child(button().on_click(ADD.call_with(draft)).text("Add"))
            .child(ul().child(each(Notes::notes(), |note| li().bind_text(note)))),
    )
}

async fn add((): (), text: String) -> Patches<Notes> {
    Patches::new()
        .list(Notes::notes().patch_append(&[text]))
        .signals(Notes::draft().patch(String::new()))
}
# assert_eq!(Notes::SHAPE, "draft:String,notes:Vec < String >");
# let _ = pages;
