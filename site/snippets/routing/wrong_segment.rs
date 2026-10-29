# use hypersig::{Page, Signals};
# use serde::{Deserialize, Serialize};
# #[derive(Signals, Serialize, Deserialize)]
# struct Team {}
# async fn member((): (), _: String) -> Option<Page<Team>> { None }
use hypersig::Route;

hypersig::routes! {
    fn pages(Router<()>);
    // error[E0277]: `String` cannot be one segment of a URL's path
    const MEMBER: Route<String> = "/members/{name}" => member;
}
# let _ = pages;
