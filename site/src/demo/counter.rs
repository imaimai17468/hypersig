//! The counter on the landing page and in the quick start, which counts in
//! the browser without a request.

use hypersig::html::{div, span};
use hypersig::style::Style;
use hypersig::style::pattern::hstack;
use hypersig::style::value::{FontWeight, TextAlign, rem};
use hypersig::{Page, Signals};
use hypersig_ui::{ButtonSize, ButtonVariant, button};
use serde::{Deserialize, Serialize};

use crate::Site;
use crate::lang::L;

#[derive(Signals, Serialize, Deserialize)]
pub struct Counter {
    count: i32,
}

pub fn page(site: Site) -> Page<Counter> {
    let count = Counter::count();
    Page::new(
        site.t(L {
            en: "Counter",
            ja: "カウンター",
        }),
        Counter { count: 0 },
    )
    .child(
        div()
            .css(hstack(rem(1)))
            .child(
                button(ButtonVariant::Outline, ButtonSize::Default)
                    .on_click(count.set(count - 1))
                    .text("-1"),
            )
            .child(
                span()
                    .css(
                        Style::new()
                            .min_width(rem(3))
                            .text_align(TextAlign::Center)
                            .font_size(rem(1.5))
                            .font_weight(FontWeight::Number(600.0)),
                    )
                    .bind_text(count.to_text()),
            )
            .child(
                button(ButtonVariant::Default, ButtonSize::Default)
                    .on_click(count.set(count + 1))
                    .text("+1"),
            ),
    )
}
