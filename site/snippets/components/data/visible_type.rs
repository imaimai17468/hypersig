use hypersig::Signals;
use hypersig_ui::Column;
use serde::{Deserialize, Serialize};

#[derive(Signals, Serialize, Deserialize)]
struct Payment {
    id: i32,
}

#[derive(Signals, Serialize, Deserialize)]
struct Ledger {
    payments: Vec<Payment>,
    columns: String,
}

// error[E0277]: `Sig<Ledger, String>` is not an expression of type `bool`
//     on the page whose signals are `_`
let id: Column<Ledger, Payment> = Column::number("Id", Payment::id()).visible(Ledger::columns());
