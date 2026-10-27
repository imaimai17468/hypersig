//! One test per kind of statement the simulation runs, records, or refuses.

use std::time::Duration;

use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::html::{Event, Id, attr, button, div, each, form, input, option, p, select, when};
use crate::testing::nth;
use crate::{
    Action, Base, CalendarDate, DialogRef, Expr, FileInput, Method, NodeRef, Page, Route, Signals,
    Stmt, StreamAction, Upload,
};

#[derive(Signals, Serialize, Deserialize, Clone, Debug, PartialEq)]
struct Row {
    name: String,
    done: bool,
    rank: i32,
}

#[derive(Signals, Serialize, Deserialize)]
struct Board {
    count: i32,
    name: String,
    age: Option<i32>,
    agreed: bool,
    color: String,
    rows: Vec<Row>,
    numbers: Vec<i32>,
    saving: bool,
    error: String,
    #[signals(computed)]
    doubled: i32,
}

const GO: NodeRef<Board> = NodeRef::new("GO");
const OTHER: NodeRef<Board> = NodeRef::new("OTHER");
const FIELD: NodeRef<Board> = NodeRef::new("FIELD");
const ROW: NodeRef<Board> = NodeRef::new("ROW");
const SHEET: DialogRef<Board> = DialogRef::new("SHEET");
const PICTURE: FileInput<Board> = FileInput::new("picture");
const BOX: Id = Id::new("box");
const SAVE: Action<Board, i32> = Action::at(Method::Post, Base::ROOT, "/save");
const FEED: StreamAction<Board> = StreamAction::at(Base::ROOT, "/feed");
const SEND: Upload<Board> = Upload::at(Base::ROOT, "/send", 1024);

fn row(name: &str, rank: i32) -> Row {
    Row {
        name: name.to_owned(),
        done: false,
        rank,
    }
}

fn board() -> Board {
    Board {
        count: 1,
        name: String::new(),
        age: Some(30),
        agreed: false,
        color: "red".to_owned(),
        rows: vec![row("b", 2), row("a", 1), row("c", 3)],
        numbers: vec![1, 2, 3],
        saving: false,
        error: "failed".to_owned(),
        doubled: 0,
    }
}

/// A page whose button `GO` runs `stmt` on click.
fn page(stmt: impl crate::IntoStmt<Board>) -> Page<Board> {
    Page::new("Board", board())
        .computed(Board::doubled(), Board::count() * 2)
        .child(button().node_ref(GO).on_click(stmt).text("Go"))
}

fn names(rows: &[Row]) -> Vec<&str> {
    rows.iter().map(|row| row.name.as_str()).collect()
}

#[test]
fn set_writes_a_signal_and_the_computed_signals_follow_between_steps() {
    let count = Board::count();
    let page = page(
        count
            .set(count + 1)
            .then(Board::name().set(Board::doubled().to_text())),
    );
    let mut browser = page.simulate();
    assert_eq!(browser.value_of(Board::doubled()), 2);
    browser.click(GO);
    assert_eq!(browser.value_of(count), 2);
    assert_eq!(browser.value_of(Board::doubled()), 4);
    assert_eq!(browser.value_of(Board::name()), "4");
}

#[test]
fn if_else_runs_the_branch_its_condition_picks() {
    let count = Board::count();
    let page = page(Stmt::if_else(
        count.gt(5),
        Board::name().set("big"),
        Board::name().set("small"),
    ));
    let mut browser = page.simulate();
    browser.click(GO);
    assert_eq!(browser.value_of(Board::name()), "small");
}

#[test]
fn let_in_stmt_names_a_value_once() {
    let count = Board::count();
    let page = page((count + 2).let_in_stmt(|three| count.set(&three * &three)));
    let mut browser = page.simulate();
    browser.click(GO);
    assert_eq!(browser.value_of(count), 9);
}

#[test]
fn the_position_statements_change_a_list_as_the_runtime_does() {
    let numbers = Board::numbers();
    let run = |stmt: Stmt<Board>| {
        let page = page(stmt);
        let mut browser = page.simulate();
        browser.click(GO);
        browser.value_of(numbers)
    };
    assert_eq!(run(numbers.push(4)), [1, 2, 3, 4]);
    assert_eq!(run(numbers.append(&vec![4, 5])), [1, 2, 3, 4, 5]);
    assert_eq!(run(numbers.prepend(&vec![0])), [0, 1, 2, 3]);
    assert_eq!(run(numbers.remove_at(1)), [1, 3]);
    assert_eq!(run(numbers.remove_at(7)), [1, 2, 3]);
    assert_eq!(run(numbers.remove_at(-1)), [1, 2, 3]);
    assert_eq!(run(numbers.swap(0, 2)), [3, 2, 1]);
    assert_eq!(run(numbers.swap(0, 3)), [1, 2, 3]);
    assert_eq!(run(numbers.extend(2, |i| i * 10)), [1, 2, 3, 0, 10]);
    assert_eq!(run(numbers.extend(-1, |i| i)), [1, 2, 3]);
    assert_eq!(run(numbers.clear()), Vec::<i32>::new());
}

