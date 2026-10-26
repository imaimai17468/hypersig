//! Clicking and typing on a [`Page`] in a test, without a browser.
//!
//! [`Page::simulate`] starts a [`Simulation`] from the values the page's
//! signals start at. Its [`click`](Simulation::click),
//! [`input`](Simulation::input), [`change`](Simulation::change), and
//! [`submit`](Simulation::submit) run the statements the page attaches to
//! that event, and [`value_of`](Simulation::value_of) and
//! [`element`](Simulation::element) read what they changed. A test finds an
//! element by the [`Id`], [`NodeRef`], [`DialogRef`], or [`FileInput`] the
//! page put on it, so a renamed handle moves the test with it.
//!
//! The simulation runs a statement on the server's evaluator, as
//! `runtime/src/eval/stmt/` runs it in the browser, and recomputes each
//! [`Page::computed`] signal where the runtime does: between the steps of a
//! [`Stmt::then`](crate::Stmt::then) chain and after the event. It runs
//! these statements:
//!
//! - `set`, `toggle`, `Stmt::when`, `Stmt::if_else`, `then`, and `let_in_stmt`;
//! - the list statements `push`, `append`, `prepend`, `remove_at`, `clear`,
//!   `swap`, `extend`, `retain`, `update_where`, `set_all`, `sort_by`, and
//!   `sort_by_desc`;
//! - a statement given to [`El::on_debounced`](crate::html::El::on_debounced),
//!   once [`Simulation::pause`] says the visitor stopped. Two elements
//!   outside a list that debounce one statement share one wait here, where
//!   the browser keeps one per element.
//!
//! It records these in the [`Effects`] the event returns, and runs nothing
//! for them, because they need the server or the browser:
//!
//! - a call of an [`Action`], a [`StreamAction`], or an [`Upload`], with the
//!   arguments it sends. The simulation writes what the runtime writes when
//!   it sends the call (the indicator set to `true`, the error set to `""`,
//!   the status set to 0) and never answers it, so the indicator stays
//!   `true`, `on_success` and `on_error` do not run, and a second
//!   `exclusive` call to the same action is skipped;
//! - [`Link::navigate`](crate::Link::navigate),
//!   [`Link::push_url`](crate::Link::push_url), and
//!   [`Link::replace_url`](crate::Link::replace_url), with the URL each one
//!   evaluates to;
//! - the statements of a [`NodeRef`] and a [`DialogRef`], such as `focus`
//!   and `show_modal`, and [`FileInput::clear`].
//!
//! It panics on a statement whose value only the browser has, such as one
//! that reads [`Expr::now`](crate::Expr::now),
//! [`Expr::random_below`](crate::Expr::random_below), or a date in the
//! visitor's zone, or one that computes a number JSON cannot hold.
//!
//! It never runs the statements of [`El::effect`](crate::html::El::effect),
//! [`El::on_signal_change`](crate::html::El::on_signal_change), the timers,
//! [`El::on_load`](crate::html::El::on_load),
//! [`El::on_visible`](crate::html::El::on_visible), or a key shortcut, even
//! where the browser would run one after a click changed a signal it reads.
//! It fires no event apart from the four above, so a statement on a double
//! click, on focus or blur, on the pointer entering or leaving, on a key,
//! or on an [`El::on_custom`](crate::html::El::on_custom) event never runs.
//! It applies no patch of the server's answer. A test reaches those through
//! a browser.

use std::cmp::Ordering;
use std::fmt;
use std::marker::PhantomData;

use serde::de::DeserializeOwned;
use serde_json::Value;

use crate::expr::IntoStmt;
use crate::html::find::{self, Found, ItemAt, Mark};
use crate::html::{Attr, Event, Id};
use crate::route::Url;
use crate::signal::{FromPage, Sig, Signals};
use crate::tape::Ref;
use crate::{Action, DialogRef, FileInput, NodeRef, Page, StreamAction, Upload};

mod run;

