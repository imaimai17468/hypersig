//! Render time and allocations of `Page::render` for a list page.
//!
//! Run with `cargo bench -p hypersig --bench render`.

use divan::{AllocProfiler, Bencher};
use hypersig::html::{Class, El, Node, attr, each_keyed, input, li, span, ul};
use hypersig::style::value::{Color, Display, rem};
use hypersig::style::{Media, PseudoClass, Style};
use hypersig::{Page, Signals};
use serde::{Deserialize, Serialize};

#[global_allocator]
static ALLOC: AllocProfiler = AllocProfiler::system();

const DONE: Class = Class::new("done");

#[derive(Signals, Serialize, Deserialize)]
struct Todo {
    id: i32,
    title: String,
    done: bool,
}

#[derive(Signals, Serialize, Deserialize)]
struct Todos {
    items: Vec<Todo>,
    filter: String,
    #[signals(computed)]
    left: i32,
}

fn signals(len: i32) -> Todos {
    Todos {
        items: (0..len)
            .map(|id| Todo {
                id,
                title: format!("todo {id}"),
                done: id % 3 == 0,
            })
            .collect(),
        filter: "all".to_owned(),
        left: 0,
    }
}

/// Each item carries a class, a visibility condition, a checkbox, and a text binding.
fn list(left: impl Fn() -> El<Todos>) -> Node<Todos> {
    each_keyed(
        Todos::items(),
        |todo| todo.at(Todo::id()),
        |todo| {
            li().class_when(DONE, todo.at(Todo::done()))
                .show(Todos::filter().equals("all").or(!todo.at(Todo::done())))
                .child(
                    input()
                        .attr(attr::TYPE, "checkbox")
                        .bind(todo.at(Todo::done())),
                )
                .child(span().bind_text(todo.at(Todo::title())))
                .child(left())
        },
    )
}

fn main() {
    divan::main();
}

#[divan::bench(args = [100, 1_000, 10_000])]
fn list_items(bencher: Bencher, len: i32) {
    let page = Page::new("bench", signals(len)).child(ul().child(list(span)));
    bencher.bench_local(|| page.render());
}

/// The items of `list_items` with atomic classes: five declarations under three
/// conditions, and one toggled style.
#[divan::bench(args = [100, 1_000, 10_000])]
fn list_items_styled(bencher: Bencher, len: i32) {
    let item = Style::new()
        .display(Display::Flex)
        .padding(rem(1))
        .color(Color::hex(0x0022_2222))
        .on(PseudoClass::Hover, Style::new().color(Color::CRIMSON))
        .on(Media::Md, Style::new().padding(rem(2)));
    let page = Page::new("bench", signals(len)).child(ul().child(each_keyed(
        Todos::items(),
        |todo| todo.at(Todo::id()),
        |todo| {
            li().css(item)
                .css_when(Style::new().color(Color::GRAY), todo.at(Todo::done()))
                .child(span().bind_text(todo.at(Todo::title())))
        },
    )));
    bencher.bench_local(|| page.render());
}

/// Every item counts the whole list, so the work grows with the square of the length.
#[divan::bench(args = [100, 1_000])]
fn count_in_each_item(bencher: Bencher, len: i32) {
    let page = Page::new("bench", signals(len)).child(ul().child(list(|| {
        span().bind_text(
            Todos::items()
                .count(|todo| !todo.at(Todo::done()))
                .to_text(),
        )
    })));
    bencher.bench_local(|| page.render());
}

/// The same count as `count_in_each_item`, computed once by `Page::computed`.
#[divan::bench(args = [100, 1_000])]
fn count_computed_once(bencher: Bencher, len: i32) {
    let page = Page::new("bench", signals(len))
        .computed(
            Todos::left(),
            Todos::items().count(|todo| !todo.at(Todo::done())),
        )
        .child(ul().child(list(|| span().bind_text(Todos::left().to_text()))));
    bencher.bench_local(|| page.render());
}
