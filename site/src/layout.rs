//! The frame around every page: the header, the sidebar, and the links to
//! the previous and the next page.

use hypersig::html::{a, attr, div, el, footer, header, img, main, nav, p, span};
use hypersig::style::pattern::{container, hstack, stack};
use hypersig::style::value::{
    AlignItems, BorderBottomStyle, BorderStyle, BorderTopStyle, Color, Display, FlexWrap,
    FontWeight, JustifyContent, LineHeight, OverflowY, Position, TextAlign, TextDecorationLine,
    WhiteSpace, ZIndex, px, raw, rem,
};
use hypersig::style::{Element, Global, Media, PseudoClass, Style};
use hypersig::{OpenGraph, OpenGraphType, Page, SafeUrl, if_else};
use hypersig_ui::{ButtonVariant, icon_button, token};

use crate::doc::{Doc, Library};
use crate::lang::{L, Lang};
use crate::prose::{self, Docs, Part, href};
use crate::{Site, code, own};
use hypersig_i18n::Locale as _;

/// The repository the header links to.
pub const REPOSITORY: SafeUrl = SafeUrl::new("https://github.com/imaimai17468/hypersig");

/// The id of the sidebar, which the menu button controls.
const SIDEBAR: &str = "sidebar";

/// A page of `doc` with the header and `body` below it, full width.
pub fn document(site: Site, doc: Doc, body: Part) -> String {
    let title = if doc == Doc::Home {
        // The title search results show for the site, so it says what
        // hypersig is. The other pages end in the bare name.
        format!(
            "{} | {}",
            doc.title(site),
            site.t(L {
                en: "A Rust web framework that type-checks the page and the server",
                ja: "画面とサーバーを型で検査する Rust の Web フレームワーク",
            })
        )
    } else if doc.is_landing() {
        doc.title(site).to_owned()
    } else {
        format!("{} | {}", doc.title(site), doc.library().home().title(site))
    };
    let mut page = Page::new(title, Docs::default())
        .lang(site.lang().tag())
        .description(prose::plain(site.t(doc.leads())));
    let mut card = OpenGraph::new()
        .site_name("hypersig")
        .kind(if doc.is_landing() {
            OpenGraphType::Website
        } else {
            OpenGraphType::Article
        })
        .locale(site.lang().og_locale());
    if let Some((origin, url)) = site.canonical(&doc.path()) {
        page = page.canonical(origin, &url);
        card = card.image(
            origin,
            format!("/{}", crate::og_image(site.lang()).0),
            site.t(L {
                en: "hypersig, a web framework written entirely in Rust: server-owned state, type-checked strings",
                ja: "hypersig、Rust だけで書く Web フレームワーク。サーバーが状態を、型が正しさを。",
            }),
        );
        for &lang in Lang::ALL {
            let (origin, url) = site
                .with_lang(lang)
                .canonical(&doc.path())
                .expect("every language of a site shares its origin");
            page = page.alternate(origin, &url, lang.tag());
            if lang == Lang::En {
                page = page.alternate_default(origin, &url);
            }
        }
    }
    for &lang in Lang::ALL.iter().filter(|&&lang| lang != site.lang()) {
        card = card.alternate_locale(lang.og_locale());
    }
    page.open_graph(card)
        .runtime_src(site.runtime())
        .icon(site.icon())
        .global(hypersig_ui::global())
        .global(
            Global::new().rule(
                Element::A,
                Style::new()
                    .color(Color::CURRENT_COLOR)
                    .text_decoration_line(TextDecorationLine::None),
            ),
        )
        .theme(hypersig_ui::theme())
        .theme(code::theme())
        .child(top_bar(site, doc))
        .child(body)
        .render()
}

/// A page of `doc` with the sidebar of its section and `content` as an
/// article beside it.
pub fn article(site: Site, doc: Doc, content: Part) -> String {
    let (before, after) = doc.neighbors();
    let body = div()
        .css(
            container(rem(88), rem(1)).on(
                Media::Md,
                Style::new()
                    .display(Display::Grid)
                    .grid_template_columns(raw("15rem minmax(0, 1fr)"))
                    .column_gap(rem(3)),
            ),
        )
        .child(sidebar(site, doc))
        .child(
            main()
                .css(
                    Style::new()
                        .padding_top(rem(2.5))
                        .padding_bottom(rem(4))
                        .max_width(rem(46))
                        .min_width(px(0)),
                )
                .child(el("article").child(content))
                .child(pager(site, before, after))
                .child(page_footer(site)),
        );
    document(site, doc, body)
}

