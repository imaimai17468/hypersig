use hypersig::html::{div, p};
use hypersig::{Page, Route, Signals, Url};
use hypersig_ui::NavList;
use serde::{Deserialize, Serialize};

#[derive(Signals, Serialize, Deserialize)]
struct Admin {}

hypersig::routes! {
    fn pages(Router<()>);
    const DASHBOARD: Route = "/" => dashboard;
    const USERS: Route = "/users" => users;
    const USER: Route<u32> = "/users/{id}" => user;
}

/// The sidebar of every page, marking the section of the page at `current`.
fn sidebar(current: Url) -> NavList<Admin> {
    NavList::new("Main", current)
        .item("Dashboard", DASHBOARD, ())
        .item("Users", USERS, ())
}

async fn dashboard((): (), (): ()) -> Option<Page<Admin>> {
    Some(Page::new("Dashboard", Admin {}).child(sidebar(DASHBOARD.url(()))))
}

async fn users((): (), (): ()) -> Option<Page<Admin>> {
    Some(Page::new("Users", Admin {}).child(sidebar(USERS.url(()))))
}

async fn user((): (), id: u32) -> Option<Page<Admin>> {
    let page = Page::new("User", Admin {})
        .child(sidebar(USER.url(id)))
        .child(div().child(p().text(format!("User {id}"))));
    Some(page)
}

# let _ = pages;
# let html = futures::executor::block_on(user((), 7)).unwrap().render();
# assert!(html.contains(r#"href="/users" aria-current="true""#));
# assert!(html.contains(r#"<nav aria-label="Main">"#));
# let html = futures::executor::block_on(users((), ())).unwrap().render();
# assert!(html.contains(r#"href="/users" aria-current="page""#));
# assert!(!html.contains(r#"href="/" aria-current"#));
