//! The Components page on `kbd`: a key or a shortcut drawn as a small cap in
//! running text.

use crate::Site;
use crate::code::{self, Lang};
use crate::demo::{self, Demo};
use crate::doc::{Component, Doc};
use crate::lang::L;
use crate::learn::snippet::{Snippet, show};
use crate::prose::{Part, heading, lead, para, parts};

pub fn page(site: Site, head: Part) -> Part {
    parts(vec![
        head,
        lead(site, Doc::Components(Component::Kbd).leads()),
        demo::show(site, Demo::Kbd),
        accessibility(site),
        building(site),
        html(site),
        compile_errors(site),
        unchecked(site),
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
                en: "It is a native `<kbd>`, which a screen reader reads as the text it holds, as part of the sentence around it. Write the key's name in words where a symbol would be read as something else, such as \"Ctrl\" rather than \"^\". The key cannot be selected with the pointer, so copying the sentence around it leaves the key out. It takes no keys of its own.",
                ja: "ネイティブの `<kbd>` で、スクリーンリーダーは中のテキストを周りの文の一部として読み上げます。記号が別のものとして読まれる場合は、「^」ではなく「Ctrl」のように、キーの名前を言葉で書いてください。キーはポインタで選択できないので、周りの文をコピーしてもキーは含まれません。このコンポーネント自体はキー操作を受けません。",
            },
        ),
    ])
}

fn building(site: Site) -> Part {
    parts([
        heading(
            site,
            L {
                en: "Building a key",
                ja: "キーを作る",
            },
        ),
        para(
            site,
            L {
                en: "[`kbd`](api:hypersig_ui/fn.kbd.html) returns an empty `<kbd>`, whose text comes from `El::text`. For a shortcut, place one `kbd` per key side by side:",
                ja: "[`kbd`](api:hypersig_ui/fn.kbd.html) は空の `<kbd>` を返し、テキストは `El::text` で入れます。ショートカットはキーごとに 1 つの `kbd` を並べて書きます。",
            },
        ),
        code::block(Lang::Rust, None, "pub fn kbd<S>() -> El<S>;"),
        para(
            site,
            L {
                en: "The key is drawn in `token::muted` with the font of the text around it, rather than a monospace font, and stays on one line.",
                ja: "キーは `token::muted` の色で、周りのテキストと同じフォントで描かれ、折り返しません。",
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
                en: "The example at the end of this page renders this HTML, with the classes left out:",
                ja: "このページの最後の例は次の HTML を出力します（クラスは省いています）。",
            },
        ),
        code::block(
            Lang::Html,
            None,
            r"<span><kbd>Ctrl</kbd><kbd>Shift</kbd><kbd>P</kbd></span>
<p>Press <kbd>Esc</kbd> to close the dialog.</p>",
        ),
    ])
}

fn compile_errors(site: Site) -> Part {
    parts([
        heading(
            site,
            L {
                en: "What fails to compile",
                ja: "コンパイルエラーになるもの",
            },
        ),
        para(
            site,
            L {
                en: "`kbd` takes no arguments and holds only text, and its types stop no mistake. Text is escaped like any element's, so a key such as `<` shows as itself.",
                ja: "`kbd` は引数を取らず、テキストを持つだけなので、型が防ぐ誤りはありません。テキストはほかの要素と同じくエスケープされるので、`<` のようなキーもそのまま表示されます。",
            },
        ),
    ])
}

fn unchecked(site: Site) -> Part {
    parts([
        heading(
            site,
            L {
                en: "What is not checked",
                ja: "検査されないこと",
            },
        ),
        para(
            site,
            L {
                en: "Nothing checks that the text names a key, or that it matches the shortcut the page answers to. A shortcut written as one `kbd` holding \"Ctrl+Shift+P\" renders as one wide key. Ctrl and Command differ by platform, and the text shows only what you write, so a page with `Key::primary` names both or picks one per visitor on the server.",
                ja: "テキストがキーの名前か、ページが受け付けるショートカットと合っているかは検査されません。「Ctrl+Shift+P」を 1 つの `kbd` に入れると、幅の広い 1 つのキーとして描かれます。Ctrl と Command はプラットフォームによって違い、テキストは書いたとおりにしか表示されないので、`Key::primary` を使うページでは両方を書くか、サーバーで訪問者ごとにどちらかを選びます。",
            },
        ),
    ])
}

fn example(site: Site) -> Part {
    parts([
        heading(
            site,
            L {
                en: "Example: a shortcut and a key in a sentence",
                ja: "例: ショートカットと文中のキー",
            },
        ),
        show(Snippet::UiKbd),
        para(
            site,
            L {
                en: "[Keyboard patterns](learn/keyboard/) covers the shortcuts a page answers to with `Page::on_key`. A tooltip that shows a button's shortcut is a [Tooltip](components/tooltip/).",
                ja: "ページが `Page::on_key` で受け付けるショートカットは [キーボード操作](learn/keyboard/) で説明しています。ボタンのショートカットを表示するツールチップには [ツールチップ](components/tooltip/) を使います。",
            },
        ),
    ])
}