fn top_bar(site: Site, doc: Doc) -> Part {
    let library = doc.library();
    header()
        .css(
            Style::new()
                .position(Position::Sticky)
                .top(px(0))
                .z_index(ZIndex::Integer(40))
                .border_bottom_width(px(1))
                .border_bottom_style(BorderBottomStyle::Solid)
                .border_bottom_color(token::border().var())
                .background_color(token::background().var()),
        )
        .child(
            div()
                .css(
                    container(rem(88), rem(1))
                        .display(Display::Flex)
                        .align_items(AlignItems::Center)
                        .gap(rem(1))
                        .height(rem(3.5))
                        .on(Media::Md, Style::new().gap(rem(1.5))),
                )
                .child(menu_button(site))
                .child(brand(site, library))
                .child(
                    nav()
                        .attr(
                            attr::ARIA_LABEL,
                            site.t(L {
                                en: "Sections",
                                ja: "セクション",
                            }),
                        )
                        .css(
                            hstack(rem(1.25))
                                .font_size(rem(0.875))
                                .white_space(WhiteSpace::Nowrap)
                                // On a phone, the menu lists the sections
                                // instead.
                                .display(Display::None)
                                .on(Media::Md, Style::new().display(Display::Flex)),
                        )
                        .children(library.sections().iter().map(|&(label, target)| {
                            let current = Place::current_if(in_section(doc, target));
                            mark_section(a().href(own(target.url(site))), current)
                                .css(nav_link_style(current))
                                .text(site.t(label))
                        }))
                        .child(
                            a().href(href(site, library.api()))
                                .css(nav_link_style(Place::Other).merge(wide_only()))
                                .text("API"),
                        ),
                )
                .child(
                    div()
                        .css(
                            hstack(rem(1.25)).margin_left(raw("auto")).merge(
                                wide_only().on(Media::Md, Style::new().display(Display::Flex)),
                            ),
                        )
                        .child(switcher(site, library))
                        .child(languages(site, doc))
                        .child(
                            a().href(REPOSITORY)
                                .css(nav_link_style(Place::Other).font_size(rem(0.875)))
                                .text("GitHub"),
                        ),
                )
                .child(other_language(site, doc)),
        )
        .children((!doc.has_sidebar()).then(|| menu(site, doc)))
}

/// The menu below the header on a phone, for a page without a sidebar to
/// hold the sections.
fn menu(site: Site, doc: Doc) -> Part {
    nav()
        .id(SIDEBAR)
        .attr(
            attr::ARIA_LABEL,
            site.t(L {
                en: "Pages",
                ja: "ページ",
            }),
        )
        .css(
            container(rem(88), rem(1))
                .display(Display::None)
                .padding_block(rem(1)),
        )
        // The menu button that opens it is hidden from the medium
        // breakpoint up.
        .css_when(Style::new().display(Display::Block), Docs::menu())
        .child(phone_sections(site, doc))
}

/// A link to `doc` in the other language, at the right of the header on a
/// phone, where the group with the language links is hidden.
fn other_language(site: Site, doc: Doc) -> Part {
    let other = if site.lang() == Lang::En {
        Lang::Ja
    } else {
        Lang::En
    };
    a().href(own(doc.url(site.with_lang(other))))
        .attr(attr::HREFLANG, other.tag())
        .attr(attr::LANG, other.tag())
        .css(
            nav_link_style(Place::Other)
                .margin_left(raw("auto"))
                .font_size(rem(0.875))
                .white_space(WhiteSpace::Nowrap)
                .on(Media::Md, Style::new().display(Display::None)),
        )
        .text(other.name())
}

