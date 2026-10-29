//! The landing pages of the Style and Components docs, and the parts every
//! landing page shares.

use hypersig::html::{Id, a, div, h1, h2, h3, p, section, span};
use hypersig::style::pattern::{container, stack, wrap};
use hypersig::style::value::{
    AlignItems, BorderStyle, Display, FontWeight, JustifyContent, LineHeight, TextAlign, TextWrap,
    WordBreak, px, raw, rem,
};
use hypersig::style::{Media, PseudoClass, Style};
use hypersig_ui::{ButtonSize, ButtonVariant, button_style, token};

use crate::demo::{self, Demo};
use crate::doc::{COMPONENTS, Doc, Library};
use crate::lang::{L, Text};
use crate::learn::snippet::{Snippet, show};
use crate::prose::{Part, href, inline, para};
use crate::{Site, layout, own, prose};

/// A landing page of `doc` holding `sections`, with the page footer.
pub fn page(site: Site, doc: Doc, sections: impl IntoIterator<Item = Part>) -> String {
    let body = div()
        .css(container(rem(64), rem(1)).padding_bottom(rem(3)))
        .children(sections)
        .child(layout::page_footer(site));
    layout::document(site, doc, body)
}

/// One button of a hero: its label, its target as `prose::href` reads it,
/// and its look.
pub type Action = (&'static str, &'static str, ButtonVariant);

/// The top of a landing page: the crate's name above a heading, a line of
/// text, and buttons.
pub fn hero(site: Site, eyebrow: &str, heading: &str, text: &str, actions: &[Action]) -> Part {
    section()
        .css(
            stack(rem(1.5))
                .padding_top(rem(5))
                .padding_bottom(rem(3))
                .text_align(TextAlign::Center)
                .align_items(AlignItems::Center),
        )
        .child(
            p().css(
                Style::new()
                    .padding_block(rem(0.25))
                    .padding_inline(rem(0.75))
                    .border_width(px(1))
                    .border_style(BorderStyle::Solid)
                    .border_color(token::border().var())
                    .border_radius(px(999))
                    .font_size(rem(0.8125))
                    .color(token::muted_foreground().var()),
            )
            .children(inline(site, eyebrow)),
        )
        .child(
            h1().css(
                Style::new()
                    .max_width(rem(48))
                    .font_size(rem(2.5))
                    .line_height(LineHeight::Number(1.1))
                    .font_weight(FontWeight::Number(700.0))
                    .letter_spacing(rem(-0.05))
                    // Breaks a Japanese heading between phrases rather than
                    // inside a word, and evens out the lines. A browser
                    // without `auto-phrase` breaks it as usual.
                    .word_break(WordBreak::AutoPhrase)
                    .text_wrap(TextWrap::Balance)
                    .on(Media::Md, Style::new().font_size(rem(3.75))),
            )
            .text(heading),
        )
        .child(
            p().css(
                Style::new()
                    .max_width(rem(42))
                    .font_size(rem(1.125))
                    .line_height(LineHeight::Length(rem(1.75)))
                    .color(token::muted_foreground().var()),
            )
            .children(inline(site, text)),
        )
        .child(
            div()
                .css(wrap(rem(0.75)).justify_content(JustifyContent::Center))
                .children(actions.iter().map(|&(label, target, variant)| {
                    a().href(href(site, target))
                        .css(button_style(variant, ButtonSize::Lg))
                        .text(label)
                })),
        )
}

/// A row of cards, each a heading and a line of text.
pub fn features(site: Site, cards: &[(&str, &str)]) -> Part {
    div()
        .css(
            stack(rem(1)).on(
                Media::Md,
                Style::new()
                    .display(Display::Grid)
                    .grid_template_columns(raw("repeat(3, minmax(0, 1fr))")),
            ),
        )
        .children(cards.iter().map(|&(heading, text)| {
            div()
                .css(card_style())
                .child(
                    h3().css(Style::new().font_weight(FontWeight::Number(600.0)))
                        .text(heading),
                )
                .child(p().css(small_muted()).children(inline(site, text)))
        }))
}