#[test]
fn the_per_item_statements_read_each_item() {
    let rows = Board::rows();
    let run = |stmt: Stmt<Board>| {
        let page = page(stmt);
        let mut browser = page.simulate();
        browser.click(GO);
        browser.value_of(rows)
    };
    let kept = run(rows.retain(|row| row.at(Row::rank()).gt(1)));
    assert_eq!(names(&kept), ["b", "c"]);
    let sorted = run(rows.sort_by(|row| row.at(Row::name())));
    assert_eq!(names(&sorted), ["a", "b", "c"]);
    let sorted = run(rows.sort_by_desc(|row| row.at(Row::rank())));
    assert_eq!(names(&sorted), ["c", "b", "a"]);
    let updated = run(rows.update_where(
        |row| row.at(Row::rank()).ge(2),
        Row::name(),
        |row| row.at(Row::name()).to_uppercase(),
    ));
    assert_eq!(names(&updated), ["B", "a", "C"]);
    let all = run(rows.set_all(Row::done(), true));
    assert!(all.iter().all(|row| row.done));
}

#[test]
fn a_click_on_a_list_item_writes_that_item() {
    let page = Page::new("Board", board())
        .computed(Board::doubled(), Board::count() * 2)
        .child(div().child(each(Board::rows(), |row| {
            p().child(
                button()
                    .node_ref(ROW)
                    .on_click(row.at(Row::done()).toggle())
                    .bind_text(row.at(Row::name())),
            )
        })));
    let mut browser = page.simulate();
    assert_eq!(browser.count(ROW), 3);
    assert_eq!(browser.element(nth(ROW, 2)).text(), "c");
    browser.click(nth(ROW, 1));
    let done: Vec<bool> = browser
        .value_of(Board::rows())
        .iter()
        .map(|row| row.done)
        .collect();
    assert_eq!(done, [false, true, false]);
}

#[test]
fn typing_writes_the_bound_signal_before_on_input_runs() {
    let page = Page::new("Board", board())
        .computed(Board::doubled(), Board::count() * 2)
        .child(
            input()
                .node_ref(FIELD)
                .bind(Board::name())
                .on_input(Board::color().set(Board::name().to_uppercase())),
        )
        .child(input().node_ref(GO).bind(Board::count()))
        .child(input().node_ref(OTHER).bind(Board::age()));
    let mut browser = page.simulate();
    browser.input(FIELD, "tea");
    assert_eq!(browser.value_of(Board::name()), "tea");
    assert_eq!(browser.value_of(Board::color()), "TEA");
    browser.input(GO, "7.9");
    assert_eq!(browser.value_of(Board::count()), 7);
    assert_eq!(browser.value_of(Board::doubled()), 14);
    browser.input(GO, "seven");
    assert_eq!(browser.value_of(Board::count()), 7);
    browser.input(GO, "");
    assert_eq!(browser.value_of(Board::count()), 7);
    browser.input(OTHER, " ");
    assert_eq!(browser.value_of(Board::age()), None);
    assert_eq!(browser.element(FIELD).attr(attr::VALUE), Some("tea"));
}

#[derive(Signals, Serialize, Deserialize)]
struct Trip {
    start: CalendarDate,
    end: Option<CalendarDate>,
}

#[test]
fn typing_a_date_writes_it_only_where_it_is_a_date() {
    const START: NodeRef<Trip> = NodeRef::new("START");
    const END: NodeRef<Trip> = NodeRef::new("END");
    let trip = Trip {
        start: CalendarDate::new(2026, 1, 5),
        end: Some(CalendarDate::new(2026, 1, 9)),
    };
    let page = Page::new("Trip", trip)
        .child(input().node_ref(START).bind(Trip::start()))
        .child(input().node_ref(END).bind(Trip::end()));
    let mut browser = page.simulate();
    assert_eq!(browser.element(START).attr(attr::VALUE), Some("2026-01-05"));
    browser.input(START, "2026-02-01");
    assert_eq!(
        browser.value_of(Trip::start()),
        CalendarDate::new(2026, 2, 1)
    );
    browser.input(START, "2026-02-30");
    browser.input(START, "");
    assert_eq!(
        browser.value_of(Trip::start()),
        CalendarDate::new(2026, 2, 1)
    );
    browser.input(END, "");
    assert_eq!(browser.value_of(Trip::end()), None);
}

