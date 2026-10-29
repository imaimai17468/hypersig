use hypersig::{Action, Patches, Segment, Signals};
use hypersig_ui::{SortOrder, TableState};
use serde::{Deserialize, Serialize};

#[derive(Signals, Serialize, Deserialize)]
struct Users {
    table: TableState,
}

#[derive(Segment, Serialize, Deserialize)]
enum By {
    Email,
}

# async fn load((): (), _: TableState) -> Patches<Users> { Patches::new() }
hypersig::routes! {
    fn actions(Router<()>);
    action LOAD: Action<Users, TableState> = post "/users/rows" => load;
}

// The state after the visitor typed "ada", sorted by email descending,
// and moved to the second page.
let mut state = TableState::default().with_sorting(&By::Email, SortOrder::Descending);
state.query = "ada".into();
state.page = 1;
let request = LOAD.request(&state);
assert_eq!(request.method(), "POST");
assert_eq!(request.uri(), "/users/rows");
# use http_body_util::BodyExt;
# let body = futures::executor::block_on(request.into_body().collect()).unwrap().to_bytes();
# assert_eq!(body, r#"{"query":"ada","page":1,"sort":"Email","descending":true}"#);
# let _ = actions;