/// A titled section of a landing page, whose id comes from the English heading.
pub fn titled<'a>(
    site: Site,
    heading: impl Into<Text<'a, str>>,
    children: impl IntoIterator<Item = Part>,
) -> Part {
    let heading = heading.into();
    section()
        .css(Style::new().margin_top(rem(5)))
        .child(
            h2().id(Id::from_text(prose::anchor(heading.english())))
                .css(
                    Style::new()
                        .font_size(rem(1.875))
                        .line_height(LineHeight::Length(rem(2.25)))
                        .font_weight(FontWeight::Number(700.0))
                        .letter_spacing(rem(-0.025)),
                )
                .text(heading.get(site.lang())),
        )
        .children(children)
}

/// A card that links to the landing page of `library`, for the landing
/// page of another one.
pub fn library_card(site: Site, library: Library, krate: &str, text: &str) -> Part {
    a().href(own(library.home().url(site)))
        .css(card_style().on(
            PseudoClass::Hover,
            Style::new().background_color(token::accent().var()),
        ))
        .child(
            div()
                .css(wrap(rem(0.5)).align_items(AlignItems::Baseline))
                .child(
                    h3().css(
                        Style::new()
                            .font_size(rem(1.25))
                            .font_weight(FontWeight::Number(600.0)),
                    )
                    .text(library.name(site)),
                )
                .child(prose::inline_code(krate)),
        )
        .child(p().css(small_muted()).children(inline(site, text)))
        .child(
            span()
                .css(
                    Style::new()
                        .font_size(rem(0.875))
                        .font_weight(FontWeight::Number(500.0)),
                )
                .text(site.t(L {
                    en: format!("Explore {} →", library.name(site)),
                    ja: format!("{}を見る →", library.name(site)),
                })),
        )
}

fn card_style() -> Style {
    stack(rem(0.5))
        .padding(rem(1.5))
        .border_width(px(1))
        .border_style(BorderStyle::Solid)
        .border_color(token::border().var())
        .border_radius(token::radius().var())
        .background_color(token::card().var())
}

fn small_muted() -> Style {
    Style::new()
        .font_size(rem(0.875))
        .line_height(LineHeight::Length(rem(1.5)))
        .color(token::muted_foreground().var())
}