#[test]
fn a_date_shows_as_its_text_and_compares_in_day_order() {
    const START: NodeRef<Trip> = NodeRef::new("START");
    const SHOWN: NodeRef<Trip> = NodeRef::new("SHOWN");
    const EARLY: NodeRef<Trip> = NodeRef::new("EARLY");
    let trip = Trip {
        start: CalendarDate::new(2026, 1, 5),
        end: None,
    };
    let february = CalendarDate::new(2026, 2, 1);
    let page = Page::new("Trip", trip)
        .child(input().node_ref(START).bind(Trip::start()))
        .child(p().node_ref(SHOWN).bind_text(Trip::start().to_text()))
        .child(p().node_ref(EARLY).bind_text(crate::if_else(
            Trip::start().lt(february),
            "early",
            "late",
        )));
    let mut browser = page.simulate();
    assert_eq!(browser.element(SHOWN).text(), "2026-01-05");
    assert_eq!(browser.element(EARLY).text(), "early");
    browser.input(START, "2026-12-24");
    assert_eq!(browser.element(SHOWN).text(), "2026-12-24");
    assert_eq!(browser.element(EARLY).text(), "late");
}

#[test]
fn change_reads_a_select_as_json_and_runs_on_change() {
    let page = Page::new("Board", board())
        .computed(Board::doubled(), Board::count() * 2)
        .child(
            select()
                .node_ref(FIELD)
                .bind_select(
                    Board::color(),
                    [("red".to_owned(), "Red"), ("blue".to_owned(), "Blue")],
                )
                .on(Event::Change, Board::name().set(Board::color())),
        )
        .child(option::<Board>());
    let mut browser = page.simulate();
    browser.input(FIELD, "\"blue\"");
    assert_eq!(browser.value_of(Board::color()), "red");
    browser.change(FIELD, "\"blue\"");
    assert_eq!(browser.value_of(Board::color()), "blue");
    assert_eq!(browser.value_of(Board::name()), "blue");
}

#[test]
fn a_click_checks_a_bound_checkbox_and_runs_on_change_after_on_click() {
    let page = Page::new("Board", board())
        .computed(Board::doubled(), Board::count() * 2)
        .child(
            input()
                .node_ref(GO)
                .bind(Board::agreed())
                .on_click(Board::name().set(Board::agreed().to_text()))
                .on(Event::Change, Board::color().set(Board::agreed().to_text())),
        );
    let mut browser = page.simulate();
    browser.click(GO);
    assert!(browser.value_of(Board::agreed()));
    assert_eq!(browser.value_of(Board::name()), "false");
    assert_eq!(browser.value_of(Board::color()), "true");
}

#[test]
fn a_submit_button_submits_its_form_and_a_plain_button_does_not() {
    let count = Board::count();
    let page = Page::new("Board", board())
        .computed(Board::doubled(), Board::count() * 2)
        .child(
            form()
                .id(BOX)
                .on_submit(count.set(count + 10))
                .child(button().node_ref(GO).text("Send"))
                .child(
                    button()
                        .node_ref(OTHER)
                        .attr(attr::TYPE, "button")
                        .on_click(count.set(count + 1))
                        .text("Add"),
                ),
        );
    let mut browser = page.simulate();
    browser.click(GO);
    assert_eq!(browser.value_of(count), 11);
    browser.click(OTHER);
    assert_eq!(browser.value_of(count), 12);
    browser.submit(&BOX);
    assert_eq!(browser.value_of(count), 22);
}

#[test]
fn a_call_is_recorded_with_its_arguments_and_the_writes_made_on_sending() {
    let count = Board::count();
    let page = page(
        SAVE.call_with(count)
            .indicator(Board::saving())
            .error(Board::error())
            .exclusive()
            .then(count.set(count + 1))
            .then(FEED.call())
            .then(SEND.call(PICTURE)),
    );
    let mut browser = page.simulate();
    let effects = browser.click(GO);
    assert_eq!(effects.calls(&SAVE), [1]);
    assert_eq!(effects.calls(&FEED), [()]);
    assert_eq!(effects.calls(&SEND), [()]);
    assert!(browser.value_of(Board::saving()));
    assert_eq!(browser.value_of(Board::error()), "");
    // The first call has no answer, so the exclusive call is skipped.
    let effects = browser.click(GO);
    assert_eq!(effects.calls(&SAVE), Vec::<i32>::new());
    assert_eq!(effects.calls(&FEED), [()]);
}

