//! The page on light and dark: following the system with `theme()`, and
//! letting a signal pick the scheme with `light()` and `dark()`.

use crate::Site;
use crate::code::{self, Lang};
use crate::demo::{self, Demo};
use crate::doc::{Component, Doc};
use crate::lang::L;
use crate::learn::snippet::{Snippet, show};
use crate::prose::{Part, bullets, heading, lead, para, parts};

pub fn page(site: Site, head: Part) -> Part {
    parts(vec![
        head,
        lead(site, Doc::Components(Component::Theming).leads()),
        demo::show(site, Demo::Theme),
        system(site),
        picked(site),
        whole_page(site),
        unchecked(site),
        next(site),
    ])
}

fn system(site: Site) -> Part {
    parts([
        heading(
            site,
            L {
                en: "Following the system",
                ja: "システムの設定に従う",
            },
        ),
        para(
            site,
            L {
                en: "Every token's default is its light value, so a page without a theme is light. [`theme()`](api:hypersig_ui/fn.theme.html) returns a `Theme` that sets the dark value of each color token under `prefers-color-scheme: dark`, and `Page::theme` writes it to the page:",
                ja: "どのトークンも既定値はライトの値なので、テーマのないページはライトになります。[`theme()`](api:hypersig_ui/fn.theme.html) は、`prefers-color-scheme: dark` のときに色のトークンをそれぞれダークの値にする `Theme` を返し、`Page::theme` がそれをページに書き出します。",
            },
        ),
        code::block(Lang::Rust, None, "pub fn theme() -> Theme"),
        code::block(
            Lang::Html,
            None,
            "@media (prefers-color-scheme: dark) {\n  :root {\n    --background: #09090b;\n    --foreground: #fafafa;\n    --card: #18181b;\n    --primary: #e4e4e7;\n    --muted-foreground: #a1a1aa;\n    --input: #70707a;\n    --ring: #d4d4d8;\n    /* and the other color tokens */\n  }\n}",
        ),
        para(
            site,
            L {
                en: "The radius is the same in both schemes, so `theme()` leaves it out. `hypersig_ui::global()` sets `color-scheme: light dark` on `:root`, so the browser draws its own scrollbars and form controls in the scheme the system prefers. [Installation](components/installation/) shows both added to a page.",
                ja: "角の丸みは両方の配色で同じなので、`theme()` には含まれません。`hypersig_ui::global()` は `:root` に `color-scheme: light dark` を設定するので、ブラウザが自分で描くスクロールバーやフォームのコントロールも、システムが選んだ配色になります。両方をページに追加する例は[インストール](components/installation/)にあります。",
            },
        ),
    ])
}

fn picked(site: Site) -> Part {
    parts([
        heading(
            site,
            L {
                en: "Light and dark",
                ja: "ライトとダーク",
            },
        ),
        para(
            site,
            L {
                en: "To let a signal pick the scheme instead of the system, put [`dark()`](api:hypersig_ui/fn.dark.html) or [`light()`](api:hypersig_ui/fn.light.html) on an element with `El::css_when` or `El::css_match`. Each returns a `Style` that sets every color token, and `color-scheme`, on the element and those inside it:",
                ja: "システムの代わりにシグナルで配色を選ぶには、[`dark()`](api:hypersig_ui/fn.dark.html) か [`light()`](api:hypersig_ui/fn.light.html) を `El::css_when` や `El::css_match` で要素に付けます。どちらも色のトークンすべてと `color-scheme` を、その要素と内側の要素に設定する `Style` を返します。",
            },
        ),
        code::block(
            Lang::Rust,
            None,
            "pub fn dark() -> Style\npub fn light() -> Style",
        ),
        show(Snippet::UiPreview),
        para(
            site,
            L {
                en: "While `dark` is true, the `<div>` has these declarations, and every component inside it reads the dark values:",
                ja: "`dark` が true の間、`<div>` には次の宣言が付き、内側のコンポーネントはすべてダークの値を読みます。",
            },
        ),
        code::block(
            Lang::Html,
            None,
            "color-scheme: dark;\n--background: #09090b;\n--foreground: #fafafa;\n--primary: #e4e4e7;\n/* and the other color tokens */",
        ),
        para(
            site,
            L {
                en: "`light()` writes `color-scheme: light` and the light values in the same way. It needs a place of its own where the system prefers dark: `theme()` has already set the dark values on `:root`, and an element without `light()` inherits them.",
                ja: "`light()` も同じように `color-scheme: light` とライトの値を書き出します。システムがダークを選んでいるときには `light()` が必要です。`theme()` が `:root` にダークの値を設定済みで、`light()` のない要素はその値を継承するからです。",
            },
        ),
        para(
            site,
            L {
                en: "`css_when` and `css_match` take a `Style`, and `theme()` returns a `Theme`, which sets tokens on `:root` for the whole page. A theme passed where an element's style belongs fails to compile with E0308:",
                ja: "`css_when` と `css_match` は `Style` を受け取ります。`theme()` が返すのは、ページ全体の `:root` にトークンを設定する `Theme` です。要素のスタイルを渡す場所にテーマを渡すと、E0308 でコンパイルできません。",
            },
        ),
        show(Snippet::UiThemeAsStyle),
    ])
}

