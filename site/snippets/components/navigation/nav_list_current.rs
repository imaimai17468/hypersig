use hypersig::{Route, Signals};
use hypersig_ui::NavList;
use serde::{Deserialize, Serialize};

#[derive(Signals, Serialize, Deserialize)]
struct Admin {}

# async fn page<P>((): (), _: P) -> Option<hypersig::Page<Admin>> { None }
hypersig::routes! {
    fn pages(Router<()>);
    const USERS: Route = "/users" => page;
}

// error[E0308]: mismatched types: expected `Url`, found `&str`
let sidebar = NavList::<Admin>::new("Main", "/users").item("Users", USERS, ());
# let _ = pages;
