use hypersig::html::div;
use hypersig::{NumberFormat, Page, Signals};
use hypersig_ui::{Column, DataTable, TableState, switch};
use serde::{Deserialize, Serialize};

#[derive(Signals, Serialize, Deserialize)]
struct Payment {
    id: i32,
    email: String,
    amount: f64,
}

#[derive(Signals, Serialize, Deserialize)]
struct Ledger {
    payments: Vec<Payment>,
    show_ids: bool,
    table: TableState,
}

const DOLLARS: NumberFormat = NumberFormat::new().digits(2).prefix("$");

let table = DataTable::new("Payments", Ledger::table(), Ledger::payments(), |payment| {
    payment.at(Payment::id())
})
.filter("Filter emails", |payment| payment.at(Payment::email()))
.paginate(4)
.column(Column::number("Id", Payment::id()).visible(Ledger::show_ids()))
.column(Column::text("Email", Payment::email()))
.column(Column::formatted("Amount", Payment::amount(), DOLLARS));

let payments = (1..=7)
    .map(|id| Payment { id, email: format!("user{id}@example.com"), amount: 100.0 * f64::from(id) })
    .collect();
let state = Ledger { payments, show_ids: false, table: TableState::default() };
let html = Page::new("Payments", state)
    .child(div().child(switch("Show ids", Ledger::show_ids())).child(table))
    .render();
# assert!(html.contains(r#"type="search" aria-label="Filter emails" placeholder="Filter emails""#));
# assert!(html.contains("1–4 / 7"));
# assert!(html.contains(r#"aria-label="Previous page" disabled"#));
