# use hypersig::{Page, Signals};
# use serde::{Deserialize, Serialize};
# #[derive(Signals, Serialize, Deserialize)]
# struct Org {}
# async fn org((): (), _: PathSegment) -> Option<Page<Org>> { None }
# async fn member((): (), _: (i32, PathSegment)) -> Option<Page<Org>> { None }
use hypersig::{PathSegment, Route};

hypersig::routes! {
    fn pages(Router<()>);
    const ORG: Route<PathSegment> = "/orgs/{org}" => org;
    // error[E0277]: `(i32, PathSegment)` does not start with `PathSegment`,
    // the parameters of the parent route
    const MEMBER: Route<(i32, PathSegment)> = "/orgs/{org}/members/{id}" in ORG => member;
}
# let _ = pages;
