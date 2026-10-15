//! TodoMVC-2000 in Leptos 0.8, server rendering with hydration.
//!
//! Pattern: the server renders the whole list into the first HTML response
//! (`OnceResource::new_blocking` read under `<Suspense>`, so the list is in the
//! HTML rather than streamed in later) and the browser hydrates it. Each row
//! holds its `done` flag in its own `ArcRwSignal`, so a toggle re-renders only
//! that row; the list is a `RwSignal<Vec<Item>>` rendered with `<For>` keyed
//! by id. Add, toggle, and delete update the signals at once and then call a
//! `#[server]` function through a `ServerAction`, which applies the same change
//! on the server. Filtering is a browser-only signal; the count is a `Memo`.
//!
//! Choices a reader might dispute:
//! - Rows use `ArcRwSignal` rather than arena `RwSignal`, so a deleted row's
//!   signal is freed with the row instead of living until its owner is dropped.
//! - Hidden rows get `display: none` instead of being removed from `each`, so
//!   switching filters does not rebuild up to 2000 rows of DOM.
//! - The browser picks the id of a new todo (largest id + 1) and sends it to the
//!   server, so the row can be keyed before the server answers.
//! - The checkbox sets the `checked` attribute once (so the server HTML shows
//!   completed rows ticked before hydration) and `prop:checked` reactively.
//! - The 2000 todos appear twice in the HTML: as rendered rows and as the
//!   serialized `OnceResource` value that hydration reads. That is how Leptos
//!   hands server data to the client.
//! - The server keeps the list in a process-wide `static` behind a `Mutex`
//!   instead of passing state through Leptos context; the benchmark has a single
//!   list and `/bench-reset` is a plain axum route that shares it.

use leptos::prelude::*;
use serde::{Deserialize, Serialize};

const STYLE: &str = include_str!("../../shared/todomvc.css");

/// One todo as the server stores it and sends it to the browser.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct Todo {
    pub id: u32,
    pub title: String,
    pub done: bool,
}

#[cfg(feature = "ssr")]
pub mod store {
    use std::sync::{LazyLock, Mutex, MutexGuard};

    use super::Todo;

    static TODOS: LazyLock<Mutex<Vec<Todo>>> = LazyLock::new(|| Mutex::new(seed()));

    pub fn seed() -> Vec<Todo> {
        (1..=2000)
            .map(|id| Todo {
                id,
                title: format!("todo {id}"),
                done: id % 3 == 0,
            })
            .collect()
    }

    pub fn todos() -> MutexGuard<'static, Vec<Todo>> {
        TODOS.lock().expect("todo lock poisoned")
    }

    pub fn reseed() {
        *todos() = seed();
    }
}

#[server]
pub async fn get_todos() -> Result<Vec<Todo>, ServerFnError> {
    Ok(store::todos().clone())
}

#[server]
pub async fn add_todo(id: u32, title: String) -> Result<(), ServerFnError> {
    store::todos().push(Todo {
        id,
        title,
        done: false,
    });
    Ok(())
}

#[server]
pub async fn set_done(id: u32, done: bool) -> Result<(), ServerFnError> {
    if let Some(todo) = store::todos().iter_mut().find(|todo| todo.id == id) {
        todo.done = done;
    }
    Ok(())
}

#[server]
pub async fn delete_todo(id: u32) -> Result<(), ServerFnError> {
    store::todos().retain(|todo| todo.id != id);
    Ok(())
}

/// One row in the browser: the title never changes, `done` is the row's signal.
#[derive(Clone)]
struct Item {
    id: u32,
    title: String,
    done: ArcRwSignal<bool>,
}

impl Item {
    fn new(todo: Todo) -> Self {
        Self {
            id: todo.id,
            title: todo.title,
            done: ArcRwSignal::new(todo.done),
        }
    }
}

#[derive(Clone, Copy, PartialEq)]
enum Filter {
    All,
    Active,
    Completed,
}

impl Filter {
    fn shows(self, done: bool) -> bool {
        match self {
            Filter::All => true,
            Filter::Active => !done,
            Filter::Completed => done,
        }
    }
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
                <AutoReload options=options.clone()/>
                <HydrationScripts options/>
            </head>
            <body>
                <App/>
            </body>
        </html>
    }
}

#[component]
pub fn App() -> impl IntoView {
    let initial = OnceResource::new_blocking(get_todos());
    view! {
        <section class="todoapp">
            <Suspense>
                {move || Suspend::new(async move {
                    let todos = initial.await.unwrap_or_default();
                    view! { <TodoApp todos/> }
                })}
            </Suspense>
        </section>
    }
}

#[component]
fn TodoApp(todos: Vec<Todo>) -> impl IntoView {
    let items = RwSignal::new(todos.into_iter().map(Item::new).collect::<Vec<_>>());
    let filter = RwSignal::new(Filter::All);
    let left = Memo::new(move |_| {
        items.with(|items| items.iter().filter(|item| !item.done.get()).count())
    });

    let add = ServerAction::<AddTodo>::new();
    let toggle = ServerAction::<SetDone>::new();
    let delete = ServerAction::<DeleteTodo>::new();

    let new_todo = NodeRef::<leptos::html::Input>::new();
    let on_keydown = move |ev: leptos::ev::KeyboardEvent| {
        if ev.key() != "Enter" {
            return;
        }
        let Some(input) = new_todo.get_untracked() else {
            return;
        };
        let title = input.value().trim().to_string();
        if title.is_empty() {
            return;
        }
        input.set_value("");
        let id = items.with(|items| items.iter().map(|item| item.id).max().unwrap_or(0)) + 1;
        items.update(|items| {
            items.push(Item::new(Todo {
                id,
                title: title.clone(),
                done: false,
            }))
        });
        add.dispatch(AddTodo { id, title });
    };

    let filter_link = move |value: Filter, text: &'static str| {
        view! {
            <li>
                <a
                    href="#"
                    class:selected=move || filter.get() == value
                    on:click=move |ev| {
                        ev.prevent_default();
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
            <input class="new-todo" placeholder="What needs to be done?" autofocus node_ref=new_todo on:keydown=on_keydown/>
        </header>
        <section class="main">
            <ul class="todo-list">
                <For each=move || items.get() key=|item| item.id let(item)>
                    {
                        let Item { id, title, done } = item;
                        let shown = {
                            let done = done.clone();
                            move || filter.get().shows(done.get())
                        };
                        let completed = done.clone();
                        let checked = done.clone();
                        view! {
                            <li
                                class:completed=move || completed.get()
                                style:display=move || (!shown()).then_some("none")
                            >
                                    <input
                                        class="toggle"
                                        type="checkbox"
                                        checked=checked.get_untracked()
                                        prop:checked=move || checked.get()
                                        on:change=move |ev| {
                                            let value = event_target_checked(&ev);
                                            done.set(value);
                                            toggle.dispatch(SetDone { id, done: value });
                                        }
                                    />
                                    <label>{title}</label>
                                    <button
                                        class="destroy"
                                        on:click=move |_| {
                                            items.update(|items| items.retain(|item| item.id != id));
                                            delete.dispatch(DeleteTodo { id });
                                        }
                                    ></button>
                            </li>
                        }
                    }
                </For>
            </ul>
        </section>
        <footer class="footer">
            <span class="todo-count">
                <strong>{left}</strong>
                <span>{move || if left.get() == 1 { " item left" } else { " items left" }}</span>
            </span>
            <ul class="filters">
                {filter_link(Filter::All, "All")}
                {filter_link(Filter::Active, "Active")}
                {filter_link(Filter::Completed, "Completed")}
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
