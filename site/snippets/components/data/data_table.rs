use hypersig::style::value::rem;
use hypersig::{NumberFormat, Page, Signals};
use hypersig_ui::{BadgeVariant, Column, DataTable, TableState, badge};
use serde::{Deserialize, Serialize};

#[derive(Signals, Serialize, Deserialize)]
struct Invoice {
    number: i32,
    email: String,
    amount: f64,
    status: String,
}

#[derive(Signals, Serialize, Deserialize)]
struct Billing {
    invoices: Vec<Invoice>,
    table: TableState,
}

const DOLLARS: NumberFormat = NumberFormat::new().digits(2).prefix("$");

let table = DataTable::new("Invoices", Billing::table(), Billing::invoices(), |invoice| {
    invoice.at(Invoice::number())
})
.column(Column::number("Number", Invoice::number()))
.column(Column::text("Email", Invoice::email()))
.column(Column::custom("Status", |invoice| {
    badge(BadgeVariant::Outline).bind_text(invoice.at(Invoice::status()))
}))
.column(Column::formatted("Amount", Invoice::amount(), DOLLARS))
.max_height(rem(20));

let invoices = vec![
    Invoice { number: 1001, email: "ken99@example.com".into(), amount: 316.0, status: "Paid".into() },
    Invoice { number: 1002, email: "abe45@example.com".into(), amount: 1242.5, status: "Pending".into() },
];
let html = Page::new("Invoices", Billing { invoices, table: TableState::default() })
    .child(table)
    .render();
# assert!(html.contains(r#"aria-label="Invoices""#));
# assert!(html.contains(">$1,242.50</span>"));
# assert!(html.contains(">Pending</span>"));
# assert_eq!(html.matches(r#"scope="col""#).count(), 4);
