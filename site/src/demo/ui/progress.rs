//! A progress bar that buttons move, for the progress page.

use hypersig::html::div;
use hypersig::style::pattern::{stack, wrap};
use hypersig::style::value::rem;
use hypersig::{Page, Signals};
use hypersig_ui::{ButtonSize, ButtonVariant, button, progress};
use serde::{Deserialize, Serialize};

use crate::Site;
use crate::lang::L;

#[derive(Signals, Serialize, Deserialize)]
pub struct Upload {
    percent: i32,
}

pub fn page(site: Site) -> Page<Upload> {
    let percent = Upload::percent();
    Page::new(
        site.t(L {
            en: "Upload",
            ja: "アップロード",
        }),
        Upload { percent: 40 },
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
                    ),
            ),
    )
}
