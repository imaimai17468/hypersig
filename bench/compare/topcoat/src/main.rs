//! TodoMVC-2000 in Topcoat 0.10.0.
//!
//! Pattern: a `#[shard]` rendered on the server from the server's store. The
//! shard creates a `revision` signal and reads it in server code, so a change
//! to it in the browser re-renders only the shard. Each change is an event
//! handler that awaits a `#[procedure]` with that change's typed arguments and
//! then increments `revision`, so the shard renders the new list and count:
//!
//! - Add: `input.new-todo` binds a `draft` signal (`:value` and `@input`).
//!   Enter in its `@keydown` handler calls `add_todo(title)` and clears the
//!   signal. Blank text calls nothing, and the procedure trims it again.
//! - Toggle: `@change` on `input.toggle` calls `toggle_todo(id)`.
//! - Delete: `@click` on `button.destroy` calls `delete_todo(id)`.
//!
//! The loop is keyed by `#[key(todo.id)]`, and each `li` has the todo's id as
//! its `id`, so the browser matches elements to items when it patches the
//! re-rendered shard in place.
//!
//! Filtering stays in the browser, as in the other apps: the `.filters a`
//! handlers set a `filter` signal, `:data-filter` binds it onto the list, and
//! CSS hides the other items with `display: none`. The shard reads `filter`
//! only inside `$(...)`, so changing it renders nothing on the server.
//!
//! Choice a reader might dispute: every add, toggle, and delete re-renders all
//! ~2000 items on the server and sends them to the browser, which patches the
//! list in place. Runtime expressions have no `Vec` operations that change a
//! list, and a view's `for` loop runs on the server, so a list in Topcoat
//! changes only through a server render.

use std::sync::Mutex;

use tokio::net::TcpListener;
use topcoat::{
    Result,
    asset::{AssetBundle, RouterBuilderAssetExt},
    context::{Cx, app_context},
    router::{RouterBuilderDiscoverExt, StatusCode, module_router, page, route},
    runtime::{Event, RouterBuilderRuntimeExt, procedure, shard, signal},
    view::{View, view},
};

const STYLE: &str = include_str!("../../shared/todomvc.css");
const FILTER_STYLE: &str = ".todo-list[data-filter=active] li.completed, \
.todo-list[data-filter=completed] li:not(.completed) { display: none; }";

#[derive(Clone)]
struct Todo {
    id: u32,
    title: String,
    done: bool,
}

struct Todos {
    items: Vec<Todo>,
    next_id: u32,
}

struct Store(Mutex<Todos>);

impl Store {
    fn lock(&self) -> std::sync::MutexGuard<'_, Todos> {
        self.0.lock().expect("todo lock poisoned")
    }
}

fn seed() -> Todos {
    Todos {
        items: (1..=2000)
            .map(|id| Todo {
                id,
                title: format!("todo {id}"),
                done: id % 3 == 0,
            })
            .collect(),
        next_id: 2001,
    }
}

#[page]
async fn home() -> Result<impl View> {
    Ok(view! {
        <!DOCTYPE html>
        <html lang="en">
            <head>
                <meta charset="utf-8">
                <meta name="viewport" content="width=device-width, initial-scale=1">
                <title>"Topcoat TodoMVC"</title>
                <link rel="icon" href="data:,">
                <style>
                    (STYLE)
                    (FILTER_STYLE)
                </style>
                topcoat::runtime::script()
            </head>
            <body><section class="todoapp">todo_app()</section></body>
        </html>
    })
}

