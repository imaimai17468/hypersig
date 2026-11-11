# use axum::Router;
# use hypersig::{Page, Route, RouterExt, Signals};
use std::process::ExitCode;

use hypersig_desktop::{App, ErrorKind};
# use serde::{Deserialize, Serialize};
# #[derive(Signals, Serialize, Deserialize)]
# struct Home {}
# hypersig::routes! {
#     fn pages(Router<()>);
#     const HOME: Route = "/" => home;
# }
# async fn home((): (), (): ()) -> Page<Home> {
#     Page::new("Home", Home {})
# }

const APP: App = App::new("dev.example.home");

fn main() -> ExitCode {
    let run = APP.run(&HOME.url(()), |_os| async {
        pages(Router::new()).hypersig_runtime()
    });
    match run {
        Ok(()) => ExitCode::SUCCESS,
        Err(error) if error.kind() == ErrorKind::AlreadyRunning => {
            eprintln!("The application is already open.");
            ExitCode::SUCCESS
        }
        Err(error) => {
            eprintln!("{error}");
            ExitCode::FAILURE
        }
    }
}
