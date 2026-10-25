//! TodoMVC-2000 in Dioxus 0.7.10 fullstack, server-rendered and hydrated.
//!
//! Pattern: Dioxus's `todomvc_store` example for the list, and its
//! `custom_axum_serve` example for the server. `use_server_future` calls the
//! `get_todos` server function, which runs on the server for the first
//! response, and the browser reads its serialized result while hydrating. The
//! list is a `Store` derived on `TodoState`, so each `TodoEntry` component
//! subscribes only to its own item and a toggle re-renders that item. The
//! count is a `use_memo` over the store.
//!
//! Each add, toggle, and delete updates the store at once and spawns a task
//! that calls that change's `#[server]` function, without awaiting it.
//!
//! Choices a reader might dispute:
//! - The items are a `BTreeMap` keyed by id, where the example has a
//!   `HashMap`, so they keep the order of their ids without sorting.
//! - The browser picks the id of a new todo (the largest id in the list plus
//!   one) and sends it, so the item shown and the item stored agree, as in the
//!   React app.
//! - A toggle sends the new `done` value rather than "flip", so it is idempotent.
//! - Filtered-out items are left out of the list, as in the example.

use std::collections::BTreeMap;

use dioxus::prelude::*;
use serde::{Deserialize, Serialize};

const STYLE: &str = include_str!("../../shared/todomvc.css");

/// A todo as the server stores it and as server functions send it.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
struct TodoData {
    id: u32,
    title: String,
    done: bool,
}

#[cfg(feature = "server")]
mod store {
    use std::sync::{LazyLock, Mutex, MutexGuard};

    use super::TodoData;

    static TODOS: LazyLock<Mutex<Vec<TodoData>>> = LazyLock::new(|| Mutex::new(seed()));

    pub fn seed() -> Vec<TodoData> {
        (1..=2000)
            .map(|id| TodoData {
                id,
                title: format!("todo {id}"),
                done: id % 3 == 0,
            })
            .collect()
    }

    pub fn lock() -> MutexGuard<'static, Vec<TodoData>> {
        TODOS.lock().expect("todo lock poisoned")
    }
}

#[server]
async fn get_todos() -> Result<Vec<TodoData>> {
    Ok(store::lock().clone())
}

#[server]
async fn add_todo(id: u32, title: String) -> Result<()> {
    let title = title.trim();
    if !title.is_empty() {
        store::lock().push(TodoData {
            id,
            title: title.to_owned(),
            done: false,
        });
    }
    Ok(())
}

#[server]
async fn set_done(id: u32, done: bool) -> Result<()> {
    if let Some(todo) = store::lock().iter_mut().find(|todo| todo.id == id) {
        todo.done = done;
    }
    Ok(())
}

#[server]
async fn delete_todo(id: u32) -> Result<()> {
    store::lock().retain(|todo| todo.id != id);
    Ok(())
}

fn main() {
    #[cfg(not(feature = "server"))]
    dioxus::launch(App);

    #[cfg(feature = "server")]
    dioxus::serve(|| async move {
        use dioxus::server::axum::{http::StatusCode, routing::post};

        Ok(dioxus::server::router(App).route(
            "/bench-reset",
            post(|| async {
                *store::lock() = store::seed();
                StatusCode::NO_CONTENT
            }),
        ))
    });
}

#[derive(Store, PartialEq, Clone, Debug)]
struct TodoState {
    todos: BTreeMap<u32, TodoItem>,
    filter: Filter,
}

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
enum Filter {
    All,
    Active,
    Completed,
}

#[derive(Store, PartialEq, Clone, Debug)]
struct TodoItem {
    done: bool,
    title: String,
}

#[store]
impl<Lens> Store<TodoState, Lens> {
    fn shown_ids(&self) -> Vec<u32> {
        let filter = self.filter().cloned();
        self.todos()
            .iter()
            .filter_map(|(id, item)| {
                let done = item.done().cloned();
                let shown = match filter {
                    Filter::All => true,
                    Filter::Active => !done,
                    Filter::Completed => done,
                };
                shown.then_some(id)
            })
            .collect()
    }