/// A page as a browser holds it, which a test clicks and types into.
///
/// [`Page::simulate`] starts one. The [module documentation](self) lists
/// which statements it runs, which it records, and which it refuses.
///
/// # Examples
///
/// ```
/// use hypersig::html::{button, p};
/// use hypersig::{Action, NodeRef, Page, Patches, Signals};
/// use serde::{Deserialize, Serialize};
///
/// #[derive(Signals, Serialize, Deserialize)]
/// struct Counter {
///     count: i32,
///     saving: bool,
/// }
///
/// hypersig::routes! {
///     fn actions(Router<()>);
///     action SAVE: Action<Counter, i32> = post "/save" => save;
/// }
/// # async fn save((): (), _: i32) -> Patches<Counter> { Patches::new() }
///
/// hypersig::node_ref!(const ADD: NodeRef<Counter>);
/// hypersig::node_ref!(const SHOWN: NodeRef<Counter>);
/// hypersig::node_ref!(const STORE: NodeRef<Counter>);
///
/// # fn main() {
/// let count = Counter::count();
/// let page = Page::new("Counter", Counter { count: 0, saving: false })
///     .child(p().node_ref(SHOWN).bind_text(count.to_text()))
///     .child(button().node_ref(ADD).on_click(count.set(count + 1)).text("Add"))
///     .child(
///         button()
///             .node_ref(STORE)
///             .on_click(SAVE.call_with(count).indicator(Counter::saving()))
///             .text("Save"),
///     );
///
/// let mut browser = page.simulate();
/// browser.click(ADD);
/// browser.click(ADD);
/// assert_eq!(browser.value_of(count), 2);
/// assert_eq!(browser.element(SHOWN).text(), "2");
///
/// let effects = browser.click(STORE);
/// assert_eq!(effects.calls(&SAVE), [2]);
/// assert!(browser.value_of(Counter::saving()));
/// # let _ = actions;
/// # }
/// ```
pub struct Simulation<'p, S> {
    page: &'p Page<S>,
    values: Value,
    /// The key of each call sent, none of which gets an answer, which an
    /// `exclusive` call checks as `Flights::busy` in the runtime does.
    sent: Vec<String>,
    /// The debounced statements that run on [`Simulation::pause`].
    waiting: Vec<Waiting<'p>>,
}

/// A debounced statement, with what it runs against.
struct Waiting<'p> {
    /// The statement and the list items around it, as the runtime keys a
    /// timer by its element and its statement.
    key: String,
    stmt: &'p Value,
    scope: Vec<ItemAt>,
    vars: Vec<Value>,
}

impl<S> fmt::Debug for Simulation<'_, S> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Simulation")
            .field("values", &self.values)
            .finish_non_exhaustive()
    }
}

impl<'p, S: Signals> Simulation<'p, S> {
    pub(crate) fn new(page: &'p Page<S>, values: Value) -> Self {
        Self {
            page,
            values,
            sent: Vec::new(),
            waiting: Vec::new(),
        }
    }

    /// The value `signal` holds now.
    ///
    /// # Panics
    ///
    /// Panics if the value does not deserialize into `T`, which happens
    /// where a statement wrote a value of another shape, such as `null`
    /// into a number through a list item that does not exist.
    #[must_use]
    pub fn value_of<T: DeserializeOwned, M: FromPage>(&self, signal: Sig<S, T, M>) -> T {
        let value = self.read(&signal.raw_path().dotted());
        serde_json::from_value(value).expect("a signal's value deserializes into the signal's type")
    }

    /// The element `target` names, as the page shows it now.
    ///
    /// # Panics
    ///
    /// Panics if the page shows no element that carries `target`, or several.
    /// An element inside a list shows once per item, and [`nth`] picks one.
    #[must_use]
    pub fn element(&self, target: impl Target<S>) -> Element {
        let (found, _) = self.one(target);
        Element::parse(&found.render(&self.values), !found.hidden)
    }

    /// The number of elements the page shows now that carry `target`.
    ///
    /// An element inside a [`when`](crate::html::when) whose condition is
    /// false counts as none, and one inside a list counts once per item.
    #[must_use]
    pub fn count(&self, target: impl Target<S>) -> usize {
        self.all(&target.into_mark()).len()
    }

