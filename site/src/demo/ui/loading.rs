//! A spinner shown while a signal says work is running, for the spinner page.

use hypersig::html::{div, p};
use hypersig::style::pattern::hstack;
use hypersig::style::value::rem;
use hypersig::{Page, Signals, if_else};
use hypersig_ui::{ButtonSize, ButtonVariant, button, spinner};
use serde::{Deserialize, Serialize};

use crate::Site;
use crate::lang::L;

#[derive(Signals, Serialize, Deserialize)]
pub struct Sync {
    busy: bool,
}

pub fn page(site: Site) -> Page<Sync> {
    let busy = Sync::busy();
    Page::new(
        site.t(L {
            en: "Sync",
            ja: "同期",
        }),
        Sync { busy: true },
    )
    .child(
        div()
            .css(hstack(rem(0.75)))
            .child(
                spinner(site.t(L {
                    en: "Syncing",
                    ja: "同期中",
                }))
                .show(busy),
            )
            .child(p().show(!busy).text(site.t(L {
                en: "Up to date.",
                ja: "最新の状態です。",
            })))
            .child(
                button(ButtonVariant::Outline, ButtonSize::Sm)
                    .on_click(busy.toggle())
                    .bind_text(if_else(
                        busy,
                        site.t(L {
                            en: "Stop",
                            ja: "停止",
                        }),
                        site.t(L {
                            en: "Sync",
                            ja: "同期",
                        }),
                    )),
            ),
    )
}
