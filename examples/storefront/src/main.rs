//! A shop whose pages share a newsletter form, and whose wish list saves
//! each wish in its own call.
//!
//! The newsletter is a `Part`: its signals, its element, and its actions are
//! written once against `Newsletter`. The home page holds it at the field
//! `footer`, and the about page holds a `Shell` part that holds it at its own
//! field `footer`, so on that page its signals sit at `shell.footer`. One
//! handler per action serves both pages, and its patches land where the call
//! came from.
//!
//! The wish list adds a wish before the server answers, and marks only the
//! wish whose save failed, because each call names its own wish in
//! `Call::on_error` and `Call::on_success`.
//!
//! Run with `cargo run -p example-storefront` and open <http://localhost:3008>.

use std::collections::BTreeSet;
use std::sync::{Arc, Mutex};
use std::time::Duration;

use axum::Router;
use axum::http::StatusCode;
use hypersig::html::{
    El, Id, Role, Stylesheet, a, attr, button, div, each, footer, form, h1, h2, input, label, li,
    nav, p, part, span, ul,
};
use hypersig::{Action, Expr, Fragment, Page, Part, Patches, Route, RouterExt, Signals};
use serde::{Deserialize, Serialize};

hypersig::routes! {
    fn pages(Router<AppState>);
    const HOME: Route = "/" => home;
    const ABOUT: Route = "/about" => about;
    // The newsletter part's actions, which serve every page that embeds it.
    action JOIN: Action<Newsletter, String> = post "/newsletter/join" => join;
    action ANOTHER: Action<Newsletter> = post "/newsletter/another" => another;
    action WISH: Action<Home, String> = post "/wish" => wish;
}