    /// Clicks the element `target` names and returns what the server and browser must do.
    ///
    /// It runs the `on_click` statement of the element or of the closest
    /// element around it that has one, as a click bubbles. A checkbox or a
    /// radio bound with [`El::bind`](crate::html::El::bind) or
    /// [`El::bind_radio`](crate::html::El::bind_radio) is then checked, as
    /// the browser checks it, and the `on_input` and `on_change` statements
    /// around it run. A `<button>` inside a `<form>`, unless its `type` is
    /// `button` or `reset`, then submits the form, as
    /// [`Simulation::submit`] does. A button that names its form with
    /// [`El::form_owner`](crate::html::El::form_owner) is not followed to it.
    ///
    /// # Panics
    ///
    /// Panics where [`Simulation::element`] does, where the click runs no
    /// statement, and on a statement the [module documentation](self) lists
    /// as refused.
    pub fn click(&mut self, target: impl Target<S>) -> Effects<S> {
        let (found, name) = self.one(target);
        let mut effects = Effects::new();
        let last = found.last();
        let checked = self.checked_by_click(&found);
        let mut ran = self.dispatch(&found, last, Event::Click, &mut effects);
        if let Some((place, value)) = checked {
            self.dispatch(&found, last, Event::Input, &mut effects);
            self.set(&find::resolve(place, &found.scope), value);
            self.dispatch(&found, last, Event::Change, &mut effects);
            ran = true;
        }
        if submits(&found)
            && let Some(form) = found.form()
        {
            ran |= self.dispatch(&found, form, Event::Submit, &mut effects);
        }
        assert!(
            ran,
            "clicking {name} runs no statement: neither it nor an element around it has `on_click`"
        );
        self.settle();
        effects
    }

    /// Types `text` into the control `target` names, as one `input` event.
    ///
    /// The control's [`bind`](crate::html::El::bind) reads `text` as the
    /// runtime does: a number control leaves its signal alone for text that
    /// holds no number and truncates an `i32` or `i64`, an emptied `Option`
    /// control sets `None`, and a [`bind_select`](crate::html::El::bind_select)
    /// reads only on [`Simulation::change`]. Then the `on_input` statement of
    /// the control or the closest element around it runs. `on_change` does
    /// not, because the browser fires `change` when the visitor leaves the
    /// field.
    ///
    /// # Panics
    ///
    /// Panics where [`Simulation::element`] does, where the control is
    /// neither bound nor under an `on_input`, where it is a checkbox or a
    /// radio, which [`Simulation::click`] checks, and on a statement the
    /// [module documentation](self) lists as refused.
    pub fn input(&mut self, target: impl Target<S>, text: &str) -> Effects<S> {
        let (found, name) = self.one(target);
        let mut effects = Effects::new();
        let ran = self.event(&found, text, Event::Input, &mut effects);
        assert!(
            ran,
            "typing into {name} writes no signal and runs no statement: it has no `bind`, and \
             neither it nor an element around it has `on_input`"
        );
        self.settle();
        effects
    }

    /// Sets the control `target` names to `text` and commits it.
    ///
    /// It stands for picking an option or leaving a field after typing. It fires `input` as [`Simulation::input`] does, then `change`, on
    /// which a [`bind_select`](crate::html::El::bind_select) reads `text` as
    /// the JSON of an option's value, and the `on_change` statement of the
    /// control or the closest element around it runs.
    ///
    /// # Panics
    ///
    /// Panics where [`Simulation::input`] does, with `on_change` counting
    /// as well as `on_input`.
    pub fn change(&mut self, target: impl Target<S>, text: &str) -> Effects<S> {
        let (found, name) = self.one(target);
        let mut effects = Effects::new();
        let typed = self.event(&found, text, Event::Input, &mut effects);
        let changed = self.event(&found, text, Event::Change, &mut effects);
        assert!(
            typed || changed,
            "changing {name} writes no signal and runs no statement: it has no `bind`, and \
             neither it nor an element around it has `on_input` or `on_change`"
        );
        self.settle();
        effects
    }

