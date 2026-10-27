# use hypersig::html::a;
# use serde::{Deserialize, Serialize};
use hypersig::{Expr, PathSegment, Route, Search, Signals};

#[derive(Signals, Search, Serialize, Deserialize)]
struct Filter {
    q: String,
    page: i32,
    tag: Option<String>,
}

# async fn team((): (), _: (PathSegment, Filter)) -> Option<hypersig::Page<Filter>> { None }
hypersig::routes! {
    fn pages(Router<()>);
    // `team` receives `(PathSegment, Filter)`.
    const TEAM: Route<PathSegment, Filter> = "/teams/{team}" => team;
}

let first = Filter { q: "rust".to_owned(), page: 1, tag: None };
const RED: PathSegment = PathSegment::new("red");
let link = a::<Filter>().href(TEAM.link_with((RED, &first)));
assert_eq!(link.render(), r#"<a href="/teams/red?page=1&amp;q=rust"></a>"#);

// A field can come from a signal, here the page the reader is on.
let next = Expr::lit(&first).with(Filter::page(), Filter::page() + 1);
let _ = a().href(TEAM.link_with((RED, next)));
# let _ = pages;
