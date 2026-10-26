//! The elements of a page that carry one handle, found where the browser
//! holds them once the runtime has started, for [`crate::testing`].

use serde_json::Value;

use super::{El, Event, Node, NodeKind};
use crate::ssr::{self, Ctx, Slot};
use crate::tape::Ref;

/// What a test finds an element by.
///
/// It is `pub` inside a crate-private module, so the sealed trait of
/// `crate::testing` can return it.
#[derive(Clone, Debug)]
pub enum Mark {
    /// The `id` an [`Id`](super::Id) wrote.
    Id(String),
    /// The name of a [`NodeRef`](crate::NodeRef) or a [`DialogRef`](crate::DialogRef).
    Ref(&'static str),
    /// The name of a [`FileInput`](crate::FileInput).
    Files(&'static str),
}

impl Mark {
    fn matches<S>(&self, el: &El<S>) -> bool {
        match self {
            Mark::Id(id) => el.id.as_ref().is_some_and(|own| own.as_str() == id),
            Mark::Ref(name) => el.node_ref == Some(*name),
            Mark::Files(name) => el.file_input == Some(*name),
        }
    }
}

/// The store path of an enclosing list item, and its position in its list.
#[derive(Clone, Debug)]
pub(crate) struct ItemAt {
    pub(crate) path: String,
    pub(crate) index: usize,
}

/// One element that carries the handle, with what surrounds it.
pub(crate) struct Found<'n, S> {
    /// The elements from the outermost one down to the found one, each with
    /// the number of list items around it.
    chain: Vec<(&'n El<S>, usize)>,
    /// The list items around the found element, outermost first.
    pub(crate) scope: Vec<ItemAt>,
    /// Whether `show` hides it or an element around it.
    pub(crate) hidden: bool,
}

/// The elements under `nodes` that `mark` names, in document order, as the
/// page shows them with the signal values `root`.
///
/// A list writes its template once per item, and a condition writes its
/// element only while it holds, so an element inside either is found once
/// per copy the page shows.
pub(crate) fn find<'n, S>(nodes: &'n [Node<S>], root: &Value, mark: &Mark) -> Vec<Found<'n, S>> {
    let mut walk = Walk {
        root,
        mark,
        chain: Vec::new(),
        found: Vec::new(),
    };
    for node in nodes {
        walk.node(node, &[], false);
    }
    walk.found
}

struct Walk<'n, 'r, 'm, S> {
    root: &'r Value,
    mark: &'m Mark,
    chain: Vec<(&'n El<S>, usize)>,
    found: Vec<Found<'n, S>>,
}

impl<'n, S> Walk<'n, '_, '_, S> {
    fn node(&mut self, node: &'n Node<S>, scope: &[ItemAt], hidden: bool) {
        match &node.0 {
            NodeKind::Element(el) => self.el(el, scope, hidden),
            NodeKind::Text(_) => {}
            NodeKind::Each {
                list,
                depth,
                template,
            } => {
                let path = resolve(list, scope);
                let len = Ref::from(self.root)
                    .get_path(&path)
                    .and_then(Ref::items)
                    .map_or(0, Iterator::count);
                let depth = usize::from(*depth).min(scope.len());
                for index in 0..len {
                    let mut inner = scope[..depth].to_vec();
                    inner.push(ItemAt {
                        path: join(&path, &index.to_string()),
                        index,
                    });
                    self.el(template, &inner, hidden);
                }
            }
            NodeKind::When { cond, template } => {
                if evaluate(cond.as_json(), self.root, scope, &[]).is_some_and(|v| ssr::truthy(&v))
                {
                    self.el(template, scope, hidden);
                }
            }
        }
    }

    fn el(&mut self, el: &'n El<S>, scope: &[ItemAt], hidden: bool) {
        let hidden = hidden
            || el.show.as_ref().is_some_and(|show| {
                evaluate(show.as_json(), self.root, scope, &[]).is_some_and(|v| !ssr::truthy(&v))
            });
        self.chain.push((el, scope.len()));
        if self.mark.matches(el) {
            self.found.push(Found {
                chain: self.chain.clone(),
                scope: scope.to_vec(),
                hidden,
            });
        }
        for child in &el.children {
            self.node(child, scope, hidden);
        }
        self.chain.pop();
    }
}