    /// Submits the `<form>` `target` names, which runs the closest `on_submit` from it up.
    ///
    /// # Panics
    ///
    /// Panics where [`Simulation::element`] does, where the element is not a
    /// `<form>`, where no `on_submit` is found, and on a statement the
    /// [module documentation](self) lists as refused.
    pub fn submit(&mut self, target: impl Target<S>) -> Effects<S> {
        let (found, name) = self.one(target);
        assert!(
            found.tag() == "form",
            "{name} is a `<{}>`, and only a `<form>` is submitted: click its submit button, or \
             name the form",
            found.tag()
        );
        let mut effects = Effects::new();
        let ran = self.dispatch(&found, found.last(), Event::Submit, &mut effects);
        assert!(
            ran,
            "submitting {name} runs no statement: neither it nor an element around it has \
             `on_submit`"
        );
        self.settle();
        effects
    }

    /// Runs the debounced statements, as the browser does once the visitor stops typing.
    ///
    /// Each statement given to
    /// [`El::on_debounced`](crate::html::El::on_debounced) waits here from
    /// its event, and a second event before `pause` replaces it, so it runs
    /// once however many events came before.
    ///
    /// # Panics
    ///
    /// Panics on a statement the [module documentation](self) lists as refused.
    pub fn pause(&mut self) -> Effects<S> {
        let mut effects = Effects::new();
        for waiting in std::mem::take(&mut self.waiting) {
            self.run(waiting.stmt, &waiting.scope, &waiting.vars, &mut effects);
            self.settle();
        }
        effects
    }

    fn all(&self, (mark, nth): &(Mark, Option<usize>)) -> Vec<Found<'p, S>> {
        let mut found = find::find(self.page.body(), &self.values, mark);
        match *nth {
            None => found,
            Some(index) if index < found.len() => vec![found.swap_remove(index)],
            Some(_) => Vec::new(),
        }
    }

    /// The one element `target` names, with words that name it for a panic.
    fn one(&self, target: impl Target<S>) -> (Found<'p, S>, String) {
        let picked = target.into_mark();
        let name = describe(&picked);
        let mut all = self.all(&picked);
        match all.len() {
            1 => (all.pop().expect("one element was found"), name),
            0 => panic!(
                "the page shows no element that carries {name}: none carries it, or a `when` \
                 around it is false, or its list is empty"
            ),
            n => panic!(
                "the page shows {n} elements that carry {name}: pick one with `testing::nth`"
            ),
        }
    }

    /// Runs the statement for `event` of the closest element from the
    /// chain's position `at` up that has one, and says whether one ran.
    fn dispatch(
        &mut self,
        found: &Found<'p, S>,
        at: usize,
        event: Event,
        effects: &mut Effects<S>,
    ) -> bool {
        let Some((stmt, scope)) = found.handler(at, event) else {
            return false;
        };
        let scope = scope.to_vec();
        self.run(stmt, &scope, &[], effects);
        true
    }

    /// Fires `event` on a control holding `text`: its binding reads it, then
    /// the closest handler runs. Says whether either happened.
    fn event(
        &mut self,
        found: &Found<'p, S>,
        text: &str,
        event: Event,
        effects: &mut Effects<S>,
    ) -> bool {
        let bound = self.read_control(found, text, event);
        let ran = self.dispatch(found, found.last(), event, effects);
        bound || ran
    }

    /// The place a click checks and the value it writes there, for a bound
    /// checkbox and for a bound radio that is not checked yet.
    fn checked_by_click(&self, found: &Found<'p, S>) -> Option<(&'p Value, Value)> {
        let bind = found.bind()?;
        let place = bind.get(0)?;
        let current = || self.read(&find::resolve(place, &found.scope));
        match bind.get(1).and_then(Value::as_str) {
            Some("bool") => Some((place, Value::Bool(!crate::ssr::truthy(&current())))),
            Some("radio") => {
                let choice = bind.get(2)?;
                (!crate::ssr::equal(&current(), choice)).then(|| (place, choice.clone()))
            }
            _ => None,
        }
    }

    /// Writes `text` to the place the control is bound to, as `Controls.read`
    /// in `runtime/src/runtime/binding/control.ts` does for an event of
    /// `event`. Says whether the control is bound.
    fn read_control(&mut self, found: &Found<'p, S>, text: &str, event: Event) -> bool {
        let Some(bind) = found.bind() else {
            return false;
        };
        let Some(place) = bind.get(0) else {
            return false;
        };
        let path = find::resolve(place, &found.scope);
        let kind = bind.get(1).and_then(Value::as_str).unwrap_or("text");
        let value = match kind {
            "bool" | "radio" => panic!(
                "{} is a checkbox or a radio, which takes no text: `Simulation::click` checks it",
                found.tag()
            ),
            "json" if event == Event::Change => serde_json::from_str(text).ok(),
            "json" => None,
            "int" | "float" | "opt_int" | "opt_float" => read_number(text, kind),
            "opt_text" if event == Event::Input || found.tag() == "select" => {
                Some(if text.is_empty() {
                    Value::Null
                } else {
                    Value::from(text)
                })
            }
            _ if event == Event::Input || found.tag() == "select" => Some(Value::from(text)),
            _ => None,
        };
        if let Some(value) = value {
            self.set(&path, value);
        }
        true
    }

    /// The value at the store path `path`, or `null` where there is none.
    fn read(&self, path: &str) -> Value {
        Ref::from(&self.values)
            .get_path(path)
            .map_or(Value::Null, Ref::to_value)
    }

    fn set(&mut self, path: &str, value: Value) {
        self.values = crate::ssr::set_path(&self.values, path, value);
    }

    /// Recomputes each computed signal in the order the page declares
    /// them, keeping the value of one the server cannot compute, as the
    /// runtime keeps a computed signal that fails.
    fn settle(&mut self) {
        for (path, expr) in self.page.computed_signals() {
            if let Some(value) = find::evaluate(expr, &self.values, &[], &[]) {
                self.set(path, value);
            }
        }
    }
}

