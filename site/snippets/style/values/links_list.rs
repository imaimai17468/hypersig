use hypersig::html::{El, a, li, ul};
use hypersig::style::value::{quoted, rem, url};
use hypersig::style::{PseudoElement, Style};
use hypersig::{Page, SafeUrl, Signals};
use serde::{Deserialize, Serialize};

#[derive(Signals, Serialize, Deserialize)]
struct Docs {}

/// A list of links to other sites, each item marked with an arrow and
/// followed by an icon.
fn elsewhere(links: &[(&str, SafeUrl)]) -> El<Docs> {
    ul().css(Style::new().list_style_type(quoted("→ ")).padding_left(rem(1.5)))
        .children(links.iter().map(|(text, href)| {
            li().child(
                a().href(href.clone())
                    .css(Style::new().pseudo(
                        PseudoElement::After,
                        Style::new()
                            .content(url("/icons/external.svg"))
                            .margin_left(rem(0.25)),
                    ))
                    .text(*text),
            )
        }))
}

let html = Page::new("Docs", Docs {})
    .child(elsewhere(&[("MDN", SafeUrl::new("https://developer.mozilla.org"))]))
    .render();
# assert!(html.contains(r#"{list-style-type:"→ "}"#), "{html}");
# assert!(html.contains(r#"::after{content:url("/icons/external.svg")}"#), "{html}");
