use hypersig::Signals;
use hypersig_ui::{DataTable, TableState};
use serde::{Deserialize, Serialize};

#[derive(Signals, Serialize, Deserialize)]
struct Payment {
    id: i32,
}

#[derive(Signals, Serialize, Deserialize)]
struct Ledger {
    payments: Vec<Payment>,
    table: TableState,
}

// Panics: a page holds at least one row, not 0.
let table = DataTable::new("Payments", Ledger::table(), Ledger::payments(), |payment| {
    payment.at(Payment::id())
})
.paginate(0);