/// Whether a click on the element submits the form around it.
fn submits<S>(found: &Found<'_, S>) -> bool {
    let kind = found.static_attr("type");
    match found.tag() {
        "button" => !matches!(kind, Some("button" | "reset")),
        "input" => kind == Some("submit"),
        _ => false,
    }
}

/// The number the text of a number control holds, as `readNumber` in
/// `runtime/src/runtime/binding/control.ts` reads it: `null` for an emptied
/// `Option`, nothing for other text that holds no finite number, and the
/// number truncated for an integer.
fn read_number(text: &str, kind: &str) -> Option<Value> {
    let trimmed = text.trim();
    if trimmed.is_empty() {
        return kind.starts_with("opt_").then_some(Value::Null);
    }
    let number = trimmed.parse::<f64>().ok().filter(|n| n.is_finite())?;
    let number = if kind.ends_with("int") {
        number.trunc()
    } else {
        number
    };
    Some(json_number(number))
}

/// `value` as JSON, an integer where it is one, as the store holds a number the runtime wrote.
fn json_number(value: f64) -> Value {
    #[expect(
        clippy::cast_possible_truncation,
        reason = "only integral values inside i64's range reach the cast"
    )]
    if value.fract() == 0.0 && value.abs() < 9e15 {
        Value::from(value as i64)
    } else {
        serde_json::Number::from_f64(value).map_or(Value::Null, Value::Number)
    }
}

/// The order JavaScript's `<` and `>` give two sort keys: strings by UTF-16
/// code unit, and anything else as numbers. `Equal` stands for neither.
fn js_order(a: &Value, b: &Value) -> Ordering {
    match (a, b) {
        (Value::String(a), Value::String(b)) => a.encode_utf16().cmp(b.encode_utf16()),
        _ => match (js_to_number(a), js_to_number(b)) {
            (Some(a), Some(b)) => a.partial_cmp(&b).unwrap_or(Ordering::Equal),
            _ => Ordering::Equal,
        },
    }
}

fn js_to_number(value: &Value) -> Option<f64> {
    match value {
        Value::Null => Some(0.0),
        Value::Bool(b) => Some(f64::from(u8::from(*b))),
        Value::Number(n) => n.as_f64(),
        Value::String(s) if s.trim().is_empty() => Some(0.0),
        Value::String(s) => s.trim().parse().ok(),
        Value::Array(_) | Value::Object(_) => None,
    }
}

