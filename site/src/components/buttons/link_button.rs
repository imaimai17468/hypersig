//! The page on `button_style` on an `<a>`: a link that looks like a button.

use crate::Site;
use crate::code::{self, Lang};
use crate::demo::{self, Demo};
use crate::doc::{Component, Doc};
use crate::lang::L;
use crate::learn::snippet::{Snippet, show};
use crate::prose::{Part, bullets, heading, lead, para, parts, table_of};

pub fn page(site: Site, head: Part) -> Part {
    parts(vec![
        head,
        lead(site, Doc::Components(Component::LinkButton).leads()),
        demo::show(site, Demo::LinkButton),
        accessibility(site),
        function(site),
        unchecked(site),
        example(site),
        next(site),
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
                en: "A link that goes to another page should be a link, even where it looks like a button: a screen reader announces it as a link, Enter follows it, and the browser offers to open it in a new tab or copy its address. A `<button>` does none of these. The demo above has two links and one button, and only the button lacks them.",
                ja: "別のページへ移動するものは、ボタンの見た目でもリンクにします。リンクならスクリーンリーダーはリンクとして読み上げ、Enter で移動し、ブラウザは新しいタブで開く操作やアドレスのコピーを提供します。`<button>` にはどれもありません。上のデモには 2 つのリンクと 1 つのボタンがあり、これらがないのはボタンだけです。",
            },
        ),
        table_of(
            site,
            L {
                en: &["Key", "What it does"],
                ja: &["キー", "動作"],
            },
            L {
                en: &[&[
                    "Enter",
                    "Follows the link. Space scrolls the page, as on any link.",
                ]],
                ja: &[&[
                    "Enter",
                    "リンク先に移動します。Space はほかのリンクと同じくページをスクロールします。",
                ]],
            },
        ),
    ])
}

fn function(site: Site) -> Part {
    parts([
        heading(
            site,
            L {
                en: "`button_style`",
                ja: "`button_style`",
            },
        ),
        para(
            site,
            L {
                en: "[`button_style`](api:hypersig_ui/fn.button_style.html) returns the `Style` that `button` puts on its `<button>`, for the same [`ButtonVariant`](api:hypersig_ui/enum.ButtonVariant.html) and [`ButtonSize`](api:hypersig_ui/enum.ButtonSize.html). Put it on an `<a>` with `El::css`:",
                ja: "[`button_style`](api:hypersig_ui/fn.button_style.html) は、同じ [`ButtonVariant`](api:hypersig_ui/enum.ButtonVariant.html) と [`ButtonSize`](api:hypersig_ui/enum.ButtonSize.html) で `button` が `<button>` に付けるのと同じ `Style` を返します。これを `El::css` で `<a>` に付けます。",
            },
        ),
        code::block(
            Lang::Rust,
            None,
            "pub fn button_style(variant: ButtonVariant, size: ButtonSize) -> Style",
        ),
        para(
            site,
            L {
                en: "It adds only a class, so the `<a>` keeps its own attributes. `a().href(SIGNUP.link()).css(button_style(ButtonVariant::Default, ButtonSize::Lg)).text(\"Sign up\")` renders as follows, without the `class`:",
                ja: "付くのはクラスだけなので、`<a>` は自分の属性を保ちます。`a().href(SIGNUP.link()).css(button_style(ButtonVariant::Default, ButtonSize::Lg)).text(\"Sign up\")` は、`class` を除くと次のように描画されます。",
            },
        ),
        code::block(Lang::Html, None, "<a href=\"/signup\">Sign up</a>"),
        para(
            site,
            L {
                en: "The address comes from a route or a `SafeUrl`, as on any link. `href` has no conversion from `&str`, so an address written as text fails to compile with E0277:",
                ja: "ほかのリンクと同じく、アドレスはルートか `SafeUrl` から取ります。`href` には `&str` からの変換がないので、文字列で書いたアドレスは E0277 でコンパイルできません。",
            },
        ),
        show(Snippet::UiLinkHrefString),
        para(
            site,
            L {
                en: "`button_style` itself takes the two enums of `button`, so a variant written as text fails to compile with E0308, as [Button](components/button/) shows.",
                ja: "`button_style` 自体も `button` と同じ 2 つの列挙型を受け取るので、バリアントを文字列で書くと E0308 でコンパイルできません。[ボタン](components/button/)にその例があります。",
            },
        ),
    ])
}

fn unchecked(site: Site) -> Part {
    parts([
        heading(
            site,
            L {
                en: "What the types leave unchecked",
                ja: "型では検査しないこと",
            },
        ),
        bullets(
            site,
            &[
                L {
                    en: "`button_style` is a `Style` and goes on any element. On a `<div>` or a `<span>`, the element looks like a button but takes no focus, ignores Enter and Space, and has no role. Put it on an `<a>` with an `href`.",
                    ja: "`button_style` は `Style` なので、どの要素にも付けられます。`<div>` や `<span>` に付けると、ボタンのように見えても、フォーカスを受け取らず、Enter と Space に反応せず、ロールもありません。`href` を持つ `<a>` に付けます。",
                },
                L {
                    en: "An `<a>` without `href` is not a link: it takes no focus and a screen reader does not announce it as a link.",
                    ja: "`href` のない `<a>` はリンクではありません。フォーカスを受け取らず、スクリーンリーダーもリンクとして読み上げません。",
                },
                L {
                    en: "`El::disabled` does not apply to a link. A link that should not be followed is left out, or shown as text.",
                    ja: "`El::disabled` はリンクには効きません。移動させたくないリンクは置かないか、テキストとして表示します。",
                },
            ],
        ),
    ])
}

fn example(site: Site) -> Part {
    parts([
        heading(
            site,
            L {
                en: "Example: links to a page and to another site",
                ja: "例: ページへのリンクと別サイトへのリンク",
            },
        ),
        show(Snippet::UiLinkButton),
    ])
}

fn next(site: Site) -> Part {
    parts([
        heading(
            site,
            L {
                en: "Next",
                ja: "次に読むページ",
            },
        ),
        para(
            site,
            L {
                en: "[Links and redirects](learn/links/) builds an `href` from a route. [Pagination](components/pagination/) draws its links to the pages with the same style.",
                ja: "ルートから `href` を作る方法は[リンクとリダイレクト](learn/links/)で説明しています。[ページネーション](components/pagination/)はページへのリンクを同じスタイルで描きます。",
            },
        ),
    ])
}
