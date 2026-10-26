//! Checks the public API of the library crates against the rules of
//! `.claude/rules/rust.md` that rustdoc's JSON output can decide.
//!
//! The JSON comes from a nightly rustdoc, so the check is ignored by default.
//! Write the JSON of each crate in `CRATES` with
//!
//! ```text
//! cargo +nightly-2026-10-14 rustdoc -p <crate> --all-features -- -Z unstable-options --output-format json
//! ```
//!
//! then run `cargo test -p hypersig --test public_api -- --ignored`.
//!
//! Two rules are checked against a list a person keeps next to this file:
//! `public_api_strings.txt` holds the public functions that take a string
//! (M-STRONG-TYPES), and `public_api_summaries.txt` the items whose first doc
//! sentence has 15 words or more (M-FIRST-DOC-SENTENCE). An item missing
//! from its list fails the check, and so does a line naming an item that no
//! longer needs it. `HYPERSIG_PUBLIC_API=overwrite` writes both lists from
//! the current API.

use std::collections::{BTreeMap, BTreeSet, HashSet};
use std::fmt::Write as _;
use std::path::PathBuf;

use serde_json::Value;

const CRATES: [&str; 4] = ["hypersig", "hypersig_ui", "hypersig_style", "hypersig_i18n"];

/// The rustdoc JSON format the walk below reads, which nightly-2026-10-14
/// writes.
const FORMAT_VERSION: u64 = 61;

/// The public types with a third type parameter, each justified in
/// `.claude/rules/rust.md` under M-SIMPLE-ABSTRACTIONS.
const THREE_TYPE_PARAMETERS: [&str; 2] = [
    // `R` records whether the path starts at the page or at a list item, and
    // whether the field is computed.
    "hypersig::Sig",
    // `R` is the `R` of the `Sig` that holds the chart's rows, so that a
    // chart draws a computed list the way it draws a field of the page.
    "hypersig_ui::Chart",
];

/// The crates of the standard library.
const STANDARD: [&str; 4] = ["std", "core", "alloc", "proc_macro"];

/// The crates of this workspace, whose items are not third-party.
const WORKSPACE: [&str; 5] = [
    "hypersig",
    "hypersig_ui",
    "hypersig_style",
    "hypersig_i18n",
    "hypersig_proc",
];

/// The third-party items `.claude/rules/rust.md` allows in the public API
/// under M-DONT-LEAK-TYPES, as the crate that defines each and its path
/// there. `serde_core::*` allows every trait of that crate, which is where
/// `serde` defines its traits.
const THIRD_PARTY: [&str; 14] = [
    "serde_core::*",
    "futures_core::stream::Stream",
    "axum::routing::Router",
    "axum_core::extract::FromRequestParts",
    "axum_core::response::into_response::IntoResponse",
    "axum_core::response::into_response_parts::IntoResponseParts",
    // The type `IntoResponseParts::into_response_parts` takes and returns.
    "axum_core::response::into_response_parts::ResponseParts",
    "http::request::Parts",
    "http::response::Response",
    // axum's name for `http::response::Response<Body>`.
    "axum_core::response::Response",
    "http::status::StatusCode",
    "bytes::bytes::Bytes",
    "http::request::Request",
    "axum_core::body::Body",
];

/// The most words a first doc sentence may have under M-FIRST-DOC-SENTENCE,
/// which asks for fewer than 15.
const SUMMARY_WORDS: usize = 14;

const STRINGS_FILE: &str = "tests/public_api_strings.txt";
const STRINGS_HEADER: &str = "\
# Public functions with a parameter whose type is or holds `str` or `String`,
# as `path(parameter)`. A function missing here fails tests/public_api.rs until
# a person decides that the string is not a signal path, an element id, a URL,
# an expression, or CSS (M-STRONG-TYPES), and adds the line.
";

const SUMMARIES_FILE: &str = "tests/public_api_summaries.txt";
const SUMMARIES_HEADER: &str = "\
# Public items whose first doc sentence has 15 words or more, which
# M-FIRST-DOC-SENTENCE does not allow. Shorten a summary and delete its line.
# A new item missing here fails tests/public_api.rs.
";

struct Krate {
    name: &'static str,
    json: Value,
}