fn whole_page(site: Site) -> Part {
    parts([
        heading(
            site,
            L {
                en: "A picker for the whole page",
                ja: "ページ全体の配色を選ぶ",
            },
        ),
        para(
            site,
            L {
                en: "For a picker that colors the whole page, put the styles on `<body>` with `Page::css_match`, so the tokens reach the background of `<body>` too. Keep the choice with [`Page::persist`](learn/persisted-signals/), so the next page load starts from it, and run the change inside `Stmt::view_transition` to animate it, as the theme button in this site's header does:",
                ja: "ページ全体の配色を選ぶには、`Page::css_match` でスタイルを `<body>` に付けます。こうすると、トークンが `<body>` 自身の背景にも届きます。選んだ配色は [`Page::persist`](learn/persisted-signals/) で保存すると、次にページを読み込んだときもその配色で始まります。切り替えを `Stmt::view_transition` の中で実行すると、変化がアニメーションになります。このサイトのヘッダーにある配色のボタンも同じ作りです。",
            },
        ),
        show(Snippet::UiScheme),
        para(
            site,
            L {
                en: "`Scheme::System` has no case, so `<body>` gets no tokens and the values from `theme()` apply. `Scheme::Light` and `Scheme::Dark` set their values on `<body>`, which win over those on `:root` for everything inside `<body>`.",
                ja: "`Scheme::System` にはケースがないので、`<body>` にはトークンが付かず、`theme()` の値が適用されます。`Scheme::Light` と `Scheme::Dark` は `<body>` に値を設定し、`<body>` の内側のすべてについて、その値が `:root` の値より優先されます。",
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
                    en: "The page's own scrollbar follows `color-scheme` on `<html>`, which `global()` sets to `light dark`. While `<body>` shows a picked scheme, the scrollbar keeps the system's.",
                    ja: "ページ自体のスクロールバーは `<html>` の `color-scheme` に従い、`global()` はこれを `light dark` にします。そのため、`<body>` が選ばれた配色になっても、スクロールバーはシステムの配色のままです。",
                },
                L {
                    en: "A color of your own that reads no token keeps its value in both schemes. Read every color that should change from a [token](components/tokens/).",
                    ja: "トークンを読まない独自の色は、どちらの配色でも同じ値のままです。配色に合わせて変えたい色はすべて[トークン](components/tokens/)から読みます。",
                },
                L {
                    en: "The server renders the first HTML from the signal's initial value, because it never sees the stored one. Where they differ, the page stays hidden until the stored scheme is drawn, as [Signals kept across page loads](learn/persisted-signals/) explains.",
                    ja: "サーバーは保存された値を受け取らないので、最初の HTML はシグナルの初期値で描画します。両者が異なる場合は、保存された配色が描画されるまでページは隠れたままです。詳しくは[ページをまたいで保つシグナル](learn/persisted-signals/)で説明しています。",
                },
            ],
        ),
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
                en: "[Tokens](components/tokens/) lists every token and changes their values. [Dark mode](style/dark-mode/) covers the same switch for styles that do not use these components.",
                ja: "[トークン](components/tokens/)ではすべてのトークンを挙げ、その値を変えます。これらのコンポーネントを使わないスタイルで同じ切り替えをする方法は、[ダークモード](style/dark-mode/)で説明しています。",
            },
        ),
    ])
}
