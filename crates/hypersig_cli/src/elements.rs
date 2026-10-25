//! `cargo hypersig elements`: writes the Rust module that names a library's
//! custom elements, their attributes, and their events, from the library's
//! Custom Elements Manifest.
//!
//! The manifest's schema is at
//! <https://github.com/webcomponents/custom-elements-manifest>. This reads
//! the `modules` of schema 1.x: each class declaration with a `tagName`, or
//! that an export of kind `custom-element-definition` names, with the
//! `attributes` and `events` of the class, its superclasses, and its mixins
//! that the manifest also declares.

mod ts;

use std::collections::{HashMap, HashSet};
use std::fmt::Write as _;
use std::path::Path;

use serde_json::Value;

use crate::Error;
use ts::Ty;

/// Prints the module for the manifest at `path`.
pub fn run(path: &Path) -> Result<(), Error> {
    let text = std::fs::read_to_string(path)
        .map_err(|error| Error::new(format!("cannot read {}: {error}", path.display())))?;
    let manifest: Value = serde_json::from_str(&text)
        .map_err(|error| Error::new(format!("{} is not JSON: {error}", path.display())))?;
    let name = path.file_name().map_or_else(
        || path.display().to_string(),
        |name| name.to_string_lossy().into_owned(),
    );
    print!("{}", generate(&manifest, &name)?);
    Ok(())
}

/// A custom element, with what its class and the classes it extends declare.
struct Element {
    tag: String,
    class: String,
    module: String,
    attributes: Vec<Member>,
    events: Vec<Member>,
}

/// An attribute or an event: its name and the text of its `type`.
struct Member {
    name: String,
    ty: Option<String>,
}

/// The source of the module for `manifest`, which the header names `source`.
pub fn generate(manifest: &Value, source: &str) -> Result<String, Error> {
    let elements = elements(manifest)?;
    let mut out = String::new();
    header(&mut out, source);
    let mut names = Names::default();
    let mut first = true;
    for element in &elements {
        if !first {
            out.push('\n');
        }
        first = false;
        if !is_tag(&element.tag) {
            let _ = writeln!(
                out,
                "// {:?} is left out: it is not a custom element name.",
                element.tag
            );
            continue;
        }
        let name = names.snake(&element.tag);
        write_element(&mut out, element, &name);
    }
    Ok(out)
}

fn header(out: &mut String, source: &str) {
    let source = code(&ts::one_line(source));
    let _ = write!(
        out,
        "\
//! The custom elements of {source}, written by `cargo hypersig elements`.
//!
//! Each element has a function that creates it, a module `attr` with the
//! names of its attributes for `El::attr`, `El::bind_attr`, and
//! `El::attr_when`, and a module `event` with its events for
//! `El::on_custom`. The names and the `detail` types come from the manifest,
//! so they hold as far as the manifest describes the library's script. An
//! attribute or an event that a superclass or a mixin outside the manifest
//! declares is not here.
//!
//! Run the command again after updating the library, rather than editing
//! this file.

#![allow(dead_code, reason = \"a page uses some of a library's elements\")]

use hypersig::html::{{El, el}};

"
    );
}

fn write_element(out: &mut String, element: &Element, name: &str) {
    let tag = &element.tag;
    let _ = write!(
        out,
        "\
/// Creates `<{tag}>`, the element of {class} in {module}.
#[must_use]
pub fn {name}<S>() -> El<S> {{
    el(hypersig::tag!({tag:?}))
}}
",
        class = code(&element.class),
        module = code(&element.module),
    );
    let mut parts = Vec::new();
    if !element.attributes.is_empty() {
        parts.push(attributes(tag, &element.attributes));
    }
    if !element.events.is_empty() {
        parts.push(events(tag, &element.events));
    }
    if parts.is_empty() {
        return;
    }
    let _ = write!(
        out,
        "\n/// The attributes and events of `<{tag}>`.\npub mod {name} {{\n{}}}\n",
        parts.join("\n")
    );
}