/// The name in the corner of the header: the icon and hypersig, followed by
/// the library for the Style and Components docs, each linking to its
/// landing page.
fn brand(site: Site, library: Library) -> Part {
    let name = Style::new()
        .font_weight(FontWeight::Number(700.0))
        .letter_spacing(rem(-0.025))
        .white_space(WhiteSpace::Nowrap);
    div()
        .css(hstack(rem(0.5)))
        .child(
            a().href(own(Doc::Home.url(site)))
                .css(name.clone().merge(hstack(rem(0.5))))
                .child(
                    img()
                        .src(own(site.icon()))
                        .attr(attr::ALT, "")
                        .attr(attr::WIDTH, "24")
                        .attr(attr::HEIGHT, "24"),
                )
                .text("hypersig"),
        )
        .children((library != Library::Framework).then(|| {
            span()
                .attr(attr::ARIA_HIDDEN, "true")
                .css(Style::new().color(token::muted_foreground().var()))
                .text("/")
        }))
        .children((library != Library::Framework).then(|| {
            a().href(own(library.home().url(site)))
                .css(name.font_weight(FontWeight::Number(500.0)))
                .text(library.name(site))
        }))
}

/// Links to the three libraries, the current one marked, in a bordered
/// group.
fn switcher(site: Site, current: Library) -> Part {
    nav()
        .attr(
            attr::ARIA_LABEL,
            site.t(L {
                en: "Libraries",
                ja: "ライブラリ",
            }),
        )
        .css(
            hstack(px(2))
                .padding(px(2))
                .border_width(px(1))
                .border_style(BorderStyle::Solid)
                .border_color(token::border().var())
                .border_radius(px(8)),
        )
        .children(Library::ALL.into_iter().map(|library| {
            let here = Place::current_if(library == current);
            let link = a().href(own(library.home().url(site))).css(
                Style::new()
                    .padding_block(rem(0.25))
                    .padding_inline(rem(0.625))
                    .border_radius(px(6))
                    .font_size(rem(0.8125))
                    .merge(nav_link_style(here))
                    .merge(if here == Place::Current {
                        Style::new().background_color(token::accent().var())
                    } else {
                        Style::new()
                    }),
            );
            mark_section(link, here).text(library.name(site))
        }))
}

/// Links to `doc` in each language, the current one marked, for the header.
fn languages(site: Site, doc: Doc) -> Part {
    nav()
        .attr(
            attr::ARIA_LABEL,
            site.t(L {
                en: "Languages",
                ja: "言語",
            }),
        )
        .css(hstack(rem(0.75)).font_size(rem(0.875)))
        .children(Lang::ALL.iter().map(|&lang| {
            let here = Place::current_if(lang == site.lang());
            let link = a()
                .href(own(doc.url(site.with_lang(lang))))
                .attr(attr::HREFLANG, lang.tag())
                .attr(attr::LANG, lang.tag())
                .css(nav_link_style(here));
            mark_current(link, here).text(lang.name())
        }))
}

/// Whether `doc` belongs to the section whose first page is `first`.
fn in_section(doc: Doc, first: Doc) -> bool {
    matches!(
        (doc, first),
        (Doc::Learn(_), Doc::Learn(_))
            | (Doc::UseCases, Doc::UseCases)
            | (Doc::Reference(_), Doc::Reference(_))
            | (Doc::Style(_), Doc::Style(_))
            | (Doc::Components(_), Doc::Components(_))
            | (Doc::I18n(_), Doc::I18n(_))
    )
}

/// Whether a link leads to where the reader is: the page, its section, its
/// library, or its language.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Place {
    Current,
    Other,
}

impl Place {
    /// [`Place::Current`] where `here` is true.
    fn current_if(here: bool) -> Self {
        if here { Self::Current } else { Self::Other }
    }
}

/// `link` marked as the current page where it leads to the current place.
fn mark_current(link: Part, place: Place) -> Part {
    match place {
        Place::Current => link.attr(attr::ARIA_CURRENT, "page"),
        Place::Other => link,
    }
}

/// `link` to a section, marked as the section the page is in where it leads
/// to the current place. The page itself is marked in the sidebar.
fn mark_section(link: Part, place: Place) -> Part {
    match place {
        Place::Current => link.attr(attr::ARIA_CURRENT, "true"),
        Place::Other => link,
    }
}

/// Hidden below the medium breakpoint, where the header has no room. The
/// sidebar and the page footer link to the same places.
fn wide_only() -> Style {
    Style::new()
        .display(Display::None)
        .on(Media::Md, Style::new().display(Display::Inline))
}

