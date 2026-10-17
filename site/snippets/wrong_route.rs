# use hypersig::{Page, Signals};
# use serde::{Deserialize, Serialize};
# #[derive(Signals, Serialize, Deserialize)]
# struct Team {}
# async fn member((): (), _: i32) -> Option<Page<Team>> { None }
use hypersig::Route;

hypersig::routes! {
    fn pages(Router<()>);
    // error[E0080]: evaluation panicked: the route's `{name}` segments differ
    // in number from the values of its parameter type
    const MEMBER: Route<i32> = "/members" => member;
}
# let _ = pages;
