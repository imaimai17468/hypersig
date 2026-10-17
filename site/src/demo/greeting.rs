//! A text field and a greeting that follows it, for the page on describing
//! the page.

use hypersig::html::{div, p, span};
use hypersig::style::Style;
use hypersig::style::pattern::stack;
use hypersig::style::value::rem;
use hypersig::{Page, Signals};
use hypersig_ui::TextField;
use serde::{Deserialize, Serialize};

use crate::Site;
use crate::lang::L;

#[derive(Signals, Serialize, Deserialize)]
pub struct Greeting {
    name: String,
}

pub fn page(site: Site) -> Page<Greeting> {
    let name = Greeting::name();
    let typed = !name.trim().is_empty();
    Page::new(
        site.t(L {
            en: "Greeting",
            ja: "あいさつ",
        }),
        Greeting {
            name: String::new(),
        },
    )
    .child(
        div()
            .css(stack(rem(1)).max_width(rem(20)))
            .child(TextField::new(
                site.t(L {
                    en: "Your name",
                    ja: "名前",
                }),
                name,
            ))
            .child(p().show(!typed.clone()).text(site.t(L {
                en: "Type your name above.",
                ja: "上の欄に名前を入力してください。",
            })))
            .child(
                p().show(typed)
                    .text(site.t(L {
                        en: "Hello, ",
                        ja: "こんにちは、",
                    }))
                    .child(span().bind_text(name.trim()))
                    .text(site.t(L {
                        en: "!",
                        ja: "さん。",
                    })),
            )
            .child(
                p().css(Style::new().font_size(rem(0.875)))
                    .show(name.char_count().gt(12))
                    .text(site.t(L {
                        en: "That is a long name.",
                        ja: "長い名前です。",
                    })),
            ),
    )
}
