use hypersig::Signals;
use hypersig_ui::Column;
use serde::{Deserialize, Serialize};

#[derive(Signals, Serialize, Deserialize)]
struct Invoice {
    number: i32,
    amount: f64,
}

#[derive(Signals, Serialize, Deserialize)]
struct Billing {
    invoices: Vec<Invoice>,
}

// error[E0308]: mismatched types: expected `Sig<Invoice, String>`, found `Sig<Invoice, f64>`
let amount: Column<Billing, Invoice> = Column::text("Amount", Invoice::amount());
