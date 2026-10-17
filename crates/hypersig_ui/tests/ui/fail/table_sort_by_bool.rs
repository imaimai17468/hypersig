use hypersig::Signals;
use hypersig_ui::Column;
use serde::{Deserialize, Serialize};

#[derive(Signals, Serialize, Deserialize)]
struct Task {
    title: String,
    done: bool,
}

#[derive(Signals, Serialize, Deserialize)]
struct Board {
    tasks: Vec<Task>,
}

fn main() {
    // The runtime orders only numbers and strings.
    let _: Column<Board, Task> =
        Column::text("Title", Task::title()).sort_by(|task| task.at(Task::done()));
}
