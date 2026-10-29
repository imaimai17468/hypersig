//! The component page on `card` and its parts: a bordered box with a
//! header, content, and a footer, whose title takes a `Level`.

use crate::Site;
use crate::code::{self, Lang};
use crate::demo::{self, Demo};
use crate::doc::{Component, Doc};
use crate::lang::L;
use crate::learn::snippet::{Snippet, show};
use crate::prose::{Part, heading, lead, para, parts, table_of};

pub fn page(site: Site, head: Part) -> Part {
    parts(vec![
        head,
        lead(site, Doc::Components(Component::Card).leads()),
        demo::show(site, Demo::Card),
        accessibility(site),
        building(site),
        level(site),
        html(site),
        example(site),
    ])
}

fn accessibility(site: Site) -> Part {
    parts([
        heading(
            site,
            L {
                en: "Accessibility",
                ja: "アクセシビリティ",
            },
        ),
        para(
            site,
            L {
                en: "A card is a `<div>` with no role, so a screen reader reads its content in order and announces no card. Its title is a real heading, which a screen reader lists among the page's headings and jumps to with its heading keys. The card takes no keys of its own.",
                ja: "カードはロールを持たない `<div>` なので、スクリーンリーダーは中身を順に読み上げ、カードであることは伝えません。タイトルは本物の見出しなので、スクリーンリーダーはページの見出しの一覧にそれを含め、見出しへ移るキーで移動できます。カード自体が受け付けるキーはありません。",
            },
        ),
    ])
}

fn building(site: Site) -> Part {
    parts([
        heading(
            site,
            L {
                en: "The parts",
                ja: "部品",
            },
        ),
        para(
            site,
            L {
                en: "Each part is a function that returns an empty element, styled, for the children you add:",
                ja: "どの部品もスタイルの付いた空の要素を返す関数で、中身はあとから加えます。",
            },
        ),
        table_of(
            site,
            L {
                en: &["Function", "Element", "What it is"],
                ja: &["関数", "要素", "役割"],
            },
            L {
                en: &[
                    &[
                        "[`card<S>() -> El<S>`](api:hypersig_ui/fn.card.html)",
                        "`<div>`",
                        "The box: a border, a shadow, and `token::card` behind it, with its parts in a column.",
                    ],
                    &[
                        "[`card_header<S>() -> El<S>`](api:hypersig_ui/fn.card_header.html)",
                        "`<div>`",
                        "The top, holding the title and the description.",
                    ],
                    &[
                        "[`card_title<S>(level: Level) -> El<S>`](api:hypersig_ui/fn.card_title.html)",
                        "`<h2>`, `<h3>`, or `<h4>`",
                        "The title, as a heading of `level`.",
                    ],
                    &[
                        "[`card_description<S>() -> El<S>`](api:hypersig_ui/fn.card_description.html)",
                        "`<p>`",
                        "A line under the title, in `token::muted_foreground`.",
                    ],
                    &[
                        "[`card_content<S>() -> El<S>`](api:hypersig_ui/fn.card_content.html)",
                        "`<div>`",
                        "The body.",
                    ],
                    &[
                        "[`card_footer<S>() -> El<S>`](api:hypersig_ui/fn.card_footer.html)",
                        "`<div>`",
                        "The bottom, laying its children out in a row.",
                    ],
                ],
                ja: &[
                    &[
                        "[`card<S>() -> El<S>`](api:hypersig_ui/fn.card.html)",
                        "`<div>`",
                        "ボックス本体。枠線と影を持ち、背景は `token::card` で、部品を縦に並べます。",
                    ],
                    &[
                        "[`card_header<S>() -> El<S>`](api:hypersig_ui/fn.card_header.html)",
                        "`<div>`",
                        "上部。タイトルと説明を入れます。",
                    ],
                    &[
                        "[`card_title<S>(level: Level) -> El<S>`](api:hypersig_ui/fn.card_title.html)",
                        "`<h2>`、`<h3>`、`<h4>`",
                        "タイトル。`level` の見出しです。",
                    ],
                    &[
                        "[`card_description<S>() -> El<S>`](api:hypersig_ui/fn.card_description.html)",
                        "`<p>`",
                        "タイトルの下の 1 行。色は `token::muted_foreground` です。",
                    ],
                    &[
                        "[`card_content<S>() -> El<S>`](api:hypersig_ui/fn.card_content.html)",
                        "`<div>`",
                        "本体。",
                    ],
                    &[
                        "[`card_footer<S>() -> El<S>`](api:hypersig_ui/fn.card_footer.html)",
                        "`<div>`",
                        "下部。子要素を横に並べます。",
                    ],
                ],
            },
        ),
        para(
            site,
            L {
                en: "The parts take only children and text, so apart from the title's level the types stop no mistake here. A part used outside a `card` is a styled `<div>` with the same padding, and a card with no header or footer is still a card.",
                ja: "部品は子要素とテキストしか受け取らないので、タイトルのレベルを除けば、型が防ぐ間違いはありません。`card` の外で使った部品は、同じ余白を持つスタイル付きの `<div>` になり、ヘッダーやフッターのないカードもカードとして描かれます。",
            },
        ),
    ])
}

