use hypersig::{Action, Patches, Signals};
use hypersig_ui::{Column, ServerTable, TableState};
use serde::{Deserialize, Serialize};

#[derive(Signals, Serialize, Deserialize)]
struct User {
    id: i32,
    email: String,
}

#[derive(Signals, Serialize, Deserialize)]
struct Users {
    users: Vec<User>,
    table: TableState,
}

# async fn load((): (), _: TableState) -> Patches<Users> { Patches::new() }
hypersig::routes! {
    fn actions(Router<()>);
    action LOAD: Action<Users, TableState> = post "/users/rows" => load;
}

// error[E0277]: `&str` cannot be one segment of a URL's path
let table = ServerTable::new(
    "Users",
    Users::table(),
    Users::users(),
    |user| user.at(User::id()),
    LOAD.call_with(Users::table()),
)
.sortable_column(Column::text("Email", User::email()), &"email");
# let _ = actions;
