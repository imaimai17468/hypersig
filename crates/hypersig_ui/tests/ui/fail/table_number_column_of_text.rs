use hypersig::Signals;
use hypersig_ui::Column;
use serde::{Deserialize, Serialize};

#[derive(Signals, Serialize, Deserialize)]
struct Task {
    title: String,
}

#[derive(Signals, Serialize, Deserialize)]
struct Board {
    tasks: Vec<Task>,
}

fn main() {
    let _: Column<Board, Task> = Column::number("Title", Task::title());
}
