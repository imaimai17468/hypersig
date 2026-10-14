// The example in README.md, kept compiling.

use hypersig::html::{Key, button, div, each, input, li, span, ul};
use hypersig::{Expr, Signals, Stmt};
use serde::{Deserialize, Serialize};

#[derive(Signals, Serialize, Deserialize, Clone, PartialEq)]
struct Todo {
    id: i32,
    title: String,
    done: bool,
}

#[derive(Signals, Serialize, Deserialize)]
struct Todos {
    todos: Vec<Todo>,
    draft: String,
    next_id: i32,
}

fn app() -> hypersig::html::El<Todos> {
    let (todos, draft, next_id) = (Todos::todos(), Todos::draft(), Todos::next_id());
    let blank = Expr::lit(&Todo { id: 0, title: String::new(), done: false });
    let add = Stmt::when(
        !draft.trim().is_empty(),
        todos
            .push(blank.with(Todo::id(), next_id).with(Todo::title(), draft.trim()))
            .then(next_id.set(next_id + 1))
            .then(draft.set("")),
    );
    div()
        .child(input().bind(draft).on_key(Key::Enter, add))
        .child(ul().child(each(todos, |todo| {
            li().key(todo.at(Todo::id()))
                .class_when("done", todo.at(Todo::done()))
                .child(input().attr("type", "checkbox").bind(todo.at(Todo::done())))
                .child(span().bind_text(todo.at(Todo::title())))
                .child(button().on_click(todo.remove()).text("×"))
        })))
        .child(span().bind_text(todos.count(|todo| !todo.at(Todo::done())).to_text()))
}

fn main() {
    let _ = app().render();
}
