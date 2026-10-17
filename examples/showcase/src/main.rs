//! Components with their own state, intervals, server search, forms, and typed
//! styles, one section each.
//!
//! Run with `cargo run -p example-showcase` and open <http://localhost:3003>.

use std::time::Duration;

use axum::Router;
use hypersig::html::{
    El, Event, Key, Stylesheet, button, div, form, h1, h2, input, label, li, p, section, select,
    small, span, textarea, ul,
};
use hypersig::{
    Action, Fragment, NodeRef, Page, Patches, Route, RouterExt, Sig, Signals, Stmt, if_else,
};
use serde::{Deserialize, Serialize};

/// State owned by one [`disclosure`]. Each placement gets its own copy.
#[derive(Signals, Serialize, Deserialize, Default)]
struct Disclosure {
    open: bool,
}

#[derive(Serialize, Deserialize, Clone, Copy, PartialEq)]
enum Size {
    Small,
    Large,
}

#[derive(Signals, Serialize, Deserialize, Default)]
struct Stopwatch {
    running: bool,
    elapsed: f64,
}

#[derive(Signals, Serialize, Deserialize, Default)]
struct Search {
    query: String,
    searching: bool,
}

#[derive(Signals, Serialize, Deserialize)]
struct Order {
    quantity: i32,
    size: Size,
    gift: bool,
    note: String,
    placing: bool,
    status: String,
    error: String,
}

#[derive(Serialize, Deserialize, Clone, Copy, PartialEq)]
enum Tone {
    Calm,
    Loud,
}

#[derive(Signals, Serialize, Deserialize)]
struct Look {
    dark: bool,
    tone: Tone,
}

#[derive(Signals, Serialize, Deserialize)]
struct Showcase {
    shipping: Disclosure,
    returns: Disclosure,
    adults: i32,
    children: i32,
    stopwatch: Stopwatch,
    search: Search,
    order: Order,
    look: Look,
}

const INDEX: Route = Route::new("/");
const SEARCH: Action<Showcase, String> = Action::post("/search");
const ORDER: Action<Showcase, (i32, Size, bool)> = Action::post("/order");
const SEARCH_FIELD: NodeRef<Showcase> = NodeRef::new("search");

const NOTE_LIMIT: i32 = 40;

/// A component with its own state: a heading that opens and closes its body.
fn disclosure<S>(state: Sig<S, Disclosure>, title: &str, body: &str) -> El<S> {
    let open = state.at(Disclosure::open());
    div()
        .class(class::DISCLOSURE)
        .child(
            button()
                .class(class::DISCLOSURE_TITLE)
                .bind_attr("aria-expanded", if_else(open, "true", "false"))
                .on_click(open.toggle())
                .text(title),
        )
        .child(p().show(open).text(body))
}

/// A component that takes a signal as a prop and edits it within bounds.
fn stepper<S>(name: &str, value: Sig<S, i32>, min: i32, max: i32) -> El<S> {
    div()
        .class(class::STEPPER)
        .child(span().class(class::NAME).text(name))
        .child(
            button()
                .attr("aria-label", "fewer")
                .attr_when("disabled", value.le(min))
                .on_click(value.set(value - 1))
                .text("−"),
        )
        .child(span().class(class::VALUE).bind_text(value.to_text()))
        .child(
            button()
                .attr("aria-label", "more")
                .attr_when("disabled", value.ge(max))
                .on_click(value.set(value + 1))
                .text("+"),
        )
}

fn components() -> El<Showcase> {
    let adults = Showcase::adults();
    let children = Showcase::children();
    section()
        .id("components")
        .child(h2().text("Components with their own state"))
        .child(disclosure(
            Showcase::shipping(),
            "Shipping",
            "Orders leave the warehouse within two days.",
        ))
        .child(disclosure(
            Showcase::returns(),
            "Returns",
            "Return anything within thirty days.",
        ))
        .child(stepper("Adults", adults, 1, 6))
        .child(stepper("Children", children, 0, 6))
        .child(
            p().class(class::TOTAL)
                .bind_text((adults + children).to_text().concat(" guests")),
        )
}

fn stopwatch() -> El<Showcase> {
    let running = Showcase::stopwatch().at(Stopwatch::running());
    let elapsed = Showcase::stopwatch().at(Stopwatch::elapsed());
    section()
        .id("stopwatch")
        .on_interval(
            Duration::from_millis(100),
            Stmt::when(running, elapsed.set(elapsed + 0.1)),
        )
        .child(h2().text("Intervals"))
        .child(
            span()
                .class(class::ELAPSED)
                .bind_text(elapsed.to_fixed(1).concat(" s")),
        )
        .child(
            button()
                .class(class::START)
                .on_click(running.toggle())
                .bind_text(if_else(running, "Stop", "Start")),
        )
        .child(
            button()
                .class(class::RESET)
                .attr_when("disabled", running)
                .on_click(elapsed.set(0.0))
                .text("Reset"),
        )
}

