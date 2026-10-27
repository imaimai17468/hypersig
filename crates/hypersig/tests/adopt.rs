//! Writes pages built from a seeded generator to
//! `runtime/test/fixtures/adopt.json`, which `runtime/test/adopt.test.ts`
//! loads, starts the runtime on, and compares with the document the server
//! wrote, so a value the server renders differently from the runtime, or an
//! element the runtime replaces where it should adopt it, fails there.
//!
//! The pages combine text, attribute, class, and visibility bindings, inputs
//! bound to signals, `when`, and `each` and `each_keyed` two lists deep, over
//! values that hold markup characters, line breaks, characters outside the
//! BMP, and empty strings.
//!
//! The test fails when the file differs from what the generator writes now.
//! Run it with `HYPERSIG_FIXTURES=overwrite` to rewrite it.

use std::path::PathBuf;
use std::rc::Rc;

use hypersig::html::{
    Class, El, Node, attr, button, div, each, each_keyed, input, label, li, p, span, textarea, ul,
    when,
};
use hypersig::{Expr, IntoExpr as _, Page, Signals};
use serde::{Deserialize, Serialize};
use serde_json::json;

/// The seed of the generator, fixed so that the file changes only with the code.
const SEED: u64 = 0x6164_6f70_7473_6967;

const PAGES: usize = 300;

/// The most elements from the page's root to a leaf.
const MAX_DEPTH: u32 = 4;

const RUNTIME_SRC: hypersig::Asset = hypersig::Asset::new("/_hypersig/runtime.js");

const ON: Class = Class::new("on");
const MARK: Class = Class::new("mark");

#[derive(Signals, Serialize, Deserialize, Clone)]
struct Tag {
    label: String,
    on: bool,
}

#[derive(Signals, Serialize, Deserialize, Clone)]
struct Row {
    id: i32,
    label: String,
    done: bool,
    tags: Vec<Tag>,
}

#[derive(Signals, Serialize, Deserialize)]
struct Board {
    title: String,
    note: String,
    count: i32,
    on: bool,
    maybe: Option<String>,
    rows: Vec<Row>,
}

/// `SplitMix64`, as `tests/link_roundtrip.rs` draws its cases.
struct Rng(u64);

impl Rng {
    fn next(&mut self) -> u64 {
        self.0 = self.0.wrapping_add(0x9E37_79B9_7F4A_7C15);
        let mut z = self.0;
        z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
        z ^ (z >> 31)
    }

    fn below(&mut self, n: usize) -> usize {
        usize::try_from(self.next() % u64::try_from(n).unwrap()).unwrap()
    }

    fn chance(&mut self, percent: usize) -> bool {
        self.below(100) < percent
    }

    fn pick<T: Clone>(&mut self, from: &[T]) -> T {
        from[self.below(from.len())].clone()
    }
}

/// Strings that a text or an attribute writes in a way that differs from
/// their characters: markup, entities, line breaks, outside the BMP, empty.
const TEXTS: [&str; 12] = [
    "",
    " ",
    "tea",
    "<b>&amp;</b>",
    "\"quoted\" 'single'",
    "a\nb",
    "  padded  ",
    "😀 emoji",
    "日本語",
    "\r\n",
    "a&b<c>d",
    "\u{a0}nbsp",
];

/// A way to bind an input to a writable signal of the scope it sits in.
type Binder = Rc<dyn Fn(El<Board>) -> El<Board>>;

/// What the elements at one place of the tree can read and bind.
#[derive(Clone)]
struct Scope {
    texts: Vec<Expr<Board, String>>,
    conds: Vec<Expr<Board, bool>>,
    text_inputs: Vec<Binder>,
    checkboxes: Vec<Binder>,
    /// Whether a list of the page's rows can start here.
    rows: bool,
    /// The tags of the row this place sits in, which a list can draw.
    tags: Option<hypersig::Sig<Board, Vec<Tag>, hypersig::Local>>,
}

