//! A data table whose rows are selected and expanded, for the page on
//! selecting and expanding rows.

use hypersig::html::{div, p, span};
use hypersig::style::pattern::stack;
use hypersig::style::value::rem;
use hypersig::{NumberFormat, Page, Signals};
use hypersig_ui::{Column, DataTable, TableState, TableText};
use serde::{Deserialize, Serialize};

use crate::Site;
use crate::lang::L;

#[derive(Signals, Serialize, Deserialize)]
pub struct Payment {
    id: i32,
    email: String,
    amount: f64,
    note: String,
}

#[derive(Signals, Serialize, Deserialize)]
pub struct Ledger {
    payments: Vec<Payment>,
    chosen: Vec<i32>,
    open: Vec<i32>,
    table: TableState,
}

pub fn page(site: Site) -> Page<Ledger> {
    let payments = [
        (
            "ken99@example.com",
            316.0,
            site.t(L {
                en: "Paid by card.",
                ja: "カードで支払い済みです。",
            }),
        ),
        (
            "abe45@example.com",
            242.0,
            site.t(L {
                en: "Refund requested on Monday.",
                ja: "月曜日に返金の依頼がありました。",
            }),
        ),
        (
            "monserrat44@example.com",
            837.0,
            site.t(L {
                en: "Paid by bank transfer.",
                ja: "銀行振込で支払い済みです。",
            }),
        ),
        (
            "silas22@example.com",
            874.0,
            site.t(L {
                en: "Waiting for the bank.",
                ja: "銀行の処理を待っています。",
            }),
        ),
    ]
    .into_iter()
    .zip(1..)
    .map(|((email, amount, note), id)| Payment {
        id,
        email: email.to_owned(),
        amount,
        note: note.to_owned(),
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
    .selectable(Ledger::chosen(), |payment| payment.at(Payment::id()))
    .expandable(
        Ledger::open(),
        |payment| payment.at(Payment::id()),
        |payment| p().bind_text(payment.at(Payment::note())),
    )
    .text(TableText {
        select_row: site
            .t(L {
                en: "Select row",
                ja: "行を選ぶ",
            })
            .to_owned(),
        select_all: site
            .t(L {
                en: "Select all rows",
                ja: "すべての行を選ぶ",
            })
            .to_owned(),
        expand_row: site
            .t(L {
                en: "Show row details",
                ja: "行の詳細を表示",
            })
            .to_owned(),
        ..TableText::default()
    })
    .column(Column::text(
        site.t(L {
            en: "Email",
            ja: "メールアドレス",
        }),
        Payment::email(),
    ))
    .column(Column::formatted(
        site.t(L {
            en: "Amount",
            ja: "金額",
        }),
        Payment::amount(),
        NumberFormat::new().digits(2).prefix("$"),
    ));
    Page::new(
        site.t(L {
            en: "Payments",
            ja: "支払い",
        }),
        Ledger {
            payments,
            chosen: Vec::new(),
            open: Vec::new(),
            table: TableState::default(),
        },
    )
    .child(
        div().css(stack(rem(1))).child(table).child(
            p().child(span().bind_text(Ledger::chosen().len().to_text()))
                .text(site.t(L {
                    en: " selected",
                    ja: " 件を選択中",
                })),
        ),
    )
}
