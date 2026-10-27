//! Writes statements, the signals they run against, and the signals the
//! simulation leaves after running each to
//! `runtime/test/fixtures/differential_stmt.json`, which
//! `runtime/test/differential_stmt.test.ts` runs again with
//! `runtime/src/eval/stmt/` and compares. Each case holds only the signals
//! whose value the statement changed.
//!
//! The statements are drawn with the expression generator of
//! `ssr/differential.rs` and keep to the shapes the Rust builder in
//! `expr.rs` and `signal.rs` writes. A case runs at the page's root or
//! inside an item of `rows`, as a statement on an element of a list item
//! runs. Only the statements the simulation runs to the end, without an
//! expression it cannot evaluate, go into the file.
//!
//! It runs with `cargo test -p hypersig --lib testing::differential`, and
//! `HYPERSIG_FIXTURES=overwrite` rewrites the file.

use std::collections::BTreeSet;
use std::fmt::Write as _;
use std::panic::{self, AssertUnwindSafe};
use std::path::PathBuf;
use std::sync::Arc;
use std::thread;

use serde::{Deserialize, Serialize};
use serde_json::{Value, json};

use super::{Effects, Simulation};
use crate::html::find::ItemAt;
use crate::ssr::differential::{El, Gen, Rng, TYPES, Ty, operations, row_fields, sig_paths};
use crate::{Page, Signals};

/// The seed of the generator, fixed so that the file changes only with the code.
const SEED: u64 = 0x7374_6d74_7473_6967;

const GROUPS: usize = 60;

const CASES_PER_GROUP: usize = 100;

/// The most statements from a case's root to a leaf statement.
const MAX_DEPTH: u32 = 3;

/// The most operations from an expression's root to a leaf.
const EXPR_DEPTH: u32 = 2;

/// Every statement the simulation runs on the page's signals.
const STATEMENTS: [&str; 15] = [
    "set", "if", "let", "seq", "push", "append", "prepend", "remove", "swap", "extend", "retain",
    "update", "sort", "set_all", "toggle",
];

/// The one list whose items have fields, which `update` and `set_all` write.
const ROWS: &str = "rows";

/// The page the simulation runs against. Its signals are the values of
/// each group, and it computes no signal, so `seq` changes nothing between
/// its steps.
#[derive(Signals, Serialize, Deserialize)]
#[expect(
    clippy::empty_structs_with_brackets,
    reason = "`derive(Signals)` takes a struct with named fields"
)]
struct Blank {}

impl Gen {
    /// A place a statement writes a value of type `ty` to: a page signal, a
    /// field of an enclosing row, or an enclosing item itself.
    fn target(&mut self, ty: Ty) -> Option<Value> {
        let mut options: Vec<Value> = sig_paths(ty).iter().map(|path| json!(path)).collect();
        for (depth, el) in self.items.iter().enumerate() {
            if *el == El::Row {
                for field in row_fields(ty) {
                    options.push(json!(["item", depth, field]));
                }
            }
            if el.ty() == ty {
                options.push(json!(["item", depth, ""]));
            }
        }
        if options.is_empty() {
            return None;
        }
        let at = self.rng.below(options.len());
        Some(options.swap_remove(at))
    }

    /// An expression of type `ty` read with one more list item around it,
    /// of type `el`, as the body of a list statement reads its item.
    fn per_item(&mut self, el: El, ty: Ty) -> (usize, Value) {
        let depth = self.items.len();
        self.items.push(el);
        let body = self.expr(ty, EXPR_DEPTH);
        self.items.pop();
        (depth, body)
    }

    fn stmt(&mut self, budget: u32) -> Value {
        let b = budget.saturating_sub(1);
        let nested = budget > 0;
        loop {
            match *self.rng.pick(&STATEMENTS) {
                "set" => {
                    let ty = *self.rng.pick(&TYPES);
                    if let Some(place) = self.target(ty) {
                        return json!(["set", place, self.expr(ty, EXPR_DEPTH)]);
                    }
                }
                // `toggle`, which writes `set` of the negation.
                "toggle" => {
                    let place = self.target(Ty::Bool).expect("a flag to toggle");
                    let read = match &place {
                        Value::String(path) => json!(["sig", path]),
                        item => item.clone(),
                    };
                    return json!(["set", place, ["not", read]]);
                }
                "if" if nested => {
                    let cond = self.expr(Ty::Bool, EXPR_DEPTH);
                    let then = self.stmt(b);
                    // `Stmt::when` writes an empty `seq` where it has no other branch.
                    let otherwise = if self.rng.chance(50) {
                        json!(["seq", []])
                    } else {
                        self.stmt(b)
                    };
                    return json!(["if", cond, then, otherwise]);
                }
                "let" if nested => {
                    let ty = *self.rng.pick(&TYPES);
                    let value = self.expr(ty, EXPR_DEPTH);
                    let depth = self.vars.len();
                    self.vars.push(ty);
                    let body = self.stmt(b);
                    self.vars.pop();
                    return json!(["let", value, depth, body]);
                }
                "seq" if nested => {
                    let steps: Vec<Value> =
                        (0..2 + self.rng.below(2)).map(|_| self.stmt(b)).collect();
                    return json!(["seq", steps]);
                }
                op => {
                    if let Some(stmt) = self.list_stmt(op) {
                        return stmt;
                    }
                }
            }
        }
    }

