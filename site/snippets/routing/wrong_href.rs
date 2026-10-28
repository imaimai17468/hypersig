# use hypersig::{Page, Signals};
# use serde::{Deserialize, Serialize};
# #[derive(Signals, Serialize, Deserialize)]
# struct Team {}
# async fn member((): (), _: i32) -> Option<Page<Team>> { None }
use hypersig::Route;
use hypersig::html::a;

hypersig::routes! {
    fn pages(Router<()>);
    const MEMBER: Route<i32> = "/members/{id}" => member;
}

// error[E0277]: `&str` is not a URL an `href` or a `src` takes on the page
// whose signals are `Team`
let _link = a::<Team>().href("/members/7");
# let _ = (pages, MEMBER);
