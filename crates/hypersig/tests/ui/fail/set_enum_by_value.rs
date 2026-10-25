//! An enum value passed to `set` by value, where an expression takes it by reference.

use hypersig::html::button;
use hypersig::Signals;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
enum Column {
    Todo,
    Done,
}

#[derive(Signals, Serialize, Deserialize)]
struct Board {
    column: Column,
}

fn main() {
    let _ = button::<Board>().on_click(Board::column().set(Column::Done));
}