const STYLE: Stylesheet = Stylesheet::new(
    "
body { font-family: system-ui, sans-serif; max-width: 36rem; margin: 2rem auto; padding: 0 1rem; }
nav { display: flex; gap: 1rem; }
form { display: flex; gap: .5rem; align-items: center; }
input { font-size: 1rem; padding: .3rem; flex: 1; }
button { font-size: 1rem; padding: .3rem .8rem; }
footer { margin-top: 2rem; padding-top: 1rem; border-top: 1px solid #ccc; }
.problem { color: crimson; }
",
);

mod class {
    use hypersig::html::Class;

    use super::STYLE;

    pub const PROBLEM: Class = STYLE.class("problem");
}

#[derive(Clone, Default)]
struct AppState {
    readers: Arc<Mutex<BTreeSet<String>>>,
    wishes: Arc<Mutex<BTreeSet<String>>>,
}

// The newsletter part.

#[derive(Signals, Serialize, Deserialize, Default)]
struct Newsletter {
    email: String,
    joining: bool,
    problem: String,
}

const NEWSLETTER_EMAIL: Id = Id::new("newsletter-email");

/// What the newsletter shows below its heading: the form, or the thanks
/// that replace it once the server has added the address.
enum Body {
    Form,
    Joined { readers: usize },
}

impl Fragment for Body {
    type Signals = Newsletter;
    const ID: Id = Id::new("newsletter");

    fn render(&self) -> El<Newsletter> {
        let email = Newsletter::email();
        let joining = Newsletter::joining();
        match self {
            Self::Form => div()
                .child(
                    form()
                        .on_submit(
                            JOIN.call_with(email)
                                .indicator(joining)
                                .error(Newsletter::problem()),
                        )
                        .child(label().label_for(&NEWSLETTER_EMAIL).text("Email"))
                        .child(
                            input()
                                .id(NEWSLETTER_EMAIL)
                                .attr(attr::TYPE, "email")
                                .bind(email),
                        )
                        .child(
                            button()
                                .attr(attr::TYPE, "submit")
                                .attr_when(attr::DISABLED, joining)
                                .text("Join"),
                        ),
                )
                .child(
                    p().class(class::PROBLEM)
                        .role(Role::Alert)
                        .bind_text(Newsletter::problem()),
                ),
            // The address comes from the signal the handler patched, which
            // this fragment reads under the field the part sits at.
            Self::Joined { readers } => div()
                .child(
                    p().id(Id::new("newsletter-joined"))
                        .text("Thanks, ")
                        .child(span().bind_text(email))
                        .text(format!(". The newsletter has {readers} readers.")),
                )
                .child(
                    button()
                        .on_click(ANOTHER.call())
                        .text("Add another address"),
                ),
        }
    }
}

impl Part for Newsletter {
    fn render() -> El<Self> {
        footer()
            .child(h2().text("Newsletter"))
            .child(hypersig::fragment(&Body::Form))
    }
}

async fn join(state: AppState, email: String) -> Result<Patches<Newsletter>, (StatusCode, String)> {
    let email = email.trim().to_owned();
    if !email.contains('@') {
        let problem = "An email address has an @ in it.".to_owned();
        return Err((StatusCode::UNPROCESSABLE_ENTITY, problem));
    }
    let readers = {
        let mut readers = state
            .readers
            .lock()
            .expect("no handler panics holding the lock");
        readers.insert(email.clone());
        readers.len()
    };
    Ok(Patches::new()
        .signals(Newsletter::email().patch(email))
        .element(&Body::Joined { readers }))
}

async fn another(_state: AppState, (): ()) -> Patches<Newsletter> {
    Patches::new()
        .signals(Newsletter::email().patch(String::new()))
        .element(&Body::Form)
}

// The shell part, which holds the newsletter one field deeper.

#[derive(Signals, Serialize, Deserialize, Default)]
struct Shell {
    footer: Newsletter,
}

impl Part for Shell {
    fn render() -> El<Self> {
        div()
            .child(
                nav()
                    .child(a().href(HOME.link()).text("Home"))
                    .child(a().href(ABOUT.link()).text("About")),
            )
            .child(h1().text("About the shop"))
            .child(p().text("We sell tea, and the newsletter says which tea."))
            .child(part(Shell::footer()))
    }
}

#[derive(Signals, Serialize, Deserialize)]
struct About {
    shell: Shell,
}

async fn about(_state: AppState, (): ()) -> Page<About> {
    Page::new(
        "About",
        About {
            shell: Shell::default(),
        },
    )
    .style(STYLE)
    .child(part(About::shell()))
}

// The home page and its wish list.

#[derive(Signals, Serialize, Deserialize, Clone)]
struct Wish {
    id: i32,
    name: String,
    /// `saving`, `saved`, or the reason the save failed.
    state: String,
}

#[derive(Signals, Serialize, Deserialize)]
struct Home {
    wishes: Vec<Wish>,
    draft: String,
    next_id: i32,
    footer: Newsletter,
}

const WISH_FIELD: Id = Id::new("wish");

async fn home(state: AppState, (): ()) -> Page<Home> {
    let wishes: Vec<Wish> = state
        .wishes
        .lock()
        .expect("no handler panics holding the lock")
        .iter()
        .zip(1..)
        .map(|(name, id)| Wish {
            id,
            name: name.clone(),
            state: "saved".to_owned(),
        })
        .collect();
    let next_id = i32::try_from(wishes.len()).unwrap_or(i32::MAX - 1) + 1;
    let signals = Home {
        wishes,
        draft: String::new(),
        next_id,
        footer: Newsletter::default(),
    };
    Page::new("Shop", signals).style(STYLE).child(
        div()
            .child(
                nav()
                    .child(a().href(HOME.link()).text("Home"))
                    .child(a().href(ABOUT.link()).text("About")),
            )
            .child(h1().text("Wish list"))
            .child(wish_form())
            .child(
                ul().id(Id::new("wishes"))
                    .child(each(Home::wishes(), |wish| {
                        li().child(span().bind_text(wish.at(Wish::name())))
                            .text(" ")
                            .child(
                                span()
                                    .class(class::PROBLEM)
                                    .bind_text(wish.at(Wish::state())),
                            )
                    })),
            )
            .child(part(Home::footer())),
    )
}

/// Adds the draft as a wish at once, then saves it, naming the wish's id
/// when the call is sent so the answer marks that wish and no other.
fn wish_form() -> El<Home> {
    let wishes = Home::wishes();
    let draft = Home::draft();
    let next_id = Home::next_id();
    let add = next_id.let_in_stmt(|id| {
        let mark = |state: Expr<Home, String>| {
            wishes.update_where(
                |wish| wish.at(Wish::id()).equals(&id),
                Wish::state(),
                |_| state,
            )
        };
        let blank = Expr::lit(&Wish {
            id: 0,
            name: String::new(),
            state: "saving".to_owned(),
        });
        wishes
            .push(blank.with(Wish::id(), &id).with(Wish::name(), draft.trim()))
            .then(
                WISH.call_with(draft.trim())
                    .on_error(mark)
                    .on_success(mark(Expr::text("saved"))),
            )
            .then(next_id.set(next_id + 1))
            .then(draft.set(""))
    });
    form()
        .on_submit(hypersig::Stmt::when(!draft.trim().is_empty(), add))
        .child(label().label_for(&WISH_FIELD).text("Wish"))
        .child(input().id(WISH_FIELD).bind(draft))
        .child(button().attr(attr::TYPE, "submit").text("Add"))
}

async fn wish(state: AppState, name: String) -> Result<Patches<Home>, (StatusCode, String)> {
    // A pause long enough that a visitor can add several wishes before the
    // first answer arrives.
    tokio::time::sleep(Duration::from_millis(300)).await;
    let added = state
        .wishes
        .lock()
        .expect("no handler panics holding the lock")
        .insert(name.clone());
    if added {
        Ok(Patches::new())
    } else {
        Err((
            StatusCode::CONFLICT,
            format!("{name} is already on the list."),
        ))
    }
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let app = pages(Router::new())
        .hypersig_runtime()
        .with_state(AppState::default());
    let addr = std::env::var("ADDR").unwrap_or_else(|_| "127.0.0.1:3008".to_owned());
    let listener = tokio::net::TcpListener::bind(&addr).await?;
    println!("listening on http://{addr}");
    axum::serve(listener, app).await
}
