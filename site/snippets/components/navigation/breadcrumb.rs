use hypersig::html::h1;
use hypersig::{Page, Route, Signals};
use hypersig_ui::Breadcrumb;
use serde::{Deserialize, Serialize};

#[derive(Signals, Serialize, Deserialize)]
struct Tracker {}

# async fn home((): (), (): ()) -> Option<Page<Tracker>> { None }
# async fn project((): (), _: u32) -> Option<Page<Tracker>> { None }
hypersig::routes! {
    fn pages(Router<()>);
    const HOME: Route = "/" => home;
    const PROJECT: Route<u32> = "/projects/{id}" => project;
    const ISSUE: Route<(u32, u32)> = "/projects/{id}/issues/{number}" => issue;
}

async fn issue((): (), (id, number): (u32, u32)) -> Option<Page<Tracker>> {
    let title = format!("Issue {number}");
    let trail = Breadcrumb::new("Breadcrumb")
        .link("Home", HOME.link())
        .link(format!("Project {id}"), PROJECT.link_with(&id))
        .current(title.clone());
    Some(Page::new(title.clone(), Tracker {}).child(trail).child(h1().text(title)))
}

# let _ = pages;
# let html = futures::executor::block_on(issue((), (3, 41))).unwrap().render();
# assert!(html.contains(r#"<nav aria-label="Breadcrumb">"#));
# assert!(html.contains(r#"href="/projects/3""#));
# assert!(html.contains(r#"aria-current="page""#));
