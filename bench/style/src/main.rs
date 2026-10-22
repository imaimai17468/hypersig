//! Renders TodoMVC-2000 once per way of styling it and writes each page to
//! `out/<name>/index.html`.
//!
//! Every page has the markup of `bench/compare/hypersig`. The `hypersig` page
//! styles it with `hypersig::style`, and every other page reads the class
//! names a tool wrote to `out/<name>/classes.json` and links the tool's
//! `style.css`. Each page is rendered 50 times, and the median time goes to
//! `out/<name>/render.json`.

use std::collections::HashMap;
use std::fs;
use std::path::Path;
use std::time::Instant;

use hypersig::IntoExpr;
use hypersig::html::{
    Class, El, a, button, each_keyed, footer, h1, header, input, label, li, section, span, strong,
    ul,
};
use hypersig::{Page, SafeUrl, Signals, if_else};
use serde::{Deserialize, Serialize};

#[derive(Signals, Serialize, Deserialize, Clone, PartialEq)]
struct Todo {
    id: i32,
    title: String,
    done: bool,
}

#[derive(Serialize, Deserialize, Clone, Copy, PartialEq)]
enum Filter {
    All,
    Active,
    Completed,
}

#[derive(Signals, Serialize, Deserialize)]
struct TodoMvc {
    todos: Vec<Todo>,
    draft: String,
    filter: Filter,
    left: i32,
}

/// An element of the page that a styling gives a look.
#[derive(Clone, Copy)]
enum Part {
    App,
    Heading,
    NewTodo,
    Main,
    List,
    Item,
    ItemDone,
    Toggle,
    Title,
    TitleDone,
    Destroy,
    Footer,
    Filters,
    FilterLink,
    Selected,
}

impl Part {
    /// The key of this part in `classes.json`.
    fn key(self) -> &'static str {
        match self {
            Self::App => "app",
            Self::Heading => "heading",
            Self::NewTodo => "newTodo",
            Self::Main => "main",
            Self::List => "list",
            Self::Item => "item",
            Self::ItemDone => "itemDone",
            Self::Toggle => "toggle",
            Self::Title => "title",
            Self::TitleDone => "titleDone",
            Self::Destroy => "destroy",
            Self::Footer => "footer",
            Self::Filters => "filters",
            Self::FilterLink => "filterLink",
            Self::Selected => "selected",
        }
    }
}

/// How a page is styled.
enum Look {
    /// `hypersig::style`.
    Atoms,
    /// The class names a tool generated, by `Part::key`.
    Classes(HashMap<String, Vec<Class>>),
}

impl Look {
    /// Reads `classes.json`. The names are leaked because `Class` holds a
    /// `&'static str`, and this program renders once and exits.
    fn read(path: &Path) -> Self {
        let text = fs::read_to_string(path).unwrap_or_else(|e| panic!("{}: {e}", path.display()));
        let names: HashMap<String, String> = serde_json::from_str(&text).expect("classes.json");
        Self::Classes(
            names
                .into_iter()
                .map(|(key, value)| {
                    let classes = value
                        .split_ascii_whitespace()
                        .map(|name| Class::new(String::leak(name.to_owned())))
                        .collect();
                    (key, classes)
                })
                .collect(),
        )
    }

    fn dress(&self, el: El<TodoMvc>, part: Part) -> El<TodoMvc> {
        match self {
            Self::Atoms => el.css(styles::of(part)),
            Self::Classes(classes) => classes[part.key()]
                .iter()
                .fold(el, |el, &class| el.class(class)),
        }
    }

    fn dress_when(
        &self,
        el: El<TodoMvc>,
        part: Part,
        when: impl IntoExpr<TodoMvc, bool> + Clone,
    ) -> El<TodoMvc> {
        match self {
            Self::Atoms => {
                let style = styles::of(part);
                if style.is_empty() {
                    el
                } else {
                    el.css_when(style, when)
                }
            }
            Self::Classes(classes) => classes[part.key()]
                .iter()
                .fold(el, |el, &class| el.class_when(class, when.clone())),
        }
    }
}