impl Krate {
    fn load(name: &'static str) -> Self {
        let dir = std::env::var_os("CARGO_TARGET_DIR").map_or_else(
            || PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../target"),
            PathBuf::from,
        );
        let path = dir.join("doc").join(format!("{name}.json"));
        let text = std::fs::read(&path).unwrap_or_else(|error| {
            panic!(
                "{} cannot be read ({error}). Write it with\n  cargo +nightly-2026-10-14 rustdoc -p {name} --all-features -- -Z unstable-options --output-format json",
                path.display()
            )
        });
        let json: Value = serde_json::from_slice(&text).expect("rustdoc writes JSON");
        assert_eq!(
            json["format_version"].as_u64(),
            Some(FORMAT_VERSION),
            "{name}.json is in a format this test does not read; write it again with nightly-2026-10-14"
        );
        Self { name, json }
    }

    fn item(&self, id: &Value) -> Option<&Value> {
        self.json["index"].get(id.as_u64()?.to_string())
    }

    /// The crate that defines the item `id` names, and its path there.
    fn origin(&self, id: &Value) -> Option<(&str, String)> {
        let summary = self.json["paths"].get(id.as_u64()?.to_string())?;
        let krate = summary["crate_id"].as_u64()?;
        let krate = if krate == 0 {
            self.name
        } else {
            self.json["external_crates"][krate.to_string()]["name"].as_str()?
        };
        let path: Vec<&str> = array(&summary["path"])
            .iter()
            .filter_map(Value::as_str)
            .collect();
        Some((krate, path.join("::")))
    }

    fn is_trait(&self, id: &Value) -> bool {
        id.as_u64()
            .is_some_and(|id| self.json["paths"][id.to_string()]["kind"] == "trait")
    }
}

fn kind(item: &Value) -> &str {
    item["inner"]
        .as_object()
        .and_then(|inner| inner.keys().next())
        .map_or("", String::as_str)
}

fn array(value: &Value) -> &[Value] {
    value.as_array().map_or(&[], Vec::as_slice)
}

/// The public API of one crate.
struct Api<'a> {
    krate: &'a Krate,
    /// Each item a module holds or re-exports, with every path that names it.
    /// rustdoc leaves `#[doc(hidden)]` items out, so `_private` is not here.
    paths: BTreeMap<u64, BTreeSet<String>>,
    /// Each item a caller sees, with the path it is reported at: the items
    /// above, and their fields, variants, impls, and associated items.
    items: Vec<(String, &'a Value)>,
    /// The associated items of trait impls, whose signatures and docs the
    /// trait decides.
    in_trait_impl: HashSet<u64>,
}

impl<'a> Api<'a> {
    fn new(krate: &'a Krate) -> Self {
        let mut api = Self {
            krate,
            paths: BTreeMap::new(),
            items: Vec::new(),
            in_trait_impl: HashSet::new(),
        };
        api.module(&krate.json["root"], krate.name, &mut HashSet::new());
        let named: Vec<(u64, String)> = api
            .paths
            .iter()
            .filter_map(|(id, paths)| Some((*id, paths.first()?.clone())))
            .collect();
        let mut seen = HashSet::new();
        for (id, path) in named {
            if let Some(item) = krate.item(&Value::from(id)) {
                api.children(&path, item, &mut seen);
            }
        }
        api
    }

    /// Records the items of the module `id` under `path`, following
    /// re-exports. `open` holds the modules being walked, so a module that
    /// re-exports its parent ends the walk.
    fn module(&mut self, id: &Value, path: &str, open: &mut HashSet<u64>) {
        let Some(number) = id.as_u64() else {
            return;
        };
        if !open.insert(number) {
            return;
        }
        let items = self
            .krate
            .item(id)
            .map_or(&[][..], |module| array(&module["inner"]["module"]["items"]));
        for child_id in items {
            let Some(child) = self.krate.item(child_id) else {
                continue;
            };
            if kind(child) == "use" {
                let reexport = &child["inner"]["use"];
                if reexport["is_glob"] == true {
                    self.glob(&reexport["id"], path, open);
                } else if let Some(name) = reexport["name"].as_str() {
                    self.name(&reexport["id"], &format!("{path}::{name}"), open);
                }
            } else if let Some(name) = child["name"].as_str() {
                self.name(child_id, &format!("{path}::{name}"), open);
            }
        }
        open.remove(&number);
    }

    /// Records what a glob re-export brings in under `path`: the items of a
    /// module, or the variants of an enum.
    fn glob(&mut self, target: &Value, path: &str, open: &mut HashSet<u64>) {
        let Some(item) = self.krate.item(target) else {
            return;
        };
        if kind(item) == "module" {
            self.module(target, path, open);
        } else {
            for variant in array(&item["inner"]["enum"]["variants"]) {
                if let Some(name) = self.krate.item(variant).and_then(|v| v["name"].as_str()) {
                    self.name(variant, &format!("{path}::{name}"), open);
                }
            }
        }
    }