const FRUITS: &[&str] = &[
    "apple",
    "apricot",
    "banana",
    "blackberry",
    "blueberry",
    "cherry",
    "grape",
    "lemon",
    "mango",
    "orange",
    "peach",
    "pear",
    "plum",
];

struct Results {
    matches: Vec<&'static str>,
}

impl Results {
    fn for_query(query: &str) -> Self {
        let query = query.trim().to_lowercase();
        Self {
            matches: FRUITS
                .iter()
                .copied()
                .filter(|fruit| fruit.contains(&query))
                .collect(),
        }
    }
}

impl Fragment for Results {
    type Signals = Showcase;
    const ID: &'static str = "results";

    fn render(&self) -> El<Showcase> {
        ul().children(self.matches.iter().map(|fruit| li().text(*fruit)))
            .children(
                self.matches
                    .is_empty()
                    .then(|| li().class(class::EMPTY).text("No match")),
            )
    }
}

fn search() -> El<Showcase> {
    let query = Showcase::search().at(Search::query());
    let searching = Showcase::search().at(Search::searching());
    section()
        .id("search")
        .child(h2().text("Server search as you type"))
        .child(
            div()
                .class(class::ROW)
                .child(
                    input()
                        .node_ref(SEARCH_FIELD)
                        .attr("placeholder", "Search fruit")
                        .bind(query)
                        .on_debounced(
                            Event::Input,
                            Duration::from_millis(250),
                            SEARCH.call_with(query).indicator(searching),
                        )
                        .on_key(
                            Key::Escape,
                            query
                                .set("")
                                .then(SEARCH.call_with("").indicator(searching)),
                        ),
                )
                .child(
                    button()
                        .class(class::FOCUS)
                        .on_click(SEARCH_FIELD.focus())
                        .text("Focus"),
                ),
        )
        .child(
            small()
                .class(class::SEARCHING)
                .show(searching)
                .text("Searching…"),
        )
        .child(hypersig::fragment(&Results::for_query("")))
}

fn order_form() -> El<Showcase> {
    let order = Showcase::order();
    let quantity = order.at(Order::quantity());
    let size = order.at(Order::size());
    let note = order.at(Order::note());
    let placing = order.at(Order::placing());
    let unit = if_else(size.equals(&Size::Large), 4.5, 3.0);
    let total = unit * quantity.to_f64();
    let too_long = note.char_count().gt(NOTE_LIMIT);
    let invalid = quantity.lt(1).or(too_long.clone());
    section()
        .id("order")
        .child(h2().text("Forms"))
        .child(
            form()
                .on_submit(
                    ORDER
                        .call_with((
                            order.at(Order::quantity()),
                            order.at(Order::size()),
                            order.at(Order::gift()),
                        ))
                        .indicator(placing)
                        .error(order.at(Order::error())),
                )
                .child(
                    label().text("Quantity ").child(
                        input()
                            .class(class::QUANTITY)
                            .attr("type", "number")
                            .attr("min", "0")
                            .bind(quantity),
                    ),
                )
                .child(
                    label().text("Size ").child(
                        select()
                            .class(class::SIZE)
                            .bind_select(size, [(Size::Small, "Small"), (Size::Large, "Large")]),
                    ),
                )
                .child(
                    label()
                        .child(
                            input()
                                .class(class::GIFT)
                                .attr("type", "checkbox")
                                .bind(order.at(Order::gift())),
                        )
                        .text(" Gift wrap"),
                )
                .child(
                    textarea()
                        .class(class::NOTE)
                        .attr("placeholder", "Note")
                        .bind(note),
                )
                .child(
                    small()
                        .class(class::COUNT)
                        .class_when(class::OVER, too_long)
                        .bind_text(
                            note.char_count()
                                .to_text()
                                .concat(format!(" / {NOTE_LIMIT}")),
                        ),
                )
                .child(
                    p().class(class::PRICE)
                        .bind_text(total.to_fixed(2).concat(" USD")),
                )
                .child(
                    button()
                        .class(class::PLACE)
                        .attr_when("disabled", invalid.or(placing))
                        .bind_text(if_else(placing, "Placing…", "Place order")),
                ),
        )
        .child(
            p().class(class::STATUS)
                .bind_text(order.at(Order::status())),
        )
        .child(p().class(class::ERROR).bind_text(order.at(Order::error())))
}

