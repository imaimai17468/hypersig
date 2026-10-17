//! An exclusive accordion, for the accordion page.

use hypersig::html::{div, p, span};
use hypersig::style::pattern::stack;
use hypersig::style::value::rem;
use hypersig::{Page, Signals};
use hypersig_ui::{Accordion, ButtonSize, ButtonVariant, button};
use serde::{Deserialize, Serialize};

use crate::Site;
use crate::lang::L;

#[derive(Signals, Serialize, Deserialize)]
pub struct Faq {
    helpful: i32,
}

pub fn page(site: Site) -> Page<Faq> {
    let helpful = Faq::helpful();
    let answers = Accordion::exclusive("faq")
        .item(
            site.t(L {
                en: "Does it need JavaScript?",
                ja: "JavaScript は必要ですか？",
            }),
            p().text(site.t(L {
                en: "No. Each section is a <details>, which opens without it.",
                ja: "いいえ。各セクションは <details> なので、JavaScript がなくても開きます。",
            })),
        )
        .item(
            site.t(L {
                en: "Can I restyle it?",
                ja: "見た目を変えられますか？",
            }),
            p().text(site.t(L {
                en: "Set the tokens in a Theme, or add a Style to any element.",
                ja: "Theme でトークンを設定するか、好きな要素に Style を追加します。",
            })),
        )
        .item(
            site.t(L {
                en: "Is it accessible?",
                ja: "アクセシブルですか？",
            }),
            p().text(site.t(L {
                en: "The browser announces each summary as a button that opens.",
                ja: "ブラウザは各 summary を、セクションを開くボタンとして読み上げます。",
            })),
        );
    Page::new(
        site.t(L {
            en: "FAQ",
            ja: "よくある質問",
        }),
        Faq { helpful: 0 },
    )
    .child(
        div().css(stack(rem(1))).child(answers).child(
            p().child(
                button(ButtonVariant::Outline, ButtonSize::Sm)
                    .on_click(helpful.set(helpful + 1))
                    .text(site.t(L {
                        en: "This helped",
                        ja: "役に立った",
                    })),
            )
            .text(" ")
            .child(span().bind_text(helpful.to_text()))
            .text(site.t(L {
                en: " found it helpful",
                ja: " 人が役に立ったと答えました",
            })),
        ),
    )
}
