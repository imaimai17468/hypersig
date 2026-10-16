//! Typed styles: a theme a signal switches, a recipe, a responsive grid,
//! group and peer conditions, an animation, and cards whose rules arrive in
//! a patch.
//!
//! Run with `cargo run -p example-style` and open <http://localhost:3004>.

use axum::Router;
use axum::response::Html;
use axum::routing::get;
use hypersig::html::{El, button, div, h1, h2, input, label, p, section, span};
use hypersig::{Action, Fragment, Page, Patches, RouterExt, Signals, if_else};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Copy, PartialEq)]
enum Size {
    Small,
    Large,
}

#[derive(Serialize, Deserialize, Clone, Copy, PartialEq)]
enum Tone {
    Solid,
    Outline,
}

#[derive(Signals, Serialize, Deserialize)]
struct Gallery {
    dark: bool,
    size: Size,
    tone: Tone,
}

const MORE: Action<Gallery> = Action::post("/more");

/// Cards the server sends on request. Their widths use the `md` breakpoint,
/// which nothing on the first page uses, so their rules arrive in the patch.
struct More;

impl Fragment for More {
    type Signals = Gallery;
    const ID: &'static str = "more";

    fn render(&self) -> El<Gallery> {
        div().css(look::loaded_row()).children(
            ["Loaded", "From the", "Server"]
                .into_iter()
                .map(|title| div().css(look::loaded_card()).text(title)),
        )
    }
}

fn theme_section() -> El<Gallery> {
    let dark = Gallery::dark();
    section()
        .id("theme")
        .css(look::panel())
        .child(h2().text("Theme"))
        .child(p().text("The button switches the tokens this page reads its colors from."))
        .child(
            button()
                .id("switch-theme")
                .css(look::button())
                .css_match(
                    Gallery::size(),
                    [(Size::Small, look::small()), (Size::Large, look::large())],
                )
                .css_match(
                    Gallery::tone(),
                    [
                        (Tone::Solid, look::solid()),
                        (Tone::Outline, look::outline()),
                    ],
                )
                .on_click(dark.toggle())
                .bind_text(if_else(dark, "Light theme", "Dark theme")),
        )
}

fn recipe_section() -> El<Gallery> {
    let size = Gallery::size();
    let tone = Gallery::tone();
    let choice = |id: &'static str, text: &str| button().id(id).css(look::chip()).text(text);
    section()
        .id("recipe")
        .css(look::panel())
        .child(h2().text("Recipe"))
        .child(p().text("The theme button above takes its size and tone from these."))
        .child(
            div()
                .css(look::row())
                .child(choice("small", "Small").on_click(size.set(&Size::Small)))
                .child(choice("large", "Large").on_click(size.set(&Size::Large)))
                .child(choice("solid", "Solid").on_click(tone.set(&Tone::Solid)))
                .child(choice("outline", "Outline").on_click(tone.set(&Tone::Outline))),
        )
}

fn grid_section() -> El<Gallery> {
    section()
        .id("grid")
        .css(look::panel())
        .child(h2().text("Grid, group, and peer"))
        .child(div().id("cards").css(look::cards()).children(
            ["Tea", "Coffee", "Cocoa"].into_iter().map(|name| {
                div()
                    .css(look::card())
                    .child(span().text(name))
                    .child(span().css(look::card_action()).text("Order"))
            }),
        ))
        .child(
            div()
                .css(look::row())
                .child(
                    input()
                        .id("details-toggle")
                        .attr("type", "checkbox")
                        .css(look::peer()),
                )
                .child(label().attr("for", "details-toggle").text("Show details"))
                .child(
                    p().id("details")
                        .css(look::details())
                        .text("Each card shows Order on hover."),
                ),
        )
}

fn motion_section() -> El<Gallery> {
    section()
        .id("motion")
        .css(look::panel())
        .child(h2().text("Animation and patches"))
        .child(
            div()
                .css(look::row())
                .child(div().id("spinner").css(look::spinner()))
                .child(
                    button()
                        .id("load")
                        .css(look::chip())
                        .on_click(MORE.call())
                        .text("Load cards"),
                ),
        )
        .child(div().id("more"))
}

async fn index() -> Html<String> {
    let page = Page::new(
        "hypersig style",
        Gallery {
            dark: false,
            size: Size::Small,
            tone: Tone::Solid,
        },
    )
    .global(look::global())
    .theme(look::theme())
    .child(
        div()
            .id("app")
            .css(look::app())
            .css_when(look::dark(), Gallery::dark())
            .child(h1().text("Typed styles"))
            .child(theme_section())
            .child(recipe_section())
            .child(grid_section())
            .child(motion_section()),
    );
    Html(page.render())
}

async fn more((): (), (): ()) -> Patches<Gallery> {
    Patches::new().element(&More)
}

/// The styles of the page, one function per kind of element.
mod look {
    use hypersig::style::pattern::{circle, grid, hstack, stack, wrap};
    use hypersig::style::value::{
        AnimationIterationCount, AnimationTimingFunction, BorderStyle, BoxSizing, Color, Cursor,
        Display, FontFamily, FontWeight, LineHeight, Opacity, Transform, Visibility, ms, pct, px,
        raw, rem,
    };
    use hypersig::style::{
        Condition, Element, Global, Keyframes, Media, PseudoClass, Style, Theme, Token,
    };

    pub fn surface() -> Token<Color> {
        Token::new("surface", Color::WHITE)
    }

    pub fn ink() -> Token<Color> {
        Token::new("ink", Color::hex(0x001f_2328))
    }

    pub fn brand() -> Token<Color> {
        Token::new("brand", Color::hex(0x002f_6feb))
    }

