//! The Reference section, one page per group of items.
//!
//! Each row links to the item in the rustdoc output that the site serves
//! under `api/`, and its summary is the first sentence of the item's doc
//! comment, shortened where it runs past one line of the table. The binary checks every link against that output when it is
//! given `--rustdoc`, so a renamed or removed item fails the build.

use crate::Site;
use crate::doc::{Doc, Reference};
use crate::prose::{Part, heading, lead, para, parts, table_of, title};

/// One item: its name, its rustdoc page and anchor, and its summary.
type Row = (&'static str, &'static str, &'static str);

/// A section of a reference page: a heading, an optional paragraph, and a
/// table of items.
struct Section {
    heading: &'static str,
    text: &'static str,
    rows: &'static [Row],
}

/// The article of `page`.
pub fn content(site: Site, page: Reference) -> Part {
    let doc = Doc::Reference(page);
    if page == Reference::Overview {
        return overview(site);
    }
    let (summary, sections) = page_items(page);
    let mut children = vec![title(doc.title()), lead(site, summary)];
    for section in sections {
        children.push(heading(section.heading));
        if !section.text.is_empty() {
            children.push(para(site, section.text));
        }
        let cells: Vec<[String; 2]> = section
            .rows
            .iter()
            .map(|(name, target, summary)| {
                [
                    format!("[`{name}`](api:hypersig/{target})"),
                    (*summary).to_owned(),
                ]
            })
            .collect();
        let rows: Vec<Vec<&str>> = cells
            .iter()
            .map(|[name, summary]| vec![name.as_str(), summary.as_str()])
            .collect();
        let rows: Vec<&[&str]> = rows.iter().map(Vec::as_slice).collect();
        children.push(table_of(site, &["Item", "Summary"], &rows));
    }
    parts(children)
}

fn overview(site: Site) -> Part {
    let rows: Vec<[String; 2]> = crate::doc::REFERENCE
        .iter()
        .flat_map(|group| group.docs.iter())
        .filter_map(|&doc| match doc {
            Doc::Reference(page) if page != Reference::Overview => Some([
                format!("[{}]({})", doc.title(), doc.path()),
                page_items(page).0.to_owned(),
            ]),
            _ => None,
        })
        .collect();
    let rows: Vec<Vec<&str>> = rows
        .iter()
        .map(|[page, summary]| vec![page.as_str(), summary.as_str()])
        .collect();
    let rows: Vec<&[&str]> = rows.iter().map(Vec::as_slice).collect();
    parts([
        title(Doc::Reference(Reference::Overview).title()),
        lead(
            site,
            "The items of the `hypersig` crate, grouped by what they build. Each name links to its full documentation.",
        ),
        para(
            site,
            "The [complete API documentation](api:hypersig/index.html) is the rustdoc output of the same commit as this site, with every method, its signature, and its examples. The pages here list the items you reach for most, each with the first sentence of its doc comment, shortened where it runs long.",
        ),
        table_of(site, &["Page", "What it covers"], &rows),
        para(
            site,
            "Styles live in `hypersig::style`, re-exported from the `hypersig_style` crate. See [Styling](learn/styling/) and the [`style` module](api:hypersig/style/index.html).",
        ),
    ])
}

/// The summary and the sections of `page`.
#[expect(clippy::too_many_lines, reason = "one table of data per page")]
fn page_items(page: Reference) -> (&'static str, &'static [Section]) {
    match page {
        Reference::Overview => ("", &[]),
        Reference::Page => (
            "`Page` is the full HTML document for a page, with its title, its initial signals, and the runtime.",
            &[Section {
                heading: "Page",
                text: "`Page::render` evaluates every binding with the initial signals, so the first HTML already shows what the runtime would compute.",
                rows: &[
                    (
                        "Page::new",
                        "struct.Page.html#method.new",
                        "Starts a page with its title and initial signal values.",
                    ),
                    (
                        "child",
                        "struct.Page.html#method.child",
                        "Appends a node to `<body>`.",
                    ),
                    (
                        "computed",
                        "struct.Page.html#method.computed",
                        "Keeps `signal` equal to `value`, recomputed when a signal `value` reads changes.",
                    ),
                    (
                        "theme",
                        "struct.Page.html#method.theme",
                        "Sets token values for the whole page.",
                    ),
                    (
                        "global",
                        "struct.Page.html#method.global",
                        "Adds rules for elements by their tag, and fonts, to the page.",
                    ),
                    (
                        "style",
                        "struct.Page.html#method.style",
                        "Adds `sheet` to `<head>` as a `<style>` block.",
                    ),
                    (
                        "stylesheet",
                        "struct.Page.html#method.stylesheet",
                        "Adds a `<link rel=\"stylesheet\">` for the file at `href` to `<head>`.",
                    ),
                    (
                        "runtime_src",
                        "struct.Page.html#method.runtime_src",
                        "Loads the runtime from `src` instead of `RUNTIME_PATH`, for a page served as a static file.",
                    ),
                    (
                        "render",
                        "struct.Page.html#method.render",
                        "Renders the document.",
                    ),
                    (
                        "RUNTIME_PATH",
                        "constant.RUNTIME_PATH.html",
                        "The path `Page` loads the runtime from.",
                    ),
                    (
                        "RUNTIME_JS",
                        "constant.RUNTIME_JS.html",
                        "The browser runtime, built from `runtime/` in the repository.",
                    ),
                ],
            }],
        ),
        Reference::El => (
            "`El<S>` is an HTML element on a page whose signals are `S`. Its methods return the element, so a tree is one expression.",
            &[
                Section {
                    heading: "Content and attributes",
                    text: "",
                    rows: &[
                        (
                            "child",
                            "html/struct.El.html#method.child",
                            "Appends a child element, text, or template.",
                        ),
                        (
                            "children",
                            "html/struct.El.html#method.children",
                            "Appends several children. An `Option` counts as zero or one child.",
                        ),
                        (
                            "text",
                            "html/struct.El.html#method.text",
                            "Appends static text.",
                        ),
                        (
                            "attr",
                            "html/struct.El.html#method.attr",
                            "Sets a plain attribute.",
                        ),
                        (
                            "href",
                            "html/struct.El.html#method.href",
                            "Points the element's `href` at a page the router serves.",
                        ),
                        (
                            "id",
                            "html/struct.El.html#method.id",
                            "Sets the element's `id`.",
                        ),
                        (
                            "class",
                            "html/struct.El.html#method.class",
                            "Adds a static class.",
                        ),
                        (
                            "css",
                            "html/struct.El.html#method.css",
                            "Adds the declarations of `style` as atomic classes.",
                        ),
                        (
                            "node_ref",
                            "html/struct.El.html#method.node_ref",
                            "Attaches `node_ref`, so its statements act on this element.",
                        ),
                        (
                            "render",
                            "html/struct.El.html#method.render",
                            "Renders the element and its children to an HTML string.",
                        ),
                    ],
                },
                Section {
                    heading: "Bindings",
                    text: "Each binding takes an expression over the signals and is kept up to date as they change.",
                    rows: &[
                        (
                            "bind_text",
                            "html/struct.El.html#method.bind_text",
                            "Replaces the element's text with `text`, kept up to date as signals change.",
                        ),
                        (
                            "show",
                            "html/struct.El.html#method.show",
                            "Shows the element only while `when` is true.",
                        ),
                        (
                            "bind_attr",
                            "html/struct.El.html#method.bind_attr",
                            "Keeps attribute `name` equal to `value` as signals change.",
                        ),
                        (
                            "attr_when",
                            "html/struct.El.html#method.attr_when",
                            "Sets the boolean attribute `name`, such as `disabled`, while `when` is true.",
                        ),
                        (
                            "class_when",
                            "html/struct.El.html#method.class_when",
                            "Adds `class` while `when` is true.",
                        ),
                        (
                            "css_when",
                            "html/struct.El.html#method.css_when",
                            "Adds the declarations of `style` while `when` is true.",
                        ),
                        (
                            "css_match",
                            "html/struct.El.html#method.css_match",
                            "Adds the style of the case whose value `value` equals.",
                        ),
                        (
                            "bind_style",
                            "html/struct.El.html#method.bind_style",
                            "Keeps the CSS property `property` equal to `value` as signals change.",
                        ),
                        (
                            "bind_token",
                            "html/struct.El.html#method.bind_token",
                            "Keeps `token` equal to `value` on this element and those inside it.",
                        ),
                        (
                            "bind",
                            "html/struct.El.html#method.bind",
                            "Binds a form control to a signal in both directions.",
                        ),
                        (
                            "bind_checked",
                            "html/struct.El.html#method.bind_checked",
                            "Keeps a checkbox's `checked` state equal to `when`, in one direction.",
                        ),
                        (
                            "bind_radio",
                            "html/struct.El.html#method.bind_radio",
                            "Makes this radio button set `signal` to `value` when checked.",
                        ),
                        (
                            "bind_select",
                            "html/struct.El.html#method.bind_select",
                            "Fills this `<select>` with `options` and binds it to `signal`.",
                        ),
                    ],
                },
                Section {
                    heading: "Events",
                    text: "",
                    rows: &[
                        (
                            "on",
                            "html/struct.El.html#method.on",
                            "Runs `stmt` on `event`. A second call for the same event runs after the first.",
                        ),
                        (
                            "on_click",
                            "html/struct.El.html#method.on_click",
                            "Runs `stmt` on click.",
                        ),
                        (
                            "on_double_click",
                            "html/struct.El.html#method.on_double_click",
                            "Runs `stmt` on double click.",
                        ),
                        (
                            "on_input",
                            "html/struct.El.html#method.on_input",
                            "Runs `stmt` on every edit of a text field.",
                        ),
                        (
                            "on_change",
                            "html/struct.El.html#method.on_change",
                            "Runs `stmt` on `change`.",
                        ),
                        (
                            "on_submit",
                            "html/struct.El.html#method.on_submit",
                            "Runs `stmt` when a form is submitted, after `preventDefault`.",
                        ),
                        (
                            "on_blur",
                            "html/struct.El.html#method.on_blur",
                            "Runs `stmt` when focus leaves the element.",
                        ),
                        (
                            "on_key",
                            "html/struct.El.html#method.on_key",
                            "Runs `stmt` when `key` is pressed while focus is in this element.",
                        ),
                        (
                            "on_debounced",
                            "html/struct.El.html#method.on_debounced",
                            "Runs `stmt` once `event` has stopped firing for `quiet`.",
                        ),
                    ],
                },
                Section {
                    heading: "Lifecycle and focus",
                    text: "",
                    rows: &[
                        (
                            "on_load",
                            "html/struct.El.html#method.on_load",
                            "Runs `stmt` once when the element enters the document.",
                        ),
                        (
                            "on_interval",
                            "html/struct.El.html#method.on_interval",
                            "Runs `stmt` every `period` while the element is in the document.",
                        ),
                        (
                            "on_timeout",
                            "html/struct.El.html#method.on_timeout",
                            "Runs `stmt` once, `delay` after the element enters the document, if it is still in it then.",
                        ),
                        (
                            "effect",
                            "html/struct.El.html#method.effect",
                            "Runs `stmt` when the element enters the document and whenever a signal it reads changes.",
                        ),
                        (
                            "on_signal_change",
                            "html/struct.El.html#method.on_signal_change",
                            "Runs `stmt` whenever `signal`, or any field inside it, changes.",
                        ),
                        (
                            "roving_focus",
                            "html/struct.El.html#method.roving_focus",
                            "Moves focus between the items inside this element on the arrow keys, Home, and End.",
                        ),
                        (
                            "active_descendant",
                            "html/struct.El.html#method.active_descendant",
                            "Tracks the active option of `listbox` in `active` on the up and down arrows.",
                        ),
                    ],
                },
            ],
        ),
        Reference::Html => (
            "The `hypersig::html` module builds elements, lists, and conditional parts.",
            &[
                Section {
                    heading: "Functions",
                    text: "Besides these, the module has one function per common tag, such as `div`, `p`, `button`, `input`, and `ul`.",
                    rows: &[
                        ("el", "html/fn.el.html", "Creates an element with any tag."),
                        (
                            "each",
                            "html/fn.each.html",
                            "Renders `render` once per item of `list`, in the browser, kept in step with the list.",
                        ),
                        (
                            "each_keyed",
                            "html/fn.each_keyed.html",
                            "`each`, with each item's element identified by `key`.",
                        ),
                        (
                            "when",
                            "html/fn.when.html",
                            "Puts `el` in the document only while `cond` is true.",
                        ),
                    ],
                },
                Section {
                    heading: "Types",
                    text: "",
                    rows: &[
                        (
                            "El",
                            "html/struct.El.html",
                            "An HTML element belonging to a page whose signals are `S`.",
                        ),
                        (
                            "Node",
                            "html/struct.Node.html",
                            "A node in the tree: an element, a run of text, or a template the browser expands.",
                        ),
                        (
                            "Event",
                            "html/enum.Event.html",
                            "A DOM event a statement can run on.",
                        ),
                        (
                            "Key",
                            "html/enum.Key.html",
                            "A key that `El::on_key` filters `keydown` by.",
                        ),
                        (
                            "Orientation",
                            "html/enum.Orientation.html",
                            "A horizontal or vertical direction, which `El::roving_focus` reads as its arrow keys.",
                        ),
                        (
                            "Stylesheet",
                            "html/struct.Stylesheet.html",
                            "CSS text whose class selectors `Stylesheet::class` checks at compile time.",
                        ),
                        (
                            "Class",
                            "html/struct.Class.html",
                            "A class name that `El::class` and `El::class_when` put on an element.",
                        ),
                    ],
                },
            ],
        ),
        Reference::Signals => (
            "`Signals` marks the page's state struct, and `Sig<S, T>` is a typed reference to one of its fields.",
            &[
                Section {
                    heading: "Types",
                    text: "",
                    rows: &[
                        (
                            "Signals",
                            "trait.Signals.html",
                            "A struct whose fields are signals.",
                        ),
                        (
                            "Sig",
                            "struct.Sig.html",
                            "A typed reference to a signal of type `T` inside the signal struct `S`.",
                        ),
                        (
                            "Item",
                            "struct.Item.html",
                            "The current item of a list, inside `each` or a list predicate such as `Sig::count`.",
                        ),
                        (
                            "Root",
                            "enum.Root.html",
                            "Marks a `Sig` rooted at the page's signal struct.",
                        ),
                        (
                            "Local",
                            "enum.Local.html",
                            "Marks a `Sig` rooted at the current item of an `each` or a list predicate.",
                        ),
                        (
                            "MAX_DEPTH",
                            "constant.MAX_DEPTH.html",
                            "The deepest chain of fields a `Sig` can name.",
                        ),
                    ],
                },
                Section {
                    heading: "Sig",
                    text: "A `Sig` is also an expression, so the methods of `Expr` apply to it.",
                    rows: &[
                        (
                            "at",
                            "struct.Sig.html#method.at",
                            "The field `field` of the struct this signal holds.",
                        ),
                        (
                            "set",
                            "struct.Sig.html#method.set",
                            "A statement that assigns `value` to this signal in the browser.",
                        ),
                        (
                            "toggle",
                            "struct.Sig.html#method.toggle",
                            "A statement that flips this flag.",
                        ),
                        (
                            "patch",
                            "struct.Sig.html#method.patch",
                            "A patch that sets this signal to `value` when the server sends it.",
                        ),
                        (
                            "path",
                            "struct.Sig.html#method.path",
                            "The dotted field path this signal is stored under.",
                        ),
                    ],
                },
                Section {
                    heading: "Lists",
                    text: "These exist on a `Sig` whose value is a `Vec`.",
                    rows: &[
                        (
                            "push",
                            "struct.Sig.html#method.push",
                            "A statement that appends `value` to this list.",
                        ),
                        (
                            "extend",
                            "struct.Sig.html#method.extend",
                            "A statement that appends `count` items, building each from its position `i`.",
                        ),
                        (
                            "remove_at",
                            "struct.Sig.html#method.remove_at",
                            "A statement that removes the item at `index`, if there is one.",
                        ),
                        (
                            "clear",
                            "struct.Sig.html#method.clear",
                            "A statement that empties this list.",
                        ),
                        (
                            "retain",
                            "struct.Sig.html#method.retain",
                            "A statement that keeps only the items for which `keep` is true.",
                        ),
                        (
                            "set_all",
                            "struct.Sig.html#method.set_all",
                            "A statement that sets `field` to `value` on every item.",
                        ),
                        (
                            "update_where",
                            "struct.Sig.html#method.update_where",
                            "A statement that sets `field` to `value` on the items for which `when` is true.",
                        ),
                        (
                            "swap",
                            "struct.Sig.html#method.swap",
                            "A statement that exchanges the items at `a` and `b`, if both exist.",
                        ),
                        (
                            "sort_by",
                            "struct.Sig.html#method.sort_by",
                            "A statement that orders the items by `key`, ascending, keeping the order of equal keys.",
                        ),
                        (
                            "count",
                            "struct.Sig.html#method.count",
                            "The number of items for which `matches` is true.",
                        ),
                    ],
                },
                Section {
                    heading: "Item",
                    text: "",
                    rows: &[
                        (
                            "at",
                            "struct.Item.html#method.at",
                            "The field `field` of the item.",
                        ),
                        (
                            "value",
                            "struct.Item.html#method.value",
                            "The item itself, as a signal that can be read and written.",
                        ),
                        (
                            "index",
                            "struct.Item.html#method.index",
                            "The item's position in its list, starting at 0.",
                        ),
                        (
                            "remove",
                            "struct.Item.html#method.remove",
                            "A statement that removes this item from its list.",
                        ),
                    ],
                },
            ],
        ),
        Reference::Expr => (
            "`Expr<S, T>` is an expression over the signals of `S` that evaluates to a `T` in the browser.",
            &[
                Section {
                    heading: "Building expressions",
                    text: "`+`, `-`, `*`, and `%` work on `i32` and `f64`, `/` on `f64`, and `!` on `bool`.",
                    rows: &[
                        (
                            "Expr::lit",
                            "struct.Expr.html#method.lit",
                            "A constant value, serialized to JSON once at render time.",
                        ),
                        (
                            "if_else",
                            "fn.if_else.html",
                            "The expression `then` while `cond` is true, and `otherwise` while it is false.",
                        ),
                        (
                            "Expr::now",
                            "struct.Expr.html#method.now",
                            "The browser's current time, in milliseconds since the Unix epoch.",
                        ),
                        (
                            "Expr::random_below",
                            "struct.Expr.html#method.random_below",
                            "A random integer from 0 up to but not including `bound`, drawn in the browser.",
                        ),
                        (
                            "at",
                            "struct.Expr.html#method.at",
                            "The field `field` of the struct this expression evaluates to.",
                        ),
                        (
                            "with",
                            "struct.Expr.html#method.with",
                            "A copy of this struct with `field` replaced by `value`.",
                        ),
                    ],
                },
                Section {
                    heading: "Comparisons and logic",
                    text: "",
                    rows: &[
                        (
                            "equals",
                            "struct.Expr.html#method.equals",
                            "Whether this value equals `rhs`.",
                        ),
                        (
                            "not_equals",
                            "struct.Expr.html#method.not_equals",
                            "Whether this value differs from `rhs`, comparing JSON forms.",
                        ),
                        (
                            "gt",
                            "struct.Expr.html#method.gt",
                            "Whether this value is strictly greater than `rhs`.",
                        ),
                        (
                            "ge",
                            "struct.Expr.html#method.ge",
                            "Whether this value is greater than or equal to `rhs`.",
                        ),
                        (
                            "lt",
                            "struct.Expr.html#method.lt",
                            "Whether this value is strictly less than `rhs`.",
                        ),
                        (
                            "le",
                            "struct.Expr.html#method.le",
                            "Whether this value is less than or equal to `rhs`.",
                        ),
                        (
                            "and",
                            "struct.Expr.html#method.and",
                            "Whether both this value and `rhs` are true.",
                        ),
                        (
                            "or",
                            "struct.Expr.html#method.or",
                            "Whether this value or `rhs` is true.",
                        ),
                    ],
                },
                Section {
                    heading: "Numbers and text",
                    text: "",
                    rows: &[
                        (
                            "min",
                            "struct.Expr.html#method.min",
                            "The smaller of this value and `rhs`.",
                        ),
                        (
                            "max",
                            "struct.Expr.html#method.max",
                            "The larger of this value and `rhs`.",
                        ),
                        (
                            "to_f64",
                            "struct.Expr.html#method.to_f64",
                            "This integer as an `f64`, which holds every `i32` exactly.",
                        ),
                        (
                            "to_fixed",
                            "struct.Expr.html#method.to_fixed",
                            "This number as text with `digits` digits after the point.",
                        ),
                        (
                            "to_text",
                            "struct.Expr.html#method.to_text",
                            "This value as display text.",
                        ),
                        (
                            "concat",
                            "struct.Expr.html#method.concat",
                            "Appends `rhs` to this string.",
                        ),
                        (
                            "trim",
                            "struct.Expr.html#method.trim",
                            "This string without leading and trailing whitespace.",
                        ),
                        (
                            "to_lowercase",
                            "struct.Expr.html#method.to_lowercase",
                            "This string in lowercase.",
                        ),
                        (
                            "to_uppercase",
                            "struct.Expr.html#method.to_uppercase",
                            "This string in uppercase.",
                        ),
                        (
                            "contains",
                            "struct.Expr.html#method.contains",
                            "Whether `needle` occurs in this string.",
                        ),
                        (
                            "starts_with",
                            "struct.Expr.html#method.starts_with",
                            "Whether this string starts with `prefix`.",
                        ),
                        (
                            "char_count",
                            "struct.Expr.html#method.char_count",
                            "The number of Unicode scalar values, as Rust's `chars().count()`.",
                        ),
                    ],
                },
                Section {
                    heading: "Lists and options",
                    text: "",
                    rows: &[
                        ("len", "struct.Expr.html#method.len", "The number of items."),
                        (
                            "nth",
                            "struct.Expr.html#method.nth",
                            "The item at `index`, or `None` past either end.",
                        ),
                        (
                            "is_some",
                            "struct.Expr.html#method.is_some",
                            "Whether this option holds a value.",
                        ),
                        (
                            "is_none",
                            "struct.Expr.html#method.is_none",
                            "Whether this option is `None`.",
                        ),
                        (
                            "unwrap_or",
                            "struct.Expr.html#method.unwrap_or",
                            "The held value, or `default` for `None`.",
                        ),
                        (
                            "some",
                            "struct.Expr.html#method.some",
                            "This value wrapped in `Some`.",
                        ),
                    ],
                },
            ],
        ),
        Reference::Stmt => (
            "`Stmt<S>` is a statement over the signals of `S` that the browser runs on an event.",
            &[Section {
                heading: "Stmt",
                text: "`Sig::set`, `Sig::toggle`, the list statements, `NodeRef`'s statements, and an action's `Call` all produce statements.",
                rows: &[
                    (
                        "then",
                        "struct.Stmt.html#method.then",
                        "Runs `self`, then `next`.",
                    ),
                    (
                        "Stmt::when",
                        "struct.Stmt.html#method.when",
                        "Runs `then` only while `cond` is true.",
                    ),
                    (
                        "Stmt::if_else",
                        "struct.Stmt.html#method.if_else",
                        "Runs `then` while `cond` is true and `otherwise` while it is false.",
                    ),
                    (
                        "Stmt::none",
                        "struct.Stmt.html#method.none",
                        "A statement that does nothing.",
                    ),
                    (
                        "IntoStmt",
                        "trait.IntoStmt.html",
                        "Converts a value into a statement, as `Call` does.",
                    ),
                ],
            }],
        ),
        Reference::NodeRef => (
            "`NodeRef<S>` names an element on a page whose signals are `S`, for imperative DOM calls.",
            &[Section {
                heading: "NodeRef",
                text: "Attach one to an element with `El::node_ref`.",
                rows: &[
                    (
                        "NodeRef::new",
                        "struct.NodeRef.html#method.new",
                        "A reference named `name`.",
                    ),
                    (
                        "focus",
                        "struct.NodeRef.html#method.focus",
                        "A statement that focuses the element after the next render.",
                    ),
                    (
                        "blur",
                        "struct.NodeRef.html#method.blur",
                        "A statement that removes focus from the element.",
                    ),
                    (
                        "select",
                        "struct.NodeRef.html#method.select",
                        "A statement that selects the element's text, for an `<input>`.",
                    ),
                    (
                        "scroll_into_view",
                        "struct.NodeRef.html#method.scroll_into_view",
                        "A statement that scrolls the element into view.",
                    ),
                    (
                        "show_popover",
                        "struct.NodeRef.html#method.show_popover",
                        "A statement that shows the element if it is a popover that is not open.",
                    ),
                    (
                        "hide_popover",
                        "struct.NodeRef.html#method.hide_popover",
                        "A statement that hides the element if it is an open popover.",
                    ),
                ],
            }],
        ),
        Reference::Action => (
            "`Action<S, A>` is a server endpoint on the page whose signals are `S`, which receives arguments `A`.",
            &[
                Section {
                    heading: "Action",
                    text: "Declare one as a `const` and use the same value on the page and in the router.",
                    rows: &[
                        (
                            "Action::post",
                            "struct.Action.html#method.post",
                            "An action served on `POST path`.",
                        ),
                        (
                            "Action::get",
                            "struct.Action.html#method.get",
                            "An action served on `GET path`. Use it for a long-lived stream of patches.",
                        ),
                        (
                            "call",
                            "struct.Action.html#method.call",
                            "A call to this action, which takes no arguments.",
                        ),
                        (
                            "call_with",
                            "struct.Action.html#method.call_with",
                            "A call to this action with `args`.",
                        ),
                        (
                            "path",
                            "struct.Action.html#method.path",
                            "The path the action is served on.",
                        ),
                        (
                            "method",
                            "struct.Action.html#method.method",
                            "The method the action is served on.",
                        ),
                        (
                            "Method",
                            "enum.Method.html",
                            "The HTTP method an `Action` is served on.",
                        ),
                    ],
                },
                Section {
                    heading: "Call",
                    text: "A `Call` is usable wherever a `Stmt` is.",
                    rows: &[
                        (
                            "indicator",
                            "struct.Call.html#method.indicator",
                            "Sets `flag` to true while the request is in flight and to false after.",
                        ),
                        (
                            "error",
                            "struct.Call.html#method.error",
                            "Sets `message` to the error text when the call fails, and to `\"\"` when it succeeds.",
                        ),
                        (
                            "then",
                            "struct.Call.html#method.then",
                            "Sends this call, then runs `next` without waiting for the answer.",
                        ),
                    ],
                },
            ],
        ),
        Reference::Patches => (
            "`Patches<S>` is what a handler answers with: new signal values and re-rendered fragments, applied in order.",
            &[
                Section {
                    heading: "Patches",
                    text: "",
                    rows: &[
                        (
                            "Patches::new",
                            "struct.Patches.html#method.new",
                            "An empty batch.",
                        ),
                        (
                            "signals",
                            "struct.Patches.html#method.signals",
                            "Sets signals in the browser.",
                        ),
                        (
                            "all_signals",
                            "struct.Patches.html#method.all_signals",
                            "Replaces every signal with the values in `signals`.",
                        ),
                        (
                            "element",
                            "struct.Patches.html#method.element",
                            "Re-renders `fragment` and morphs it into the element with its id.",
                        ),
                        (
                            "navigate",
                            "struct.Patches.html#method.navigate",
                            "Loads the page at `url` once the patches before it are applied.",
                        ),
                        (
                            "to_event_stream",
                            "struct.Patches.html#method.to_event_stream",
                            "The batch as a Server-Sent Events body, for use without the axum feature.",
                        ),
                        (
                            "SignalPatch",
                            "struct.SignalPatch.html",
                            "New values for one or more signals of `S`.",
                        ),
                        (
                            "SignalPatch::and",
                            "struct.SignalPatch.html#method.and",
                            "Adds the values in `other`. A signal present in both takes `other`'s value.",
                        ),
                    ],
                },
                Section {
                    heading: "Fragment",
                    text: "",
                    rows: &[
                        (
                            "Fragment",
                            "trait.Fragment.html",
                            "A part of the page the server can re-render and replace on its own.",
                        ),
                        (
                            "Fragment::ID",
                            "trait.Fragment.html#associatedconstant.ID",
                            "The element id of the fragment's root.",
                        ),
                        (
                            "Fragment::render",
                            "trait.Fragment.html#tymethod.render",
                            "Renders the fragment's root element.",
                        ),
                        (
                            "fragment",
                            "fn.fragment.html",
                            "Renders `fragment` with its root id set to `Fragment::ID`.",
                        ),
                    ],
                },
            ],
        ),
        Reference::Route => (
            "`Route<P, Q>` is a page served at a path whose parameters are `P`, with search parameters `Q`, and `Link<S>` points to one.",
            &[
                Section {
                    heading: "Route",
                    text: "Declare one as a `const` and use the same value in the router and in the links to the page.",
                    rows: &[
                        (
                            "Route::new",
                            "struct.Route.html#method.new",
                            "A page served at `path`, whose `{name}` segments take the values of `P`.",
                        ),
                        (
                            "child",
                            "struct.Route.html#method.child",
                            "A page under this one, served at the whole path `path`.",
                        ),
                        (
                            "link",
                            "struct.Route.html#method.link",
                            "A link to this page, which has no parameters.",
                        ),
                        (
                            "link_with",
                            "struct.Route.html#method.link_with",
                            "A link to this page with the arguments `args`.",
                        ),
                        (
                            "url",
                            "struct.Route.html#method.url",
                            "The URL of this page with the arguments `args`, written on the server.",
                        ),
                        (
                            "path",
                            "struct.Route.html#method.path",
                            "The path as axum matches it, with each parameter still written `{name}`.",
                        ),
                    ],
                },
                Section {
                    heading: "Parameters and links",
                    text: "",
                    rows: &[
                        (
                            "Params",
                            "trait.Params.html",
                            "The parameters of a `Route`: `()`, one `Segment`, or a tuple of two or three.",
                        ),
                        (
                            "Segment",
                            "trait.Segment.html",
                            "A type one `{name}` segment of a `Route` holds.",
                        ),
                        (
                            "Search",
                            "trait.Search.html",
                            "The search parameters of a `Route`: `()`, or a struct with `#[derive(Signals)]`.",
                        ),
                        (
                            "Extends",
                            "trait.Extends.html",
                            "Parameters that start with the parameters `P` of a parent `Route`.",
                        ),
                        (
                            "Link",
                            "struct.Link.html",
                            "A link to a `Route`, for `El::href` on the page whose signals are `S`.",
                        ),
                        (
                            "Link::navigate",
                            "struct.Link.html#method.navigate",
                            "A statement that loads the linked page, as following the link would.",
                        ),
                        (
                            "Url",
                            "struct.Url.html",
                            "The URL of a page, written on the server by `Route::url`.",
                        ),
                    ],
                },
            ],
        ),
        Reference::RouterExt => (
            "`RouterExt` adds hypersig routes to an axum `Router`. It needs the `axum` feature, which is on by default.",
            &[Section {
                heading: "RouterExt",
                text: "",
                rows: &[
                    (
                        "hypersig_runtime",
                        "trait.RouterExt.html#tymethod.hypersig_runtime",
                        "Serves the browser runtime at `RUNTIME_PATH`.",
                    ),
                    (
                        "page",
                        "trait.RouterExt.html#tymethod.page",
                        "Serves the page at `route` on `GET`, with a handler that builds it.",
                    ),
                    (
                        "action",
                        "trait.RouterExt.html#tymethod.action",
                        "Serves `action` with a handler that answers with one batch of patches.",
                    ),
                    (
                        "stream",
                        "trait.RouterExt.html#tymethod.stream",
                        "Serves `action` with a handler that answers with a stream of patches.",
                    ),
                    (
                        "PageResponse",
                        "struct.PageResponse.html",
                        "What a page handler answers with: a page, a redirect, or a 404.",
                    ),
                    (
                        "RequestError",
                        "struct.RequestError.html",
                        "A request to an action whose arguments did not deserialize.",
                    ),
                ],
            }],
        ),
    }
}