/// Typed styles: a breakpoint, a container query, tokens a signal switches,
/// a style per value of a signal, and an animation.
fn styles() -> El<Showcase> {
    let look = Showcase::look();
    let dark = look.at(Look::dark());
    let tone = look.at(Look::tone());
    section()
        .id("styles")
        .child(h2().text("Typed styles"))
        .child(
        div().css_when(look::dark_frame(), dark).child(
            div()
                .id("card")
                .css(look::card())
                .child(p().css(look::wide_note()).text(
                    "Half as wide from a 48rem viewport, bold while the card is 24rem or wider.",
                ))
                .child(
                    div()
                        .css(look::buttons())
                        .child(
                            button()
                                .id("theme")
                                .on_click(dark.toggle())
                                .text("Switch theme"),
                        )
                        .child(
                            button()
                                .id("calm")
                                .on_click(tone.set(&Tone::Calm))
                                .text("Calm"),
                        )
                        .child(
                            button()
                                .id("loud")
                                .on_click(tone.set(&Tone::Loud))
                                .text("Loud"),
                        ),
                )
                .child(
                    span()
                        .id("badge")
                        .css(look::badge())
                        .css_match(
                            tone,
                            [(Tone::Calm, look::calm()), (Tone::Loud, look::loud())],
                        )
                        .bind_text(if_else(tone.equals(&Tone::Loud), "LOUD", "calm")),
                )
                .child(div().id("spinner").css(look::spinner())),
        ),
    )
}

/// The styles of the typed-styles section.
mod look {
    use hypersig::style::pattern::{circle, hstack, stack};
    use hypersig::style::value::{
        AnimationIterationCount, AnimationTimingFunction, BorderStyle, BoxSizing, Color,
        ContainerType, FontWeight, Transform, ms, pct, px, raw, rem,
    };
    use hypersig::style::{Container, Keyframes, Media, Style, Theme, Token};

    pub fn surface() -> Token<Color> {
        Token::new("surface", Color::hex(0x00f6_f6f6))
    }

    pub fn ink() -> Token<Color> {
        Token::new("ink", Color::hex(0x0022_2222))
    }

    /// The page's dark values, for a visitor whose system prefers dark.
    pub fn theme() -> Theme {
        Theme::new().on(
            Media::Dark,
            Theme::new()
                .set(&surface(), Color::hex(0x0022_2733))
                .set(&ink(), Color::hex(0x00ee_eeee)),
        )
    }

    /// The tokens the theme button switches to, for the card inside.
    pub fn dark_frame() -> Style {
        Style::new()
            .token(&surface(), Color::hex(0x0022_2733))
            .token(&ink(), Color::hex(0x00ee_eeee))
    }

    pub fn card() -> Style {
        stack(rem(0.75))
            .box_sizing(BoxSizing::BorderBox)
            .padding(rem(1))
            .border_radius(px(8))
            .background_color(surface().var())
            .color(ink().var())
            .width(pct(100))
            .on(Media::Md, Style::new().width(pct(50)))
            .container_type(ContainerType::InlineSize)
    }

    pub fn wide_note() -> Style {
        Style::new()
            .margin(px(0))
            .on(Container::Sm, Style::new().font_weight(FontWeight::Bold))
    }

    pub fn buttons() -> Style {
        hstack(rem(0.5))
    }

    pub fn badge() -> Style {
        Style::new()
            .padding_inline(rem(0.5))
            .border_radius(px(4))
            .border_width(px(1))
            .border_style(BorderStyle::Solid)
    }

    pub fn calm() -> Style {
        Style::new().color(Color::GRAY)
    }

    pub fn loud() -> Style {
        Style::new()
            .color(Color::CRIMSON)
            .font_weight(FontWeight::Bold)
    }

    pub fn spinner() -> Style {
        let spin = Keyframes::new("spin")
            .at(0, Style::new().transform(Transform::None))
            .at(100, Style::new().transform(raw("rotate(1turn)")));
        circle(rem(1.5))
            .border_width(px(3))
            .border_style(BorderStyle::Solid)
            .border_color(Color::GRAY.opacity(30))
            .border_top_color(Color::GRAY)
            .keyframes(&spin)
            .animation_duration(ms(900))
            .animation_timing_function(AnimationTimingFunction::Linear)
            .animation_iteration_count(AnimationIterationCount::Infinite)
    }
}