#[shard]
async fn todo_app(cx: &Cx) -> Result<impl View> {
    let revision = signal(cx, || 0u32);
    let draft = signal(cx, String::new);
    let filter = signal(cx, || "all".to_owned());

    // The tracked read: incrementing `revision` in the browser re-renders this shard.
    let revision_seen = revision.get();
    // The lock guard is not held across the view, which renders lazily.
    let (todos, left) = {
        let store = app_context::<Store>(cx).lock();
        let left = store.items.iter().filter(|todo| !todo.done).count();
        (store.items.clone(), left)
    };

    Ok(view! {
        <header>
            <h1>"todos"</h1>
            <input
                class="new-todo"
                placeholder="What needs to be done?"
                autocomplete="off"
                autofocus=""
                :value=$(draft.get())
                @input=$(|e: Event| draft.set(e.target.value))
                @keydown=$(async |e: Event| {
                    if e.key == "Enter" {
                        let title = draft.get();
                        if !title.trim().is_empty() {
                            draft.set("".to_owned());
                            add_todo(title).await;
                            revision.increment();
                        }
                    }
                })
            >
        </header>
        <section class="main">
            <ul
                class="todo-list"
                data-revision=(revision_seen)
                :data-filter=$(filter.get())
            >
                #[key(todo.id)]
                for todo in todos {
                    let id = todo.id;
                    <li id=(id) class=(todo.done.then_some("completed"))>
                        <input
                            class="toggle"
                            type="checkbox"
                            checked=(todo.done)
                            @change=$(async |_e| {
                                toggle_todo(id).await;
                                revision.increment();
                            })
                        >
                        <label>(todo.title)</label>
                        <button
                            class="destroy"
                            @click=$(async |_e| {
                                delete_todo(id).await;
                                revision.increment();
                            })
                        ></button>
                    </li>
                }
            </ul>
        </section>
        <footer class="footer">
            <span class="todo-count">
                <strong>(left)</strong>
                if left == 1 {
                    " item left"
                } else {
                    " items left"
                }
            </span>
            <ul class="filters">
                <li>
                    <a
                        href="#/"
                        :class=$(if filter.get() == "all" { "selected" } else { "" })
                        @click=$(|e: Event| {
                            e.prevent_default();
                            filter.set("all".to_owned());
                        })
                    >
                        "All"
                    </a>
                </li>
                <li>
                    <a
                        href="#/active"
                        :class=$(if filter.get() == "active" { "selected" } else { "" })
                        @click=$(|e: Event| {
                            e.prevent_default();
                            filter.set("active".to_owned());
                        })
                    >
                        "Active"
                    </a>
                </li>
                <li>
                    <a
                        href="#/completed"
                        :class=$(if filter.get() == "completed" {
                            "selected"
                        } else {
                            ""
                        })
                        @click=$(|e: Event| {
                            e.prevent_default();
                            filter.set("completed".to_owned());
                        })
                    >
                        "Completed"
                    </a>
                </li>
            </ul>
        </footer>
    })
}

#[procedure]
async fn add_todo(cx: &Cx, title: String) -> Result<()> {
    let title = title.trim();
    if title.is_empty() {
        return Ok(());
    }
    let mut todos = app_context::<Store>(cx).lock();
    let id = todos.next_id;
    todos.next_id += 1;
    todos.items.push(Todo {
        id,
        title: title.to_owned(),
        done: false,
    });
    Ok(())
}

#[procedure]
async fn toggle_todo(cx: &Cx, id: u32) -> Result<()> {
    let mut todos = app_context::<Store>(cx).lock();
    if let Some(todo) = todos.items.iter_mut().find(|todo| todo.id == id) {
        todo.done = !todo.done;
    }
    Ok(())
}

#[procedure]
async fn delete_todo(cx: &Cx, id: u32) -> Result<()> {
    app_context::<Store>(cx)
        .lock()
        .items
        .retain(|todo| todo.id != id);
    Ok(())
}

#[route(POST "/bench-reset")]
async fn reset(cx: &Cx) -> Result<StatusCode> {
    *app_context::<Store>(cx).lock() = seed();
    Ok(StatusCode::NO_CONTENT)
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let router = module_router!()
        .app_context(Store(Mutex::new(seed())))
        .discover()
        .assets(AssetBundle::load().expect("run `topcoat asset bundle --release` first"))
        .runtime()
        .build();
    let listener = TcpListener::bind("127.0.0.1:4104").await?;
    topcoat::serve(listener, router).await
}
