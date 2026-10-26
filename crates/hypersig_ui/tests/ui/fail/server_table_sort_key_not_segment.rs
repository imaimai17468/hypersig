use hypersig::{Action, Patches, Signals};
use hypersig_ui::{Column, ServerTable, TableState};
use serde::{Deserialize, Serialize};

#[derive(Signals, Serialize, Deserialize)]
struct Task {
    id: i32,
    title: String,
}

#[derive(Signals, Serialize, Deserialize)]
struct Board {
    tasks: Vec<Task>,
    table: TableState,
}

// Without `#[derive(Segment)]`, the server could not read the key back.
#[derive(Serialize, Deserialize)]
enum By {
    Title,
}

hypersig::routes! {
    fn actions(Router<()>);
    action LOAD: Action<Board, TableState> = post "/tasks" => |(), _: TableState| async {
        Patches::<Board>::new()
    };
}

fn main() {
    let _ = ServerTable::new(
        "Tasks",
        Board::table(),
        Board::tasks(),
        |task| task.at(Task::id()),
        LOAD.call_with(Board::table()),
    )
    .sortable_column(Column::text("Title", Task::title()), &By::Title);
}
