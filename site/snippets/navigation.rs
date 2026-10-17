# use hypersig::Signals;
# use serde::{Deserialize, Serialize};
use hypersig::html::button;
use hypersig::{Action, PageResponse, Patches, Route};

#[derive(Signals, Serialize, Deserialize)]
struct Draft {
    saving: bool,
}

# async fn post((): (), _: i32) -> Option<hypersig::Page<Draft>> { None }
hypersig::routes! {
    fn pages(Router<()>);
    const POST: Route<i32> = "/posts/{id}" => post;
    const OLD_POST: Route<i32> = "/p/{id}" => old_post;
}

const PUBLISH: Action<Draft> = Action::post("/publish");

// A page handler answers an old address with a redirect.
async fn old_post((): (), id: i32) -> PageResponse {
    PageResponse::redirect(POST.url(id))
}

// An action sends the browser to the page it created.
async fn publish((): (), (): ()) -> Patches<Draft> {
    let id = 7;
    Patches::from(Draft::saving().patch(false)).navigate(POST.url(id))
}

// A button leaves the page after setting a signal.
let _back = button::<Draft>().on_click(Draft::saving().set(false).then(POST.link_with(&1).navigate()));
# let _ = (pages, PUBLISH, publish);