fn describe((mark, nth): &(Mark, Option<usize>)) -> String {
    match nth {
        None => mark_text(mark),
        Some(index) => format!("element {index} of those that carry {}", mark_text(mark)),
    }
}

fn mark_text(mark: &Mark) -> String {
    match mark {
        Mark::Id(id) => format!("the id `{id}`"),
        Mark::Ref(name) => format!("the node ref `{name}`"),
        Mark::Files(name) => format!("the file input `{name}`"),
    }
}

/// One element as the page shows it: its text, attributes, and visibility.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Element {
    attrs: Vec<(String, String)>,
    text: String,
    visible: bool,
}

impl Element {
    /// The text of the element and everything inside it, as `textContent` reads it.
    ///
    /// A bound control's own value is an attribute, read with
    /// [`attr`](Self::attr) and [`attr::VALUE`](crate::html::attr::VALUE),
    /// apart from a `<textarea>`, whose text it is.
    #[must_use]
    pub fn text(&self) -> &str {
        &self.text
    }

    /// The value of the attribute `name`, if the element carries it.
    ///
    /// A bound attribute holds the value its expression gives now, and an
    /// attribute set with [`El::attr_when`](crate::html::El::attr_when) or
    /// [`El::disabled`](crate::html::El::disabled) holds `""` while it is on.
    #[must_use]
    pub fn attr(&self, name: Attr) -> Option<&str> {
        self.attrs
            .iter()
            .find(|(attr, _)| attr.eq_ignore_ascii_case(name.name()))
            .map(|(_, value)| value.as_str())
    }

    /// Whether the element shows: no [`El::show`](crate::html::El::show) on
    /// it or around it is false.
    #[must_use]
    pub fn is_visible(&self) -> bool {
        self.visible
    }

    /// Reads the element the server wrote as `html`, whose first tag is its own.
    ///
    /// The server escapes `<`, `>`, `&`, and the quotes in an attribute, so a
    /// `>` ends the tag and a `"` ends a value. The text leaves out each
    /// `<template>`, whose content the browser keeps out of `textContent`.
    fn parse(html: &str, visible: bool) -> Self {
        let open_end = html.find('>').unwrap_or(html.len());
        let open = &html[..open_end];
        let mut attrs = Vec::new();
        let mut rest = open.find(' ').map_or("", |space| &open[space..]);
        while let Some(start) = rest.find(|c: char| c != ' ') {
            rest = &rest[start..];
            let Some(eq) = rest.find("=\"") else {
                break;
            };
            let name = &rest[..eq];
            let after = &rest[eq + 2..];
            let end = after.find('"').unwrap_or(after.len());
            attrs.push((name.to_owned(), unescape(&after[..end])));
            rest = after.get(end + 1..).unwrap_or("");
        }
        let mut text = String::new();
        let mut body = html.get(open_end + 1..).unwrap_or("");
        let mut templates = 0_usize;
        while let Some(lt) = body.find('<') {
            if templates == 0 {
                text.push_str(&body[..lt]);
            }
            let tag = &body[lt..];
            if tag.starts_with("<template") {
                templates += 1;
            } else if tag.starts_with("</template") {
                templates = templates.saturating_sub(1);
            }
            body = tag.find('>').map_or("", |gt| &tag[gt + 1..]);
        }
        if templates == 0 {
            text.push_str(body);
        }
        Self {
            attrs,
            text: unescape(&text),
            visible,
        }
    }
}

/// The text `escape_into` in `html.rs` escaped.
fn unescape(text: &str) -> String {
    text.replace("&lt;", "<")
        .replace("&gt;", ">")
        .replace("&quot;", "\"")
        .replace("&#39;", "'")
        .replace("&amp;", "&")
}

/// What an event left for the server and the browser to do.
///
/// [`Simulation::click`] and the other events return it. Each method reads
/// one kind of record, by the typed value the page built it from.
pub struct Effects<S> {
    calls: Vec<Sent>,
    visits: Vec<(Visit, Url)>,
    browser: Vec<Value>,
    marker: PhantomData<fn() -> S>,
}

/// One call sent to the server.
#[derive(Debug)]
struct Sent {
    method: &'static str,
    path: String,
    args: Value,
}