fn seed() -> Vec<Todo> {
    (1..=2000)
        .map(|id| Todo {
            id,
            title: format!("todo {id}"),
            done: id % 3 == 0,
        })
        .collect()
}

fn todo_list(look: &Look) -> El<TodoMvc> {
    let filter = TodoMvc::filter();
    look.dress(ul(), Part::List).child(each_keyed(
        TodoMvc::todos(),
        |todo| todo.at(Todo::id()),
        |todo| {
            let done = todo.at(Todo::done());
            let visible = filter
                .equals(&Filter::All)
                .or(filter.equals(&Filter::Active).and(!done))
                .or(filter.equals(&Filter::Completed).and(done));
            let item = look.dress(li(), Part::Item).show(visible);
            look.dress_when(item, Part::ItemDone, done)
                .child(
                    look.dress(input(), Part::Toggle)
                        .attr("type", "checkbox")
                        .bind(done),
                )
                .child(
                    look.dress_when(look.dress(label(), Part::Title), Part::TitleDone, done)
                        .bind_text(todo.at(Todo::title())),
                )
                .child(look.dress(button(), Part::Destroy))
        },
    ))
}

fn filter_link(look: &Look, filter: Filter, text: &str) -> El<TodoMvc> {
    let link = look.dress(a().href(SafeUrl::new("#")), Part::FilterLink);
    li().child(
        look.dress_when(link, Part::Selected, TodoMvc::filter().equals(&filter))
            .on_click(TodoMvc::filter().set(&filter))
            .text(text),
    )
}

fn page(look: &Look) -> Page<TodoMvc> {
    let list = TodoMvc::todos();
    let left = TodoMvc::left();
    let signals = TodoMvc {
        todos: seed(),
        draft: String::new(),
        filter: Filter::All,
        left: 0,
    };
    let page = Page::new("TodoMVC", signals);
    let page = match look {
        Look::Atoms => page.global(styles::global()),
        Look::Classes(_) => page.stylesheet("style.css"),
    };
    page.computed(left, list.count(|todo| !todo.at(Todo::done())))
        .child(
            look.dress(section(), Part::App)
                .child(
                    header()
                        .child(look.dress(h1(), Part::Heading).text("todos"))
                        .child(
                            look.dress(input(), Part::NewTodo)
                                .attr("placeholder", "What needs to be done?")
                                .bind(TodoMvc::draft()),
                        ),
                )
                .child(look.dress(section(), Part::Main).child(todo_list(look)))
                .child(
                    look.dress(footer(), Part::Footer)
                        .child(span().child(strong().bind_text(left.to_text())).child(
                            span().bind_text(if_else(left.equals(1), " item left", " items left")),
                        ))
                        .child(
                            look.dress(ul(), Part::Filters)
                                .child(filter_link(look, Filter::All, "All"))
                                .child(filter_link(look, Filter::Active, "Active"))
                                .child(filter_link(look, Filter::Completed, "Completed")),
                        ),
                ),
        )
}

/// The rules of `bench/compare/shared/todomvc.css`, one function per part.
mod styles {
    use hypersig::style::value::{
        AlignItems, Border, BoxSizing, Color, Content, CssWideKeyword, Cursor, Display, FontWeight,
        JustifyContent, ListStyle, MarginInline, Position, TextAlign, TextDecoration, WordBreak,
        pct, px, quoted, raw,
    };
    use hypersig::style::{
        Condition, Element, Global, Property, PseudoClass, PseudoElement, Selector, Style,
    };

    use super::Part;

    const GRAY: u32 = 0x0094_9494;

    pub fn global() -> Global {
        Global::new()
            .rule(
                Selector::Everything,
                Style::new().box_sizing(BoxSizing::BorderBox),
            )
            .rule(
                Element::Body,
                Style::new()
                    .font_size(px(14))
                    .font_family(raw("'Helvetica Neue', Helvetica, Arial, sans-serif"))
                    .background_color(Color::hex(0x00f5_f5f5))
                    .color(Color::hex(0x0011_1111))
                    .margin(px(0)),
            )
    }

    fn list() -> Style {
        Style::new()
            .margin(px(0))
            .padding(px(0))
            .list_style(ListStyle::None)
    }

