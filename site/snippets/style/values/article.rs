use hypersig::html::{El, article, h1, p};
use hypersig::style::Style;
use hypersig::style::value::{Length, MarginInline, ch, dvh, lh, pct, rem};
use hypersig::{Page, Signals};
use serde::{Deserialize, Serialize};

#[derive(Signals, Serialize, Deserialize)]
struct Post {}

fn post(title: &str, body: &str) -> El<Post> {
    article()
        .css(
            Style::new()
                .width(pct(100))
                .max_width(ch(65))
                .min_height(dvh(100))
                .margin_inline(MarginInline::Auto)
                .padding_inline(rem(1.5)),
        )
        .child(h1().css(Style::new().margin_top(Length::ZERO)).text(title))
        .child(p().css(Style::new().margin_top(lh(1))).text(body))
}

let html = Page::new("Post", Post {})
    .child(post("Lengths", "A line of this column holds about 65 characters."))
    .render();
# for css in ["{max-width:65ch}", "{min-height:100dvh}", "{margin-top:0}", "{margin-top:1lh}"] {
#     assert!(html.contains(css), "{css}");
# }
