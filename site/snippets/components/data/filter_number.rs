use hypersig::Signals;
use hypersig_ui::{DataTable, TableState};
use serde::{Deserialize, Serialize};

#[derive(Signals, Serialize, Deserialize)]
struct Payment {
    id: i32,
    amount: f64,
}

#[derive(Signals, Serialize, Deserialize)]
struct Ledger {
    payments: Vec<Payment>,
    table: TableState,
}

// error[E0277]: `Sig<Ledger, f64, Local>` is not an expression of type `String`
//     on the page whose signals are `Ledger`
let table = DataTable::new("Payments", Ledger::table(), Ledger::payments(), |payment| {
    payment.at(Payment::id())
})
.filter("Filter amounts", |payment| payment.at(Payment::amount()));
