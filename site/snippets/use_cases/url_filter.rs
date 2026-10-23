# use hypersig::html::a;
# use hypersig::{Page, Route, Search, Signals};
# use serde::{Deserialize, Serialize};
#
#[derive(Signals, Search, Serialize, Deserialize)] //+
struct Filter { //+
    status: String, //+
    page: i32, //+
} //+

# #[derive(Signals, Serialize, Deserialize)]
# struct Requests {}
#
# async fn requests((): (), _: Filter) -> Page<Requests> { Page::new("Requests", Requests {}) }
hypersig::routes! {
    fn pages(Router<()>);
//-    const REQUESTS: Route = "/requests" => requests;
    // `requests` receives the filter as a `Filter`. //+
    const REQUESTS: Route<(), Filter> = "/requests" => requests; //+
}

let waiting = Filter { status: "waiting".to_owned(), page: 1 }; //+
//-let link = a::<Requests>().href(REQUESTS.link());
let link = a::<Requests>().href(REQUESTS.link_with(&waiting)); //+
assert_eq!(link.render(), r#"<a href="/requests?page=1&amp;status=waiting"></a>"#);
# let _ = pages;