    fn name(&mut self, id: &Value, path: &str, open: &mut HashSet<u64>) {
        let Some(number) = id.as_u64() else {
            return;
        };
        self.paths
            .entry(number)
            .or_default()
            .insert(path.to_owned());
        if self
            .krate
            .item(id)
            .is_some_and(|item| kind(item) == "module")
        {
            self.module(id, path, open);
        }
    }

    /// Records `item` and what hangs off it.
    fn children(&mut self, path: &str, item: &'a Value, seen: &mut HashSet<u64>) {
        if !item["id"].as_u64().is_some_and(|id| seen.insert(id)) {
            return;
        }
        self.items.push((path.to_owned(), item));
        let inner = &item["inner"][kind(item)];
        let mut ids: Vec<&Value> = Vec::new();
        match kind(item) {
            "struct" => {
                ids.extend(array(&inner["kind"]["plain"]["fields"]));
                ids.extend(array(&inner["kind"]["tuple"]));
                ids.extend(array(&inner["impls"]));
            }
            "union" => {
                ids.extend(array(&inner["fields"]));
                ids.extend(array(&inner["impls"]));
            }
            "enum" => {
                ids.extend(array(&inner["variants"]));
                ids.extend(array(&inner["impls"]));
            }
            "variant" => {
                ids.extend(array(&inner["kind"]["struct"]["fields"]));
                ids.extend(array(&inner["kind"]["tuple"]));
            }
            "trait" => {
                ids.extend(array(&inner["items"]));
                ids.extend(array(&inner["implementations"]));
            }
            "impl" => {
                if !inner["trait"].is_null() {
                    self.in_trait_impl
                        .extend(array(&inner["items"]).iter().filter_map(Value::as_u64));
                }
                ids.extend(array(&inner["items"]));
            }
            _ => {}
        }
        for id in ids {
            let Some(child) = self.krate.item(id) else {
                continue;
            };
            let imp = &child["inner"]["impl"];
            // An auto trait impl or a blanket impl belongs to its trait.
            if imp["is_synthetic"] == true || !imp["blanket_impl"].is_null() {
                continue;
            }
            let child_path = match child["name"].as_str() {
                Some(name) => format!("{path}::{name}"),
                None => path.to_owned(),
            };
            self.children(&child_path, child, seen);
        }
    }
}

/// The parts of an item a caller writes against: a signature, a field's or
/// a constant's type, an associated type, or an impl's trait and self type.
/// The methods of a trait impl are left out, because the trait fixes them.
fn signature(item: &Value, in_trait_impl: bool) -> Vec<&Value> {
    let inner = &item["inner"][kind(item)];
    match kind(item) {
        "function" | "assoc_const" if in_trait_impl => Vec::new(),
        "function" => vec![&inner["sig"], &inner["generics"]],
        "struct_field" | "assoc_type" | "type_alias" => vec![inner],
        "constant" | "static" | "assoc_const" => vec![&inner["type"]],
        "trait" => vec![&inner["generics"], &inner["bounds"]],
        "struct" | "enum" | "union" => vec![&inner["generics"]],
        "impl" => vec![&inner["trait"], &inner["for"], &inner["generics"]],
        _ => Vec::new(),
    }
}

/// The ids of the items the types and bounds in `value` name, read from
/// rustdoc's `Path` objects, which carry a `path` and an `id`.
fn named<'v>(value: &'v Value, out: &mut Vec<&'v Value>) {
    match value {
        Value::Object(map) => {
            if map.contains_key("path") && map.get("id").is_some_and(Value::is_u64) {
                out.push(&map["id"]);
            }
            for child in map.values() {
                named(child, out);
            }
        }
        Value::Array(values) => {
            for child in values {
                named(child, out);
            }
        }
        _ => {}
    }
}

/// The first sentence of a doc comment: its first paragraph up to a `.`,
/// `?`, or `!` that ends a word outside a code span.
fn first_sentence(docs: &str) -> String {
    let paragraph = docs
        .lines()
        .map(str::trim)
        .take_while(|line| !line.is_empty())
        .collect::<Vec<_>>()
        .join(" ");
    let mut in_code = false;
    for (at, c) in paragraph.char_indices() {
        match c {
            '`' => in_code = !in_code,
            '.' | '?' | '!'
                if !in_code
                    && paragraph[at + 1..]
                        .chars()
                        .next()
                        .is_none_or(|next| next == ' ') =>
            {
                return paragraph[..=at].to_owned();
            }
            _ => {}
        }
    }
    paragraph
}