impl<'n, S> Found<'n, S> {
    fn el(&self) -> &'n El<S> {
        self.chain
            .last()
            .expect("a found element is on its own chain")
            .0
    }

    /// The element's tag name.
    pub(crate) fn tag(&self) -> &'static str {
        self.el().tag
    }

    /// The value of the attribute `name` the element was built with, leaving
    /// out the ones bound to a signal.
    pub(crate) fn static_attr(&self, name: &str) -> Option<&'n str> {
        self.el()
            .attrs
            .iter()
            .find(|(attr, _)| *attr == name)
            .map(|(_, value)| value.as_str())
    }

    /// The element's `[place, kind, choice]` binding, if it is a bound control.
    pub(crate) fn bind(&self) -> Option<&'n Value> {
        self.el().bind.as_ref()
    }

    /// The position in the chain of the closest `<form>` from the element up.
    pub(crate) fn form(&self) -> Option<usize> {
        self.chain.iter().rposition(|(el, _)| el.tag == "form")
    }

    /// The statement of the closest element from the chain's position `at`
    /// up that runs one on `event`, with the list items around that element.
    ///
    /// This is `closest` in `runtime/src/runtime/binding/event.ts`, which
    /// runs the handler of one element for an event that bubbles.
    pub(crate) fn handler(&self, at: usize, event: Event) -> Option<(&'n Value, &[ItemAt])> {
        self.chain[..=at].iter().rev().find_map(|(el, depth)| {
            el.on
                .get(&event)
                .map(|stmt| (stmt.as_json(), &self.scope[..*depth]))
        })
    }

    /// The position of the found element in its chain.
    pub(crate) fn last(&self) -> usize {
        self.chain.len() - 1
    }

    /// The element's HTML as the server writes it from `root`, without the
    /// `data-hs-*` attributes the runtime reads.
    pub(crate) fn render(&self, root: &Value) -> String {
        let mut out = String::new();
        with_ctx(root, &self.scope, &[], |ctx| {
            self.el().write(&mut out, Some(ctx), None);
        });
        out
    }
}

/// The store path a place names: `"a.b"` for a page signal, or
/// `["item",depth,"a"]` for a field of the list item at `depth`.
///
/// # Panics
///
/// Panics if `place` reads a list item that `scope` does not hold, as
/// `slotAt` in `runtime/src/eval/scope.ts` throws.
pub(crate) fn resolve(place: &Value, scope: &[ItemAt]) -> String {
    match place {
        Value::String(path) => path.clone(),
        Value::Array(parts) if parts.first().and_then(Value::as_str) == Some("item") => {
            let depth = parts
                .get(1)
                .and_then(Value::as_u64)
                .and_then(|depth| usize::try_from(depth).ok())
                .unwrap_or(usize::MAX);
            let item = scope.get(depth).unwrap_or_else(|| {
                panic!("a statement reads the list item at depth {depth}, and no list encloses it")
            });
            join(
                &item.path,
                parts.get(2).and_then(Value::as_str).unwrap_or(""),
            )
        }
        other => panic!("{other} is not a place a statement can write"),
    }
}

/// `path` and `field` joined with a dot, as `join` in `runtime/src/store/path.ts` joins them.
pub(crate) fn join(path: &str, field: &str) -> String {
    match (path, field) {
        ("", field) => field.to_owned(),
        (path, "") => path.to_owned(),
        (path, field) => format!("{path}.{field}"),
    }
}

/// `ast` evaluated against `root` with the list items of `scope` and the
/// `let` values `vars`, or `None` where the server cannot evaluate it.
pub(crate) fn evaluate<'a>(
    ast: &'a Value,
    root: &'a Value,
    scope: &[ItemAt],
    vars: &[Value],
) -> Option<Value> {
    with_ctx(root, scope, vars, |ctx| {
        ssr::evaluate(ast, ctx).map(ssr::Val::into_value)
    })
}

/// Runs `body` with the context the server renders an element in.
///
/// Every item of `scope` is read from `root` again, because a statement
/// changes `root` between two reads.
fn with_ctx<'a, R>(
    root: &'a Value,
    scope: &[ItemAt],
    vars: &[Value],
    body: impl FnOnce(&Ctx<'a>) -> R,
) -> R {
    let root = Ref::from(root);
    let ctx = Ctx {
        root,
        scope: scope
            .iter()
            .map(|item| Slot::new(root.get_path(&item.path).unwrap_or(Ref::NULL), item.index))
            .collect(),
        copy: true,
        vars: vars.to_vec(),
        rules: None,
    };
    body(&ctx)
}
