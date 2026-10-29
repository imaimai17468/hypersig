use axum::Router;
use axum::http::{HeaderMap, HeaderName, header};
use hypersig::{Action, Patches, Signals};
use serde::{Deserialize, Serialize};

#[derive(Signals, Serialize, Deserialize)]
struct Home {
    greeting: String,
}

hypersig::routes! {
    fn pages(Router<()>);
    action GREET: Action<Home> = post "/greet" => greet;
    action SIGN_OUT: Action<Home> = post "/sign-out" => sign_out;
}

// `HeaderMap` is an extractor: it reads every header of the request.
async fn greet((): (), headers: HeaderMap, (): ()) -> Patches<Home> {
    let language = headers.get(header::ACCEPT_LANGUAGE).and_then(|value| value.to_str().ok());
    let japanese = language.is_some_and(|value| value.starts_with("ja"));
    let greeting = if japanese { "こんにちは" } else { "Hello" };
    Home::greeting().patch(greeting.to_owned()).into()
}

// Headers go beside the patches.
type Cookie = [(HeaderName, &'static str); 1];

async fn sign_out((): (), (): ()) -> (Cookie, Patches<Home>) {
    let forget = [(header::SET_COOKIE, "session=; Max-Age=0; HttpOnly; Path=/")];
    (forget, Home::greeting().patch(String::new()).into())
}

let _app: Router = pages(Router::new());