/// The words of a sentence, where a code span counts as one word and a link
/// as the words of its text.
fn words(sentence: &str) -> usize {
    let mut text = String::new();
    let mut in_code = false;
    let mut target_depth = 0usize;
    let mut previous = ' ';
    for c in sentence.chars() {
        if target_depth > 0 {
            match c {
                '(' => target_depth += 1,
                ')' => target_depth -= 1,
                _ => {}
            }
        } else if c == '`' {
            if !in_code {
                text.push('x');
            }
            in_code = !in_code;
        } else if in_code || c == '[' || c == ']' {
        } else if c == '(' && previous == ']' {
            target_depth = 1;
        } else {
            text.push(c);
        }
        previous = c;
    }
    text.split_whitespace()
        .filter(|word| word.chars().any(char::is_alphanumeric))
        .count()
}

/// Whether a type is or holds `str` or `String`. Only the standard library's
/// types and traits are looked into (`Option<&str>`, `Cow<str>`,
/// `impl Into<String>`), because a `String` inside one of this workspace's
/// types, such as `Expr<S, String>`, is already a typed value.
fn is_text(krate: &Krate, value: &Value) -> bool {
    match value {
        Value::Object(map) => {
            if map.get("primitive").and_then(Value::as_str) == Some("str") {
                return true;
            }
            if let (true, Some(id)) = (map.contains_key("path"), map.get("id")) {
                return match krate.origin(id) {
                    Some((_, path)) if path == "alloc::string::String" => true,
                    Some((owner, _)) if STANDARD.contains(&owner) => {
                        map.get("args").is_some_and(|args| is_text(krate, args))
                    }
                    _ => false,
                };
            }
            map.values().any(|child| is_text(krate, child))
        }
        Value::Array(values) => values.iter().any(|child| is_text(krate, child)),
        _ => false,
    }
}

/// The bounds `generics` puts on the type parameter `name`.
fn bounds<'v>(generics: &'v Value, name: &str) -> Vec<&'v Value> {
    let params = array(&generics["params"])
        .iter()
        .filter(|param| param["name"] == name)
        .map(|param| &param["kind"]);
    let predicates = array(&generics["where_predicates"])
        .iter()
        .map(|predicate| &predicate["bound_predicate"])
        .filter(|bound| bound["type"]["generic"] == name)
        .map(|bound| &bound["bounds"]);
    params.chain(predicates).collect()
}

fn third_party_allowed(krate: &Krate, id: &Value, owner: &str, path: &str) -> bool {
    THIRD_PARTY
        .iter()
        .any(|allowed| match allowed.strip_suffix("::*") {
            Some(allowed) => owner == allowed && krate.is_trait(id),
            None => *allowed == path,
        })
}

/// What the two lists next to this file hold for the current API.
#[derive(Default)]
struct Listed {
    strings: BTreeSet<String>,
    summaries: BTreeSet<String>,
}

