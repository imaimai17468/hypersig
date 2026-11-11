# use axum::Router;
# use hypersig::{Page, Route, RouterExt, Signals};
# use serde::{Deserialize, Serialize};
# #[derive(Signals, Serialize, Deserialize)]
# struct Home {}
# hypersig::routes! {
#     fn pages(Router<()>);
#     const INDEX: Route = "/" => index;
# }
# async fn index((): (), (): ()) -> Page<Home> { Page::new("Home", Home {}) }
use std::process::ExitCode;

use hypersig_desktop::{App, ErrorKind};

const APP: App = App::new("dev.example.home");

fn main() -> ExitCode {
    let ran = APP.run(&INDEX.url(()), |_os| async {
        pages(Router::new()).hypersig_runtime()
    });
    let Err(error) = ran else {
        return ExitCode::SUCCESS;
    };
    match error.kind() {
        // The first copy goes on running; this one opened no window.
        ErrorKind::AlreadyRunning => {
            eprintln!("The application is already open.");
            ExitCode::SUCCESS
        }
        // `ErrorKind` is `#[non_exhaustive]`, so a match needs this arm.
        _ => {
            eprintln!("{error}");
            ExitCode::FAILURE
        }
    }
}
