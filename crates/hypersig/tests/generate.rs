//! Writes `src/html/attr.rs` from the attribute lists in `data/`.
//!
//! The test fails when the file on disk differs from what the generator writes
//! now. Run it with `HYPERSIG_GENERATE=overwrite` to rewrite it.

use std::collections::{BTreeMap, BTreeSet};
use std::fmt::Write as _;
use std::path::{Path, PathBuf};

use serde_json::Value;

const HTML: &str = "data/html-element-attributes-3.5.0/attributes.json";
const SVG: &str = "data/svg-element-attributes-2.2.0/attributes.json";
const ARIA: &str = "data/aria-attributes-2.0.1/attributes.json";

/// More elements than this are counted rather than listed in a constant's
/// summary.
const LISTED: usize = 6;

/// Where an attribute applies, as the summary of its constant says.
#[derive(Default)]
struct Places {
    html_global: bool,
    svg_global: bool,
    aria: bool,
    html: BTreeSet<String>,
    svg: BTreeSet<String>,
}

/// Whether `Attr::new` in `src/html.rs` accepts `name`, by the same rules.
fn accepted(name: &str) -> bool {
    let lower = name.to_ascii_lowercase();
    !name.is_empty()
        && name
            .bytes()
            .all(|b| b.is_ascii_alphanumeric() || matches!(b, b'-' | b'_' | b'.' | b':'))
        && !lower.starts_with("data-hs-")
        && !lower.starts_with("on")
        && !matches!(
            lower.as_str(),
            "id" | "class"
                | "role"
                | "href"
                | "src"
                | "srcdoc"
                | "action"
                | "formaction"
                | "xlink:href"
                | "data"
                | "http-equiv"
                | "attributename"
        )
}

/// `aria-label` becomes `ARIA_LABEL`, `viewBox` `VIEW_BOX`, and `xml:lang`
/// `XML_LANG`.
fn constant(name: &str) -> String {
    let mut out = String::new();
    let mut previous_lower = false;
    for c in name.chars() {
        if c.is_ascii_uppercase() && previous_lower {
            out.push('_');
        }
        previous_lower = c.is_ascii_lowercase() || c.is_ascii_digit();
        out.push(if matches!(c, '-' | ':' | '.') {
            '_'
        } else {
            c.to_ascii_uppercase()
        });
    }
    out
}

fn read(root: &Path, file: &str) -> Value {
    let text = std::fs::read_to_string(root.join(file)).expect("read attribute data");
    serde_json::from_str(&text).expect("parse attribute data")
}

/// The elements each attribute of a map from element to attributes is on,
/// with `*` meaning every element.
fn by_element(value: &Value) -> BTreeMap<String, (bool, BTreeSet<String>)> {
    let mut out: BTreeMap<String, (bool, BTreeSet<String>)> = BTreeMap::new();
    for (element, names) in value.as_object().expect("a map of elements") {
        for name in names.as_array().expect("a list of attributes") {
            let name = name.as_str().expect("an attribute name").to_owned();
            let entry = out.entry(name).or_default();
            if element == "*" {
                entry.0 = true;
            } else {
                entry.1.insert(element.clone());
            }
        }
    }
    out
}

fn elements(out: &mut Vec<String>, kind: &str, elements: &BTreeSet<String>) {
    if elements.is_empty() {
        return;
    }
    if elements.len() > LISTED {
        out.push(format!("{} {kind} elements", elements.len()));
    } else {
        let tags: Vec<String> = elements.iter().map(|tag| format!("`<{tag}>`")).collect();
        out.push(tags.join(", "));
    }
}

fn summary(name: &str, places: &Places) -> String {
    let mut on = Vec::new();
    if places.aria {
        on.push("every element, as ARIA defines".to_owned());
    }
    if places.html_global {
        on.push("every HTML element".to_owned());
    } else {
        elements(&mut on, "HTML", &places.html);
    }
    if places.svg_global {
        on.push("every SVG element".to_owned());
    } else {
        elements(&mut on, "SVG", &places.svg);
    }
    format!("`{name}`, on {}.", on.join("; "))
}

fn generate(root: &Path) -> String {
    let mut places: BTreeMap<String, Places> = BTreeMap::new();
    for (name, (global, tags)) in by_element(&read(root, HTML)) {
        let entry = places.entry(name).or_default();
        entry.html_global |= global;
        entry.html.extend(tags);
    }
    for (name, (global, tags)) in by_element(&read(root, SVG)) {
        let entry = places.entry(name).or_default();
        entry.svg_global |= global;
        entry.svg.extend(tags);
    }
    for name in read(root, ARIA).as_array().expect("a list of attributes") {
        let name = name.as_str().expect("an attribute name").to_owned();
        places.entry(name).or_default().aria = true;
    }

    let mut constants: BTreeMap<String, (String, Places)> = BTreeMap::new();
    for (name, place) in places {
        if !accepted(&name) {
            continue;
        }
        let key = constant(&name);
        if let Some((other, _)) = constants.get(&key) {
            panic!("`{name}` and `{other}` both become `{key}`");
        }
        constants.insert(key, (name, place));
    }

    let mut out = String::from(
        "//! The attributes of HTML, SVG, and ARIA that an [`Attr`] can name.\n\
         //!\n\
         //! The names come from the npm packages `html-element-attributes` 3.5.0,\n\
         //! `svg-element-attributes` 2.2.0, and `aria-attributes` 2.0.1, which\n\
         //! `tests/generate.rs` reads from `data/`. The attributes [`Attr`] refuses\n\
         //! have no constant here. A name outside these lists, such as `data-lat`\n\
         //! or an attribute of a custom element, is built with\n\
         //! [`attr!`](crate::attr!).\n\
         //!\n\
         //! Written by `tests/generate.rs`. Run it with\n\
         //! `HYPERSIG_GENERATE=overwrite` rather than editing this file.\n\
         \n\
         use super::Attr;\n",
    );
    for (key, (name, place)) in &constants {
        write!(
            out,
            "\n/// {}\npub const {key}: Attr = Attr::new({name:?});\n",
            summary(name, place)
        )
        .expect("write to a String");
    }
    out
}

#[test]
fn attributes() {
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let text = generate(&root);
    let path = root.join("src/html/attr.rs");
    if std::env::var("HYPERSIG_GENERATE").as_deref() == Ok("overwrite") {
        std::fs::write(&path, text).expect("write generated code");
        return;
    }
    let on_disk = std::fs::read_to_string(&path).unwrap_or_default();
    assert!(
        on_disk == text,
        "{} is out of date; rerun with HYPERSIG_GENERATE=overwrite",
        path.display()
    );
}
