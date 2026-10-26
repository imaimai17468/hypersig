use axum::Router;
use hypersig::html::{El, button, form, input, p, part, attr};
use hypersig::{Action, Page, Part, Patches, RouterExt, Signals};
use serde::{Deserialize, Serialize};

#[derive(Signals, Serialize, Deserialize, Default)]
struct Newsletter {
    email: String,
    joined: bool,
}

// The part's action is a line of the application's block, which registers it once.
hypersig::routes! {
    fn pages(Router<()>);
    action JOIN: Action<Newsletter, String> = post "/join" => join;
}

impl Part for Newsletter {
    fn render() -> El<Self> {
        let email = Newsletter::email();
        form()
            .on_submit(JOIN.call_with(email))
            .child(input().bind(email))
            .child(button().attr(attr::TYPE, "submit").text("Join"))
            .child(p().show(Newsletter::joined()).text("Thanks for joining."))
    }
}

async fn join((): (), email: String) -> Patches<Newsletter> {
    Newsletter::email()
        .patch(email.trim().to_owned())
        .and(Newsletter::joined().patch(true))
        .into()
}

// Two pages, each with the newsletter at its field `footer`.
#[derive(Signals, Serialize, Deserialize)]
struct Home {
    footer: Newsletter,
}

#[derive(Signals, Serialize, Deserialize)]
struct About {
    footer: Newsletter,
}

let home = Page::new("Home", Home { footer: Newsletter::default() })
    .child(part(Home::footer()));
let about = Page::new("About", About { footer: Newsletter::default() })
    .child(part(About::footer()));

// One handler answers the form on both pages.
let app: Router = pages(Router::new()).hypersig_runtime();
# let _ = (home, about, app);
