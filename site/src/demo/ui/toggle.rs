//! A toggle and two toggle groups styling a line of text, for the toggle page.

use hypersig::html::{div, p};
use hypersig::style::Style;
use hypersig::style::pattern::{stack, wrap};
use hypersig::style::value::{FontStyle, FontWeight, TextAlign, rem};
use hypersig::{Page, Signals};
use hypersig_ui::{MultiToggleGroup, ToggleGroup, toggle};
use serde::{Deserialize, Serialize};

use crate::Site;
use crate::lang::L;

#[derive(Serialize, Deserialize, Clone, Copy, PartialEq)]
pub enum Align {
    Start,
    Center,
    End,
}

#[derive(Signals, Serialize, Deserialize)]
pub struct Note {
    bold: bool,
    italic: bool,
    align: Align,
    pinned: bool,
}

pub fn page(site: Site) -> Page<Note> {
    let initial = Note {
        bold: false,
        italic: false,
        align: Align::Start,
        pinned: false,
    };
    Page::new(
        site.t(L {
            en: "Note",
            ja: "メモ",
        }),
        initial,
    )
    .child(
        div()
            .css(stack(rem(1)))
            .child(
                div()
                    .css(wrap(rem(0.75)))
                    .child(
                        MultiToggleGroup::new(site.t(L {
                            en: "Text style",
                            ja: "文字のスタイル",
                        }))
                        .icon_item(
                            Note::bold(),
                            site.t(L {
                                en: "Bold",
                                ja: "太字",
                            }),
                            "B",
                        )
                        .icon_item(
                            Note::italic(),
                            site.t(L {
                                en: "Italic",
                                ja: "斜体",
                            }),
                            "I",
                        ),
                    )
                    .child(
                        ToggleGroup::new(
                            site.t(L {
                                en: "Alignment",
                                ja: "配置",
                            }),
                            Note::align(),
                        )
                        .item(
                            Align::Start,
                            site.t(L {
                                en: "Start",
                                ja: "先頭",
                            }),
                        )
                        .item(
                            Align::Center,
                            site.t(L {
                                en: "Center",
                                ja: "中央",
                            }),
                        )
                        .item(
                            Align::End,
                            site.t(L {
                                en: "End",
                                ja: "末尾",
                            }),
                        ),
                    )
                    .child(toggle(Note::pinned()).text(site.t(L {
                        en: "Pin",
                        ja: "ピン留め",
                    }))),
            )
            .child(
                p().css_when(
                    Style::new().font_weight(FontWeight::Number(700.0)),
                    Note::bold(),
                )
                .css_when(Style::new().font_style(FontStyle::Italic), Note::italic())
                .css_match(
                    Note::align(),
                    [
                        (Align::Start, Style::new().text_align(TextAlign::Start)),
                        (Align::Center, Style::new().text_align(TextAlign::Center)),
                        (Align::End, Style::new().text_align(TextAlign::End)),
                    ],
                )
                .text(site.t(L {
                    en: "The quick brown fox jumps over the lazy dog.",
                    ja: "いろはにほへと ちりぬるを",
                })),
            ),
    )
}
