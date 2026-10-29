//! The page on dark mode: following the system, and a picker.

use crate::Site;
use crate::code::{self, Lang};
use crate::demo::{self, Demo};
use crate::doc::{Doc, StylePage};
use crate::lang::L;
use crate::learn::snippet::{Snippet, show};
use crate::prose::{Part, bullets, heading, lead, para, parts};

pub fn page(site: Site, head: Part) -> Part {
    parts(vec![
        head,
        lead(site, Doc::Style(StylePage::DarkMode).leads()),
        heading(
            site,
            L {
                en: "Colors as tokens",
                ja: "色をトークンにする",
            },
        ),
        para(
            site,
            L {
                en: "Read every color that changes with the scheme from a [token](style/tokens/), and give each token its light value as the default. Dark mode is then a second set of values for the same tokens, and no element's style has to change.",
                ja: "配色によって変わる色はすべて[トークン](style/tokens/)から読み、各トークンの既定値にはライトの値を与えます。そうすれば、ダークモードは同じトークンに対する 2 つ目の値の組になり、要素のスタイルは変えずに済みます。",
            },
        ),
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
                en: "`Theme::on(Media::Dark, ..)` sets the dark values while the system prefers a dark color scheme. The `color-scheme` property tells the browser to draw its own parts, such as scrollbars and form controls, in the same scheme:",
                ja: "`Theme::on(Media::Dark, ..)` は、システムがダークの配色を選んでいる間だけダークの値を設定します。`color-scheme` プロパティは、スクロールバーやフォームのコントロールのようなブラウザ自身が描く部分も同じ配色で描くようブラウザに伝えます。",
            },
        ),
        show(Snippet::DarkSystem),
        code::block(
            Lang::Html,
            None,
            ":root { color-scheme: light dark }\n\
             body { background-color: var(--background, white); color: var(--foreground, #09090b) }\n\
             @media (prefers-color-scheme: dark) { :root { --background: #09090b; --foreground: #fafafa } }",
        ),
        para(
            site,
            L {
                en: "[`ColorScheme`](api:hypersig/style/value/enum.ColorScheme.html) has `LightDark` for `light dark`, which follows the system, and `Light` and `Dark` for a page that is always one of them. A value such as `only dark` goes through `raw`, which [Unchecked values](style/unchecked/) covers.",
                ja: "[`ColorScheme`](api:hypersig/style/value/enum.ColorScheme.html) には、システムに従う `light dark` を書く `LightDark` と、常にどちらか一方の配色にするページのための `Light` と `Dark` があります。`only dark` のような値は `raw` で書きます。`raw` は[検査しない値](style/unchecked/)で説明しています。",
            },
        ),
        heading(
            site,
            L {
                en: "Letting the visitor pick",
                ja: "訪問者に選ばせる",
            },
        ),
        para(
            site,
            L {
                en: "A picker keeps the choice in a signal and sets the tokens on `<body>` with [`Page::css_match`](api:hypersig/struct.Page.html#method.css_match), so they reach every element and the background of `<body>` itself. [`Page::persist`](learn/persisted-signals/) keeps the signal in `localStorage`, so the next page load starts from the stored choice:",
                ja: "配色の切り替えでは選んだ配色をシグナルに保ち、[`Page::css_match`](api:hypersig/struct.Page.html#method.css_match) でトークンを `<body>` に設定します。こうすると、トークンはすべての要素と `<body>` 自身の背景に届きます。[`Page::persist`](learn/persisted-signals/) はシグナルを `localStorage` に保存するので、次にページを読み込んだときも保存した配色から始まります。",
            },
        ),
        show(Snippet::DarkPicker),
        para(
            site,
            L {
                en: "Each value of `Scheme` gives this result:",
                ja: "`Scheme` の値ごとの結果は次のとおりです。",
            },
        ),
        bullets(
            site,
            &[
                L {
                    en: "`System` has no case, so `<body>` gets no tokens and the theme on `:root` applies: light, or dark while the system prefers dark.",
                    ja: "`System` にはケースがないので、`<body>` にはトークンが付かず、`:root` のテーマが適用されます。通常はライトで、システムがダークを選んでいる間はダークになります。",
                },
                L {
                    en: "`Dark` sets the dark values on `<body>`, which win over the values on `:root` for everything inside `<body>`.",
                    ja: "`Dark` は `<body>` にダークの値を設定します。`<body>` の内側のすべてについて、この値が `:root` の値より優先されます。",
                },
                L {
                    en: "`Light` sets the light values on `<body>`. It needs a case of its own, because the theme sets the dark values on `:root` while the system prefers dark.",
                    ja: "`Light` は `<body>` にライトの値を設定します。システムがダークを選んでいる間は、テーマが `:root` にダークの値を設定するので、`Light` にも専用のケースが必要です。",
                },
            ],
        ),
        para(
            site,
            L {
                en: "`hypersig_ui` has these styles ready as `hypersig_ui::dark()` and `hypersig_ui::light()`. The preview below puts them on a `<div>` with `El::css_match`, so only the preview changes:",
                ja: "`hypersig_ui` には、これらのスタイルが `hypersig_ui::dark()` と `hypersig_ui::light()` として用意されています。下のプレビューは `El::css_match` でそれを `<div>` に付けているので、変わるのはプレビューの中だけです。",
            },
        ),
        demo::show(site, Demo::Theme),
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
                    en: "`Style::on(Media::Dark, ..)` on an element follows the system, not the picker. A color that should follow the picker is read from a token.",
                    ja: "要素に付けた `Style::on(Media::Dark, ..)` はシステムの設定に従い、切り替えには従いません。切り替えに従わせたい色はトークンから読みます。",
                },
                L {
                    en: "Nothing checks that the dark values cover every token the light ones do. A token left out keeps its light value in dark mode.",
                    ja: "ダークの値がライトの値と同じトークンをすべて設定しているかどうかは検査しません。設定しなかったトークンは、ダークモードでもライトの値のままです。",
                },
                L {
                    en: "The page's own scrollbar follows `color-scheme` on `<html>`, not on `<body>`. With `light dark` on `:root`, it keeps the system's scheme while `<body>` shows the picked one.",
                    ja: "ページ自体のスクロールバーは `<body>` ではなく `<html>` の `color-scheme` に従います。`:root` に `light dark` を設定していると、`<body>` が選ばれた配色になっても、スクロールバーはシステムの配色のままです。",
                },
                L {
                    en: "The server renders the first HTML from the initial value of the signal, because it never sees the stored one. Where they differ, the page stays hidden until the stored scheme is drawn, as [Signals kept across page loads](learn/persisted-signals/) explains.",
                    ja: "サーバーは保存された値を受け取らないので、最初の HTML はシグナルの初期値で描画します。両者が異なる場合は、保存された配色が描画されるまでページは隠れたままです。詳しくは[ページをまたいで保つシグナル](learn/persisted-signals/)で説明しています。",
                },
            ],
        ),
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
                en: "[Rules by tag](style/global/) styles `<body>` and the other elements of the page by their tag. [Theming](components/theming/#light-and-dark) shows the theme button of this site's header, which animates the change with a view transition.",
                ja: "[タグ単位のルール](style/global/)では `<body>` などのページの要素にタグ単位でスタイルを付けます。このサイトのヘッダーにある配色のボタンは、切り替えを View Transitions でアニメーションします。その作りは Components の[テーマ](components/theming/#light-and-dark)のページで示しています。",
            },
        ),
    ])
}