/// The landing page of the Style docs.
pub fn style(site: Site) -> String {
    page(
        site,
        Doc::StyleHome,
        [
            hero(
                site,
                site.t(L { en: "`hypersig::style`, from the `hypersig_style` crate", ja: "`hypersig_style` クレートの `hypersig::style`" }),
                site.t(L { en: "Typed atomic CSS, written in Rust", ja: "Rust で書く、型付きの atomic CSS" }),
                site.t(L { en: "Every standard CSS property is a method, and every keyword a variant. A misspelled property or a value of the wrong type is a compile error, and each declaration becomes one class.", ja: "標準の CSS プロパティはすべてメソッドで、キーワードはすべてバリアントです。プロパティの打ち間違いや型の違う値はコンパイルエラーになり、宣言 1 つが 1 つのクラスになります。" }),
                &[
                    (site.t(L { en: "Get started", ja: "はじめる" }), "style/overview/", ButtonVariant::Default),
                    (
                        "API",
                        "api:hypersig_style/index.html",
                        ButtonVariant::Outline,
                    ),
                ],
            ),
            features(
                site,
                &[
                    (
                        site.t(L { en: "Generated from MDN", ja: "MDN のデータから生成" }),
                        site.t(L { en: "The properties, their keywords, the pseudo-classes, and the named colors come from MDN's CSS data, so the methods follow the standard.", ja: "プロパティ、そのキーワード、擬似クラス、名前付きの色は MDN の CSS データから作るので、メソッドは標準に沿います。" }),
                    ),
                    (
                        site.t(L { en: "One class per declaration", ja: "宣言 1 つにクラス 1 つ" }),
                        site.t(L { en: "As in Panda CSS and Tailwind CSS, each declaration is one class, and the page writes only the rules its elements use.", ja: "Panda CSS や Tailwind CSS と同じく、宣言 1 つが 1 つのクラスになり、ページは要素が使うルールだけを書き出します。" }),
                    ),
                    (
                        site.t(L { en: "Changes with the signals", ja: "シグナルに合わせて変わる" }),
                        site.t(L { en: "`css_when` and `css_match` switch styles with the page's signals, and a patch includes the rules of the elements it adds.", ja: "`css_when` と `css_match` はページのシグナルに合わせてスタイルを切り替えます。差分には、追加する要素のルールも含まれます。" }),
                    ),
                ],
            ),
            titled(
                site,
                L { en: "A token from a signal", ja: "シグナルから決まるトークン" },
                [
                    para(
                        site,
                        L { en: "The swatches read one token, and `bind_token` sets it from the radio group's signal. [Tokens](style/tokens/) explains it.", ja: "色見本は 1 つのトークンを読み、`bind_token` がラジオグループのシグナルからそのトークンを設定します。詳しくは [トークン](style/tokens/) で説明します。" },
                    ),
                    demo::show(site, Demo::Accent),
                ],
            ),
            titled(site, L { en: "Benchmarks", ja: "ベンチマーク" }, crate::style::benchmarks(site)),
            titled(
                site,
                L { en: "Part of hypersig", ja: "hypersig の一部" },
                [
                    para(
                        site,
                        L { en: "The crate is reached as `hypersig::style`, and an `El` takes a `Style` with `css`. The components of `hypersig_ui` are styled with it.", ja: "このクレートは `hypersig::style` から使い、`El` は `css` で `Style` を受け取ります。`hypersig_ui` のコンポーネントもこれでスタイルを付けています。" },
                    ),
                    two_cards(
                        site,
                        [
                            (
                                Library::Framework,
                                "hypersig",
                                site.t(L { en: "The framework itself. The page, its behavior, and the server are all written in Rust.", ja: "フレームワーク本体です。画面もその動きもサーバーも、すべて Rust で書きます。" }),
                            ),
                            (
                                Library::Components,
                                "hypersig_ui",
                                site.t(L { en: "Accessible components styled after shadcn/ui, built with these styles.", ja: "shadcn/ui の見た目を持つアクセシブルなコンポーネントです。このスタイルで作っています。" }),
                            ),
                        ],
                    ),
                ],
            ),
        ],
    )
}

/// The landing page of the i18n docs.
pub fn i18n(site: Site) -> String {
    page(
        site,
        Doc::I18nHome,
        [
            hero(
                site,
                site.t(L {
                    en: "The `hypersig_i18n` crate",
                    ja: "`hypersig_i18n` クレート",
                }),
                site.t(L {
                    en: "Translations the compiler checks",
                    ja: "コンパイラが検査する翻訳",
                }),
                site.t(L {
                    en: "Each text holds its translations in one value, so a text without one of them fails to compile. The language is part of each page's route, and `/` redirects to the page in the language the browser asks for.",
                    ja: "1 つの文はその訳を 1 つの値にまとめて持つので、どれかの訳が欠けた文はコンパイルエラーになります。言語は各ページのルートの一部で、`/` はブラウザが求める言語のページへリダイレクトします。",
                }),
                &[
                    (
                        site.t(L {
                            en: "Get started",
                            ja: "はじめる",
                        }),
                        "i18n/overview/",
                        ButtonVariant::Default,
                    ),
                    ("API", "api:hypersig_i18n/index.html", ButtonVariant::Outline),
                ],
            ),
            features(
                site,
                &[
                    (
                        site.t(L {
                            en: "A missing translation fails to compile",
                            ja: "訳の欠けはコンパイルエラー",
                        }),
                        site.t(L {
                            en: "A text is a struct with one field per language, so a text without its Japanese, or a language added without its texts, is a compile error.",
                            ja: "文は言語ごとに 1 つのフィールドを持つ構造体なので、日本語の訳がない文や、文を書かずに追加した言語はコンパイルエラーになります。",
                        }),
                    ),
                    (
                        site.t(L {
                            en: "The language is in the route",
                            ja: "言語はルートの一部",
                        }),
                        site.t(L {
                            en: "A locale is the first segment of a page's route, so the link to the page in another language is checked like any other link.",
                            ja: "言語はページのルートの最初のセグメントなので、別の言語のページへのリンクも、ほかのリンクと同じく検査されます。",
                        }),
                    ),
                    (
                        site.t(L {
                            en: "No macro",
                            ja: "マクロを使わない",
                        }),
                        site.t(L {
                            en: "The texts are plain struct literals and the languages a plain enum, so the compiler's own checks find what is missing.",
                            ja: "文はただの構造体のリテラルで、言語はただの enum です。欠けているものは、コンパイラの通常の検査が見つけます。",
                        }),
                    ),
                ],
            ),
            titled(
                site,
                L {
                    en: "A text without its translation",
                    ja: "訳のない文",
                },
                [
                    para(
                        site,
                        L {
                            en: "A text without its Japanese is a compile error at that line. [Overview](i18n/overview/) shows how the languages are declared.",
                            ja: "文の日本語を書き忘れると、その行がコンパイルエラーになります。言語の宣言のしかたは [概要](i18n/overview/) で説明します。",
                        },
                    ),
                    show(Snippet::I18nMissing),
                ],
            ),
            titled(
                site,
                L {
                    en: "Part of hypersig",
                    ja: "hypersig の一部",
                },
                [
                    para(
                        site,
                        L {
                            en: "This site is translated with it: every page is rendered once per language from the same Rust function.",
                            ja: "このサイトもこのクレートで翻訳しています。どのページも同じ Rust の関数から言語ごとに 1 回ずつレンダリングしています。",
                        },
                    ),
                    two_cards(
                        site,
                        [
                            (
                                Library::Framework,
                                "hypersig",
                                site.t(L {
                                    en: "The framework that provides the routes and pages.",
                                    ja: "ルートとページを提供するフレームワーク本体です。",
                                }),
                            ),
                            (
                                Library::Components,
                                "hypersig_ui",
                                site.t(L {
                                    en: "Accessible components, whose labels the pages translate.",
                                    ja: "アクセシブルなコンポーネントです。ラベルはページの側で訳します。",
                                }),
                            ),
                        ],
                    ),
                ],
            ),
        ],
    )
}

