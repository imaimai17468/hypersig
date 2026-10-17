# use hypersig::html::a;
# use serde::{Deserialize, Serialize};
use hypersig::{Expr, Route, Signals};

#[derive(Signals, Serialize, Deserialize)]
struct Filter {
    q: String,
    page: i32,
    tag: Option<String>,
}

const TEAM: Route<String, Filter> = Route::new("/teams/{team}");

// Served with `.page(TEAM, |state, (team, filter): (String, Filter)| async move { ... })`.
let first = Filter { q: "rust".to_owned(), page: 1, tag: None };
let link = a::<Filter>().href(TEAM.link_with(("red", &first)));
assert_eq!(link.render(), r#"<a href="/teams/red?page=1&amp;q=rust"></a>"#);

// A field can come from a signal, here the page the reader is on.
let next = Expr::lit(&first).with(Filter::page(), Filter::page() + 1);
let _ = a().href(TEAM.link_with(("red", next)));
