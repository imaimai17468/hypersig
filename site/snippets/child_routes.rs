# use hypersig::Page;
# async fn org((): (), _: String) -> Option<Page<Org>> { None }
# async fn member((): (), _: (String, i32)) -> Option<Page<Org>> { None }
use hypersig::html::a;
use hypersig::{Route, Signals};
use serde::{Deserialize, Serialize};

#[derive(Signals, Serialize, Deserialize)]
struct Org {}

hypersig::routes! {
    fn pages(Router<()>);
    const ORG: Route<String> = "/orgs/{org}" => org;
    // The child repeats the parent's path and parameters, then adds its own.
    const MEMBER: Route<(String, i32)> = "/orgs/{org}/members/{id}" in ORG => member;
}

let link = a::<Org>().href(MEMBER.link_with(("acme", 9)));
assert_eq!(link.render(), r#"<a href="/orgs/acme/members/9"></a>"#);
# let _ = pages;
