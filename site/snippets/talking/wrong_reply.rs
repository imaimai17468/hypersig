use hypersig::{Action, Signals};
use serde::{Deserialize, Serialize};

#[derive(Signals, Serialize, Deserialize)]
struct Review {
    comment: String,
}

// error[E0277]: `String` is not a reply of an action on the page whose signals are `Review`
hypersig::routes! {
    fn pages(Router<()>);
    action EDIT: Action<Review, String> = post "/edit" => edit;
}

async fn edit((): (), comment: String) -> String {
    comment
}
# let _ = pages;
