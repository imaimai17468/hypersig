//! A data table of text, number, formatted, and custom columns, for the
//! data table page.

use hypersig::style::value::rem;
use hypersig::{NumberFormat, Page, Signals};
use hypersig_ui::{BadgeVariant, Column, DataTable, TableState, badge};
use serde::{Deserialize, Serialize};

use crate::Site;
use crate::lang::L;

#[derive(Signals, Serialize, Deserialize)]
pub struct Invoice {
    number: i32,
    email: String,
    amount: f64,
    status: String,
}

#[derive(Signals, Serialize, Deserialize)]
pub struct Billing {
    invoices: Vec<Invoice>,
    table: TableState,
}

const DOLLARS: NumberFormat = NumberFormat::new().digits(2).prefix("$");

pub fn page(site: Site) -> Page<Billing> {
    let paid = site.t(L {
        en: "Paid",
        ja: "支払い済み",
    });
    let pending = site.t(L {
        en: "Pending",
        ja: "未払い",
    });
    let invoices = [
        ("ken99@example.com", 316.0, paid),
        ("abe45@example.com", 242.0, pending),
        ("monserrat44@example.com", 837.0, paid),
        ("silas22@example.com", 874.0, paid),
        ("carmella@example.com", 721.0, pending),
        ("jason78@example.com", 1450.0, paid),
        ("sarah23@example.com", 120.5, paid),
    ]
    .into_iter()
    .zip(1001..)
    .map(|((email, amount, status), number)| Invoice {
        number,
        email: email.to_owned(),
        amount,
        status: status.to_owned(),
    })
    .collect();
    let table = DataTable::new(
        site.t(L {
            en: "Invoices",
            ja: "請求書",
        }),
        Billing::table(),
        Billing::invoices(),
        |invoice| invoice.at(Invoice::number()),
    )
    .column(Column::number(
        site.t(L {
            en: "Number",
            ja: "番号",
        }),
        Invoice::number(),
    ))
    .column(Column::text(
        site.t(L {
            en: "Email",
            ja: "メールアドレス",
        }),
        Invoice::email(),
    ))
    .column(Column::custom(
        site.t(L {
            en: "Status",
            ja: "状態",
        }),
        |invoice| badge(BadgeVariant::Outline).bind_text(invoice.at(Invoice::status())),
    ))
    .column(Column::formatted(
        site.t(L {
            en: "Amount",
            ja: "金額",
        }),
        Invoice::amount(),
        DOLLARS,
    ))
    .max_height(rem(14));
    Page::new(
        site.t(L {
            en: "Invoices",
            ja: "請求書",
        }),
        Billing {
            invoices,
            table: TableState::default(),
        },
    )
    .child(table)
}
