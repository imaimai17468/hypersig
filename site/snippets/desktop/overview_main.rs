use axum::Router;
use hypersig::{Page, Route, RouterExt, Signals};
use hypersig_desktop::App;
use serde::{Deserialize, Serialize};

#[derive(Signals, Serialize, Deserialize)]
struct Home {}

hypersig::routes! {
    fn pages(Router<()>);
    const INDEX: Route = "/" => index;
}

async fn index((): (), (): ()) -> Page<Home> {
    Page::new("Home", Home {})
}

const APP: App = App::new("dev.example.home");

fn main() -> Result<(), hypersig_desktop::Error> {
    APP.size(800, 600).run(&INDEX.url(()), |_os| async {
        pages(Router::new()).hypersig_runtime()
    })
}
