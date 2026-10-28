//! The statement interpreter of a [`Simulation`], one arm per statement of
//! `runtime/src/eval/stmt/`.

use std::cmp::Ordering;

use serde_json::Value;

use super::{Effects, Sent, Simulation, Visit, Waiting, js_order};
use crate::html::find::{self, ItemAt, join};
use crate::route::Url;
use crate::signal::Signals;
use crate::ssr;

impl<'p, S: Signals> Simulation<'p, S> {
    /// Runs the statement `ast` with the list items `scope` and the `let`
    /// values `vars` around it.
    ///
    /// # Panics
    ///
    /// Panics on a statement the browser alone can run, and on one whose
    /// value the server cannot evaluate.
    #[expect(
        clippy::too_many_lines,
        reason = "one arm per statement, as `statements` in runtime/src/eval.ts lists them"
    )]
    pub(super) fn run(
        &mut self,
        ast: &'p Value,
        scope: &[ItemAt],
        vars: &[Value],
        effects: &mut Effects<S>,
    ) {
        let parts = match ast.as_array() {
            Some(parts) => parts.as_slice(),
            None => panic!("{ast} is not a statement"),
        };
        let op = parts.first().and_then(Value::as_str).unwrap_or("");
        let arg = |i: usize| parts.get(i).unwrap_or(&Value::Null);
        match op {
            "set" => {
                let path = find::resolve(arg(1), scope);
                let value = self.eval(arg(2), scope, vars, op);
                self.set(&path, value);
            }
            "if" => {
                let cond = self.eval(arg(1), scope, vars, op);
                let branch = if ssr::truthy(&cond) { arg(2) } else { arg(3) };
                self.run(branch, scope, vars, effects);
            }
            "let" => {
                let value = self.eval(arg(1), scope, vars, op);
                let mut inner = vars[..index(arg(2)).min(vars.len())].to_vec();
                inner.push(value);
                self.run(arg(3), scope, &inner, effects);
            }
            "seq" => {
                for (at, step) in arg(1).as_array().into_iter().flatten().enumerate() {
                    if at > 0 {
                        self.settle();
                    }
                    self.run(step, scope, vars, effects);
                }
            }
            // The browser animates the change; the writes are the same.
            "transition" => self.run(arg(1), scope, vars, effects),
            "debounce" => {
                let key = format!(
                    "{}|{}",
                    arg(2),
                    scope
                        .iter()
                        .map(|item| item.path.as_str())
                        .collect::<Vec<_>>()
                        .join("|")
                );
                self.waiting.retain(|waiting| waiting.key != key);
                self.waiting.push(Waiting {
                    key,
                    stmt: arg(2),
                    scope: scope.to_vec(),
                    vars: vars.to_vec(),
                });
            }
            "push" => {
                let (path, mut items) = self.list(arg(1), scope);
                items.push(self.eval(arg(2), scope, vars, op));
                self.set(&path, Value::Array(items));
            }
            "append" | "prepend" => {
                let (path, items) = self.list(arg(1), scope);
                let added = self.eval(arg(2), scope, vars, op);
                let Some(added) = added.as_array().filter(|added| !added.is_empty()) else {
                    return;
                };
                let next = if op == "prepend" {
                    added.iter().chain(&items).cloned().collect()
                } else {
                    items.iter().chain(added).cloned().collect()
                };
                self.set(&path, Value::Array(next));
            }
            "remove" => {
                let (path, mut items) = self.list(arg(1), scope);
                let at = self.eval(arg(2), scope, vars, op);
                if let Some(at) = position(&at, items.len()) {
                    items.remove(at);
                    self.set(&path, Value::Array(items));
                }
            }
            "swap" => {
                let (path, mut items) = self.list(arg(1), scope);
                let a = self.eval(arg(2), scope, vars, op);
                let b = self.eval(arg(3), scope, vars, op);
                if let (Some(a), Some(b)) = (position(&a, items.len()), position(&b, items.len()))
                    && a != b
                {
                    items.swap(a, b);
                    self.set(&path, Value::Array(items));
                }
            }
            "extend" => {
                let (path, mut items) = self.list(arg(1), scope);
                let count = self.eval(arg(2), scope, vars, op);
                let count = count
                    .as_f64()
                    .filter(|count| *count > 0.0)
                    .map_or(0, |count| {
                        #[expect(
                            clippy::cast_possible_truncation,
                            clippy::cast_sign_loss,
                            reason = "a positive count truncates, as `Array.from({ length })` does"
                        )]
                        let count = count as usize;
                        count
                    });
                let depth = index(arg(3)).min(scope.len());
                for at in 0..count {
                    // The runtime's slot for the new item holds only its
                    // position, which `["index",depth]` reads.
                    let mut inner = scope[..depth].to_vec();
                    inner.push(ItemAt {
                        path: at.to_string(),
                        index: at,
                    });
                    items.push(self.eval(arg(4), &inner, vars, op));
                }
                if count > 0 {
                    self.set(&path, Value::Array(items));
                }
            }
            "retain" => {
                let (path, items) = self.list(arg(1), scope);
                let keep = self.each(&path, items.len(), index(arg(2)), arg(3), scope, vars, op);
                let kept: Vec<Value> = items
                    .iter()
                    .zip(&keep)
                    .filter(|(_, keep)| ssr::truthy(*keep))
                    .map(|(item, _)| item.clone())
                    .collect();
                if kept.len() != items.len() {
                    self.set(&path, Value::Array(kept));
                }
            }
            "update" => {
                let (path, items) = self.list(arg(1), scope);
                let depth = index(arg(2));
                let when = self.each(&path, items.len(), depth, arg(3), scope, vars, op);
                let field = arg(4).as_str().unwrap_or("");
                // Every value is evaluated before the first write, as
                // `setMany` in the runtime writes them together.
                let mut writes = Vec::new();
                for (at, when) in when.iter().enumerate() {
                    if ssr::truthy(when) {
                        let inner = item_scope(scope, depth, &path, at);
                        let value = self.eval(arg(5), &inner, vars, op);
                        writes.push((join(&join(&path, &at.to_string()), field), value));
                    }
                }
                for (target, value) in writes {
                    self.set(&target, value);
                }
            }
            "sort" => {
                let (path, items) = self.list(arg(1), scope);
                let keys = self.each(&path, items.len(), index(arg(2)), arg(3), scope, vars, op);
                let descending = arg(4) == &Value::Bool(true);
                let mut order: Vec<usize> = (0..items.len()).collect();
                order.sort_by(|&a, &b| match js_order(&keys[a], &keys[b]) {
                    Ordering::Equal => a.cmp(&b),
                    order if descending => order.reverse(),
                    order => order,
                });
                if order.iter().enumerate().any(|(at, from)| at != *from) {
                    let sorted = order.iter().map(|from| items[*from].clone()).collect();
                    self.set(&path, Value::Array(sorted));
                }
            }
            "set_all" => {
                let (path, items) = self.list(arg(1), scope);
                let field = arg(2).as_str().unwrap_or("");
                let value = self.eval(arg(3), scope, vars, op);
                for at in 0..items.len() {
                    self.set(&join(&join(&path, &at.to_string()), field), value.clone());
                }
            }
            "call" => {
                let method = match arg(1).as_str() {
                    Some("GET") => "GET",
                    _ => "POST",
                };
                self.call(method, &parts[2..], scope, vars, effects);
            }
            "upload" => self.call("POST", &parts[1..], scope, vars, effects),
            "dom" | "clear-files" => effects.browser.push(ast.clone()),
            "navigate" => {
                let url = self.url(arg(1), scope, vars, op);
                effects.visits.push((Visit::Navigate, url));
            }
            "history" => {
                let url = self.url(arg(2), scope, vars, op);
                let visit = if arg(1) == "replace" {
                    Visit::Replace
                } else {
                    Visit::Push
                };
                effects.visits.push((visit, url));
            }
            "upsert" | "remove_keys" => panic!(
                "`{op}` is a list statement that only a server patch sends, and the \
                 simulation applies no patch"
            ),
            _ => panic!("`{op}` is not a statement the runtime runs: {ast}"),
        }
    }

    /// Records a call, after the writes `Actions.call` in
    /// `runtime/src/runtime/action/call.ts` makes before it sends one.
    ///
    /// `rest` is `[path, args, indicator, error, flags?, outcomes?]`, the
    /// call's operands after its method. The arguments are evaluated after
    /// those writes, as the runtime builds the request after them.
    fn call(
        &mut self,
        method: &'static str,
        rest: &'p [Value],
        scope: &[ItemAt],
        vars: &[Value],
        effects: &mut Effects<S>,
    ) {
        let arg = |i: usize| rest.get(i).unwrap_or(&Value::Null);
        let path = arg(0).as_str().unwrap_or("").to_owned();
        let flags: Vec<&str> = arg(4)
            .as_array()
            .into_iter()
            .flatten()
            .filter_map(Value::as_str)
            .collect();
        let outcomes = arg(5);
        let part = outcomes.get("part").and_then(Value::as_str).unwrap_or("");
        let key = format!("{method} {path} {part}");
        if flags.contains(&"exclusive") && self.sent.contains(&key) {
            return;
        }
        if let Some(error) = arg(3).as_str() {
            self.set(error, Value::from(""));
        }
        if let Some(status) = outcomes.get("status").and_then(Value::as_str) {
            self.set(status, Value::from(0));
        }
        if !flags.contains(&"connected")
            && let Some(indicator) = arg(2).as_str()
        {
            self.set(indicator, Value::Bool(true));
        }
        let args = match arg(1) {
            Value::Null => Value::Null,
            args => self.eval(args, scope, vars, "call"),
        };
        self.sent.push(key);
        effects.calls.push(Sent { method, path, args });
    }

    /// The URL `ast` evaluates to, written as `String(value)` writes it.
    fn url(&self, ast: &Value, scope: &[ItemAt], vars: &[Value], op: &str) -> Url {
        Url::evaluated(ssr::js_string(&self.eval(ast, scope, vars, op)))
    }

    /// The store path of the list at `place`, and its items, none where it holds no list.
    fn list(&self, place: &Value, scope: &[ItemAt]) -> (String, Vec<Value>) {
        let path = find::resolve(place, scope);
        let items = match self.read(&path) {
            Value::Array(items) => items,
            _ => Vec::new(),
        };
        (path, items)
    }

    /// `ast` evaluated once per item of the list at `path`, which has
    /// `len` items, with the item standing at `depth`.
    #[expect(
        clippy::too_many_arguments,
        reason = "the operands of the list statement and the context it runs in"
    )]
    fn each(
        &self,
        path: &str,
        len: usize,
        depth: usize,
        ast: &Value,
        scope: &[ItemAt],
        vars: &[Value],
        op: &str,
    ) -> Vec<Value> {
        (0..len)
            .map(|at| self.eval(ast, &item_scope(scope, depth, path, at), vars, op))
            .collect()
    }

    /// The value of `ast`, which the statement `op` reads.
    ///
    /// # Panics
    ///
    /// Panics where the server cannot evaluate it.
    fn eval(&self, ast: &Value, scope: &[ItemAt], vars: &[Value], op: &str) -> Value {
        find::evaluate(ast, &self.values, scope, vars).unwrap_or_else(|| {
            assert!(
                !reads_browser(ast),
                "the `{op}` statement reads a value only the browser has, such as `Expr::now`, \
                 `Expr::random_below`, or a date in the visitor's zone, so a simulation cannot \
                 run it: {ast}"
            );
            panic!(
                "the `{op}` statement computes a value the server cannot evaluate, such as a \
                 number JSON cannot hold, so a simulation cannot run it: {ast}"
            )
        })
    }
}

