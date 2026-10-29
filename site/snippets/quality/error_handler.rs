# use hypersig::{Action, Signals};
# use serde::{Deserialize, Serialize};
# #[derive(Signals, Serialize, Deserialize)]
# struct Board {
#     title: String,
# }
hypersig::routes! {
    fn actions(Router<()>);
    // error[E0277]: `String` is not a reply of an action on the page whose
    // signals are `Board`
    action RENAME: Action<Board, String> = post "/rename" => rename;
}

async fn rename((): (), title: String) -> String {
    title
}
# let _ = actions;