/// The module `attr` of the element `<tag>`.
fn attributes(tag: &str, attributes: &[Member]) -> String {
    let mut out = format!(
        "    /// The attributes of `<{tag}>`.\n    pub mod attr {{\n        use hypersig::html::Attr;\n\n"
    );
    let mut names = Names::default();
    for attribute in attributes {
        let name = &attribute.name;
        if let Some(reason) = refused(name) {
            let _ = writeln!(out, "        // {} is left out: {reason}", shown(name));
            continue;
        }
        let ty = attribute.ty.as_deref().map(ts::one_line);
        let boolean = ty
            .as_deref()
            .and_then(ts::parse)
            .and_then(|ty| ts::rust_type(&ty))
            .is_some_and(|ty| ty == "bool" || ty == "Option<bool>");
        let doc = match &ty {
            Some(ty) if boolean => {
                format!(
                    "`{name}`, of type {}. Set it with `El::attr_when`.",
                    code(ty)
                )
            }
            Some(ty) => format!("`{name}`, of type {}.", code(ty)),
            None => format!("`{name}`."),
        };
        let ident = names.screaming(name);
        let line = format!("        pub const {ident}: Attr = hypersig::attr!({name:?});");
        let line = if line.chars().count() > RUSTFMT_WIDTH {
            format!("        pub const {ident}: Attr =\n            hypersig::attr!({name:?});")
        } else {
            line
        };
        let _ = writeln!(out, "        /// {doc}\n{line}");
    }
    out.push_str("    }\n");
    out
}

/// Why `hypersig::attr!` refuses the attribute `name`, as `Attr::new` in
/// hypersig's `html.rs` decides, or `None`. A refused name would stop the
/// written module from compiling, so the module leaves it out.
fn refused(name: &str) -> Option<&'static str> {
    let lower = name.to_ascii_lowercase();
    if name.is_empty()
        || !name
            .bytes()
            .all(|byte| byte.is_ascii_alphanumeric() || matches!(byte, b'-' | b'_' | b'.' | b':'))
    {
        return Some("it is not a name `hypersig::attr!` accepts.");
    }
    Some(match lower.as_str() {
        "id" => "set it with `El::id`.",
        "class" => "set it with `El::class`.",
        "role" => "set it with `El::role`, which takes a `Role`.",
        "for"
        | "aria-labelledby"
        | "aria-describedby"
        | "aria-controls"
        | "aria-owns"
        | "aria-details"
        | "aria-flowto"
        | "aria-errormessage"
        | "aria-activedescendant"
        | "commandfor"
        | "popovertarget"
        | "list"
        | "form"
        | "headers"
        | "itemref" => "its value names an element, so the `El` method that takes an `Id` sets it.",
        "href" => "set it with `El::href`, which takes a `Link` or a `SafeUrl`.",
        "src" => "set it with `El::src`, which takes a `SafeUrl`.",
        "attributename" => "`hypersig::attr!` refuses it, because its value could name `href`.",
        _ if lower.starts_with("data-hs-") => "hypersig writes the `data-hs-` attributes.",
        "srcdoc" | "action" | "formaction" | "xlink:href" | "data" | "http-equiv" => {
            "its value can run a script, so `hypersig::attr!` refuses it."
        }
        _ if lower.starts_with("on") => {
            "its value can run a script, so `hypersig::attr!` refuses it."
        }
        _ => return None,
    })
}

/// rustfmt's default `max_width`. Past it, rustfmt moves the value of a
/// `const` to the next line, so the module breaks the line there too.
const RUSTFMT_WIDTH: usize = 100;

