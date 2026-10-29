use hypersig::{Route, Signals};
use hypersig_ui::Breadcrumb;
use serde::{Deserialize, Serialize};

#[derive(Signals, Serialize, Deserialize)]
struct Tracker {}

# async fn page((): (), (): ()) -> Option<hypersig::Page<Tracker>> { None }
hypersig::routes! {
    fn pages(Router<()>);
    const HOME: Route = "/" => page;
}

// error[E0599]: no method named `link` found for struct `El<S>` in the current scope
let trail = Breadcrumb::<Tracker>::new("Breadcrumb")
    .current("Issue 41")
    .link("Home", HOME.link());
# let _ = pages;
