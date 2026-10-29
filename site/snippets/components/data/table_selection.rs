use hypersig::html::{div, p};
use hypersig::{Action, Page, Patches, Signals};
use hypersig_ui::{ButtonSize, ButtonVariant, Column, DataTable, TableState, button};
use serde::{Deserialize, Serialize};

#[derive(Signals, Serialize, Deserialize)]
struct Payment {
    id: i32,
    email: String,
    note: String,
}

#[derive(Signals, Serialize, Deserialize)]
struct Ledger {
    payments: Vec<Payment>,
    chosen: Vec<i32>,
    open: Vec<i32>,
    table: TableState,
}

hypersig::routes! {
    fn actions(Router<()>);
    action REFUND: Action<Ledger, Vec<i32>> = post "/refunds" => refund;
}

/// Receives the ids of the selected payments.
async fn refund((): (), ids: Vec<i32>) -> Patches<Ledger> {
    // Refund each payment in `ids` here, then clear the selection.
    let _ = ids;
    Ledger::chosen().patch(Vec::new()).into()
}

let table = DataTable::new("Payments", Ledger::table(), Ledger::payments(), |payment| {
    payment.at(Payment::id())
})
.selectable(Ledger::chosen(), |payment| payment.at(Payment::id()))
.expandable(
    Ledger::open(),
    |payment| payment.at(Payment::id()),
    |payment| p().bind_text(payment.at(Payment::note())),
)
.column(Column::text("Email", Payment::email()));

let refund_button = button(ButtonVariant::Destructive, ButtonSize::Default)
    .on_click(REFUND.call_with(Ledger::chosen()))
    .text("Refund selected");

let payments = vec![
    Payment { id: 1, email: "ken99@example.com".into(), note: "Paid by card.".into() },
    Payment { id: 2, email: "abe45@example.com".into(), note: "Refund requested.".into() },
];
let state = Ledger { payments, chosen: vec![2], open: Vec::new(), table: TableState::default() };
let html = Page::new("Payments", state)
    .child(div().child(table).child(refund_button))
    .render();
# assert!(html.contains(r#"role="checkbox" aria-label="Select all rows" aria-checked="mixed""#));
# assert!(html.contains(r#"aria-label="Show row details" aria-expanded="false""#));
# let _ = actions;