/// The list items of `scope` up to `depth`, then the item at `at` of the list at `path`.
fn item_scope(scope: &[ItemAt], depth: usize, path: &str, at: usize) -> Vec<ItemAt> {
    let mut inner = scope[..depth.min(scope.len())].to_vec();
    inner.push(ItemAt {
        path: join(path, &at.to_string()),
        index: at,
    });
    inner
}

/// `value` as a depth or a count written into a statement.
fn index(value: &Value) -> usize {
    value
        .as_u64()
        .and_then(|value| usize::try_from(value).ok())
        .unwrap_or(0)
}

/// The position `value` names in a list of `len` items, where it names one,
/// as `index in items` tests it in JavaScript.
fn position(value: &Value, len: usize) -> Option<usize> {
    let at = value.as_f64()?;
    if at < 0.0 || at.fract() != 0.0 {
        return None;
    }
    #[expect(
        clippy::cast_possible_truncation,
        clippy::cast_sign_loss,
        reason = "a non-negative integer, compared with `len` below"
    )]
    let at = at as usize;
    (at < len).then_some(at)
}

/// Whether `ast` reads the browser's clock, draws a random number, or
/// writes a date in the browser's zone, which the server leaves out.
fn reads_browser(ast: &Value) -> bool {
    let Some(parts) = ast.as_array() else {
        return false;
    };
    match parts.first().and_then(Value::as_str) {
        Some("now" | "random") => true,
        Some("date") if parts.get(5).is_some_and(Value::is_null) => true,
        _ => parts.iter().any(reads_browser),
    }
}

#[cfg(test)]
mod tests;