    fn left(&self) -> usize {
        self.todos()
            .values()
            .filter(|item| !item.done().cloned())
            .count()
    }
}

#[component]
fn App() -> Element {
    let initial = use_server_future(get_todos)?;
    let list = initial().and_then(Result::ok).unwrap_or_default();
    rsx! {
        document::Title { "Dioxus TodoMVC" }
        document::Link { rel: "icon", href: "data:," }
        document::Style { {STYLE} }
        TodoMvc { list }
    }
}

#[component]
fn TodoMvc(list: Vec<TodoData>) -> Element {
    let todos = use_store(|| TodoState {
        todos: list
            .iter()
            .map(|todo| {
                (
                    todo.id,
                    TodoItem {
                        done: todo.done,
                        title: todo.title.clone(),
                    },
                )
            })
            .collect(),
        filter: Filter::All,
    });
    let left = use_memo(move || todos.left());
    let shown = use_memo(move || todos.shown_ids());

    rsx! {
        section { class: "todoapp",
            TodoHeader { todos }
            section { class: "main",
                ul { class: "todo-list",
                    for id in shown() {
                        TodoEntry { key: "{id}", id, todos }
                    }
                }
            }
            ListFooter { left, todos }
        }
    }
}

#[component]
fn TodoHeader(mut todos: Store<TodoState>) -> Element {
    let mut draft = use_signal(String::new);

    let onkeydown = move |event: KeyboardEvent| {
        if event.key() != Key::Enter {
            return;
        }
        let title = draft.read().trim().to_owned();
        if title.is_empty() {
            return;
        }
        let id = todos.todos().iter().map(|(id, _)| id).max().unwrap_or(0) + 1;
        todos.todos().insert(
            id,
            TodoItem {
                done: false,
                title: title.clone(),
            },
        );
        draft.set(String::new());
        spawn(async move {
            _ = add_todo(id, title).await;
        });
    };

    rsx! {
        header {
            h1 { "todos" }
            input {
                class: "new-todo",
                placeholder: "What needs to be done?",
                autocomplete: "off",
                autofocus: true,
                value: "{draft}",
                oninput: move |event| draft.set(event.value()),
                onkeydown,
            }
        }
    }
}

#[component]
fn TodoEntry(mut todos: Store<TodoState>, id: u32) -> Element {
    // Reading one item out of the store subscribes this component to that item only.
    let entry = todos.todos().get(id).unwrap();
    let done = entry.done();
    let title = entry.title();

    // A toggle under a filter and every delete unmount this component, and
    // `spawn` would cancel its task with it, so the requests use `spawn_forever`.
    rsx! {
        li { class: if done() { "completed" },
            input {
                class: "toggle",
                r#type: "checkbox",
                checked: done(),
                oninput: move |event| {
                    let now = event.checked();
                    entry.done().set(now);
                    dioxus::core::spawn_forever(async move {
                        _ = set_done(id, now).await;
                    });
                },
            }
            label { "{title}" }
            button {
                class: "destroy",
                onclick: move |_| {
                    todos.todos().remove(&id);
                    dioxus::core::spawn_forever(async move {
                        _ = delete_todo(id).await;
                    });
                },
            }
        }
    }
}

#[component]
fn ListFooter(todos: Store<TodoState>, left: ReadSignal<usize>) -> Element {
    let mut filter = todos.filter();

    rsx! {
        footer { class: "footer",
            span { class: "todo-count",
                strong { "{left}" }
                if left() == 1 { " item left" } else { " items left" }
            }
            ul { class: "filters",
                for (value, text, href) in [
                    (Filter::All, "All", "#/"),
                    (Filter::Active, "Active", "#/active"),
                    (Filter::Completed, "Completed", "#/completed"),
                ] {
                    li {
                        a {
                            href,
                            class: if filter() == value { "selected" },
                            onclick: move |event| {
                                event.prevent_default();
                                filter.set(value);
                            },
                            {text}
                        }
                    }
                }
            }
        }
    }
}
