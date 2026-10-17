//! A stopwatch driven by an interval, for the page on escape hatches.

use std::time::Duration;

use hypersig::html::{div, span};
use hypersig::style::Style;
use hypersig::style::pattern::hstack;
use hypersig::style::value::{FontWeight, rem};
use hypersig::{NodeRef, Page, Signals, Stmt, if_else};
use hypersig_ui::{ButtonSize, ButtonVariant, button};
use serde::{Deserialize, Serialize};

#[derive(Signals, Serialize, Deserialize)]
pub struct Stopwatch {
    running: bool,
    tenths: i32,
}

const START: NodeRef<Stopwatch> = NodeRef::new("start");

pub fn page() -> Page<Stopwatch> {
    let running = Stopwatch::running();
    let tenths = Stopwatch::tenths();
    let seconds = tenths.to_f64() / 10.0;
    Page::new(
        "Stopwatch",
        Stopwatch {
            running: false,
            tenths: 0,
        },
    )
    .child(
        div()
            .css(hstack(rem(1)))
            .on_interval(
                Duration::from_millis(100),
                Stmt::when(running, tenths.set(tenths + 1)),
            )
            .child(
                span()
                    .css(
                        Style::new()
                            .min_width(rem(4))
                            .font_size(rem(1.5))
                            .font_weight(FontWeight::Number(600.0)),
                    )
                    .bind_text(seconds.to_fixed(1)),
            )
            .child(
                button(ButtonVariant::Default, ButtonSize::Default)
                    .node_ref(START)
                    .on_click(running.toggle())
                    .bind_text(if_else(running, "Stop", "Start")),
            )
            .child(
                button(ButtonVariant::Outline, ButtonSize::Default)
                    .on_click(running.set(false).then(tenths.set(0)).then(START.focus()))
                    .text("Reset"),
            ),
    )
}
