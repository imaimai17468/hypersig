//! A progress bar and a skeleton, for the progress page.

use hypersig::html::{div, p, attr};
use hypersig::style::Style;
use hypersig::style::pattern::{stack, wrap};
use hypersig::style::value::rem;
use hypersig::{Page, Signals};
use hypersig_ui::{ButtonSize, ButtonVariant, button, progress, skeleton};
use serde::{Deserialize, Serialize};

use crate::Site;
use crate::lang::L;

#[derive(Signals, Serialize, Deserialize)]
pub struct Upload {
    percent: i32,
    loading: bool,
}

pub fn page(site: Site) -> Page<Upload> {
    let percent = Upload::percent();
    let loading = Upload::loading();
    let initial = Upload {
        percent: 40,
        loading: true,
    };
    Page::new(
        site.t(L {
            en: "Upload",
            ja: "アップロード",
        }),
        initial,
    )
    .child(
        div()
            .css(stack(rem(1)).max_width(rem(24)))
            .child(progress(
                site.t(L {
                    en: "Upload",
                    ja: "アップロード",
                }),
                percent,
            ))
            .child(
                div()
                    .css(wrap(rem(0.5)))
                    .child(
                        button(ButtonVariant::Secondary, ButtonSize::Sm)
                            .on_click(percent.set((percent - 10).max(0)))
                            .text("-10%"),
                    )
                    .child(
                        button(ButtonVariant::Secondary, ButtonSize::Sm)
                            .on_click(percent.set((percent + 10).min(100)))
                            .text("+10%"),
                    )
                    .child(
                        button(ButtonVariant::Outline, ButtonSize::Sm)
                            .on_click(loading.toggle())
                            .text(site.t(L {
                                en: "Toggle loading",
                                ja: "読み込み中の表示を切り替える",
                            })),
                    ),
            )
            .child(
                div()
                    .attr(attr::ARIA_BUSY, "true")
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
}
