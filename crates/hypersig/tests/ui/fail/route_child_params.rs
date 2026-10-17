//! A child route whose parameters do not start with its parent's.

use hypersig::Route;

const ORG: Route<String> = Route::new("/orgs/{org}");
const MEMBER: Route<(i32, i32)> = ORG.child("/orgs/{org}/members/{id}");

fn main() {
    let _ = MEMBER;
}
