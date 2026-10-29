use hypersig::Signals;
use hypersig_ui::{Column, DataTable, TableState};
use serde::{Deserialize, Serialize};

#[derive(Signals, Serialize, Deserialize)]
struct Invoice {
    number: i32,
    email: String,
}

#[derive(Signals, Serialize, Deserialize)]
struct Customer {
    email: String,
}

#[derive(Signals, Serialize, Deserialize)]
struct Billing {
    invoices: Vec<Invoice>,
    table: TableState,
}

// error[E0308]: mismatched types: expected `Sig<Invoice, String>`,
//     found `Sig<Customer, String>`
let table = DataTable::new("Invoices", Billing::table(), Billing::invoices(), |invoice| {
    invoice.at(Invoice::number())
})
.column(Column::text("Email", Customer::email()));