    pub fn global() -> Global {
        Global::preflight().rule(
            Element::Body,
            Style::new()
                .font_family(FontFamily::SystemUi)
                .line_height(LineHeight::Number(1.5)),
        )
    }

    /// The colors a visitor whose system prefers dark sees until the button
    /// picks a theme.
    pub fn theme() -> Theme {
        Theme::new().on(
            Media::Dark,
            Theme::new()
                .set(&surface(), Color::hex(0x000d_1117))
                .set(&ink(), Color::hex(0x00e6_edf3)),
        )
    }

    pub fn app() -> Style {
        stack(rem(1.5))
            .min_height(raw("100vh"))
            .padding(rem(1.5))
            .background_color(surface().var())
            .color(ink().var())
    }

    pub fn dark() -> Style {
        Style::new()
            .token(&surface(), Color::hex(0x000d_1117))
            .token(&ink(), Color::hex(0x00e6_edf3))
            .token(&brand(), Color::hex(0x0058_a6ff))
    }

    pub fn panel() -> Style {
        stack(rem(0.75))
            .padding(rem(1))
            .border_width(px(1))
            .border_style(BorderStyle::Solid)
            .border_color(ink().var().opacity(15))
            .border_radius(px(8))
    }

    pub fn row() -> Style {
        wrap(rem(0.5))
    }

    pub fn chip() -> Style {
        Style::new()
            .padding_block(rem(0.25))
            .padding_inline(rem(0.75))
            .border_width(px(1))
            .border_style(BorderStyle::Solid)
            .border_color(ink().var().opacity(30))
            .border_radius(px(999))
            .background_color(Color::TRANSPARENT)
            .color(ink().var())
            .cursor(Cursor::Pointer)
            .on(
                PseudoClass::Hover,
                Style::new().background_color(ink().var().opacity(8)),
            )
    }

    /// The base of the recipe, which the size and tone variants add to.
    pub fn button() -> Style {
        Style::new()
            .border_width(px(2))
            .border_style(BorderStyle::Solid)
            .border_color(brand().var())
            .border_radius(px(6))
            .font_weight(FontWeight::Number(600.0))
            .cursor(Cursor::Pointer)
    }

    pub fn small() -> Style {
        Style::new()
            .padding_block(rem(0.25))
            .padding_inline(rem(0.5))
            .font_size(rem(0.875))
    }

    pub fn large() -> Style {
        Style::new()
            .padding_block(rem(0.75))
            .padding_inline(rem(1.5))
            .font_size(rem(1.25))
    }

    pub fn solid() -> Style {
        Style::new()
            .background_color(brand().var())
            .color(Color::WHITE)
    }

    pub fn outline() -> Style {
        Style::new()
            .background_color(Color::TRANSPARENT)
            .color(brand().var())
    }

    /// One column, then three from the `lg` breakpoint.
    pub fn cards() -> Style {
        grid(1, rem(0.75)).on(Media::Lg, grid(3, rem(0.75)))
    }

    pub fn card() -> Style {
        hstack(rem(0.5))
            .justify_content(hypersig::style::value::JustifyContent::SpaceBetween)
            .padding(rem(1))
            .border_radius(px(8))
            .background_color(ink().var().opacity(6))
            .group()
    }

    /// Hidden until the pointer is over its card.
    pub fn card_action() -> Style {
        Style::new()
            .visibility(Visibility::Hidden)
            .color(brand().var())
            .on(
                Condition::Group(PseudoClass::Hover),
                Style::new().visibility(Visibility::Visible),
            )
    }

    pub fn peer() -> Style {
        Style::new().peer()
    }

    /// Shown while the checkbox before it is checked.
    pub fn details() -> Style {
        Style::new().display(Display::None).flex_basis(pct(100)).on(
            Condition::Peer(PseudoClass::Checked),
            Style::new().display(Display::Block),
        )
    }

    pub fn spinner() -> Style {
        let spin = Keyframes::new("spin").at(100, Style::new().transform(raw("rotate(1turn)")));
        circle(rem(1.5))
            .border_width(px(3))
            .border_style(BorderStyle::Solid)
            .border_color(ink().var().opacity(20))
            .border_top_color(brand().var())
            .transform(Transform::None)
            .keyframes(&spin)
            .animation_duration(ms(900))
            .animation_timing_function(AnimationTimingFunction::Linear)
            .animation_iteration_count(AnimationIterationCount::Infinite)
            .on(
                Media::ReducedMotion,
                Style::new().opacity(Opacity::Number(0.5)),
            )
    }

    /// The loaded cards side by side, with no gap, so each width is a plain
    /// share of the row.
    pub fn loaded_row() -> Style {
        Style::new()
            .display(Display::Flex)
            .flex_wrap(hypersig::style::value::FlexWrap::Wrap)
    }

    /// Full width, half from `md`, and a quarter from `lg`. The page has a
    /// rule under `lg` but none under `md`, so the patch's `md` rules have to
    /// land before the page's `lg` rules for the quarter to win.
    pub fn loaded_card() -> Style {
        Style::new()
            .box_sizing(BoxSizing::BorderBox)
            .width(pct(100))
            .padding(rem(1))
            .border_radius(px(8))
            .background_color(brand().var().opacity(15))
            .on(Media::Md, Style::new().width(pct(50)))
            .on(Media::Lg, Style::new().width(pct(25)))
    }
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let app = Router::new()
        .route("/", get(index))
        .hypersig_runtime()
        .action(MORE, more);
    let addr = std::env::var("ADDR").unwrap_or_else(|_| "127.0.0.1:3004".to_owned());
    let listener = tokio::net::TcpListener::bind(&addr).await?;
    println!("listening on http://{addr}");
    axum::serve(listener, app).await
}
