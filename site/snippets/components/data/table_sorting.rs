use hypersig::{NumberFormat, Page, Signals};
use hypersig_ui::{Column, DataTable, TableState};
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
    table: TableState,
}

const DOLLARS: NumberFormat = NumberFormat::new().digits(2).prefix("$");

let table = DataTable::new("Payments", Ledger::table(), Ledger::payments(), |payment| {
    payment.at(Payment::id())
})
.column(Column::number("Id", Payment::id()))
.sortable_column(Column::text("Email", Payment::email()), |payment| {
    payment.at(Payment::email())
})
.sortable_column(Column::formatted("Amount", Payment::amount(), DOLLARS), |payment| {
    payment.at(Payment::amount())
});

let payments = vec![
    Payment { id: 1, email: "ken99@example.com".into(), amount: 316.0 },
    Payment { id: 2, email: "abe45@example.com".into(), amount: 242.0 },
];
let html = Page::new("Payments", Ledger { payments, table: TableState::default() })
    .child(table)
    .render();
# assert_eq!(html.matches(r#"aria-sort="none""#).count(), 2);
# assert_eq!(html.matches(r#"scope="col""#).count(), 3);