/// How a statement changed the URL.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Visit {
    Navigate,
    Push,
    Replace,
}

impl<S> fmt::Debug for Effects<S> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Effects")
            .field("calls", &self.calls)
            .field("visits", &self.visits)
            .field("browser", &self.browser)
            .finish()
    }
}

impl<S> Effects<S> {
    fn new() -> Self {
        Self {
            calls: Vec::new(),
            visits: Vec::new(),
            browser: Vec::new(),
            marker: PhantomData,
        }
    }

    /// The arguments of each call to `endpoint` the event sent, in order.
    ///
    /// An [`Action`] that takes no arguments gives one `()` per call. The
    /// arguments are the ones the call evaluated when the event ran it.
    ///
    /// # Panics
    ///
    /// Panics if the arguments do not deserialize into the endpoint's
    /// argument type, which happens where a `Serialize` impl disagrees with
    /// its `Deserialize` impl.
    #[must_use]
    pub fn calls<E: Endpoint<S>>(&self, endpoint: &E) -> Vec<E::Args>
    where
        E::Args: DeserializeOwned,
    {
        let (method, path) = endpoint.method_and_path();
        self.calls
            .iter()
            .filter(|sent| sent.method == method && sent.path == path)
            .map(|sent| {
                serde_json::from_value(sent.args.clone())
                    .expect("a call's arguments deserialize into the endpoint's argument type")
            })
            .collect()
    }

    /// The URL the first [`Link::navigate`](crate::Link::navigate) the event ran loads.
    #[must_use]
    pub fn navigates_to(&self) -> Option<&Url> {
        self.visit(Visit::Navigate)
    }

    /// The URL the first [`Link::push_url`](crate::Link::push_url) the event ran shows.
    #[must_use]
    pub fn pushes_url(&self) -> Option<&Url> {
        self.visit(Visit::Push)
    }

    /// The URL the first [`Link::replace_url`](crate::Link::replace_url) the event ran shows.
    #[must_use]
    pub fn replaces_url(&self) -> Option<&Url> {
        self.visit(Visit::Replace)
    }

    /// Whether the event asked the browser to run `stmt`, such as `FIELD.focus()`.
    ///
    /// It covers the statements of a [`NodeRef`] and a [`DialogRef`] and
    /// [`FileInput::clear`]. For `a.then(b)`, it says whether the event
    /// asked for both. Any other statement gives `false`, because the
    /// simulation runs it itself.
    #[must_use]
    pub fn runs_in_browser(&self, stmt: impl IntoStmt<S>) -> bool {
        let ast = stmt.into_stmt().into_json();
        match ast.as_array() {
            Some(parts) if parts.first() == Some(&Value::from("seq")) => parts
                .get(1)
                .and_then(Value::as_array)
                .is_some_and(|steps| steps.iter().all(|step| self.browser.contains(step))),
            _ => self.browser.contains(&ast),
        }
    }

    fn visit(&self, kind: Visit) -> Option<&Url> {
        self.visits
            .iter()
            .find(|(visit, _)| *visit == kind)
            .map(|(_, url)| url)
    }
}

/// A handle a test finds an element by.
///
/// It is an [`Id`], a [`NodeRef`], a [`DialogRef`], a [`FileInput`], or
/// one of several elements that carry one, through [`nth`].
///
/// The trait is sealed, because each handle is matched against what the
/// page writes for it.
pub trait Target<S>: sealed::Target {}

/// An endpoint whose calls [`Effects::calls`] reads: an [`Action`], a
/// [`StreamAction`], or an [`Upload`].
///
/// The trait is sealed, because each endpoint is matched against the call
/// the page writes for it.
pub trait Endpoint<S>: sealed::Endpoint {
    /// The type of the arguments a call sends.
    type Args;
}

mod sealed {
    use crate::html::find::Mark;

    pub trait Target {
        /// What the element carries, and which of several to pick.
        fn into_mark(self) -> (Mark, Option<usize>);
    }

