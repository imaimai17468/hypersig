//! [TodoMVC](https://todomvc.com/), with every interaction handled in the browser.
//!
//! The list lives in a `Vec` signal. The server only renders the first page
//! and stores the list whenever it changes, so a reload keeps it.
//!
//! Run with `cargo run -p example-todomvc` and open <http://localhost:3002>.

use std::sync::{Arc, Mutex};

use axum::Router;
use axum::extract::State;
use axum::response::Html;
use axum::routing::get;
use hypersig::html::{
    El, Key, a, button, each_keyed, footer, h1, header, input, label, li, section, span, strong,
    ul, when,
};
use hypersig::{Action, Expr, NodeRef, Page, Patches, RouterExt, Signals, Stmt, if_else};
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
    next_id: i32,
    draft: String,
    filter: Filter,
    editing: Option<i32>,
    edit_text: String,
    left: i32,
    completed: i32,
}

const SAVE: Action<TodoMvc> = Action::post("/save");
const EDIT: NodeRef<TodoMvc> = NodeRef::new("edit");

#[derive(Clone, Default)]
struct AppState {
    todos: Arc<Mutex<Vec<Todo>>>,
}

fn new_todo() -> El<TodoMvc> {
    let todos = TodoMvc::todos();
    let draft = TodoMvc::draft();
    let next_id = TodoMvc::next_id();
    let blank = Expr::lit(&Todo {
        id: 0,
        title: String::new(),
        done: false,
    });
    let add = Stmt::when(
        !draft.trim().is_empty(),
        todos
            .push(
                blank
                    .with(Todo::id(), next_id)
                    .with(Todo::title(), draft.trim()),
            )
            .then(next_id.set(next_id + 1))
            .then(draft.set("")),
    );
    input()
        .class("new-todo")
        .attr("placeholder", "What needs to be done?")
        .attr("autofocus", "")
        .bind(draft)
        .on_key(Key::Enter, add)
}

fn todo_list() -> El<TodoMvc> {
    let todos = TodoMvc::todos();
    let filter = TodoMvc::filter();
    let editing = TodoMvc::editing();
    let edit_text = TodoMvc::edit_text();
    ul().class("todo-list").child(each_keyed(
        todos,
        |todo| todo.at(Todo::id()),
        |todo| {
            let id = todo.at(Todo::id());
            let title = todo.at(Todo::title());
            let done = todo.at(Todo::done());
            let is_editing = editing.equals(id.some());
            let visible = filter
                .equals(&Filter::All)
                .or(filter.equals(&Filter::Active).and(!done))
                .or(filter.equals(&Filter::Completed).and(done));
            let commit = Stmt::when(
                is_editing.clone(),
                Stmt::if_else(
                    edit_text.trim().is_empty(),
                    todo.remove(),
                    title.set(edit_text.trim()),
                )
                .then(editing.set(&None)),
            );
            li().show(visible)
                .class_when("completed", done)
                .class_when("editing", is_editing)
                .child(input().class("toggle").attr("type", "checkbox").bind(done))
                .child(
                    label().bind_text(title).on_double_click(
                        edit_text
                            .set(title)
                            .then(editing.set(id.some()))
                            .then(EDIT.focus()),
                    ),
                )
                .child(button().class("destroy").on_click(todo.remove()))
                .child(when(
                    editing.equals(id.some()),
                    input()
                        .class("edit")
                        .node_ref(EDIT)
                        .bind(edit_text)
                        .on_key(Key::Enter, commit.clone())
                        .on_key(Key::Escape, editing.set(&None))
                        .on_blur(commit),
                ))
        },
    ))
}

fn filter_link(filter: Filter, text: &str) -> El<TodoMvc> {
    li().child(
        a().attr("href", "#")
            .class_when("selected", TodoMvc::filter().equals(&filter))
            .on_click(TodoMvc::filter().set(&filter))
            .text(text),
    )
}

