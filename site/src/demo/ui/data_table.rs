//! A data table, for the data table page.

use hypersig::{NumberFormat, Page, Signals};
use hypersig_ui::{Column, DataTable, TableState};
use serde::{Deserialize, Serialize};

use crate::Site;
use crate::lang::L;

#[derive(Signals, Serialize, Deserialize)]
pub struct Payment {
    id: i32,
    email: String,
    amount: f64,
    selected: bool,
}

#[derive(Signals, Serialize, Deserialize)]
pub struct Ledger {
    payments: Vec<Payment>,
    table: TableState,
}

pub fn page(site: Site) -> Page<Ledger> {
    let payments = [
        ("ken99@example.com", 316.0),
        ("abe45@example.com", 242.0),
        ("monserrat44@example.com", 837.0),
        ("silas22@example.com", 874.0),
        ("carmella@example.com", 721.0),
        ("jason78@example.com", 450.0),
        ("sarah23@example.com", 120.5),
    ]
    .into_iter()
    .zip(1..)
    .map(|((email, amount), id)| Payment {
        id,
        email: email.to_owned(),
        amount,
        selected: false,
    })
    .collect();
    let table = DataTable::new(
        site.t(L {
            en: "Payments",
            ja: "支払い",
        }),
        Ledger::table(),
        Ledger::payments(),
        |payment| payment.at(Payment::id()),
    )
    .selectable(Payment::selected())
    .filter(
        site.t(L {
            en: "Filter emails",
            ja: "メールアドレスで絞り込む",
        }),
        |payment| payment.at(Payment::email()),
    )
    .paginate(5)
    .empty(site.t(L {
        en: "No results.",
        ja: "該当する行はありません。",
    }))
    .sortable_column(
        Column::text(
            site.t(L {
                en: "Email",
                ja: "メールアドレス",
            }),
            Payment::email(),
        ),
        |payment| payment.at(Payment::email()),
    )
    .sortable_column(
        Column::formatted(
            site.t(L {
                en: "Amount",
                ja: "金額",
            }),
            Payment::amount(),
            NumberFormat::new().digits(2).prefix("$"),
        ),
        |payment| payment.at(Payment::amount()),
    );
    Page::new(
        site.t(L {
            en: "Payments",
            ja: "支払い",
        }),
        Ledger {
            payments,
            table: TableState::default(),
        },
    )
    .child(table)
}