    pub trait Endpoint {
        /// The method and the path a call names.
        fn method_and_path(&self) -> (&'static str, String);
    }
}

impl sealed::Target for Id {
    fn into_mark(self) -> (Mark, Option<usize>) {
        (Mark::Id(self.text.into_owned()), None)
    }
}

impl<S> Target<S> for Id {}

impl sealed::Target for &Id {
    fn into_mark(self) -> (Mark, Option<usize>) {
        (Mark::Id(self.as_str().to_owned()), None)
    }
}

impl<S> Target<S> for &Id {}

impl<S> sealed::Target for NodeRef<S> {
    fn into_mark(self) -> (Mark, Option<usize>) {
        (Mark::Ref(self.name()), None)
    }
}

impl<S> Target<S> for NodeRef<S> {}

impl<S> sealed::Target for DialogRef<S> {
    fn into_mark(self) -> (Mark, Option<usize>) {
        (Mark::Ref(self.name()), None)
    }
}

impl<S> Target<S> for DialogRef<S> {}

impl<S> sealed::Target for FileInput<S> {
    fn into_mark(self) -> (Mark, Option<usize>) {
        (Mark::Files(self.name()), None)
    }
}

impl<S> Target<S> for FileInput<S> {}

/// The `index`th element, from 0, that carries `target`, in document order.
///
/// A [`NodeRef`] inside a list template sits on every item, so `nth`
/// picks the item a test clicks or reads.
///
/// ```
/// use hypersig::html::{button, each, li, ul};
/// use hypersig::{NodeRef, Page, Signals};
/// use hypersig::testing::nth;
/// use serde::{Deserialize, Serialize};
///
/// #[derive(Signals, Serialize, Deserialize)]
/// struct Todo {
///     title: String,
///     done: bool,
/// }
///
/// #[derive(Signals, Serialize, Deserialize)]
/// struct List {
///     todos: Vec<Todo>,
/// }
///
/// hypersig::node_ref!(const DONE: NodeRef<List>);
///
/// # fn main() {
/// let todos = vec![
///     Todo { title: "Tea".into(), done: false },
///     Todo { title: "Milk".into(), done: false },
/// ];
/// let page = Page::new("Todos", List { todos }).child(ul().child(each(List::todos(), |todo| {
///     li().child(
///         button()
///             .node_ref(DONE)
///             .on_click(todo.at(Todo::done()).set(true))
///             .bind_text(todo.at(Todo::title())),
///     )
/// })));
///
/// let mut browser = page.simulate();
/// assert_eq!(browser.count(DONE), 2);
/// browser.click(nth(DONE, 1));
/// assert_eq!(browser.element(nth(DONE, 1)).text(), "Milk");
/// let done: Vec<bool> = browser.value_of(List::todos()).iter().map(|todo: &Todo| todo.done).collect();
/// assert_eq!(done, [false, true]);
/// # }
/// ```
#[must_use]
pub fn nth<T>(target: T, index: usize) -> Nth<T> {
    Nth { target, index }
}

/// One of several elements that carry a handle, which [`nth`] picks.
#[derive(Debug, Clone, Copy)]
pub struct Nth<T> {
    target: T,
    index: usize,
}

impl<T: sealed::Target> sealed::Target for Nth<T> {
    fn into_mark(self) -> (Mark, Option<usize>) {
        (self.target.into_mark().0, Some(self.index))
    }
}

impl<S, T: Target<S>> Target<S> for Nth<T> {}

impl<S, A> sealed::Endpoint for Action<S, A> {
    fn method_and_path(&self) -> (&'static str, String) {
        (self.method().as_str(), self.call_path())
    }
}

impl<S, A> Endpoint<S> for Action<S, A> {
    type Args = A;
}

impl<S, A> sealed::Endpoint for StreamAction<S, A> {
    fn method_and_path(&self) -> (&'static str, String) {
        ("GET", self.call_path())
    }
}

impl<S, A> Endpoint<S> for StreamAction<S, A> {
    type Args = A;
}

impl<S, A> sealed::Endpoint for Upload<S, A> {
    fn method_and_path(&self) -> (&'static str, String) {
        ("POST", self.call_path())
    }
}

impl<S, A> Endpoint<S> for Upload<S, A> {
    type Args = A;
}