/// The landing page of the Components docs.
pub fn components(site: Site) -> String {
    page(
        site,
        Doc::ComponentsHome,
        [
            hero(
                site,
                site.t(L { en: "The `hypersig_ui` crate", ja: "`hypersig_ui` クレート" }),
                site.t(L { en: "Accessible components, styled after shadcn/ui", ja: "shadcn/ui の見た目を持つ、アクセシブルなコンポーネント" }),
                site.t(L { en: "Buttons, fields, pickers, tabs, dialogs, menus, tooltips, and toasts, built on the browser's native elements and following the WAI-ARIA Authoring Practices. Each returns an `El`, so it takes handlers and children like any other element.", ja: "ボタン、入力欄、選択、タブ、ダイアログ、メニュー、ツールチップ、トーストを、ブラウザのネイティブ要素を使い、WAI-ARIA Authoring Practices に沿って作っています。どれも `El` を返すので、ほかの要素と同じようにハンドラや子要素を追加できます。" }),
                &[
                    (
                        site.t(L { en: "Get started", ja: "はじめる" }),
                        "components/installation/",
                        ButtonVariant::Default,
                    ),
                    ("API", "api:hypersig_ui/index.html", ButtonVariant::Outline),
                ],
            ),
            features(
                site,
                &[
                    (
                        site.t(L { en: "Native elements first", ja: "ネイティブ要素を先に使う" }),
                        site.t(L { en: "The browser already gives `<button>`, `<select>`, `<details>`, `<dialog>`, and `popover` their behavior, and the components use it as it is.", ja: "`<button>`、`<select>`、`<details>`、`<dialog>`、`popover` にはブラウザがもともと持つ振る舞いがあり、コンポーネントはそれをそのまま使います。" }),
                    ),
                    (
                        site.t(L { en: "WAI-ARIA keyboard interaction", ja: "WAI-ARIA のキーボード操作" }),
                        site.t(L { en: "Tabs, listbox options, and menu items move with the arrow keys, Home, and End, and a combobox keeps focus in its text field.", ja: "タブ、リストボックスの選択肢、メニューの項目の間は、矢印キー、Home、End で移ります。コンボボックスはフォーカスをテキスト欄に置いたままにします。" }),
                    ),
                    (
                        site.t(L { en: "Contrast that meets WCAG 2.2 AA", ja: "WCAG 2.2 AA を満たすコントラスト" }),
                        site.t(L { en: "The colors start from shadcn/ui's zinc theme, with a control's border at 3:1, secondary text at 4.5:1, and focus drawn as a solid outline.", ja: "shadcn/ui の zinc テーマをもとに、コントロールの境界線を 3:1、補足のテキストを 4.5:1 にし、フォーカスを単色の outline で描きます。" }),
                    ),
                ],
            ),
            titled(
                site,
                L { en: "A dialog in a few lines", ja: "数行で書けるダイアログ" },
                [
                    para(
                        site,
                        L { en: "A `const` handle connects the dialog with the buttons that open and close it, and the browser opens it as a modal through invoker commands.", ja: "`const` のハンドルで、ダイアログとそれを開閉するボタンを対応付けます。ブラウザは invoker commands を通して、ダイアログをモーダルとして開きます。" },
                    ),
                    demo::show(site, Demo::Dialog),
                ],
            ),
            titled(site, L { en: "Every component", ja: "すべてのコンポーネント" }, [catalog(site)]),
            titled(
                site,
                L { en: "Part of hypersig", ja: "hypersig の一部" },
                [two_cards(
                    site,
                    [
                        (
                            Library::Framework,
                            "hypersig",
                            site.t(L { en: "The framework the components run on: signals, statements, and actions.", ja: "コンポーネントが動くフレームワーク本体です。シグナル、文、アクションを持ちます。" }),
                        ),
                        (
                            Library::Style,
                            "hypersig_style",
                            site.t(L { en: "The typed styles the components are built with, which can also change how they look.", ja: "コンポーネントを作っている型付きのスタイルです。コンポーネントの見た目を変えるのにも使えます。" }),
                        ),
                    ],
                )],
            ),
        ],
    )
}

