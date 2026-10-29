//! A data table with a filter, pages, and a column a switch shows, for the
//! page on filtering and pages.

use hypersig::html::div;
use hypersig::style::pattern::stack;
use hypersig::style::value::rem;
use hypersig::{NumberFormat, Page, Signals};
use hypersig_ui::{Column, DataTable, TableState, TableText, switch};
use serde::{Deserialize, Serialize};

use crate::Site;
use crate::lang::L;

#[derive(Signals, Serialize, Deserialize)]
pub struct Payment {
    id: i32,
    email: String,
    amount: f64,
}

#[derive(Signals, Serialize, Deserialize)]
pub struct Ledger {
    payments: Vec<Payment>,
    show_ids: bool,
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
    .filter(
        site.t(L {
            en: "Filter emails",
            ja: "メールアドレスで絞り込む",
        }),
        |payment| payment.at(Payment::email()),
    )
    .paginate(4)
    .text(TableText {
        empty: site
            .t(L {
                en: "No results.",
                ja: "該当する行はありません。",
            })
            .to_owned(),
        previous_page: site
            .t(L {
                en: "Previous page",
                ja: "前のページ",
            })
            .to_owned(),
        next_page: site
            .t(L {
                en: "Next page",
                ja: "次のページ",
            })
            .to_owned(),
        ..TableText::default()
    })
    .column(Column::number("Id", Payment::id()).visible(Ledger::show_ids()))
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
            show_ids: false,
            table: TableState::default(),
        },
    )
    .child(
        div()
            .css(stack(rem(1)))
            .child(switch(
                site.t(L {
                    en: "Show ids",
                    ja: "Id を表示する",
                }),
                Ledger::show_ids(),
            ))
            .child(table),
    )
}
