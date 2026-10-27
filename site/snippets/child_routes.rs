# use hypersig::Page;
# async fn org((): (), _: PathSegment) -> Option<Page<Org>> { None }
# async fn member((): (), _: (PathSegment, i32)) -> Option<Page<Org>> { None }
use hypersig::html::a;
use hypersig::{PathSegment, Route, Signals};
use serde::{Deserialize, Serialize};

#[derive(Signals, Serialize, Deserialize)]
struct Org {}

hypersig::routes! {
    fn pages(Router<()>);
    const ORG: Route<PathSegment> = "/orgs/{org}" => org;
    // The child repeats the parent's path and parameters, then adds its own.
    const MEMBER: Route<(PathSegment, i32)> = "/orgs/{org}/members/{id}" in ORG => member;
}

let link = a::<Org>().href(MEMBER.link_with((PathSegment::new("acme"), 9)));
assert_eq!(link.render(), r#"<a href="/orgs/acme/members/9"></a>"#);
# let _ = pages;
