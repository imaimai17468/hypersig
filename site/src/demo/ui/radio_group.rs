//! A radio group, for the radio group page.

use hypersig::html::{div, p};
use hypersig::style::pattern::stack;
use hypersig::style::value::rem;
use hypersig::{Page, Signals, if_else};
use hypersig_ui::radio_group;
use serde::{Deserialize, Serialize};

use crate::Site;
use crate::lang::L;

#[derive(Serialize, Deserialize, Clone, Copy, PartialEq)]
pub enum Plan {
    Hobby,
    Pro,
    Team,
}

#[derive(Signals, Serialize, Deserialize)]
pub struct Billing {
    plan: Plan,
}

pub fn page(site: Site) -> Page<Billing> {
    let plan = Billing::plan();
    Page::new(
        site.t(L {
            en: "Billing",
            ja: "お支払い",
        }),
        Billing { plan: Plan::Pro },
    )
    .child(
        div()
            .css(stack(rem(1)))
            .child(radio_group(
                site.t(L {
                    en: "Plan",
                    ja: "プラン",
                }),
                plan,
                [
                    (
                        Plan::Hobby,
                        site.t(L {
                            en: "Hobby",
                            ja: "Hobby",
                        }),
                    ),
                    (
                        Plan::Pro,
                        site.t(L {
                            en: "Pro",
                            ja: "Pro",
                        }),
                    ),
                    (
                        Plan::Team,
                        site.t(L {
                            en: "Team",
                            ja: "Team",
                        }),
                    ),
                ],
            ))
            .child(p().bind_text(if_else(
                plan.equals(&Plan::Hobby),
                site.t(L {
                    en: "Free for personal projects.",
                    ja: "個人のプロジェクトには無料です。",
                }),
                site.t(L {
                    en: "Billed monthly.",
                    ja: "毎月請求されます。",
                }),
            ))),
    )
}
