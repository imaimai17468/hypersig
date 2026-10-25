//! TodoMVC-2000 in Leptos 0.8.22 with axum, server-rendered and hydrated.
//!
//! Pattern: Leptos's `todomvc` example for the list, and its
//! `todo_app_sqlite_axum` example for the server. A `Resource` calls the
//! `get_todos` server function, and `<Suspense>` renders the app once it
//! resolves, on the server for the first response and again in the browser
//! from the serialized resource while hydrating. The app turns the loaded list
//! into a `RwSignal<Vec<Todo>>` whose items hold their `done` state in their
//! own `RwSignal<bool>`, so a toggle updates one item, and `<For>` keys the
//! items by id. The count is a `Memo` over the items' signals.
//!
//! Each add, toggle, and delete updates the signals at once and dispatches a
//! `ServerAction` for that change's `#[server]` function, without awaiting it.
//!
//! Choices a reader might dispute:
//! - The browser picks the id of a new todo (the largest id seen plus one) and
//!   sends it, so the item shown and the item stored agree, as in the React app.
//! - A toggle sends the new `done` value rather than "flip", so it is idempotent.
//! - Filtered-out items are left out of `<For>`, as in Leptos's `todomvc`.

use leptos::prelude::*;
use serde::{Deserialize, Serialize};

const STYLE: &str = include_str!("../../shared/todomvc.css");

/// A todo as the server stores it and as server functions send it.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct TodoData {
    pub id: u32,
    pub title: String,
    pub done: bool,
}

#[cfg(feature = "ssr")]
pub mod store {
    use std::sync::Mutex;

    use super::TodoData;

    pub static TODOS: Mutex<Vec<TodoData>> = Mutex::new(Vec::new());

    pub fn seed() -> Vec<TodoData> {
        (1..=2000)
            .map(|id| TodoData {
                id,
                title: format!("todo {id}"),
                done: id % 3 == 0,
            })
            .collect()
    }

    pub fn lock() -> std::sync::MutexGuard<'static, Vec<TodoData>> {
        TODOS.lock().expect("todo lock poisoned")
    }
}

#[server]
pub async fn get_todos() -> Result<Vec<TodoData>, ServerFnError> {
    Ok(store::lock().clone())
}

