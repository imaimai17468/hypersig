//! The landing pages of the Style and Components docs, and the parts every
//! landing page shares.

use hypersig::html::{a, div, h1, h2, h3, p, section, span};
use hypersig::style::pattern::{container, stack, wrap};
use hypersig::style::value::{
    AlignItems, BorderStyle, FontWeight, JustifyContent, LineHeight, TextAlign, px, raw, rem,
};
use hypersig::style::{Media, PseudoClass, Style};
use hypersig_ui::{ButtonSize, ButtonVariant, button_style, token};

use crate::demo::{self, Demo};
use crate::doc::{COMPONENTS, Doc, Library};
use crate::lang::{L, Text};
use crate::prose::{Part, href, inline, para};
use crate::{Site, layout, prose};

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
                    a().attr("href", href(site, target))
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
                    .display(hypersig::style::value::Display::Grid)
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
            h2().id(prose::anchor(heading.english()))
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
    a().attr("href", library.home().url(site))
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
                        site.t(L { en: "Follows the signals", ja: "シグナルに合わせて変わる" }),
                        site.t(L { en: "`css_when` and `css_match` switch styles with the page's signals, and a patch brings the rules of the elements it adds.", ja: "`css_when` と `css_match` はページのシグナルに合わせてスタイルを切り替えます。差分は、足す要素のルールも一緒に運びます。" }),
                    ),
                ],
            ),
            titled(
                site,
                L { en: "A token from a signal", ja: "シグナルから決まるトークン" },
                [
                    para(
                        site,
                        L { en: "The swatches read one token, and `bind_token` sets it from the radio group's signal. [Tokens and themes](style/tokens/) explains it.", ja: "色見本は 1 つのトークンを読み、`bind_token` がラジオグループのシグナルからそのトークンを設定します。詳しくは [トークンとテーマ](style/tokens/) で説明します。" },
                    ),
                    demo::show(site, Demo::Accent),
                ],
            ),
            titled(
                site,
                L { en: "Part of hypersig", ja: "hypersig の一部" },
                [
                    para(
                        site,
                        L { en: "The crate is reached as `hypersig::style`, and an `El` takes a `Style` with `css`. The components of `hypersig_ui` are styled with it.", ja: "このクレートは `hypersig::style` から使い、`El` は `css` で `Style` を受け取ります。`hypersig_ui` のコンポーネントも、これでスタイルを付けています。" },
                    ),
                    two_cards(
                        site,
                        [
                            (
                                Library::Framework,
                                "hypersig",
                                site.t(L { en: "The framework: the page, its behavior, and the server, all in Rust.", ja: "フレームワーク本体です。画面もその動きもサーバーも、すべて Rust で書きます。" }),
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

/// The landing page of the Components docs.
pub fn components(site: Site) -> String {
    page(
        site,
        Doc::ComponentsHome,
        [
            hero(
                site,
                "The `hypersig_ui` crate",
                "Accessible components, styled after shadcn/ui",
                "Buttons, fields, pickers, tabs, dialogs, menus, tooltips, and toasts on the browser's native elements and the WAI-ARIA Authoring Practices. Each returns an `El`, so it takes handlers and children like any other element.",
                &[
                    (
                        "Get started",
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
                        "Native elements first",
                        "A `<button>`, a `<select>`, a `<details>`, a `<dialog>`, and `popover` carry the behavior the browser already has.",
                    ),
                    (
                        "The keyboard of WAI-ARIA",
                        "Tabs, listbox options, and menu items move with the arrow keys, Home, and End, and a combobox keeps focus in its text field.",
                    ),
                    (
                        "Contrast for WCAG 2.2 AA",
                        "shadcn/ui's zinc theme, with a control's border at 3:1, secondary text at 4.5:1, and focus drawn as a solid outline.",
                    ),
                ],
            ),
            titled(
                site,
                "A dialog in a few lines",
                [
                    para(
                        site,
                        "A `const` handle ties the dialog to the buttons that open and close it, and the browser opens it as a modal through invoker commands.",
                    ),
                    demo::show(site, Demo::Dialog),
                ],
            ),
            titled(site, "Every component", [catalog(site)]),
            titled(
                site,
                "Part of hypersig",
                [two_cards(
                    site,
                    [
                        (
                            Library::Framework,
                            "hypersig",
                            "The framework the components run on: signals, statements, and actions.",
                        ),
                        (
                            Library::Style,
                            "hypersig_style",
                            "The typed styles the components are built with, which restyle them too.",
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
                    .display(hypersig::style::value::Display::Grid)
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
                            a().attr("href", doc.url(site))
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
    div()
        .css(
            stack(rem(1)).margin_top(rem(1.5)).on(
                Media::Md,
                Style::new()
                    .display(hypersig::style::value::Display::Grid)
                    .grid_template_columns(raw("repeat(2, minmax(0, 1fr))")),
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
