use hypersig::Signals;
use hypersig_ui::{Column, DataTable, TableState};
use serde::{Deserialize, Serialize};

#[derive(Signals, Serialize, Deserialize)]
struct Payment {
    id: i32,
    email: String,
    refunded: bool,
}

#[derive(Signals, Serialize, Deserialize)]
struct Ledger {
    payments: Vec<Payment>,
    table: TableState,
}

// error[E0277]: the trait bound `bool: Comparable` is not satisfied
let table = DataTable::new("Payments", Ledger::table(), Ledger::payments(), |payment| {
    payment.at(Payment::id())
})
.sortable_column(Column::text("Email", Payment::email()), |payment| {
    payment.at(Payment::refunded())
});