    /// A list statement `op`, or `None` where `op` is not one or the drawn
    /// statement is one the builder cannot write.
    fn list_stmt(&mut self, op: &str) -> Option<Value> {
        match op {
            "push" => {
                let (place, el) = self.place();
                Some(json!(["push", place, self.expr(el.ty(), EXPR_DEPTH)]))
            }
            op @ ("append" | "prepend") => {
                let (place, el) = self.place();
                Some(json!([op, place, self.expr(Ty::List(el), EXPR_DEPTH)]))
            }
            "remove" => {
                let (place, _) = self.place();
                Some(json!(["remove", place, self.index()]))
            }
            "swap" => {
                let (place, _) = self.place();
                Some(json!(["swap", place, self.index(), self.index()]))
            }
            "extend" => {
                let (place, el) = self.place();
                // The count is kept small, since a count of 2^31 is a
                // page that never answers on either side.
                let count = json!(["min", self.expr(Ty::Int, 1), ["lit", 3]]);
                // The new item's slot holds only its position, so the
                // body reads `["index",depth]` and no field of it.
                let depth = self.items.len();
                self.items.push(El::Int);
                let body = self.expr(el.ty(), EXPR_DEPTH);
                self.items.pop();
                if reads_item(&body, depth) {
                    return None;
                }
                Some(json!(["extend", place, count, depth, body]))
            }
            "retain" => {
                let (place, el) = self.place();
                let (depth, keep) = self.per_item(el, Ty::Bool);
                Some(json!(["retain", place, depth, keep]))
            }
            "sort" => {
                let (place, el) = self.place();
                let ty = *self.rng.pick(&[Ty::Num, Ty::Int, Ty::Str]);
                let (depth, key) = self.per_item(el, ty);
                // Most keys a page sorts by are a field of the item, and
                // a generated key is often the same for every item.
                let key = match el {
                    El::Row if self.rng.chance(70) => {
                        json!(["item", depth, *self.rng.pick(row_fields(ty))])
                    }
                    El::Num | El::Int | El::Str if self.rng.chance(70) => {
                        json!(["item", depth, ""])
                    }
                    _ => key,
                };
                Some(json!(["sort", place, depth, key, self.rng.chance(50)]))
            }
            "update" => {
                let place = json!(ROWS);
                let (depth, when) = self.per_item(El::Row, Ty::Bool);
                let ty = *self.rng.pick(&[Ty::Num, Ty::Int, Ty::Str, Ty::Bool]);
                let field = *self.rng.pick(row_fields(ty));
                let (_, value) = self.per_item(El::Row, ty);
                Some(json!(["update", place, depth, when, field, value]))
            }
            "set_all" => {
                let place = json!(ROWS);
                let ty = *self.rng.pick(&[Ty::Num, Ty::Int, Ty::Str, Ty::Bool]);
                let field = *self.rng.pick(row_fields(ty));
                Some(json!(["set_all", place, field, self.expr(ty, EXPR_DEPTH)]))
            }
            _ => None,
        }
    }

    /// A position in a list, as `remove_at` and `swap` take one: often a
    /// small literal, so that it lands inside the list.
    fn index(&mut self) -> Value {
        if self.rng.chance(60) {
            json!(["lit", i32::try_from(self.rng.below(5)).unwrap() - 1])
        } else {
            self.expr(Ty::Int, 1)
        }
    }
}

/// Whether `ast` reads a field of the item at `depth`, or the item itself.
fn reads_item(ast: &Value, depth: usize) -> bool {
    let Value::Array(parts) = ast else {
        return false;
    };
    if parts.first() == Some(&json!("lit")) {
        return false;
    }
    if parts.first() == Some(&json!("item")) && parts.get(1) == Some(&json!(depth)) {
        return true;
    }
    parts.iter().any(|part| reads_item(part, depth))
}