#[server]
pub async fn add_todo(id: u32, title: String) -> Result<(), ServerFnError> {
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
pub async fn set_done(id: u32, done: bool) -> Result<(), ServerFnError> {
    if let Some(todo) = store::lock().iter_mut().find(|todo| todo.id == id) {
        todo.done = done;
    }
    Ok(())
}

#[server]
pub async fn delete_todo(id: u32) -> Result<(), ServerFnError> {
    store::lock().retain(|todo| todo.id != id);
    Ok(())
}

pub fn shell(options: LeptosOptions) -> impl IntoView {
    view! {
        <!DOCTYPE html>
        <html lang="en">
            <head>
                <meta charset="utf-8"/>
                <meta name="viewport" content="width=device-width, initial-scale=1"/>
                <title>"Leptos TodoMVC"</title>
                <link rel="icon" href="data:,"/>
                <style>{STYLE}</style>
                <HydrationScripts options/>
            </head>
            <body>
                <App/>
            </body>
        </html>
    }
}

#[derive(Clone, Copy, PartialEq)]
enum Filter {
    All,
    Active,
    Completed,
}

#[derive(Clone, Copy)]
struct Todo {
    id: u32,
    title: StoredValue<String>,
    done: RwSignal<bool>,
}

impl Todo {
    fn new(data: TodoData) -> Self {
        Self {
            id: data.id,
            title: StoredValue::new(data.title),
            done: RwSignal::new(data.done),
        }
    }
}

#[component]
pub fn App() -> impl IntoView {
    let initial = Resource::new(|| (), |()| get_todos());
    view! {
        <section class="todoapp">
            <Suspense>
                {move || Suspend::new(async move {
                    let list = initial.await.unwrap_or_default();
                    view! { <TodoMvc list/> }
                })}
            </Suspense>
        </section>
    }
}

#[component]
fn TodoMvc(list: Vec<TodoData>) -> impl IntoView {
    let next_id = StoredValue::new(list.iter().map(|todo| todo.id).max().unwrap_or(0) + 1);
    let todos = RwSignal::new(list.into_iter().map(Todo::new).collect::<Vec<_>>());
    let filter = RwSignal::new(Filter::All);
    let draft = RwSignal::new(String::new());

    let add = ServerAction::<AddTodo>::new();
    let set = ServerAction::<SetDone>::new();
    let delete = ServerAction::<DeleteTodo>::new();

    let left =
        Memo::new(move |_| todos.with(|list| list.iter().filter(|todo| !todo.done.get()).count()));
    let shown = move || {
        let filter = filter.get();
        todos.with(|list| match filter {
            Filter::All => list.clone(),
            Filter::Active => list
                .iter()
                .filter(|todo| !todo.done.get())
                .copied()
                .collect(),
            Filter::Completed => list
                .iter()
                .filter(|todo| todo.done.get())
                .copied()
                .collect(),
        })
    };

    let on_keydown = move |event: leptos::ev::KeyboardEvent| {
        if event.key() != "Enter" {
            return;
        }
        let title = draft.get_untracked().trim().to_owned();
        if title.is_empty() {
            return;
        }
        let id = next_id.get_value();
        next_id.set_value(id + 1);
        todos.update(|list| {
            list.push(Todo::new(TodoData {
                id,
                title: title.clone(),
                done: false,
            }));
        });
        draft.set(String::new());
        add.dispatch(AddTodo { id, title });
    };

    let filter_link = move |value: Filter, href: &'static str, text: &'static str| {
        view! {
            <li>
                <a
                    href=href
                    class:selected=move || filter.get() == value
                    on:click=move |event| {
                        event.prevent_default();
                        filter.set(value);
                    }
                >
                    {text}
                </a>
            </li>
        }
    };

    view! {
        <header>
            <h1>"todos"</h1>
            <input
                class="new-todo"
                placeholder="What needs to be done?"
                autocomplete="off"
                autofocus
                prop:value=draft
                on:input=move |event| draft.set(event_target_value(&event))
                on:keydown=on_keydown
            />
        </header>
        <section class="main">
            <ul class="todo-list">
                <For each=shown key=|todo| todo.id let:todo>
                    <li class:completed=move || todo.done.get()>
                        <input
                            class="toggle"
                            type="checkbox"
                            checked=move || todo.done.get()
                            prop:checked=move || todo.done.get()
                            on:change=move |_| {
                                let done = !todo.done.get_untracked();
                                todo.done.set(done);
                                set.dispatch(SetDone { id: todo.id, done });
                            }
                        />
                        <label>{todo.title.get_value()}</label>
                        <button
                            class="destroy"
                            on:click=move |_| {
                                todos.update(|list| list.retain(|item| item.id != todo.id));
                                todo.done.dispose();
                                todo.title.dispose();
                                delete.dispatch(DeleteTodo { id: todo.id });
                            }
                        ></button>
                    </li>
                </For>
            </ul>
        </section>
        <footer class="footer">
            <span class="todo-count">
                <strong>{left}</strong>
                {move || if left.get() == 1 { " item left" } else { " items left" }}
            </span>
            <ul class="filters">
                {filter_link(Filter::All, "#/", "All")}
                {filter_link(Filter::Active, "#/active", "Active")}
                {filter_link(Filter::Completed, "#/completed", "Completed")}
            </ul>
        </footer>
    }
}

#[cfg(feature = "hydrate")]
#[wasm_bindgen::prelude::wasm_bindgen]
pub fn hydrate() {
    console_error_panic_hook::set_once();
    leptos::mount::hydrate_body(App);
}
