//! Links styled as buttons, beside a real button, for the link button page.

use hypersig::html::{a, div, p};
use hypersig::style::pattern::{stack, wrap};
use hypersig::style::value::rem;
use hypersig::{Page, SafeUrl, Signals};
use hypersig_ui::{ButtonSize, ButtonVariant, button, button_style};
use serde::{Deserialize, Serialize};

use crate::Site;
use crate::lang::L;

#[derive(Signals, Serialize, Deserialize)]
pub struct Plans {
    /// How many times the button, which leads nowhere, was pressed.
    presses: i32,
}

/// The part of this page that the links lead to, since the frame has no
/// other page to open.
const PLANS: SafeUrl = SafeUrl::new("#plans");

pub fn page(site: Site) -> Page<Plans> {
    Page::new(
        site.t(L {
            en: "Plans",
            ja: "プラン",
        }),
        Plans { presses: 0 },
    )
    .child(
        div()
            .css(stack(rem(1)))
            .child(
                div()
                    .css(wrap(rem(0.5)))
                    .child(
                        a().href(PLANS)
                            .css(button_style(ButtonVariant::Default, ButtonSize::Default))
                            .text(site.t(L {
                                en: "See the plans",
                                ja: "プランを見る",
                            })),
                    )
                    .child(
                        a().href(PLANS)
                            .css(button_style(ButtonVariant::Outline, ButtonSize::Default))
                            .text(site.t(L {
                                en: "Compare",
                                ja: "比べる",
                            })),
                    )
                    .child(
                        button(ButtonVariant::Outline, ButtonSize::Default)
                            .on_click(Plans::presses().set(Plans::presses() + 1))
                            .text(site.t(L {
                            en: "A button",
                            ja: "ボタン",
                        })),
                    ),
            )
            .child(p().id(hypersig::html::Id::new("plans")).text(site.t(L {
                en: "Hobby is free, and Pro is $20 a month.",
                ja: "Hobby は無料で、Pro は月 20 ドルです。",
            }))),
    )
}
