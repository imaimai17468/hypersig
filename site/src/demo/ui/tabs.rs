//! A tab list, for the tabs page.

use hypersig::html::p;
use hypersig::{Page, Signals};
use hypersig_ui::Tabs;
use serde::{Deserialize, Serialize};

use crate::Site;
use crate::lang::L;

#[derive(Serialize, Deserialize, Clone, Copy, PartialEq)]
pub enum Section {
    Account,
    Password,
    Billing,
}

#[derive(Signals, Serialize, Deserialize)]
pub struct Settings {
    section: Section,
}

pub fn page(site: Site) -> Page<Settings> {
    let initial = Settings {
        section: Section::Account,
    };
    Page::new(
        site.t(L {
            en: "Settings",
            ja: "設定",
        }),
        initial,
    )
    .child(
        Tabs::new(
            site.t(L {
                en: "Settings",
                ja: "設定",
            }),
            Settings::section(),
        )
        .tab(
            Section::Account,
            site.t(L {
                en: "Account",
                ja: "アカウント",
            }),
            p().text(site.t(L {
                en: "Change your name and email.",
                ja: "名前とメールアドレスを変更します。",
            })),
        )
        .tab(
            Section::Password,
            site.t(L {
                en: "Password",
                ja: "パスワード",
            }),
            p().text(site.t(L {
                en: "Passwords are at least twelve characters.",
                ja: "パスワードは 12 文字以上にしてください。",
            })),
        )
        .tab(
            Section::Billing,
            site.t(L {
                en: "Billing",
                ja: "お支払い",
            }),
            p().text(site.t(L {
                en: "Invoices arrive on the first of each month.",
                ja: "請求書は毎月 1 日に届きます。",
            })),
        ),
    )
}
