# use axum::Router;
use hypersig::html::button;
use hypersig::{Page, Route, RouterExt, Signals};
use hypersig_desktop::App;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Copy, PartialEq)]
enum Theme {
    Light,
    Dark,
}

#[derive(Signals, Serialize, Deserialize)]
struct Home {
    theme: Theme,
}

hypersig::routes! {
    fn pages(Router<()>);
    const HOME: Route = "/" => home;
}

async fn home((): (), (): ()) -> Page<Home> {
    Page::new("Home", Home { theme: Theme::Light })
        // The next start reads this back while the server keeps its port.
        .persist(Home::theme())
        .child(button().on_click(Home::theme().set(&Theme::Dark)).text("Dark"))
}

const APP: App = App::new("dev.example.home");

fn main() -> Result<(), hypersig_desktop::Error> {
    APP.run(&HOME.url(()), |_os| async {
        pages(Router::new()).hypersig_runtime()
    })
}