#[test]
fn links_are_recorded_with_the_url_they_evaluate_to() {
    let member: Route<i32> = Route::new("/members/{id}");
    let about: Route = Route::new("/about");
    let page = page(
        member
            .link_with(Board::count())
            .push_url()
            .then(about.link().replace_url())
            .then(about.link().navigate()),
    );
    let mut browser = page.simulate();
    let effects = browser.click(GO);
    assert_eq!(
        effects.pushes_url().map(crate::Url::as_str),
        Some("/members/1")
    );
    assert_eq!(
        effects.replaces_url().map(crate::Url::as_str),
        Some("/about")
    );
    assert_eq!(effects.navigates_to(), Some(&about.url(())));
}

#[test]
fn the_statements_of_an_element_are_left_to_the_browser() {
    let page = page(FIELD.focus().then(SHEET.show_modal()).then(PICTURE.clear()));
    let mut browser = page.simulate();
    let effects = browser.click(GO);
    assert!(effects.runs_in_browser(FIELD.focus()));
    assert!(effects.runs_in_browser(SHEET.show_modal().then(PICTURE.clear())));
    assert!(!effects.runs_in_browser(SHEET.close()));
}

#[test]
fn a_debounced_statement_runs_once_on_pause() {
    let count = Board::count();
    let page = Page::new("Board", board())
        .computed(Board::doubled(), Board::count() * 2)
        .child(input().node_ref(FIELD).bind(Board::name()).on_debounced(
            Event::Input,
            Duration::from_millis(300),
            count.set(count + 1),
        ));
    let mut browser = page.simulate();
    browser.input(FIELD, "t");
    browser.input(FIELD, "te");
    assert_eq!(browser.value_of(count), 1);
    browser.pause();
    assert_eq!(browser.value_of(count), 2);
    browser.pause();
    assert_eq!(browser.value_of(count), 2);
}

#[test]
fn an_element_shows_its_text_attributes_and_visibility_from_the_current_values() {
    let count = Board::count();
    let page = Page::new("Board", board())
        .computed(Board::doubled(), Board::count() * 2)
        .child(
            div()
                .id(BOX)
                .show(count.lt(3))
                .attr(attr::TITLE, "a < b")
                .bind_attr(attr::LANG, Board::color())
                .child(p().node_ref(OTHER).bind_text(Board::doubled().to_text()))
                .child(when(
                    count.equals(1),
                    p().node_ref(FIELD).text("one & only"),
                )),
        )
        .child(button().node_ref(GO).on_click(count.set(count + 5)));
    let mut browser = page.simulate();
    let shown = browser.element(&BOX);
    assert!(shown.is_visible());
    assert_eq!(shown.attr(attr::TITLE), Some("a < b"));
    assert_eq!(shown.attr(attr::LANG), Some("red"));
    assert_eq!(shown.text(), "2one & only");
    assert_eq!(browser.count(FIELD), 1);
    browser.click(GO);
    assert!(!browser.element(&BOX).is_visible());
    assert!(!browser.element(OTHER).is_visible());
    assert_eq!(browser.element(OTHER).text(), "12");
    assert_eq!(browser.count(FIELD), 0);
}

#[test]
#[should_panic(expected = "the page shows 3 elements that carry the node ref `ROW`")]
fn an_element_on_every_item_needs_nth() {
    let page = Page::new("Board", board())
        .computed(Board::doubled(), Board::count() * 2)
        .child(div().child(each(Board::rows(), |_| p().node_ref(ROW))));
    let _ = page.simulate().element(ROW);
}

#[test]
#[should_panic(expected = "the page shows no element that carries the id `box`")]
fn a_handle_no_element_carries_panics() {
    let _ = page(Stmt::none()).simulate().element(&BOX);
}

#[test]
#[should_panic(expected = "clicking the node ref `OTHER` runs no statement")]
fn a_click_that_runs_nothing_panics() {
    let page = page(Stmt::none()).child(p().node_ref(OTHER));
    page.simulate().click(OTHER);
}

#[test]
#[should_panic(expected = "the `set` statement reads a value only the browser has")]
fn a_statement_reading_the_clock_is_refused() {
    let page = page(Board::count().set(Expr::now().round().to_text().char_count()));
    page.simulate().click(GO);
}

#[test]
#[should_panic(expected = "the `set` statement computes a value the server cannot evaluate")]
fn a_statement_computing_an_infinity_is_refused() {
    let page = page(Board::name().set((Board::count().to_f64() / 0.0).to_text()));
    page.simulate().click(GO);
}

#[test]
#[should_panic(expected = "`upsert` is a list statement that only a server patch sends")]
fn a_patch_statement_is_refused() {
    let page = page(Stmt::build(|_| {
        json!(["upsert", "rows", 0, ["lit", 1], ["lit", []]])
    }));
    page.simulate().click(GO);
}