async fn index((): (), (): ()) -> Page<Showcase> {
    let signals = Showcase {
        shipping: Disclosure::default(),
        returns: Disclosure { open: true },
        adults: 2,
        children: 0,
        stopwatch: Stopwatch::default(),
        search: Search::default(),
        order: Order {
            quantity: 1,
            size: Size::Small,
            gift: false,
            note: String::new(),
            placing: false,
            status: String::new(),
            error: String::new(),
        },
        look: Look {
            dark: false,
            tone: Tone::Calm,
        },
    };
    Page::new("hypersig showcase", signals)
        .style(STYLE)
        .theme(look::theme())
        .child(h1().text("hypersig showcase"))
        .child(components())
        .child(stopwatch())
        .child(search())
        .child(order_form())
        .child(styles())
}

async fn search_fruit((): (), query: String) -> Patches<Showcase> {
    tokio::time::sleep(Duration::from_millis(200)).await;
    Patches::new().element(&Results::for_query(&query))
}

async fn order((): (), (quantity, size, gift): (i32, Size, bool)) -> Patches<Showcase> {
    tokio::time::sleep(Duration::from_millis(300)).await;
    let size = match size {
        Size::Small => "small",
        Size::Large => "large",
    };
    let wrap = if gift { ", gift wrapped" } else { "" };
    let status = format!("Ordered {quantity} {size}{wrap}.");
    Showcase::order().at(Order::status()).patch(status).into()
}

mod class {
    use hypersig::html::Class;

    use super::STYLE;

    pub const DISCLOSURE_TITLE: Class = STYLE.class("disclosure-title");
    pub const STEPPER: Class = STYLE.class("stepper");
    pub const NAME: Class = STYLE.class("name");
    pub const VALUE: Class = STYLE.class("value");
    pub const ELAPSED: Class = STYLE.class("elapsed");
    pub const ROW: Class = STYLE.class("row");
    pub const SEARCHING: Class = STYLE.class("searching");
    pub const COUNT: Class = STYLE.class("count");
    pub const OVER: Class = STYLE.class("over");
    pub const PRICE: Class = STYLE.class("price");
    pub const ERROR: Class = STYLE.class("error");

    // No rule in `STYLE` styles the classes below.
    pub const DISCLOSURE: Class = Class::new("disclosure");
    pub const TOTAL: Class = Class::new("total");
    pub const START: Class = Class::new("start");
    pub const RESET: Class = Class::new("reset");
    pub const EMPTY: Class = Class::new("empty");
    pub const FOCUS: Class = Class::new("focus");
    pub const QUANTITY: Class = Class::new("quantity");
    pub const SIZE: Class = Class::new("size");
    pub const GIFT: Class = Class::new("gift");
    pub const NOTE: Class = Class::new("note");
    pub const PLACE: Class = Class::new("place");
    pub const STATUS: Class = Class::new("status");
}

const STYLE: Stylesheet = Stylesheet::new("
body { font-family: system-ui, sans-serif; max-width: 40rem; margin: 2rem auto; padding: 0 1rem; color: #222; }
section { border: 1px solid #ddd; border-radius: 8px; padding: .5rem 1rem 1rem; margin: 1rem 0; }
h2 { font-size: 1.1rem; }
button { font-size: 1rem; padding: .3rem .8rem; cursor: pointer; }
button:disabled { cursor: not-allowed; opacity: .5; }
.disclosure-title { width: 100%; text-align: left; background: #f6f6f6; border: 1px solid #ddd; margin-top: .4rem; }
.stepper { display: flex; gap: .5rem; align-items: center; margin-top: .5rem; }
.stepper .name { width: 6rem; }
.stepper .value { min-width: 2ch; text-align: center; font-variant-numeric: tabular-nums; }
.elapsed { font-size: 2rem; font-variant-numeric: tabular-nums; margin-right: 1rem; }
.row { display: flex; gap: .5rem; }
.row input { flex: 1; font-size: 1rem; padding: .3rem; }
.searching { color: #888; }
form { display: grid; gap: .5rem; }
textarea { font: inherit; min-height: 3rem; }
.count.over { color: crimson; font-weight: bold; }
.price { font-weight: bold; }
.error { color: crimson; }
");

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let app = Router::new()
        .page(INDEX, index)
        .hypersig_runtime()
        .action(SEARCH, search_fruit)
        .action(ORDER, order);
    let addr = std::env::var("ADDR").unwrap_or_else(|_| "127.0.0.1:3003".to_owned());
    let listener = tokio::net::TcpListener::bind(&addr).await?;
    println!("listening on http://{addr}");
    axum::serve(listener, app).await
}
