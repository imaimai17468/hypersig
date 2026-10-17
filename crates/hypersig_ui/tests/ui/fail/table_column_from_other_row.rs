use hypersig::Signals;
use hypersig_ui::{Column, DataTable, TableState};
use serde::{Deserialize, Serialize};

#[derive(Signals, Serialize, Deserialize)]
struct Task {
    id: i32,
    title: String,
}

#[derive(Signals, Serialize, Deserialize)]
struct User {
    name: String,
}

#[derive(Signals, Serialize, Deserialize)]
struct Board {
    tasks: Vec<Task>,
    table: TableState,
}

fn main() {
    let _ = DataTable::new("Tasks", Board::table(), Board::tasks(), |task| task.at(Task::id()))
        .column(Column::text("Name", User::name()));
}