fn level(site: Site) -> Part {
    parts([
        heading(
            site,
            L {
                en: "The level of the title",
                ja: "タイトルのレベル",
            },
        ),
        para(
            site,
            L {
                en: "A screen reader lists a page by its headings, so a card's title sits one level below the heading of the section around it. [`Level`](api:hypersig_ui/enum.Level.html) has the three levels a card can take:",
                ja: "スクリーンリーダーはページを見出しで一覧にするので、カードのタイトルは周りのセクションの見出しより 1 つ下のレベルにします。[`Level`](api:hypersig_ui/enum.Level.html) は、カードが取りうる 3 つのレベルを持ちます。",
            },
        ),
        code::block(
            Lang::Rust,
            None,
            "#[non_exhaustive]
pub enum Level {
    H2,
    H3,
    H4,
}",
        ),
        para(
            site,
            L {
                en: "There is no `H1`: a page has one top-level heading of its own, and a card sits below it. A card title written as `Level::H1` is E0599:",
                ja: "`H1` はありません。最上位の見出しはページに 1 つで、カードはその下に置くからです。`Level::H1` と書いたカードのタイトルは E0599 になります。",
            },
        ),
        show(Snippet::UiCardLevel),
        para(
            site,
            L {
                en: "Which of the three is right depends on the headings around the card, which the compiler does not see. A title one level too deep, such as an `<h4>` under an `<h2>`, renders and leaves a gap in the screen reader's list of headings.",
                ja: "3 つのうちどれが正しいかはカードの周りの見出しで決まり、コンパイラはそれを見ません。`<h2>` の下の `<h4>` のように 1 段深すぎるタイトルもそのまま描画され、スクリーンリーダーの見出しの一覧に抜けができます。",
            },
        ),
    ])
}

fn html(site: Site) -> Part {
    parts([
        heading(
            site,
            L {
                en: "The HTML",
                ja: "出力される HTML",
            },
        ),
        para(
            site,
            L {
                en: "A card with every part renders as nested elements and nothing else. Its look comes from the styles of each part:",
                ja: "すべての部品を使ったカードは、入れ子の要素だけとして描画されます。見た目は各部品のスタイルで付きます。",
            },
        ),
        code::block(
            Lang::Html,
            None,
            r#"<div>
  <div>
    <h3>Pro</h3>
    <p>For teams of up to ten.</p>
  </div>
  <div>$20 a month</div>
  <div><button type="button">Choose</button></div>
</div>"#,
        ),
    ])
}

fn example(site: Site) -> Part {
    parts([
        heading(
            site,
            L {
                en: "Example: pricing plans",
                ja: "例: 料金プラン",
            },
        ),
        para(
            site,
            L {
                en: "The plans sit in a section headed by an `<h2>`, so each card's title is an `<h3>`:",
                ja: "プランは `<h2>` の見出しを持つセクションに置くので、各カードのタイトルは `<h3>` にします。",
            },
        ),
        show(Snippet::UiCard),
        para(
            site,
            L {
                en: "Next, [Separator](components/separator/) draws a line between groups of content.",
                ja: "次の [区切り線](components/separator/) では内容のまとまりの間に線を引きます。",
            },
        ),
    ])
}