fn page_scope() -> Scope {
    let title = Board::title();
    let note = Board::note();
    let count = Board::count();
    let on = Board::on();
    let maybe = Board::maybe();
    Scope {
        texts: vec![
            title.into_expr(),
            note.into_expr(),
            count.to_text(),
            title.concat(note),
            Board::rows().len().to_text(),
            note.trim(),
        ],
        conds: vec![on.into_expr(), !on, maybe.is_some(), count.gt(0)],
        text_inputs: vec![
            Rc::new(move |el: El<Board>| el.bind(title)),
            Rc::new(move |el: El<Board>| el.bind(note)),
        ],
        checkboxes: vec![Rc::new(move |el: El<Board>| el.bind(on))],
        rows: true,
        tags: None,
    }
}

struct Gen {
    rng: Rng,
}

/// What the HTML parser lets an element hold without moving it elsewhere,
/// because a tree it rearranges says nothing about the runtime.
#[derive(Clone, Copy, PartialEq)]
enum Content {
    /// Any element, as a `div` holds.
    Flow,
    /// Text-level elements only, as a `p`, `span`, `label`, or `button` holds.
    Phrasing,
}

impl Gen {
    fn element(&mut self, depth: u32, scope: &Scope, content: Content) -> El<Board> {
        let leaf = depth >= MAX_DEPTH || self.rng.chance(30);
        let choices = if content == Content::Flow { 9 } else { 5 };
        let (el, holds) = match self.rng.below(choices) {
            0 => {
                let binder = self.rng.pick(&scope.text_inputs);
                return self.decorate(binder(input()), scope);
            }
            1 => {
                let binder = self.rng.pick(&scope.checkboxes);
                return self.decorate(binder(input().attr(attr::TYPE, "checkbox")), scope);
            }
            2 => {
                let binder = self.rng.pick(&scope.text_inputs);
                return self.decorate(binder(textarea()), scope);
            }
            3 | 4 => (span(), Content::Phrasing),
            5 => (p(), Content::Phrasing),
            6 => (button(), Content::Phrasing),
            7 => (label(), Content::Phrasing),
            _ => (div(), Content::Flow),
        };
        let mut el = self.decorate(el, scope);
        if leaf {
            return match self.rng.below(3) {
                0 => el.bind_text(self.rng.pick(&scope.texts)),
                1 => el.text(self.rng.pick(&TEXTS)),
                _ => el,
            };
        }
        for _ in 0..=self.rng.below(3) {
            el = el.child(self.node(depth + 1, scope, holds));
        }
        el
    }

    fn decorate(&mut self, mut el: El<Board>, scope: &Scope) -> El<Board> {
        if self.rng.chance(30) {
            el = el.bind_attr(attr::TITLE, self.rng.pick(&scope.texts));
        }
        if self.rng.chance(30) {
            el = el.class_when(ON, self.rng.pick(&scope.conds));
        }
        if self.rng.chance(15) {
            el = el.class(MARK);
        }
        if self.rng.chance(25) {
            el = el.show(self.rng.pick(&scope.conds));
        }
        el
    }

    /// A list of `li` under a `ul`, with a fixed item before or after it at times.
    fn list(&mut self, depth: u32, scope: &Scope) -> El<Board> {
        let items = if let Some(tags) = scope.tags {
            each(tags, |tag| {
                let scope = tag_scope(scope, tag);
                self.item(depth + 1, &scope)
            })
        } else {
            let keyed = self.rng.chance(50);
            let render = |row: hypersig::Item<Board, Row>| {
                let scope = row_scope(scope, row);
                self.item(depth + 1, &scope)
            };
            if keyed {
                each_keyed(Board::rows(), |row| row.at(Row::id()), render)
            } else {
                each(Board::rows(), render)
            }
        };
        let mut list = self.decorate(ul(), scope);
        if self.rng.chance(25) {
            list = list.child(li().text("first"));
        }
        list = list.child(items);
        if self.rng.chance(25) {
            list = list.child(li().text("last"));
        }
        list
    }

