//! A to-do list kept in the browser, for the page on lists.

use hypersig::html::{div, each_keyed, form, input, label, li, p, span, ul};
use hypersig::style::Style;
use hypersig::style::pattern::{hstack, stack};
use hypersig::style::value::{AlignItems, Display, JustifyContent, rem};
use hypersig::{Expr, Page, Signals, Stmt};
use hypersig_ui::{ButtonSize, ButtonVariant, TextField, button, submit_button};
use serde::{Deserialize, Serialize};

#[derive(Signals, Serialize, Deserialize)]
pub struct Todo {
    id: i32,
    title: String,
    done: bool,
}

#[derive(Signals, Serialize, Deserialize)]
pub struct Todos {
    draft: String,
    next_id: i32,
    items: Vec<Todo>,
}

pub fn page() -> Page<Todos> {
    let draft = Todos::draft();
    let next_id = Todos::next_id();
    let items = Todos::items();
    let blank = Todo {
        id: 0,
        title: String::new(),
        done: false,
    };
    let add = items
        .push(
            Expr::lit(&blank)
                .with(Todo::id(), next_id)
                .with(Todo::title(), draft.trim()),
        )
        .then(next_id.set(next_id + 1))
        .then(draft.set(""));

    let initial = Todos {
        draft: String::new(),
        next_id: 2,
        items: vec![
            Todo {
                id: 0,
                title: "Read the guide".into(),
                done: true,
            },
            Todo {
                id: 1,
                title: "Write a page".into(),
                done: false,
            },
        ],
    };
    Page::new("Todos", initial).child(
        div()
            .css(stack(rem(1)).max_width(rem(24)))
            .child(
                form()
                    .on_submit(Stmt::when(!draft.trim().is_empty(), add))
                    .css(hstack(rem(0.5)).align_items(AlignItems::End))
                    .child(
                        div()
                            .css(Style::new().flex_grow(1.0))
                            .child(TextField::new("New item", draft)),
                    )
                    .child(submit_button(ButtonVariant::Default, ButtonSize::Default).text("Add")),
            )
            .child(ul().css(stack(rem(0.25))).child(each_keyed(
                items,
                |todo| todo.at(Todo::id()),
                |todo| {
                    li().css(hstack(rem(0.5)).justify_content(JustifyContent::SpaceBetween))
                        .child(
                            label()
                                .css(hstack(rem(0.5)))
                                .child(input().attr("type", "checkbox").bind(todo.at(Todo::done())))
                                .child(span().bind_text(todo.at(Todo::title()))),
                        )
                        .child(
                            button(ButtonVariant::Ghost, ButtonSize::Sm)
                                .on_click(todo.remove())
                                .text("Remove"),
                        )
                },
            )))
            .child(
                div()
                    .css(
                        Style::new()
                            .display(Display::Flex)
                            .justify_content(JustifyContent::SpaceBetween)
                            .align_items(AlignItems::Center),
                    )
                    .child(
                        p().bind_text(
                            items
                                .count(|todo| !todo.at(Todo::done()))
                                .to_text()
                                .concat(" left"),
                        ),
                    )
                    .child(
                        button(ButtonVariant::Outline, ButtonSize::Sm)
                            .on_click(items.retain(|todo| !todo.at(Todo::done())))
                            .text("Clear done"),
                    ),
            ),
    )
}