fn nav_link_style(place: Place) -> Style {
    let style = Style::new().on(
        PseudoClass::Hover,
        Style::new().color(token::foreground().var()),
    );
    match place {
        Place::Current => style
            .color(token::foreground().var())
            .font_weight(FontWeight::Number(500.0)),
        Place::Other => style.color(token::muted_foreground().var()),
    }
}

fn menu_button(site: Site) -> Part {
    let menu = Docs::menu();
    icon_button(
        ButtonVariant::Ghost,
        site.t(L {
            en: "Menu",
            ja: "メニュー",
        }),
    )
    .attr(attr::ARIA_CONTROLS, SIDEBAR)
    .bind_attr(attr::ARIA_EXPANDED, if_else(menu, "true", "false"))
    .on_click(menu.toggle())
    .css(
        Style::new()
            .margin_left(rem(-0.5))
            .on(Media::Md, Style::new().display(Display::None)),
    )
    .child(
        el("svg")
            .attr(attr::VIEW_BOX, "0 0 24 24")
            .attr(attr::FILL, "none")
            .attr(attr::STROKE, "currentColor")
            .attr(attr::STROKE_WIDTH, "2")
            .attr(attr::STROKE_LINECAP, "round")
            .attr(attr::ARIA_HIDDEN, "true")
            .css(Style::new().width(rem(1.25)).height(rem(1.25)))
            .children(
                ["M4 6h16", "M4 12h16", "M4 18h16"]
                    .into_iter()
                    .map(|d| el("path").attr(attr::D, d)),
            ),
    )
}

fn sidebar(site: Site, doc: Doc) -> Part {
    nav()
        .id(SIDEBAR)
        .attr(
            attr::ARIA_LABEL,
            site.t(L {
                en: "Pages",
                ja: "ページ",
            }),
        )
        .css(
            Style::new()
                .display(Display::None)
                .padding_block(rem(1.5))
                .border_bottom_width(px(1))
                .border_bottom_style(BorderBottomStyle::Solid)
                .border_bottom_color(token::border().var())
                .on(
                    Media::Md,
                    Style::new()
                        .display(Display::Block)
                        .position(Position::Sticky)
                        .top(rem(3.5))
                        .height(raw("calc(100vh - 3.5rem)"))
                        .overflow_y(OverflowY::Auto)
                        .padding_top(rem(2.5))
                        .border_bottom_width(px(0)),
                ),
        )
        .css_when(Style::new().display(Display::Block), Docs::menu())
        .child(
            div()
                .css(stack(rem(1.5)))
                .child(
                    div()
                        .css(
                            Style::new()
                                .padding_bottom(rem(1))
                                .border_bottom_width(px(1))
                                .border_bottom_style(BorderBottomStyle::Solid)
                                .border_bottom_color(token::border().var())
                                .on(Media::Md, Style::new().display(Display::None)),
                        )
                        .child(phone_sections(site, doc)),
                )
                .children(doc.groups().iter().map(|group| {
                    div()
                        .child(
                            p().css(
                                Style::new()
                                    .padding_inline(rem(0.5))
                                    .padding_bottom(rem(0.25))
                                    .font_size(rem(0.875))
                                    .font_weight(FontWeight::Number(600.0)),
                            )
                            .text(site.t(group.title)),
                        )
                        .child(
                            div()
                                .css(stack(px(2)))
                                .children(group.docs.iter().map(|&target| {
                                    let current = Place::current_if(target == doc);
                                    mark_current(a().href(own(target.url(site))), current)
                                        .css(sidebar_link_style(current))
                                        .text(target.title(site))
                                })),
                        )
                })),
        )
}

/// The sections, the libraries, and the outside links, in the menu on a
/// phone, where the header has no room for them.
fn phone_sections(site: Site, doc: Doc) -> Part {
    let library = doc.library();
    div()
        .css(stack(px(2)))
        .children(library.sections().iter().map(|&(label, target)| {
            let current = Place::current_if(in_section(doc, target));
            mark_section(a().href(own(target.url(site))), current)
                .css(sidebar_link_style(current))
                .text(site.t(label))
        }))
        .child(
            a().href(href(site, library.api()))
                .css(sidebar_link_style(Place::Other))
                .text("API"),
        )
        .children(
            Library::ALL
                .into_iter()
                .filter(|&other| other != library)
                .map(|other| {
                    a().href(own(other.home().url(site)))
                        .css(sidebar_link_style(Place::Other))
                        .text(other.path())
                }),
        )
        .child(
            a().href(REPOSITORY)
                .css(sidebar_link_style(Place::Other))
                .text("GitHub"),
        )
}

