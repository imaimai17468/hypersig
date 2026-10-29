use hypersig::{Action, Page, Patches, Segment, Signals};
use hypersig_ui::{Column, ServerTable, SortOrder, TableState};
use serde::{Deserialize, Serialize};

#[derive(Signals, Serialize, Deserialize, Clone)]
struct User {
    id: i32,
    email: String,
}

#[derive(Signals, Serialize, Deserialize)]
struct Users {
    users: Vec<User>,
    total: i32,
    table: TableState,
}

/// The columns the server sorts by, shared by the headers and the handler.
#[derive(Segment, Serialize, Deserialize)]
enum By {
    Id,
    Email,
}

const PER_PAGE: i32 = 10;

hypersig::routes! {
    fn actions(Router<()>);
    action LOAD: Action<Users, TableState> = post "/users/rows" => load;
}

/// The rows of the page `state` names, and how many rows pass its filter.
fn query(all: &[User], state: &TableState) -> (Vec<User>, i32) {
    let needle = state.query.trim().to_lowercase();
    let mut rows: Vec<User> =
        all.iter().filter(|user| user.email.contains(&needle)).cloned().collect();
    if let Some((by, order)) = state.sorting::<By>() {
        match by {
            By::Id => rows.sort_by_key(|user| user.id),
            By::Email => rows.sort_by(|a, b| a.email.cmp(&b.email)),
        }
        if order == SortOrder::Descending {
            rows.reverse();
        }
    }
    let total = i32::try_from(rows.len()).unwrap_or(i32::MAX);
    // `page` comes from the browser, so a large or negative one must not overflow.
    let start = usize::try_from(state.page.max(0).saturating_mul(PER_PAGE)).unwrap_or(0);
    (rows.into_iter().skip(start).take(PER_PAGE as usize).collect(), total)
}

fn everyone() -> Vec<User> {
    (1..=42).map(|id| User { id, email: format!("user{id}@example.com") }).collect()
}

async fn load((): (), state: TableState) -> Patches<Users> {
    let (rows, total) = query(&everyone(), &state);
    Users::users().patch(rows).and(Users::total().patch(total)).into()
}

let table = ServerTable::new(
    "Users",
    Users::table(),
    Users::users(),
    |user| user.at(User::id()),
    LOAD.call_with(Users::table()),
)
.filter("Filter emails")
.paginate(PER_PAGE, Users::total())
.sortable_column(Column::number("Id", User::id()), &By::Id)
.sortable_column(Column::text("Email", User::email()), &By::Email);

// The page renders its first rows with the same query the action runs.
let state = TableState::default().with_sorting(&By::Id, SortOrder::Ascending);
let (users, total) = query(&everyone(), &state);
let html = Page::new("Users", Users { users, total, table: state })
    .child(table)
    .render();
# assert!(html.contains("1–10 / 42"));
# assert!(html.contains(r#"aria-sort="ascending""#));
# let _ = actions;
