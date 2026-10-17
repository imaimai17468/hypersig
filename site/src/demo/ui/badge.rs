//! The badge variants, for the badge page.

use hypersig::html::div;
use hypersig::style::pattern::{stack, wrap};
use hypersig::style::value::rem;
use hypersig::{Page, Signals};
use hypersig_ui::{BadgeVariant, ButtonSize, ButtonVariant, badge, button};
use serde::{Deserialize, Serialize};

use crate::Site;
use crate::lang::L;

#[derive(Signals, Serialize, Deserialize)]
pub struct Build {
    failed: bool,
}

pub fn page(site: Site) -> Page<Build> {
    let failed = Build::failed();
    Page::new(
        site.t(L {
            en: "Build",
            ja: "ビルド",
        }),
        Build { failed: false },
    )
    .child(
        div()
            .css(stack(rem(1)))
            .child(
                div()
                    .css(wrap(rem(0.5)))
                    .child(badge(BadgeVariant::Default).text("Default"))
                    .child(badge(BadgeVariant::Secondary).text("Secondary"))
                    .child(badge(BadgeVariant::Outline).text("Outline"))
                    .child(
                        badge(BadgeVariant::Destructive)
                            .show(failed)
                            .text(site.t(L {
                                en: "Failed",
                                ja: "失敗",
                            })),
                    )
                    .child(badge(BadgeVariant::Secondary).show(!failed).text(site.t(L {
                        en: "Passing",
                        ja: "成功",
                    }))),
            )
            .child(
                div().child(
                    button(ButtonVariant::Outline, ButtonSize::Sm)
                        .on_click(failed.toggle())
                        .text(site.t(L {
                            en: "Toggle the build",
                            ja: "ビルドの結果を切り替える",
                        })),
                ),
            ),
    )
}
