//! Skeleton lines shown while a signal says the report is loading, for the
//! skeleton page.

use hypersig::html::{attr, div, p};
use hypersig::style::Style;
use hypersig::style::pattern::stack;
use hypersig::style::value::rem;
use hypersig::{Page, Signals};
use hypersig_ui::{ButtonSize, ButtonVariant, button, skeleton};
use serde::{Deserialize, Serialize};

use crate::Site;
use crate::lang::L;

#[derive(Signals, Serialize, Deserialize)]
pub struct Report {
    loading: bool,
}

pub fn page(site: Site) -> Page<Report> {
    let loading = Report::loading();
    Page::new(
        site.t(L {
            en: "Report",
            ja: "レポート",
        }),
        Report { loading: true },
    )
    .child(
        div()
            .css(stack(rem(1)).max_width(rem(24)))
            .child(
                div()
                    .bind_attr(attr::ARIA_BUSY, loading.to_text())
                    .css(stack(rem(0.5)))
                    .child(
                        div()
                            .css(stack(rem(0.5)))
                            .show(loading)
                            .child(skeleton().css(Style::new().height(rem(1)).width(rem(16))))
                            .child(skeleton().css(Style::new().height(rem(1)).width(rem(12)))),
                    )
                    .child(p().show(!loading).text(site.t(L {
                        en: "The report is ready.",
                        ja: "レポートの準備ができました。",
                    }))),
            )
            .child(
                div().child(
                    button(ButtonVariant::Outline, ButtonSize::Sm)
                        .on_click(loading.toggle())
                        .text(site.t(L {
                            en: "Toggle loading",
                            ja: "読み込み中の表示を切り替える",
                        })),
                ),
            ),
    )
}
