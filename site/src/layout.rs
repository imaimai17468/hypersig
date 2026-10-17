//! The frame around every page: the header, the sidebar, and the links to
//! the previous and the next page.

use hypersig::html::{a, div, el, footer, header, main, nav, p, span};
use hypersig::style::pattern::{container, hstack, stack};
use hypersig::style::value::{
    AlignItems, BorderBottomStyle, BorderStyle, BorderTopStyle, Display, FontWeight,
    JustifyContent, LineHeight, OverflowY, Position, TextDecorationLine, ZIndex, px, raw, rem,
};
use hypersig::style::{Element, Global, Media, PseudoClass, Style};
use hypersig::{Page, if_else};
use hypersig_ui::{ButtonVariant, icon_button, token};

use crate::doc::{Doc, Learn, Reference};
use crate::prose::{Docs, Part, href};
use crate::{Site, code};

/// The repository the header links to.
pub const REPOSITORY: &str = "https://github.com/imaimai17468/hypersig";

/// The id of the sidebar, which the menu button controls.
const SIDEBAR: &str = "sidebar";

/// A page of `doc` with the header and `body` below it, full width.
pub fn document(site: Site, doc: Doc, body: Part) -> String {
    let title = match doc {
        Doc::Home => "hypersig".to_owned(),
        _ => format!("{} | hypersig", doc.title()),
    };
    Page::new(title, Docs::default())
        .runtime_src(site.runtime())
        .global(hypersig_ui::global())
        .global(
            Global::new().rule(
                Element::A,
                Style::new()
                    .color(hypersig::style::value::Color::CURRENT_COLOR)
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
    let section_link = |target: Doc, label: &str, current: bool| {
        mark_current(a().attr("href", target.url(site)), current)
            .css(nav_link_style(current))
            .text(label)
    };
    let in_learn = matches!(doc, Doc::Learn(_));
    let in_reference = matches!(doc, Doc::Reference(_));
    let has_sidebar = doc != Doc::Home;
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
                        .gap(rem(1.5))
                        .height(rem(3.5)),
                )
                .children(has_sidebar.then(menu_button))
                .child(
                    a().attr("href", Doc::Home.url(site))
                        .css(
                            Style::new()
                                .font_weight(FontWeight::Number(700.0))
                                .letter_spacing(rem(-0.025)),
                        )
                        .text("hypersig"),
                )
                .child(
                    nav()
                        .attr("aria-label", "Sections")
                        .css(hstack(rem(1.25)).font_size(rem(0.875)))
                        .child(section_link(
                            Doc::Learn(Learn::QuickStart),
                            "Learn",
                            in_learn,
                        ))
                        .child(section_link(
                            Doc::Reference(Reference::Overview),
                            "Reference",
                            in_reference,
                        ))
                        .child(
                            a().attr("href", href(site, "api:hypersig/index.html"))
                                .css(nav_link_style(false))
                                .text("API"),
                        ),
                )
                .child(
                    a().attr("href", REPOSITORY)
                        .css(
                            nav_link_style(false)
                                .margin_left(raw("auto"))
                                .font_size(rem(0.875)),
                        )
                        .text("GitHub"),
                ),
        )
}

/// `link` marked as the current page where `current` is true.
fn mark_current(link: Part, current: bool) -> Part {
    if current {
        link.attr("aria-current", "page")
    } else {
        link
    }
}

fn nav_link_style(current: bool) -> Style {
    let style = Style::new().on(
        PseudoClass::Hover,
        Style::new().color(token::foreground().var()),
    );
    if current {
        style
            .color(token::foreground().var())
            .font_weight(FontWeight::Number(500.0))
    } else {
        style.color(token::muted_foreground().var())
    }
}

fn menu_button() -> Part {
    let menu = Docs::menu();
    icon_button(ButtonVariant::Ghost, "Menu")
        .attr("aria-controls", SIDEBAR)
        .bind_attr("aria-expanded", if_else(menu, "true", "false"))
        .on_click(menu.toggle())
        .css(
            Style::new()
                .margin_left(rem(-0.5))
                .on(Media::Md, Style::new().display(Display::None)),
        )
        .child(
            el("svg")
                .attr("viewBox", "0 0 24 24")
                .attr("fill", "none")
                .attr("stroke", "currentColor")
                .attr("stroke-width", "2")
                .attr("stroke-linecap", "round")
                .attr("aria-hidden", "true")
                .css(Style::new().width(rem(1.25)).height(rem(1.25)))
                .children(
                    ["M4 6h16", "M4 12h16", "M4 18h16"]
                        .into_iter()
                        .map(|d| el("path").attr("d", d)),
                ),
        )
}

fn sidebar(site: Site, doc: Doc) -> Part {
    nav()
        .id(SIDEBAR)
        .attr("aria-label", "Pages")
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
                            .text(group.title),
                        )
                        .child(
                            div()
                                .css(stack(px(2)))
                                .children(group.docs.iter().map(|&target| {
                                    let current = target == doc;
                                    mark_current(a().attr("href", target.url(site)), current)
                                        .css(sidebar_link_style(current))
                                        .text(target.title())
                                })),
                        )
                })),
        )
}

fn sidebar_link_style(current: bool) -> Style {
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
    if current {
        style
            .background_color(token::accent().var())
            .color(token::accent_foreground().var())
            .font_weight(FontWeight::Number(500.0))
    } else {
        style.color(token::muted_foreground().var())
    }
}

/// The links to the pages before and after the current one.
fn pager(site: Site, before: Option<Doc>, after: Option<Doc>) -> Part {
    let card = |doc: Doc, label: &str, end: bool| {
        a().attr("href", doc.url(site))
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
                        hypersig::style::value::TextAlign::Right
                    } else {
                        hypersig::style::value::TextAlign::Left
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
                    .text(doc.title()),
            )
    };
    nav()
        .attr("aria-label", "Previous and next page")
        .css(
            Style::new()
                .display(Display::Grid)
                .grid_template_columns(raw("repeat(2, minmax(0, 1fr))"))
                .gap(rem(1))
                .margin_top(rem(4)),
        )
        .children(before.map(|doc| card(doc, "Previous", false)))
        .children(after.map(|doc| card(doc, "Next", true)))
}

fn page_footer(site: Site) -> Part {
    footer()
        .css(
            Style::new()
                .display(Display::Flex)
                .justify_content(JustifyContent::SpaceBetween)
                .flex_wrap(hypersig::style::value::FlexWrap::Wrap)
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
            "Built with hypersig and `hypersig_ui`, and served as static files.",
        )))
        .child(p().text("MIT License"))
}
