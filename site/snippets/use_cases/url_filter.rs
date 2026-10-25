# use hypersig::html::a;
# use hypersig::{Page, Route, Search, Signals};
# use serde::{Deserialize, Serialize};
#
// Both fields are `Option`, so `/requests` without a query string still loads. //+
#[derive(Signals, Search, Serialize, Deserialize)] //+
struct Filter { //+
    status: Option<String>, //+
    page: Option<i32>, //+
} //+

# #[derive(Signals, Serialize, Deserialize)]
# struct Requests {}
#
# async fn requests((): (), filter: Filter) -> Page<Requests> {
#     let _ = (filter.status.as_deref().unwrap_or("all"), filter.page.unwrap_or(1));
#     Page::new("Requests", Requests {})
# }
hypersig::routes! {
    fn pages(Router<()>);
//-    const REQUESTS: Route = "/requests" => requests;
    // `requests` receives the filter as a `Filter`. //+
    const REQUESTS: Route<(), Filter> = "/requests" => requests; //+
}

let waiting = Filter { status: Some("waiting".to_owned()), page: None }; //+
//-let link = a::<Requests>().href(REQUESTS.link());
let link = a::<Requests>().href(REQUESTS.link_with(&waiting)); //+
assert_eq!(link.render(), r#"<a href="/requests?status=waiting"></a>"#);
# let _ = pages;
