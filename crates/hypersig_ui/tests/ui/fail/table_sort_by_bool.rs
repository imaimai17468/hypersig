use hypersig::Signals;
use hypersig_ui::{Column, DataTable, TableState};
use serde::{Deserialize, Serialize};

#[derive(Signals, Serialize, Deserialize)]
struct Task {
    id: i32,
    title: String,
    done: bool,
}

#[derive(Signals, Serialize, Deserialize)]
struct Board {
    tasks: Vec<Task>,
    table: TableState,
}

fn main() {
    // The runtime orders only numbers and strings.
    let _ = DataTable::new("Tasks", Board::table(), Board::tasks(), |task| task.at(Task::id()))
        .sortable_column(Column::text("Title", Task::title()), |task| task.at(Task::done()));
}
