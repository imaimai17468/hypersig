use hypersig::Signals;
use hypersig_ui::{DataTable, TableState};
use serde::{Deserialize, Serialize};

#[derive(Signals, Serialize, Deserialize)]
struct Payment {
    id: i32,
    email: String,
}

#[derive(Signals, Serialize, Deserialize)]
struct Ledger {
    payments: Vec<Payment>,
    chosen: Vec<i32>,
    table: TableState,
}

// error[E0277]: `Sig<Ledger, String, Local>` is not an expression of type `i32`
//     on the page whose signals are `Ledger`
let table = DataTable::new("Payments", Ledger::table(), Ledger::payments(), |payment| {
    payment.at(Payment::id())
})
.selectable(Ledger::chosen(), |payment| payment.at(Payment::email()));
