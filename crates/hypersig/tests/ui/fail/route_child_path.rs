//! A child route whose path does not continue its parent's.

use hypersig::Route;

const ORG: Route<String> = Route::new("/orgs/{org}");
const MEMBER: Route<(String, i32)> = ORG.child("/teams/{team}/members/{id}");

fn main() {
    let _ = MEMBER;
}