/// The module `event` of the element `<tag>`.
fn events(tag: &str, events: &[Member]) -> String {
    let mut consts = Vec::new();
    let mut structs = Vec::new();
    let mut skipped = Vec::new();
    let mut names = Names::default();
    let mut types = Names::default();
    let mut snake_case = true;
    for event in events {
        let name = &event.name;
        if name.is_empty() || name.chars().any(char::is_whitespace) {
            skipped.push(format!(
                "        // {} is left out: an event name holds no whitespace.\n",
                shown(name)
            ));
            continue;
        }
        let text = event.ty.as_deref().map(ts::one_line);
        let parsed = text.as_deref().and_then(ts::parse);
        let (detail, doc) = match (&text, parsed) {
            (None, _) => (
                "()".to_owned(),
                format!(
                    "`{name}`. The manifest gives its `detail` no type, so the handler's \
                     `detail` is `()`."
                ),
            ),
            (Some(text), Some(Ty::Object(fields))) => {
                let detail = types.pascal(&format!("{name}-detail"));
                match detail_struct(&detail, name, &fields) {
                    Some((source, snake)) => {
                        snake_case &= snake;
                        structs.push(source);
                        (
                            detail,
                            format!("`{name}`, whose `detail` is {}.", code(text)),
                        )
                    }
                    None => (
                        "()".to_owned(),
                        format!(
                            "`{name}`, whose `detail` is {}. No field of it maps to a Rust \
                             type, so the handler's `detail` is `()`.",
                            code(text)
                        ),
                    ),
                }
            }
            (Some(text), parsed) => match parsed.as_ref().and_then(ts::rust_type) {
                Some(rust) => (rust, format!("`{name}`, whose `detail` is {}.", code(text))),
                None => (
                    "()".to_owned(),
                    format!(
                        "`{name}`. The manifest's type for it, {}, maps to no Rust type, so \
                         the handler's `detail` is `()`.",
                        code(text)
                    ),
                ),
            },
        };
        let ident = names.screaming(name);
        let line = format!(
            "        pub const {ident}: CustomEvent<{detail}> = CustomEvent::new({name:?});"
        );
        let line = if line.chars().count() > RUSTFMT_WIDTH {
            format!(
                "        pub const {ident}: CustomEvent<{detail}> =\n            CustomEvent::new({name:?});"
            )
        } else {
            line
        };
        consts.push(format!("        /// {doc}\n{line}\n"));
    }
    let mut out = format!("    /// The events `<{tag}>` dispatches.\n");
    if !snake_case {
        out.push_str(
            "    #[allow(\n        non_snake_case,\n        reason = \"a field of a `detail` is named as the script names it\"\n    )]\n",
        );
    }
    out.push_str("    pub mod event {\n");
    if !structs.is_empty() {
        out.push_str("        use hypersig::Signals;\n");
    }
    if !consts.is_empty() {
        out.push_str("        use hypersig::html::CustomEvent;\n");
    }
    if !structs.is_empty() {
        out.push_str("        use serde::{Deserialize, Serialize};\n");
    }
    if !structs.is_empty() || !consts.is_empty() {
        out.push('\n');
    }
    let items: Vec<String> = consts.into_iter().chain(structs).collect();
    out.push_str(&items.join("\n"));
    if !skipped.is_empty() {
        if !items.is_empty() {
            out.push('\n');
        }
        out.push_str(&skipped.concat());
    }
    out.push_str("    }\n");
    out
}

/// The struct `detail` for the object literal `fields`, the `detail` of
/// event `event`, and whether every field name it keeps is snake case. `None`
/// where no field maps to a Rust type.
///
/// The fields keep the script's names, because the `Signals` derive reads a
/// field at the path its Rust name spells and takes no `#[serde(rename)]`.
fn detail_struct(detail: &str, event: &str, fields: &[ts::Field]) -> Option<(String, bool)> {
    let mut body = String::new();
    let mut skipped = String::new();
    let mut kept = 0;
    let mut snake_case = true;
    for field in fields {
        let ident = field_ident(&field.name);
        let rust = ts::rust_type(&field.ty).map(|rust| {
            if field.optional {
                ts::optional(&rust)
            } else {
                rust
            }
        });
        let shown_field = format!(
            "{}{}: {}",
            field.name,
            if field.optional { "?" } else { "" },
            field.text
        );
        match (ident, rust) {
            (Some(ident), Some(rust)) => {
                kept += 1;
                snake_case &= !field.name.chars().any(|c| c.is_ascii_uppercase());
                let _ = writeln!(body, "            /// {}.", code(&shown_field));
                let _ = writeln!(body, "            pub {ident}: {rust},");
            }
            (None, _) => {
                let _ = writeln!(
                    skipped,
                    "            // {} is left out: its name is not a Rust field name.",
                    code(&shown_field)
                );
            }
            (Some(_), None) => {
                let _ = writeln!(
                    skipped,
                    "            // {} is left out: its type maps to no Rust type.",
                    code(&shown_field)
                );
            }
        }
    }
    if kept == 0 {
        return None;
    }
    Some((
        format!(
            "        /// The `detail` of `{event}`.\n        \
             #[derive(Debug, Signals, Serialize, Deserialize)]\n        \
             pub struct {detail} {{\n{body}{skipped}        }}\n"
        ),
        snake_case,
    ))
}

/// The Rust field for the property `name`: `name` itself, raw where it is a
/// keyword, or `None` where it cannot be one.
fn field_ident(name: &str) -> Option<String> {
    let mut chars = name.chars();
    let first = chars.next()?;
    if !(first.is_ascii_alphabetic() || first == '_')
        || !chars.all(|c| c.is_ascii_alphanumeric() || c == '_')
        || matches!(name, "_" | "self" | "Self" | "super" | "crate")
    {
        return None;
    }
    Some(if KEYWORDS.contains(&name) {
        format!("r#{name}")
    } else {
        name.to_owned()
    })
}

