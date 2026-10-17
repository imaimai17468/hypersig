//! A message alert and an error alert, for the alert page.

use hypersig::html::{div, when};
use hypersig::style::pattern::stack;
use hypersig::style::value::rem;
use hypersig::{Page, Signals};
use hypersig_ui::{
    AlertVariant, ButtonSize, ButtonVariant, alert, alert_description, alert_title, button,
};
use serde::{Deserialize, Serialize};

use crate::Site;
use crate::lang::L;

#[derive(Signals, Serialize, Deserialize)]
pub struct Builds {
    failed: bool,
}

pub fn page(site: Site) -> Page<Builds> {
    let failed = Builds::failed();
    Page::new(
        site.t(L {
            en: "Builds",
            ja: "ビルド",
        }),
        Builds { failed: false },
    )
    .child(
        div()
            .css(stack(rem(1)))
            .child(
                alert(AlertVariant::Default)
                    .child(alert_title().text(site.t(L {
                        en: "Deployments are paused",
                        ja: "デプロイを停止しています",
                    })))
                    .child(alert_description().text(site.t(L {
                        en: "They resume at 18:00 UTC.",
                        ja: "UTC の 18:00 に再開します。",
                    }))),
            )
            .child(when(
                failed,
                alert(AlertVariant::Destructive)
                    .child(alert_title().text(site.t(L {
                        en: "Build failed",
                        ja: "ビルドに失敗しました",
                    })))
                    .child(alert_description().text(site.t(L {
                        en: "The last commit does not compile.",
                        ja: "最新のコミットがコンパイルできません。",
                    }))),
            ))
            .child(
                div().child(
                    button(ButtonVariant::Outline, ButtonSize::Sm)
                        .on_click(failed.toggle())
                        .text(site.t(L {
                            en: "Toggle the error",
                            ja: "エラーを切り替える",
                        })),
                ),
            ),
    )
}
