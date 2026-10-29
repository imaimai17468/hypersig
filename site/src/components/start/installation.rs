//! The page on adding `hypersig_ui`: the dependency, `global()` and
//! `theme()`, what they write to the page, and how the components are built.

use crate::Site;
use crate::code::{self, Lang};
use crate::doc::{Component, Doc};
use crate::lang::L;
use crate::learn::snippet::{Snippet, show};
use crate::prose::{Part, bullets, heading, lead, para, parts};

pub fn page(site: Site, head: Part) -> Part {
    parts(vec![
        head,
        lead(site, Doc::Components(Component::Installation).leads()),
        dependency(site),
        page_rules(site),
        built(site),
        next(site),
    ])
}

fn dependency(site: Site) -> Part {
    parts([
        heading(
            site,
            L {
                en: "Add the crate",
                ja: "クレートを追加する",
            },
        ),
        para(
            site,
            L {
                en: "Add it next to `hypersig`, at the same version:",
                ja: "`hypersig` と並べて、同じバージョンで追加します。",
            },
        ),
        code::block(
            Lang::Toml,
            Some("Cargo.toml"),
            r#"[dependencies]
hypersig = "0.14"
hypersig_ui = "0.14"
serde = { version = "1", features = ["derive"] }"#,
        ),
        para(
            site,
            L {
                en: "`hypersig_ui` builds on `hypersig`'s `El`, `Sig`, and `Style`, so a component from one version takes signals and styles only from the same version of `hypersig`. With two versions in the build, a component's `El<S>` is a different type from the page's, and the call fails to compile with E0308.",
                ja: "`hypersig_ui` は `hypersig` の `El`、`Sig`、`Style` の上に作られているので、コンポーネントが受け取れるのは同じバージョンの `hypersig` のシグナルとスタイルだけです。ビルドに 2 つのバージョンが混ざると、コンポーネントの `El<S>` はページの `El<S>` とは別の型になり、呼び出しは E0308 でコンパイルできません。",
            },
        ),
    ])
}

fn page_rules(site: Site) -> Part {
    parts([
        heading(
            site,
            L {
                en: "Add the theme to the page",
                ja: "ページにテーマを入れる",
            },
        ),
        para(
            site,
            L {
                en: "Add [`hypersig_ui::global()`](api:hypersig_ui/fn.global.html) and [`hypersig_ui::theme()`](api:hypersig_ui/fn.theme.html) to the page once, with `Page::global` and `Page::theme`:",
                ja: "[`hypersig_ui::global()`](api:hypersig_ui/fn.global.html) と [`hypersig_ui::theme()`](api:hypersig_ui/fn.theme.html) を、`Page::global` と `Page::theme` でページに 1 回ずつ追加します。",
            },
        ),
        code::block(
            Lang::Rust,
            None,
            "pub fn global() -> Global\npub fn theme() -> Theme",
        ),
        show(Snippet::UiSetup),
        para(
            site,
            L {
                en: "`global` is a reset, the font, and the page colors. `theme` sets the dark value of every color token while the system prefers dark. Together they add these rules to the page's `<style>`:",
                ja: "`global` はリセット、フォント、ページの色です。`theme` は、システムがダークの配色を選んでいる間、色のトークンをすべてダークの値にします。2 つを合わせると、ページの `<style>` に次のルールが入ります。",
            },
        ),
        code::block(
            Lang::Html,
            None,
            "*, ::before, ::after { box-sizing: border-box; margin: 0 }\n\
             body {\n  color: var(--foreground, #09090b);\n  background-color: var(--background, #ffffff);\n  font-family: ui-sans-serif, system-ui, sans-serif, \"Apple Color Emoji\", \"Segoe UI Emoji\";\n  line-height: 1.5\n}\n\
             img { display: block; max-width: 100% }\n\
             svg { display: block; max-width: 100% }\n\
             video { display: block; max-width: 100% }\n\
             button { color: currentColor; font: inherit }\n\
             input { color: currentColor; font: inherit }\n\
             select { color: currentColor; font: inherit }\n\
             textarea { color: currentColor; font: inherit }\n\
             :root { color-scheme: light dark }\n\
             code { font-family: monospace }\n\
             @media (prefers-color-scheme: dark) {\n  :root { --background: #09090b; --foreground: #fafafa; --primary: #e4e4e7; /* and the other color tokens */ }\n}",
        ),
        para(
            site,
            L {
                en: "`global()` returns a `Global` and `theme()` a `Theme`, and `Page::global` and `Page::theme` each take only their own type, so one passed to the other's method fails to compile with E0308:",
                ja: "`global()` は `Global` を、`theme()` は `Theme` を返し、`Page::global` と `Page::theme` はそれぞれ自分の型しか受け取りません。そのため、一方をもう一方のメソッドに渡すと E0308 でコンパイルできません。",
            },
        ),
        show(Snippet::UiGlobalAsTheme),
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
                    en: "Nothing checks that a page has `global()` and `theme()`. Without `global()`, the components keep their own look, but `<body>` keeps the browser's font, margin, and colors. Without `theme()`, every visitor sees the light values, whatever the system prefers.",
                    ja: "ページに `global()` と `theme()` があるかどうかは検査しません。`global()` がなくてもコンポーネント自体の見た目は変わりませんが、`<body>` はブラウザ既定のフォント、余白、色のままです。`theme()` がなければ、システムの設定にかかわらず、どの訪問者にもライトの値が表示されます。",
                },
                L {
                    en: "Adding either twice writes its rules twice. The page looks the same, and its `<style>` is longer.",
                    ja: "どちらかを 2 回追加すると、ルールが 2 回書き出されます。ページの見た目は変わらず、`<style>` が長くなるだけです。",
                },
                L {
                    en: "The reset sets `margin: 0` on every element, so a page's own `<h1>` or `<p>` loses the browser's spacing and takes its spacing from its own styles.",
                    ja: "リセットはすべての要素に `margin: 0` を設定します。そのため、ページ自身の `<h1>` や `<p>` もブラウザ既定の余白を失うので、余白は自分のスタイルで付けます。",
                },
            ],
        ),
    ])
}