/// The signals of `after` whose value differs from the one in `before`,
/// which keeps the file to what each statement wrote.
fn changed(before: &Value, after: &Value) -> Value {
    let (Value::Object(before), Value::Object(after)) = (before, after) else {
        return after.clone();
    };
    Value::Object(
        after
            .iter()
            .filter(|(name, value)| before.get(*name) != Some(*value))
            .map(|(name, value)| (name.clone(), value.clone()))
            .collect(),
    )
}

/// The signals after the simulation runs `ast` against `signals` inside
/// the items `scope`, or `None` where it stops at an expression it cannot
/// evaluate.
fn simulate(page: &Page<Blank>, signals: &Value, ast: &Value, scope: &[ItemAt]) -> Option<Value> {
    let mut simulation = Simulation::new(page, signals.clone());
    let mut effects = Effects::new();
    panic::catch_unwind(AssertUnwindSafe(|| {
        simulation.run(ast, scope, &[], &mut effects);
    }))
    .ok()?;
    Some(simulation.values)
}

#[test]
#[cfg_attr(
    miri,
    ignore = "it reads and writes a fixture file, which miri's isolation refuses, and runs 6,000 statements"
)]
fn writes_what_the_simulation_leaves_for_the_runtime_to_compare() {
    let page = Page::new("Differential", Blank {});
    let mut generator = Gen {
        rng: Rng(SEED),
        items: Vec::new(),
        vars: Vec::new(),
    };
    let mut covered = BTreeSet::new();
    let (mut run, mut stopped) = (0_usize, 0_usize);
    let mut groups = Vec::new();
    // The simulation panics on an expression it cannot evaluate, and each
    // such case is left out, so the panic messages of this thread are not
    // printed. The hook is process-wide, and a test on another thread still
    // prints through the previous one.
    let previous = Arc::new(panic::take_hook());
    let here = thread::current().id();
    let forward = Arc::clone(&previous);
    panic::set_hook(Box::new(move |info| {
        if thread::current().id() != here {
            forward(info);
        }
    }));
    for _ in 0..GROUPS {
        let signals = generator.signals();
        let rows = signals["rows"].as_array().map_or(0, Vec::len);
        let mut cases = Vec::new();
        while cases.len() < CASES_PER_GROUP {
            let inside = (rows > 0 && generator.rng.chance(30)).then(|| generator.rng.below(rows));
            if inside.is_some() {
                generator.items.push(El::Row);
            }
            let depth = 1 + u32::try_from(generator.rng.below(usize::try_from(MAX_DEPTH).unwrap()))
                .unwrap();
            let ast = generator.stmt(depth);
            generator.items.clear();
            let scope: Vec<ItemAt> = inside
                .map(|index| ItemAt {
                    path: format!("rows.{index}"),
                    index,
                })
                .into_iter()
                .collect();
            let mut used = BTreeSet::new();
            operations(&ast, &mut used);
            // Rust takes `pow` and `log10` from the platform's math library,
            // whose last bit differs from V8's (#118) and between glibc and
            // macOS, so a case that uses either would make the file
            // depend on the machine that wrote it.
            if used.contains("pow") || used.contains("log10") {
                continue;
            }
            match simulate(&page, &signals, &ast, &scope) {
                Some(after) => {
                    covered.extend(used);
                    let slot = scope.first().map(|item| item.path.clone());
                    cases.push(
                        serde_json::to_string(&json!([ast, slot, changed(&signals, &after)]))
                            .unwrap(),
                    );
                }
                None => stopped += 1,
            }
        }
        run += cases.len();
        groups.push((signals, cases));
    }
    drop(panic::take_hook());
    if let Ok(previous) = Arc::try_unwrap(previous) {
        panic::set_hook(previous);
    }
    let missing: Vec<&str> = STATEMENTS
        .iter()
        .copied()
        .filter(|op| *op != "toggle" && !covered.contains(*op))
        .collect();
    assert!(missing.is_empty(), "no case that ran uses {missing:?}");

    let mut text = String::new();
    writeln!(
        text,
        "{{\"seed\":{SEED},\"run\":{run},\"stopped\":{stopped},\"groups\":["
    )
    .unwrap();
    for (at, (signals, cases)) in groups.iter().enumerate() {
        if at > 0 {
            text.push_str(",\n");
        }
        writeln!(text, "{{\"signals\":{signals},\"cases\":[").unwrap();
        text.push_str(&cases.join(",\n"));
        text.push_str("\n]}");
    }
    text.push_str("\n]}\n");

    let path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../runtime/test/fixtures/differential_stmt.json");
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
