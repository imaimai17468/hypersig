use hypersig::{Action, Patches, Signals};
use serde::{Deserialize, Serialize};

#[derive(Signals, Serialize, Deserialize)]
struct Shop {
    results: Vec<String>,
    stars: i32,
}

hypersig::routes! {
    fn pages(Router<()>);
    action FIND: Action<Shop, String> = get "/find" => find;
    action RATE: Action<Shop, (i32, String)> = post "/rate" => rate;
}
# async fn find((): (), _: String) -> Patches<Shop> { Patches::new() }
# async fn rate((): (), _: (i32, String)) -> Patches<Shop> { Patches::new() }

// A `GET` carries the arguments as JSON in the `args` query parameter.
let find = FIND.request(&"tea".to_owned());
assert_eq!(find.method(), "GET");
assert_eq!(find.uri(), "/find?args=%22tea%22");
assert_eq!(find.headers()["accept"], "text/event-stream");
assert_eq!(find.headers()["hypersig-call"], "1");

// A `POST` carries them as its JSON body, here `[5,"Lovely"]`.
let rate = RATE.request(&(5, "Lovely".to_owned()));
assert_eq!(rate.method(), "POST");
assert_eq!(rate.uri(), "/rate");
assert_eq!(rate.headers()["content-type"], "application/json");
# let _ = pages;
