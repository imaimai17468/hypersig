use hypersig::html::a;
use hypersig::{Page, Route, Signals};
use serde::{Deserialize, Serialize};

#[derive(Signals, Serialize, Deserialize)]
struct Team {
    lead: i32,
}

#[derive(Signals, Serialize, Deserialize)]
struct Other {
    id: i32,
}

hypersig::routes! {
    fn pages(Router<()>);
    const MEMBER: Route<i32> = "/members/{id}" => no_page;
}

async fn no_page<A>((): (), _: A) -> Option<Page<Team>> {
    None
}

fn main() {
    let _ = pages;
    let _ = a::<Team>().href(MEMBER.link_with(Other::id()));
}
