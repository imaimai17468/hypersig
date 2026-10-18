use hypersig::{Page, Route, Signals};
use hypersig_ui::Breadcrumb;
use serde::{Deserialize, Serialize};

#[derive(Signals, Serialize, Deserialize)]
struct Issue {}

async fn page((): (), (): ()) -> Option<Page<Issue>> {
    None
}

hypersig::routes! {
    fn pages(Router<()>);
    const ISSUES: Route = "/issues" => page;
}

fn main() {
    let _ = pages;
    // The current page ends the trail, so no link can follow it.
    let _ = Breadcrumb::<Issue>::new("Breadcrumb")
        .current("Login is slow")
        .link("Issues", ISSUES.link());
}