/// The component pages, grouped as the sidebar groups them.
fn catalog(site: Site) -> Part {
    div()
        .css(
            stack(rem(1.5)).margin_top(rem(1.5)).on(
                Media::Md,
                Style::new()
                    .display(Display::Grid)
                    .grid_template_columns(raw("repeat(3, minmax(0, 1fr))")),
            ),
        )
        .children(
            COMPONENTS
                .iter()
                .filter(|group| group.title.en != "Get started")
                .map(|group| {
                    div()
                        .css(stack(rem(0.5)))
                        .child(
                            h3().css(Style::new().font_weight(FontWeight::Number(600.0)))
                                .text(site.t(group.title)),
                        )
                        .children(group.docs.iter().map(|&doc| {
                            a().href(own(doc.url(site)))
                                .css(Style::new().color(token::muted_foreground().var()).on(
                                    PseudoClass::Hover,
                                    Style::new().color(token::foreground().var()),
                                ))
                                .text(doc.title(site))
                        }))
                }),
        )
}

/// Cards for two libraries, side by side, each with its crate's name and a
/// line of text.
pub fn two_cards(site: Site, cards: [(Library, &str, &str); 2]) -> Part {
    library_cards(site, cards)
}

/// Cards that link to the landing pages of other libraries, `N` to a row on
/// a wide screen.
pub fn library_cards<const N: usize>(site: Site, cards: [(Library, &str, &str); N]) -> Part {
    div()
        .css(
            stack(rem(1)).margin_top(rem(1.5)).on(
                Media::Md,
                Style::new()
                    .display(Display::Grid)
                    .grid_template_columns(raw(match N {
                        3 => "repeat(3, minmax(0, 1fr))",
                        _ => "repeat(2, minmax(0, 1fr))",
                    })),
            ),
        )
        .children(
            cards
                .into_iter()
                .map(|(library, krate, text)| library_card(site, library, krate, text)),
        )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_catalog_lists_every_component_page() {
        let html = catalog(Site::new("")).render();
        for group in COMPONENTS
            .iter()
            .filter(|group| group.title.en != "Get started")
        {
            for doc in group.docs {
                assert!(html.contains(&doc.url(Site::new(""))), "{doc:?}");
            }
        }
    }
}