/// The keywords of Rust 2024 that a raw identifier can spell.
const KEYWORDS: [&str; 47] = [
    "abstract", "as", "async", "await", "become", "box", "break", "const", "continue", "do", "dyn",
    "else", "enum", "extern", "false", "final", "fn", "for", "gen", "if", "impl", "in", "let",
    "loop", "macro", "match", "mod", "move", "mut", "override", "priv", "pub", "ref", "return",
    "static", "struct", "trait", "true", "try", "type", "typeof", "unsafe", "unsized", "use",
    "virtual", "where", "while",
];

/// Whether `tag` is a custom element name as the HTML standard defines one,
/// narrowed to ASCII: a lowercase letter, then lowercase letters, digits,
/// `-`, `.`, and `_`, with at least one `-`.
fn is_tag(tag: &str) -> bool {
    tag.starts_with(|c: char| c.is_ascii_lowercase())
        && tag.contains('-')
        && tag.bytes().all(|byte| {
            byte.is_ascii_lowercase() || byte.is_ascii_digit() || matches!(byte, b'-' | b'.' | b'_')
        })
}

/// `text` as a Markdown code span.
fn code(text: &str) -> String {
    if text.contains('`') {
        format!("`` {text} ``")
    } else {
        format!("`{text}`")
    }
}

/// `name` in a comment: as a code span where it is printable on one line,
/// and as a Rust string literal otherwise.
fn shown(name: &str) -> String {
    if !name.is_empty() && name.chars().all(|c| !c.is_control() && !c.is_whitespace()) {
        code(name)
    } else {
        format!("{name:?}")
    }
}

/// The identifiers given out in one namespace, so that two names that spell
/// the same identifier get `_2`, `_3`, and so on.
#[derive(Default)]
struct Names {
    taken: HashSet<String>,
}

impl Names {
    fn take(&mut self, base: String) -> String {
        let base = if base.starts_with(|c: char| c.is_ascii_digit()) || base.is_empty() {
            format!("_{base}")
        } else {
            base
        };
        let mut ident = base.clone();
        let mut n = 2;
        while !self.taken.insert(ident.clone()) {
            ident = format!("{base}_{n}");
            n += 1;
        }
        if KEYWORDS.contains(&ident.as_str()) {
            format!("r#{ident}")
        } else {
            ident
        }
    }

    fn snake(&mut self, name: &str) -> String {
        self.take(words(name).join("_"))
    }

    fn screaming(&mut self, name: &str) -> String {
        self.take(words(name).join("_").to_ascii_uppercase())
    }

    fn pascal(&mut self, name: &str) -> String {
        let pascal = words(name)
            .iter()
            .map(|word| {
                let mut chars = word.chars();
                chars.next().map_or_else(String::new, |first| {
                    format!("{}{}", first.to_ascii_uppercase(), chars.as_str())
                })
            })
            .collect();
        self.take(pascal)
    }
}

/// The lowercase ASCII words of `name`, split at each character that is not
/// a letter or a digit and before an uppercase letter that follows a
/// lowercase one.
fn words(name: &str) -> Vec<String> {
    let mut words = Vec::new();
    let mut word = String::new();
    let mut lower_before = false;
    for c in name.chars() {
        if !c.is_ascii_alphanumeric() {
            if !word.is_empty() {
                words.push(std::mem::take(&mut word));
            }
            lower_before = false;
            continue;
        }
        if c.is_ascii_uppercase() && lower_before && !word.is_empty() {
            words.push(std::mem::take(&mut word));
        }
        lower_before = c.is_ascii_lowercase() || c.is_ascii_digit();
        word.push(c.to_ascii_lowercase());
    }
    if !word.is_empty() {
        words.push(word);
    }
    words
}