    fn item(&mut self, depth: u32, scope: &Scope) -> El<Board> {
        let mut item = self.decorate(li(), scope);
        for _ in 0..=self.rng.below(2) {
            item = item.child(self.node(depth + 1, scope, Content::Flow));
        }
        item
    }

    fn node(&mut self, depth: u32, scope: &Scope, content: Content) -> Node<Board> {
        let lists =
            content == Content::Flow && depth < MAX_DEPTH && (scope.rows || scope.tags.is_some());
        match self.rng.below(6) {
            0 => when(
                self.rng.pick(&scope.conds),
                self.element(depth, scope, content),
            ),
            1 if lists => self.list(depth, scope).into(),
            _ => self.element(depth, scope, content).into(),
        }
    }
}

fn row_scope(outer: &Scope, row: hypersig::Item<Board, Row>) -> Scope {
    let label = row.at(Row::label());
    let done = row.at(Row::done());
    let mut scope = outer.clone();
    scope.texts.extend([
        label.into_expr(),
        row.at(Row::id()).to_text(),
        row.index().to_text(),
        label.concat(Board::title()),
    ]);
    scope.conds.extend([done.into_expr(), !done]);
    scope
        .text_inputs
        .push(Rc::new(move |el: El<Board>| el.bind(label)));
    scope
        .checkboxes
        .push(Rc::new(move |el: El<Board>| el.bind(done)));
    scope.rows = false;
    scope.tags = Some(row.at(Row::tags()));
    scope
}

fn tag_scope(outer: &Scope, tag: hypersig::Item<Board, Tag>) -> Scope {
    let label = tag.at(Tag::label());
    let on = tag.at(Tag::on());
    let mut scope = outer.clone();
    scope.texts.push(label.into_expr());
    scope.conds.push(on.into_expr());
    scope
        .text_inputs
        .push(Rc::new(move |el: El<Board>| el.bind(label)));
    scope
        .checkboxes
        .push(Rc::new(move |el: El<Board>| el.bind(on)));
    scope.tags = None;
    scope
}

fn signals(rng: &mut Rng) -> Board {
    let text = |rng: &mut Rng| rng.pick(&TEXTS).to_owned();
    let rows = (0..rng.below(4))
        .map(|id| Row {
            id: i32::try_from(id).unwrap(),
            label: text(rng),
            done: rng.chance(50),
            tags: (0..rng.below(3))
                .map(|_| Tag {
                    label: text(rng),
                    on: rng.chance(50),
                })
                .collect(),
        })
        .collect();
    Board {
        title: text(rng),
        note: text(rng),
        count: i32::try_from(rng.below(3)).unwrap() - 1,
        on: rng.chance(50),
        maybe: rng.chance(50).then(|| text(rng)),
        rows,
    }
}

#[test]
fn adopted_pages() {
    let mut generator = Gen { rng: Rng(SEED) };
    let pages: Vec<String> = (0..PAGES)
        .map(|_| {
            let scope = page_scope();
            let signals = signals(&mut generator.rng);
            let mut root = div();
            for _ in 0..=generator.rng.below(3) {
                root = root.child(generator.node(1, &scope, Content::Flow));
            }
            Page::new("Adopt", signals)
                .runtime_src(RUNTIME_SRC)
                .child(root)
                .render()
        })
        .collect();
    let text = format!(
        "{}\n",
        serde_json::to_string(&json!({ "pages": pages })).expect("fixture JSON")
    );
    let path =
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../runtime/test/fixtures/adopt.json");
    if std::env::var("HYPERSIG_FIXTURES").as_deref() == Ok("overwrite") {
        std::fs::write(&path, text).expect("write fixture");
        return;
    }
    let on_disk = std::fs::read_to_string(&path).unwrap_or_default();
    assert!(
        on_disk == text,
        "{} is out of date; rerun with HYPERSIG_FIXTURES=overwrite",
        path.display()
    );
}
