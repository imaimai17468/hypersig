use hypersig::Route;
use hypersig::html::a;

const ORG: Route<String> = Route::new("/orgs/{org}");
// The child repeats the parent's path and parameters, then adds its own.
const MEMBER: Route<(String, i32)> = ORG.child("/orgs/{org}/members/{id}");

let link = a::<()>().href(MEMBER.link_with(("acme", 9)));
assert_eq!(link.render(), r#"<a href="/orgs/acme/members/9"></a>"#);
