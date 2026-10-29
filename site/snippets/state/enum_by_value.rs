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

// error[E0277]: `Column` is not an expression of type `Column` on the page whose signals are `Board`
let done = Board::column().set(Column::Done);