/// The custom elements `manifest` declares, ordered by tag.
fn elements(manifest: &Value) -> Result<Vec<Element>, Error> {
    let modules = manifest
        .get("modules")
        .and_then(Value::as_array)
        .ok_or_else(|| Error::new("the manifest has no `modules` list"))?;
    let mut classes: HashMap<(String, String), &Value> = HashMap::new();
    for module in modules {
        let path = module_path(string(module, "path").unwrap_or(""));
        for declaration in array(module, "declarations") {
            if let Some(name) = string(declaration, "name") {
                classes.insert((path.clone(), name.to_owned()), declaration);
            }
        }
    }

    let mut found: Vec<(String, String, String)> = Vec::new();
    for module in modules {
        let path = module_path(string(module, "path").unwrap_or(""));
        for declaration in array(module, "declarations") {
            if let (Some(tag), Some(name)) =
                (string(declaration, "tagName"), string(declaration, "name"))
            {
                found.push((tag.to_owned(), path.clone(), name.to_owned()));
            }
        }
        for export in array(module, "exports") {
            if string(export, "kind") != Some("custom-element-definition") {
                continue;
            }
            let (Some(tag), Some(declaration)) =
                (string(export, "name"), export.get("declaration"))
            else {
                continue;
            };
            let Some(name) = string(declaration, "name") else {
                continue;
            };
            let module = string(declaration, "module").map_or_else(|| path.clone(), module_path);
            found.push((tag.to_owned(), module, name.to_owned()));
        }
    }

    let mut seen = HashSet::new();
    let mut elements = Vec::new();
    for (tag, module, class) in found {
        if !seen.insert(tag.clone()) {
            continue;
        }
        let mut attributes = Vec::new();
        let mut events = Vec::new();
        let mut visited = HashSet::new();
        collect(
            &classes,
            &(module.clone(), class.clone()),
            &mut visited,
            &mut attributes,
            &mut events,
        );
        elements.push(Element {
            tag,
            class,
            module,
            attributes,
            events,
        });
    }
    if elements.is_empty() {
        return Err(Error::new(
            "the manifest declares no custom element: no class has a `tagName`, and no export is a `custom-element-definition`",
        ));
    }
    elements.sort_by(|a, b| a.tag.cmp(&b.tag));
    Ok(elements)
}

/// Adds the attributes and events of the class `key`, then those of its
/// superclass and its mixins, keeping the first member of each name, so a
/// class's own member wins over the one it overrides.
fn collect(
    classes: &HashMap<(String, String), &Value>,
    key: &(String, String),
    visited: &mut HashSet<(String, String)>,
    attributes: &mut Vec<Member>,
    events: &mut Vec<Member>,
) {
    let Some(class) = classes.get(key) else {
        return;
    };
    if !visited.insert(key.clone()) {
        return;
    }
    for (list, into) in [("attributes", &mut *attributes), ("events", &mut *events)] {
        for member in array(class, list) {
            let Some(name) = string(member, "name") else {
                continue;
            };
            if into.iter().any(|known| known.name == name) {
                continue;
            }
            into.push(Member {
                name: name.to_owned(),
                ty: member
                    .get("type")
                    .and_then(|ty| string(ty, "text"))
                    .map(str::to_owned),
            });
        }
    }
    let parents = class
        .get("superclass")
        .into_iter()
        .chain(array(class, "mixins"));
    for parent in parents {
        let Some(name) = string(parent, "name") else {
            continue;
        };
        let module = string(parent, "module").map_or_else(|| key.0.clone(), module_path);
        collect(
            classes,
            &(module, name.to_owned()),
            visited,
            attributes,
            events,
        );
    }
}

fn string<'a>(value: &'a Value, key: &str) -> Option<&'a str> {
    value.get(key).and_then(Value::as_str)
}

fn array<'a>(value: &'a Value, key: &str) -> impl Iterator<Item = &'a Value> {
    value
        .get(key)
        .and_then(Value::as_array)
        .into_iter()
        .flatten()
}

/// A module path as `path` writes it, without a leading `./` or `/`, which
/// manifests write for the same module in different places.
fn module_path(path: &str) -> String {
    path.trim_start_matches("./")
        .trim_start_matches('/')
        .to_owned()
}

#[cfg(test)]
mod tests {
    use super::{Names, field_ident, refused, words};

    #[test]
    fn names_follow_rust_case() {
        assert_eq!(words("sl-rating"), ["sl", "rating"]);
        assert_eq!(words("getSymbol"), ["get", "symbol"]);
        assert_eq!(words("x:href"), ["x", "href"]);
        let mut names = Names::default();
        assert_eq!(names.screaming("max-items"), "MAX_ITEMS");
        assert_eq!(names.screaming("max_items"), "MAX_ITEMS_2");
        assert_eq!(names.pascal("sl-hover-detail"), "SlHoverDetail");
        assert_eq!(names.snake("2-up"), "_2_up");
    }

    #[test]
    fn a_field_keeps_the_script_name() {
        assert_eq!(field_ident("previousName").as_deref(), Some("previousName"));
        assert_eq!(field_ident("type").as_deref(), Some("r#type"));
        assert_eq!(field_ident("self"), None);
        assert_eq!(field_ident("x-y"), None);
    }

    #[test]
    fn refuses_what_attr_refuses() {
        for name in [
            "id",
            "CLASS",
            "href",
            "src",
            "onclick",
            "formaction",
            "data-hs-x",
            "ROLE",
            "aria-describedby",
            "a b",
        ] {
            assert!(refused(name).is_some(), "{name}");
        }
        for name in ["value", "open", "data-value", "aria-label"] {
            assert!(refused(name).is_none(), "{name}");
        }
    }
}