fn check(api: &Api<'_>, failures: &mut String, listed: &mut Listed) {
    let krate = api.krate;

    for paths in api.paths.values().filter(|paths| paths.len() > 1) {
        let paths: Vec<&str> = paths.iter().map(String::as_str).collect();
        let paths = paths.join(", ");
        writeln!(
            failures,
            "M-SINGLE-ITEM-PATH: one item has the paths {paths}"
        )
        .unwrap();
    }

    for (path, item) in &api.items {
        let in_trait_impl = item["id"]
            .as_u64()
            .is_some_and(|id| api.in_trait_impl.contains(&id));

        if matches!(kind(item), "struct" | "enum" | "union" | "type_alias") {
            let types = array(&item["inner"][kind(item)]["generics"]["params"])
                .iter()
                .filter(|param| param["kind"].get("type").is_some())
                .count();
            let allowed = if THREE_TYPE_PARAMETERS.contains(&path.as_str()) {
                3
            } else {
                2
            };
            if types > allowed {
                writeln!(
                    failures,
                    "M-SIMPLE-ABSTRACTIONS: {path} has {types} type parameters"
                )
                .unwrap();
            }
        }

        let mut referenced = Vec::new();
        for part in signature(item, in_trait_impl) {
            named(part, &mut referenced);
        }
        let mut reported = BTreeSet::new();
        for id in referenced {
            let Some((owner, target)) = krate.origin(id) else {
                continue;
            };
            let wrapper = matches!(
                target.as_str(),
                "alloc::sync::Arc" | "alloc::boxed::Box" | "alloc::rc::Rc" | "core::cell::RefCell"
            );
            let leaked = !STANDARD.contains(&owner)
                && !WORKSPACE.contains(&owner)
                && !third_party_allowed(krate, id, owner, &target);
            if wrapper && reported.insert(target.clone()) {
                writeln!(failures, "M-AVOID-WRAPPERS: {path} names {target}").unwrap();
            }
            if leaked && reported.insert(target.clone()) {
                writeln!(failures, "M-DONT-LEAK-TYPES: {path} names {target}").unwrap();
            }
        }

        // The docs of an impl block and of a trait impl's items are not the
        // summary rustdoc shows for an item.
        if !in_trait_impl
            && kind(item) != "impl"
            && let Some(docs) = item["docs"].as_str()
            && words(&first_sentence(docs)) > SUMMARY_WORDS
        {
            listed.summaries.insert(path.clone());
        }

        if kind(item) == "function" && !in_trait_impl {
            let function = &item["inner"]["function"];
            for input in array(&function["sig"]["inputs"]) {
                let (Some(name), ty) = (input[0].as_str(), &input[1]) else {
                    continue;
                };
                let bound_text = ty["generic"].as_str().is_some_and(|generic| {
                    bounds(&function["generics"], generic)
                        .into_iter()
                        .any(|bound| is_text(krate, bound))
                });
                if is_text(krate, ty) || bound_text {
                    listed.strings.insert(format!("{path}({name})"));
                }
            }
        }
    }
}

/// Compares a list next to this file with what the current API needs, or
/// writes it under `HYPERSIG_PUBLIC_API=overwrite`.
fn compare(
    file: &str,
    header: &str,
    rule: &str,
    current: &BTreeSet<String>,
    failures: &mut String,
) {
    let path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join(file);
    if std::env::var("HYPERSIG_PUBLIC_API").as_deref() == Ok("overwrite") {
        let mut text = header.to_owned();
        for line in current {
            writeln!(text, "{line}").unwrap();
        }
        std::fs::write(&path, text).expect("the list is writable");
        return;
    }
    let committed = std::fs::read_to_string(&path).unwrap_or_default();
    let committed: BTreeSet<&str> = committed
        .lines()
        .filter(|line| !line.is_empty() && !line.starts_with('#'))
        .collect();
    for line in current {
        if !committed.contains(line.as_str()) {
            writeln!(failures, "{rule}: {line} is not listed in {file}").unwrap();
        }
    }
    for line in committed {
        if !current.contains(line) {
            writeln!(
                failures,
                "{rule}: {file} lists {line}, which no longer needs it"
            )
            .unwrap();
        }
    }
}

#[test]
#[ignore = "reads rustdoc JSON, which `cargo +nightly-2026-10-14 rustdoc -p <crate> --all-features -- -Z unstable-options --output-format json` writes for each crate"]
fn public_api() {
    let mut failures = String::new();
    let mut listed = Listed::default();
    for name in CRATES {
        let krate = Krate::load(name);
        check(&Api::new(&krate), &mut failures, &mut listed);
    }
    compare(
        STRINGS_FILE,
        STRINGS_HEADER,
        "M-STRONG-TYPES",
        &listed.strings,
        &mut failures,
    );
    compare(
        SUMMARIES_FILE,
        SUMMARIES_HEADER,
        "M-FIRST-DOC-SENTENCE",
        &listed.summaries,
        &mut failures,
    );
    assert!(failures.is_empty(), "\n{failures}");
}

#[test]
fn a_summary_ends_at_its_first_full_stop_outside_code() {
    assert_eq!(
        first_sentence("Sets `a.b` to one. Then more.\n\nLater."),
        "Sets `a.b` to one."
    );
    assert_eq!(
        first_sentence("No stop\nat all\n\nLater."),
        "No stop at all"
    );
}

#[test]
fn a_code_span_is_one_word_and_a_link_is_its_text() {
    assert_eq!(
        words("Sets `the long code span` on [`El::id`](super::El::id)."),
        4
    );
    assert_eq!(words("A [link text](a::b(c)) here: 1 to 2."), 7);
}
