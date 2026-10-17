//! A checkbox and a switch, for the checkbox and switch page.

use hypersig::html::{div, p};
use hypersig::style::pattern::stack;
use hypersig::style::value::rem;
use hypersig::{Page, Signals, if_else};
use hypersig_ui::{checkbox, switch};
use serde::{Deserialize, Serialize};

use crate::Site;
use crate::lang::L;

#[derive(Signals, Serialize, Deserialize)]
pub struct Settings {
    terms: bool,
    notifications: bool,
}

pub fn page(site: Site) -> Page<Settings> {
    let notifications = Settings::notifications();
    let initial = Settings {
        terms: false,
        notifications: true,
    };
    Page::new(
        site.t(L {
            en: "Settings",
            ja: "設定",
        }),
        initial,
    )
    .child(
        div()
            .css(stack(rem(1)))
            .child(checkbox(
                site.t(L {
                    en: "Accept the terms",
                    ja: "利用規約に同意する",
                }),
                Settings::terms(),
            ))
            .child(switch(
                site.t(L {
                    en: "Notifications",
                    ja: "通知",
                }),
                notifications,
            ))
            .child(p().bind_text(if_else(
                notifications,
                site.t(L {
                    en: "You will get an email for each release.",
                    ja: "リリースのたびにメールが届きます。",
                }),
                site.t(L {
                    en: "You will not get emails.",
                    ja: "メールは届きません。",
                }),
            ))),
    )
}
