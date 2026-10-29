use hypersig::{Route, Signals};
use hypersig_ui::NavList;
use serde::{Deserialize, Serialize};

#[derive(Signals, Serialize, Deserialize)]
struct Admin {}

# async fn page<P>((): (), _: P) -> Option<hypersig::Page<Admin>> { None }
hypersig::routes! {
    fn pages(Router<()>);
    const USER: Route<u32> = "/users/{id}" => page;
}

// error[E0308]: mismatched types: expected `u32`, found `&str`
let sidebar = NavList::<Admin>::new("Main", USER.url(7)).item("You", USER, "7");
# let _ = pages;