fn sidebar_link_style(place: Place) -> Style {
    let style = Style::new()
        .display(Display::Block)
        .padding_block(rem(0.375))
        .padding_inline(rem(0.5))
        .border_radius(px(6))
        .font_size(rem(0.875))
        .line_height(LineHeight::Length(rem(1.25)))
        .on(
            PseudoClass::Hover,
            Style::new()
                .background_color(token::accent().var())
                .color(token::accent_foreground().var()),
        );
    match place {
        Place::Current => style
            .background_color(token::accent().var())
            .color(token::accent_foreground().var())
            .font_weight(FontWeight::Number(500.0)),
        Place::Other => style.color(token::muted_foreground().var()),
    }
}

/// The links to the pages before and after the current one.
fn pager(site: Site, before: Option<Doc>, after: Option<Doc>) -> Part {
    let card = |doc: Doc, label: &str, end: bool| {
        a().href(own(doc.url(site)))
            .css(
                stack(rem(0.25))
                    .padding(rem(1))
                    .border_width(px(1))
                    .border_style(BorderStyle::Solid)
                    .border_color(token::border().var())
                    .border_radius(token::radius().var())
                    .on(
                        PseudoClass::Hover,
                        Style::new().background_color(token::accent().var()),
                    )
                    .text_align(if end {
                        TextAlign::Right
                    } else {
                        TextAlign::Left
                    })
                    .grid_column(raw(if end { "2" } else { "1" })),
            )
            .child(
                span()
                    .css(
                        Style::new()
                            .font_size(rem(0.75))
                            .color(token::muted_foreground().var()),
                    )
                    .text(label),
            )
            .child(
                span()
                    .css(Style::new().font_weight(FontWeight::Number(500.0)))
                    .text(doc.title(site)),
            )
    };
    nav()
        .attr(
            attr::ARIA_LABEL,
            site.t(L {
                en: "Previous and next page",
                ja: "前後のページ",
            }),
        )
        .css(
            Style::new()
                .display(Display::Grid)
                .grid_template_columns(raw("repeat(2, minmax(0, 1fr))"))
                .gap(rem(1))
                .margin_top(rem(4)),
        )
        .children(before.map(|doc| {
            card(
                doc,
                site.t(L {
                    en: "Previous",
                    ja: "前へ",
                }),
                false,
            )
        }))
        .children(after.map(|doc| {
            card(
                doc,
                site.t(L {
                    en: "Next",
                    ja: "次へ",
                }),
                true,
            )
        }))
}

/// The line at the bottom of every page, with the links the header hides on
/// a phone.
pub fn page_footer(site: Site) -> Part {
    footer()
        .css(
            Style::new()
                .display(Display::Flex)
                .justify_content(JustifyContent::SpaceBetween)
                .flex_wrap(FlexWrap::Wrap)
                .gap(rem(0.5))
                .margin_top(rem(3))
                .padding_top(rem(1.5))
                .border_top_width(px(1))
                .border_top_style(BorderTopStyle::Solid)
                .border_top_color(token::border().var())
                .font_size(rem(0.875))
                .color(token::muted_foreground().var()),
        )
        .child(p().children(crate::prose::inline(
            site,
            site.t(L {
                en: "Built with hypersig and `hypersig_ui`, and served as static files.",
                ja: "hypersig と `hypersig_ui` で作り、静的ファイルとして配信しています。",
            }),
        )))
        .child(p().children(crate::prose::inline(
            site,
            site.t(L {
                en: "[Framework](), [Style](style/), [Components](components/), [i18n](i18n/), [GitHub](https://github.com/imaimai17468/hypersig), MIT License",
                ja: "[フレームワーク](), [スタイル](style/), [コンポーネント](components/), [多言語化](i18n/), [GitHub](https://github.com/imaimai17468/hypersig), MIT License",
            }),
        )))
}
