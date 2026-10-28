# use hypersig::Signals;
# use serde::{Deserialize, Serialize};
use hypersig::html::a;
use hypersig::{Page, PathSegment, Route};

#[derive(Signals, Serialize, Deserialize)]
struct Search {
    team: PathSegment,
}

# async fn member((): (), _: (PathSegment, i32)) -> Option<hypersig::Page<Search>> { None }
hypersig::routes! {
    fn pages(Router<()>);
    const MEMBER: Route<(PathSegment, i32)> = "/teams/{team}/members/{id}" => member;
}

// Literal parameters give a fixed href, percent-encoded on the server.
let fixed = a::<Search>().href(MEMBER.link_with((PathSegment::new("red team"), 7)));
assert_eq!(fixed.render(), r#"<a href="/teams/red%20team/members/7"></a>"#);

// A parameter that reads a signal is kept up to date in the browser.
let bound = a().href(MEMBER.link_with((Search::team(), 7)));
let page = Page::new("Team", Search { team: PathSegment::new("red") }).child(bound);
// The page's first HTML holds the URL of the signal's first value.
assert!(page.render().contains(r#"<a href="/teams/red/members/7""#));
# let _ = pages;