fn page(todos: Vec<Todo>) -> Page<TodoMvc> {
    let list = TodoMvc::todos();
    let left = TodoMvc::left();
    let completed = TodoMvc::completed();
    let next_id = todos.iter().map(|todo| todo.id).max().unwrap_or(0) + 1;
    let signals = TodoMvc {
        todos,
        next_id,
        draft: String::new(),
        filter: Filter::All,
        editing: None,
        edit_text: String::new(),
        left: 0,
        completed: 0,
    };
    Page::new("hypersig TodoMVC", signals)
        .style(STYLE)
        .computed(left, list.count(|todo| !todo.at(Todo::done())))
        .computed(completed, list.count(|todo| todo.at(Todo::done())))
        .child(
            section()
                .class("todoapp")
                .on_signal_change(list, SAVE.call())
                .child(header().child(h1().text("todos")).child(new_todo()))
                .child(
                    section()
                        .class("main")
                        .show(!list.is_empty())
                        .child(
                            input()
                                .id("toggle-all")
                                .class("toggle-all")
                                .attr("type", "checkbox")
                                .bind_checked(left.equals(0))
                                .on_change(list.set_all(Todo::done(), left.gt(0))),
                        )
                        .child(
                            label()
                                .attr("for", "toggle-all")
                                .text("Mark all as complete"),
                        )
                        .child(todo_list()),
                )
                .child(
                    footer()
                        .class("footer")
                        .show(!list.is_empty())
                        .child(
                            span()
                                .class("todo-count")
                                .child(strong().bind_text(left.to_text()))
                                .child(span().bind_text(if_else(
                                    left.equals(1),
                                    " item left",
                                    " items left",
                                ))),
                        )
                        .child(
                            ul().class("filters")
                                .child(filter_link(Filter::All, "All"))
                                .child(filter_link(Filter::Active, "Active"))
                                .child(filter_link(Filter::Completed, "Completed")),
                        )
                        .child(
                            button()
                                .class("clear-completed")
                                .show(completed.gt(0))
                                .on_click(list.retain(|todo| !todo.at(Todo::done())))
                                .text("Clear completed"),
                        ),
                ),
        )
}

async fn index(State(state): State<AppState>) -> Html<String> {
    let todos = state.todos.lock().expect("todo lock poisoned").clone();
    Html(page(todos).render())
}

async fn save(state: AppState, signals: TodoMvc, (): ()) -> Patches<TodoMvc> {
    *state.todos.lock().expect("todo lock poisoned") = signals.todos;
    Patches::new()
}

const STYLE: &str = "
* { box-sizing: border-box; }
body { font: 14px 'Helvetica Neue', Helvetica, Arial, sans-serif; background: #f5f5f5; color: #111; margin: 0; }
.todoapp { background: #fff; max-width: 550px; margin: 130px auto 40px; position: relative; box-shadow: 0 2px 4px rgba(0,0,0,.2), 0 25px 50px rgba(0,0,0,.1); }
.todoapp h1 { position: absolute; top: -110px; width: 100%; font-size: 80px; font-weight: 200; text-align: center; color: #b83f45; margin: 0; }
.new-todo, .edit { width: 100%; font-size: 24px; padding: 16px 16px 16px 60px; border: none; outline: none; }
.new-todo { box-shadow: inset 0 -2px 1px rgba(0,0,0,.03); }
.main { position: relative; border-top: 1px solid #e6e6e6; }
.toggle-all { position: absolute; opacity: 0; }
.toggle-all + label { position: absolute; top: -52px; left: 0; width: 45px; height: 65px; font-size: 0; display: flex; align-items: center; justify-content: center; }
.toggle-all + label::before { content: '❯'; font-size: 22px; color: #949494; transform: rotate(90deg); }
.toggle-all:checked + label::before { color: #484848; }
.todo-list { margin: 0; padding: 0; list-style: none; }
.todo-list li { position: relative; font-size: 24px; border-bottom: 1px solid #ededed; display: flex; align-items: center; }
.todo-list li .toggle { width: 40px; height: 40px; margin: 0 10px; }
.todo-list li label { flex: 1; padding: 15px; word-break: break-all; transition: color .2s; }
.todo-list li.completed label { color: #949494; text-decoration: line-through; }
.todo-list li .destroy { display: none; border: none; background: none; font-size: 30px; color: #949494; margin-right: 10px; cursor: pointer; }
.todo-list li .destroy::after { content: '×'; }
.todo-list li:hover .destroy { display: block; }
.todo-list li.editing .toggle, .todo-list li.editing label, .todo-list li.editing .destroy { display: none; }
.todo-list li .edit { margin-left: 43px; padding: 12px 16px; border: 1px solid #999; box-shadow: inset 0 -1px 5px rgba(0,0,0,.2); }
.footer { padding: 10px 15px; display: flex; align-items: center; justify-content: space-between; border-top: 1px solid #e6e6e6; color: #111; }
.filters { margin: 0; padding: 0; list-style: none; display: flex; gap: 4px; }
.filters a { color: inherit; text-decoration: none; padding: 3px 7px; border: 1px solid transparent; border-radius: 3px; }
.filters a.selected { border-color: #ce4646; }
.clear-completed { border: none; background: none; cursor: pointer; color: inherit; }
";

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let app = Router::new()
        .route("/", get(index))
        .hypersig_runtime()
        .action(SAVE, save)
        .with_state(AppState::default());
    let addr = std::env::var("ADDR").unwrap_or_else(|_| "127.0.0.1:3002".to_owned());
    let listener = tokio::net::TcpListener::bind(&addr).await?;
    println!("listening on http://{addr}");
    axum::serve(listener, app).await
}