fn built(site: Site) -> Part {
    parts([
        heading(
            site,
            L {
                en: "How the components are built",
                ja: "コンポーネントの作り",
            },
        ),
        bullets(
            site,
            &[
                L {
                    en: "Each component is a function or a small builder that returns an `El<S>`, so it takes handlers, styles, and children like any other element. A builder such as `TextField` turns into an `El<S>` with `render()`, and `El::child` takes it without the call.",
                    ja: "どのコンポーネントも `El<S>` を返す関数か小さなビルダーなので、ほかの要素と同じようにハンドラ、スタイル、子要素を受け取ります。`TextField` のようなビルダーは `render()` で `El<S>` になり、`El::child` には `render()` を呼ばずにそのまま渡せます。",
                },
                L {
                    en: "The behavior a screen reader and a keyboard depend on comes from the browser where the browser has it: a native `<button>`, `<input>`, `<select>`, `<details>`, `<dialog>`, or `popover`. Where it does not, the component follows the WAI-ARIA Authoring Practices, with keyboard handling from `El::roving_focus` and `El::active_descendant`.",
                    ja: "スクリーンリーダーやキーボードでの操作に必要な振る舞いは、ブラウザに備わっていればブラウザのものを使います。ネイティブの `<button>`、`<input>`、`<select>`、`<details>`、`<dialog>`、`popover` です。ブラウザに備わっていない振る舞いは、コンポーネントが WAI-ARIA Authoring Practices に従って作り、キーボード操作は `El::roving_focus` と `El::active_descendant` で扱います。",
                },
                L {
                    en: "A component bound to a signal ties its label to its control through ids derived from the signal's path. A dialog, a menu, a popover, a tooltip, and a combobox take their names from a [handle](components/handles/) instead.",
                    ja: "シグナルに結び付くコンポーネントは、シグナルのパスから作った id でラベルとコントロールを結び付けます。ダイアログ、メニュー、ポップオーバー、ツールチップ、コンボボックスは、代わりに[ハンドル](components/handles/)から名前を受け取ります。",
                },
                L {
                    en: "The colors follow shadcn/ui's zinc theme, with three values changed to meet WCAG 2.2 AA, as [Tokens](components/tokens/) lists.",
                    ja: "色は shadcn/ui の zinc テーマに合わせ、WCAG 2.2 AA を満たすために 3 つの値を変えています。その一覧は[トークン](components/tokens/)にあります。",
                },
            ],
        ),
        para(
            site,
            L {
                en: "The demos on these pages run in frames on this static site, so they use signals alone. The [`ui`](https://github.com/imaimai17468/hypersig/tree/main/examples/ui) example in the repository puts every component on one page with a server, and its end-to-end test drives them with the keyboard.",
                ja: "各ページのデモはこの静的サイトの中のフレームで動くので、シグナルだけを使います。リポジトリにある例 [`ui`](https://github.com/imaimai17468/hypersig/tree/main/examples/ui) は全コンポーネントをサーバー付きで 1 ページに並べたもので、その E2E テストはキーボードでコンポーネントを操作します。",
            },
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
                en: "[Theming](components/theming/) switches the components between light and dark. [Tokens](components/tokens/) changes their colors and corner radius. [Button](components/button/) starts the pages on each component.",
                ja: "[テーマ](components/theming/)ではコンポーネントのライトとダークを切り替えます。[トークン](components/tokens/)では色と角の丸みを変えます。各コンポーネントのページは[ボタン](components/button/)から始まります。",
            },
        ),
    ])
}
