//! A spinner animated with keyframes, for the page on animation.

use hypersig::html::{div, span};
use hypersig::style::pattern::hstack;
use hypersig::style::value::{
    AnimationIterationCount, AnimationTimingFunction, BorderStyle, ms, px, raw, rem,
};
use hypersig::style::{Keyframes, Media, Style};
use hypersig::{Page, Signals, if_else};
use hypersig_ui::{ButtonSize, ButtonVariant, button, token};
use serde::{Deserialize, Serialize};

use crate::Site;
use crate::lang::L;

#[derive(Signals, Serialize, Deserialize)]
pub struct Upload {
    busy: bool,
}

pub fn page(site: Site) -> Page<Upload> {
    let busy = Upload::busy();
    let spin = Keyframes::new("spin")
        .at(0, Style::new().transform(raw("rotate(0deg)")))
        .at(100, Style::new().transform(raw("rotate(360deg)")));
    let ring = Style::new()
        .width(rem(1.5))
        .height(rem(1.5))
        .border_width(px(3))
        .border_style(BorderStyle::Solid)
        .border_color(token::muted().var())
        .border_top_color(token::primary().var())
        .border_radius(px(999))
        .keyframes(&spin)
        .animation_duration(ms(800))
        .animation_timing_function(AnimationTimingFunction::Linear)
        .animation_iteration_count(AnimationIterationCount::Infinite)
        .on(
            Media::ReducedMotion,
            Style::new().animation_duration(ms(4000)),
        );
    Page::new(
        site.t(L {
            en: "Upload",
            ja: "アップロード",
        }),
        Upload { busy: true },
    )
    .child(
        div()
            .css(hstack(rem(1)))
            .child(span().attr("aria-hidden", "true").css(ring).show(busy))
            .child(span().attr("role", "status").bind_text(if_else(
                busy,
                site.t(L {
                    en: "Uploading…",
                    ja: "アップロード中…",
                }),
                site.t(L {
                    en: "Done",
                    ja: "完了",
                }),
            )))
            .child(
                button(ButtonVariant::Outline, ButtonSize::Sm)
                    .on_click(busy.toggle())
                    .bind_text(if_else(
                        busy,
                        site.t(L {
                            en: "Stop",
                            ja: "停止",
                        }),
                        site.t(L {
                            en: "Start",
                            ja: "開始",
                        }),
                    )),
            ),
    )
}