    pub fn of(part: Part) -> Style {
        match part {
            Part::App => Style::new()
                .background_color(Color::WHITE)
                .max_width(px(550))
                .margin_top(px(130))
                .margin_inline(MarginInline::Auto)
                .margin_bottom(px(40))
                .position(Position::Relative)
                .box_shadow(raw("0 2px 4px rgba(0,0,0,.2), 0 25px 50px rgba(0,0,0,.1)")),
            Part::Heading => Style::new()
                .position(Position::Absolute)
                .top(px(-110))
                .width(pct(100))
                .font_size(px(80))
                .font_weight(FontWeight::Number(200.0))
                .text_align(TextAlign::Center)
                .color(Color::hex(0x00b8_3f45))
                .margin(px(0)),
            Part::NewTodo => Style::new().box_shadow(raw("inset 0 -2px 1px rgba(0,0,0,.03)")),
            Part::Main => Style::new()
                .position(Position::Relative)
                .border_top(raw("1px solid #e6e6e6")),
            Part::List => list(),
            // The item is the group its delete button watches for hover.
            Part::Item => Style::new()
                .position(Position::Relative)
                .font_size(px(24))
                .border_bottom(raw("1px solid #ededed"))
                .display(Display::Flex)
                .align_items(AlignItems::Center)
                .group(),
            Part::ItemDone => Style::new(),
            Part::Toggle => Style::new()
                .width(px(40))
                .height(px(40))
                .margin_block(px(0))
                .margin_inline(px(10)),
            Part::Title => Style::new()
                .flex(1)
                .padding(px(15))
                .word_break(WordBreak::BreakAll)
                .transition(raw("color .2s")),
            Part::TitleDone => Style::new()
                .color(Color::hex(GRAY))
                .text_decoration(TextDecoration::LineThrough),
            Part::Destroy => Style::new()
                .display(Display::None)
                .border(Border::None)
                .background_color(Color::TRANSPARENT)
                .font_size(px(30))
                .color(Color::hex(GRAY))
                .margin_right(px(10))
                .cursor(Cursor::Pointer)
                .pseudo(
                    PseudoElement::After,
                    Style::new().content(Content::Quoted(quoted("×"))),
                )
                .on(
                    Condition::Group(PseudoClass::Hover),
                    Style::new().display(Display::Block),
                ),
            Part::Footer => Style::new()
                .padding_block(px(10))
                .padding_inline(px(15))
                .display(Display::Flex)
                .align_items(AlignItems::Center)
                .justify_content(JustifyContent::SpaceBetween)
                .border_top(raw("1px solid #e6e6e6"))
                .color(Color::hex(0x0011_1111)),
            Part::Filters => list().display(Display::Flex).gap(px(4)),
            Part::FilterLink => Style::new()
                .keyword(Property::Color, CssWideKeyword::Inherit)
                .text_decoration(TextDecoration::None)
                .padding_block(px(3))
                .padding_inline(px(7))
                .border(raw("1px solid transparent"))
                .border_radius(px(3)),
            Part::Selected => Style::new().border_color(Color::hex(0x00ce_4646)),
        }
    }
}

fn main() {
    let out = Path::new("out");
    let mut names: Vec<String> = std::env::args().skip(1).collect();
    names.sort();
    for name in std::iter::once("hypersig".to_owned()).chain(names) {
        let look = if name == "hypersig" {
            Look::Atoms
        } else {
            Look::read(&out.join(&name).join("classes.json"))
        };
        let dir = out.join(&name);
        fs::create_dir_all(&dir).expect("create the output directory");
        let page = page(&look);
        let mut times: Vec<f64> = (0..50)
            .map(|_| {
                let start = Instant::now();
                std::hint::black_box(page.render());
                start.elapsed().as_secs_f64() * 1000.0
            })
            .collect();
        times.sort_by(f64::total_cmp);
        fs::write(dir.join("render.json"), format!("{{\"ms\":{}}}", times[25]))
            .expect("write render.json");
        fs::write(dir.join("index.html"), page.render()).expect("write index.html");
    }
}
