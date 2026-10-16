//! Writes `src/generated.rs` and `src/value/generated.rs` from the MDN CSS
//! data in `data/`.
//!
//! The test fails when a file on disk differs from what the generator writes
//! now. Run it with `HYPERSIG_GENERATE=overwrite` to rewrite it.

use std::collections::{BTreeMap, BTreeSet};
use std::fmt::Write as _;
use std::path::PathBuf;

use serde_json::{Map, Value};

const DATA: &str = "data/mdn-data-2.37.2";

/// Value types the crate defines by hand, keyed by the name MDN gives them.
/// `length-percentage` accepts either a length or a percentage.
const KINDS: &[(&str, &[Kind])] = &[
    ("length", &[Kind::Length]),
    ("length-percentage", &[Kind::Length, Kind::Percentage]),
    ("percentage", &[Kind::Percentage]),
    ("number", &[Kind::Number]),
    ("integer", &[Kind::Integer]),
    ("color", &[Kind::Color]),
    ("time", &[Kind::Time]),
    ("angle", &[Kind::Angle]),
    ("string", &[Kind::Quoted]),
    ("url", &[Kind::Url]),
];

/// Names the hand-written part of the crate already uses for a type.
const TYPE_NAMES: &[&str] = &[
    "Angle",
    "Color",
    "Condition",
    "Global",
    "Length",
    "Media",
    "Percentage",
    "Property",
    "PseudoClass",
    "PseudoElement",
    "Quoted",
    "Raw",
    "Sheet",
    "Style",
    "Time",
    "Url",
];

/// Methods `Style` defines by hand, which no property method may shadow.
const STYLE_METHODS: &[&str] = &[
    "new",
    "on",
    "pseudo",
    "global",
    "merge",
    "class_names",
    "toggled_class_names",
    "token",
    "keyframes",
    "is_empty",
    "group",
    "peer",
];

/// Strict and reserved keywords of Rust 2024, which no identifier may be.
const RUST_KEYWORDS: &[&str] = &[
    "abstract", "as", "async", "await", "become", "box", "break", "const", "continue", "crate",
    "do", "dyn", "else", "enum", "extern", "false", "final", "fn", "for", "gen", "if", "impl",
    "in", "let", "loop", "macro", "match", "mod", "move", "mut", "override", "priv", "pub", "ref",
    "return", "self", "Self", "static", "struct", "super", "trait", "true", "try", "type",
    "typeof", "unsafe", "unsized", "use", "virtual", "where", "while", "yield",
];

/// Pseudo-classes left out of `PseudoClass`: those that select a page box in
/// `@page` (`:first`, `:left`, `:right`, `:blank`) or the document or shadow
/// root (`:root`, `:scope`, `:host`), which a class on an element never matches.
const SKIPPED_PSEUDO_CLASSES: &[&str] = &[
    ":blank", ":first", ":host", ":left", ":right", ":root", ":scope",
];

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum Kind {
    Length,
    Percentage,
    Number,
    Integer,
    Color,
    Time,
    Angle,
    Quoted,
    Url,
}

impl Kind {
    fn type_name(self) -> &'static str {
        match self {
            Self::Length => "Length",
            Self::Percentage => "Percentage",
            Self::Number => "f64",
            Self::Integer => "i32",
            Self::Color => "Color",
            Self::Time => "Time",
            Self::Angle => "Angle",
            Self::Quoted => "Quoted",
            Self::Url => "Url",
        }
    }

    fn variant(self) -> &'static str {
        match self {
            Self::Length => "Length",
            Self::Percentage => "Percentage",
            Self::Number => "Number",
            Self::Integer => "Integer",
            Self::Color => "Color",
            Self::Time => "Time",
            Self::Angle => "Angle",
            Self::Quoted => "Quoted",
            Self::Url => "Url",
        }
    }

    fn doc(self) -> &'static str {
        match self {
            Self::Length => "A length.",
            Self::Percentage => "A percentage.",
            Self::Number => "A number.",
            Self::Integer => "An integer.",
            Self::Color => "A color.",
            Self::Time => "A duration.",
            Self::Angle => "An angle.",
            Self::Quoted => "A quoted string.",
            Self::Url => "A URL.",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
enum Atom {
    Keyword(String),
    Kind(Kind),
}

/// A node of the CSS value definition syntax.
#[derive(Debug)]
enum Node {
    Alt(Vec<Node>),
    AnyOrder(Vec<Node>),
    All,
    Seq(Vec<Node>),
    Repeat(Box<Node>, u32, u32),
    Keyword(String),
    Type(String),
    PropertyRef(String),
    Opaque,
}

#[derive(Debug)]
enum Token {
    Open,
    Close,
    Bar,
    DoubleBar,
    DoubleAmp,
    Term(Node),
    Repeat(u32, u32),
}

#[expect(
    clippy::too_many_lines,
    reason = "one arm per token of the value definition syntax"
)]
fn tokenize(syntax: &str) -> Vec<Token> {
    let chars: Vec<char> = syntax.chars().collect();
    let mut out = Vec::new();
    let mut i = 0;
    while i < chars.len() {
        let c = chars[i];
        match c {
            // `!` requires a group to be non-empty, which changes no single value.
            ' ' | '\t' | '\n' | '!' => i += 1,
            '[' => {
                out.push(Token::Open);
                i += 1;
            }
            ']' => {
                out.push(Token::Close);
                i += 1;
            }
            '|' if chars.get(i + 1) == Some(&'|') => {
                out.push(Token::DoubleBar);
                i += 2;
            }
            '|' => {
                out.push(Token::Bar);
                i += 1;
            }
            '&' if chars.get(i + 1) == Some(&'&') => {
                out.push(Token::DoubleAmp);
                i += 2;
            }
            '<' => {
                let end = (i..chars.len())
                    .find(|&j| chars[j] == '>')
                    .expect("closed <");
                let inner: String = chars[i + 1..end].iter().collect();
                out.push(Token::Term(type_term(&inner)));
                i = end + 1;
            }
            '?' => {
                out.push(Token::Repeat(0, 1));
                i += 1;
            }
            '*' => {
                out.push(Token::Repeat(0, u32::MAX));
                i += 1;
            }
            '+' | '#' => {
                // `#{1,4}` bounds a comma-separated list the way `{1,4}` does.
                if chars.get(i + 1) == Some(&'{') {
                    i += 1;
                } else {
                    out.push(Token::Repeat(1, u32::MAX));
                    i += 1;
                }
            }
            '{' => {
                let end = (i..chars.len())
                    .find(|&j| chars[j] == '}')
                    .expect("closed {");
                let inner: String = chars[i + 1..end].iter().collect();
                let (min, max) = match inner.split_once(',') {
                    Some((min, "")) => (min.trim().parse().expect("min"), u32::MAX),
                    Some((min, max)) => (
                        min.trim().parse().expect("min"),
                        max.trim().parse().expect("max"),
                    ),
                    None => {
                        let n = inner.trim().parse().expect("count");
                        (n, n)
                    }
                };
                out.push(Token::Repeat(min, max));
                i = end + 1;
            }
            c if c.is_ascii_alphanumeric() || c == '-' => {
                let start = i;
                while i < chars.len() && (chars[i].is_ascii_alphanumeric() || chars[i] == '-') {
                    i += 1;
                }
                if chars.get(i) == Some(&'(') {
                    let mut depth = 0;
                    while i < chars.len() {
                        match chars[i] {
                            '(' => depth += 1,
                            ')' => {
                                depth -= 1;
                                if depth == 0 {
                                    i += 1;
                                    break;
                                }
                            }
                            _ => {}
                        }
                        i += 1;
                    }
                    out.push(Token::Term(Node::Opaque));
                } else {
                    out.push(Token::Term(Node::Keyword(chars[start..i].iter().collect())));
                }
            }
            '\'' => {
                let end = (i + 1..chars.len())
                    .find(|&j| chars[j] == '\'')
                    .expect("closed '");
                out.push(Token::Term(Node::Opaque));
                i = end + 1;
            }
            _ => {
                out.push(Token::Term(Node::Opaque));
                i += 1;
            }
        }
    }
    out
}

fn type_term(inner: &str) -> Node {
    if let Some(name) = inner.strip_prefix('\'').and_then(|s| s.strip_suffix('\'')) {
        return Node::PropertyRef(name.to_owned());
    }
    let name = inner.split_whitespace().next().unwrap_or("");
    if name.ends_with(')') {
        return Node::Opaque;
    }
    Node::Type(name.to_owned())
}

struct Parser {
    tokens: Vec<Token>,
    at: usize,
}

impl Parser {
    fn parse(syntax: &str) -> Node {
        let mut parser = Self {
            tokens: tokenize(syntax),
            at: 0,
        };
        let node = parser.alt();
        assert_eq!(parser.at, parser.tokens.len(), "unparsed tail in {syntax}");
        node
    }

    fn peek(&self) -> Option<&Token> {
        self.tokens.get(self.at)
    }

    fn alt(&mut self) -> Node {
        let mut items = vec![self.any_order()];
        while matches!(self.peek(), Some(Token::Bar)) {
            self.at += 1;
            items.push(self.any_order());
        }
        if items.len() == 1 {
            items.remove(0)
        } else {
            Node::Alt(items)
        }
    }

    fn any_order(&mut self) -> Node {
        let mut items = vec![self.all()];
        while matches!(self.peek(), Some(Token::DoubleBar)) {
            self.at += 1;
            items.push(self.all());
        }
        if items.len() == 1 {
            items.remove(0)
        } else {
            Node::AnyOrder(items)
        }
    }

    fn all(&mut self) -> Node {
        let mut items = vec![self.seq()];
        while matches!(self.peek(), Some(Token::DoubleAmp)) {
            self.at += 1;
            items.push(self.seq());
        }
        if items.len() == 1 {
            items.remove(0)
        } else {
            Node::All
        }
    }

    fn seq(&mut self) -> Node {
        let mut items = Vec::new();
        while let Some(token) = self.peek() {
            if matches!(
                token,
                Token::Bar | Token::DoubleBar | Token::DoubleAmp | Token::Close
            ) {
                break;
            }
            items.push(self.repeated());
        }
        if items.len() == 1 {
            items.remove(0)
        } else {
            Node::Seq(items)
        }
    }

    fn repeated(&mut self) -> Node {
        let mut node = match std::mem::replace(&mut self.tokens[self.at], Token::Open) {
            Token::Term(node) => {
                self.at += 1;
                node
            }
            Token::Open => {
                self.at += 1;
                let inner = self.alt();
                assert!(matches!(self.peek(), Some(Token::Close)), "closed [");
                self.at += 1;
                inner
            }
            other => panic!("unexpected {other:?}"),
        };
        while let Some(Token::Repeat(min, max)) = self.peek() {
            node = Node::Repeat(Box::new(node), *min, *max);
            self.at += 1;
        }
        node
    }
}

struct Data {
    properties: Map<String, Value>,
    syntaxes: Map<String, Value>,
}

/// The values a property accepts as a single component, and whether those are
/// all the values it accepts.
struct Singles {
    atoms: BTreeSet<Atom>,
    complete: bool,
}

impl Data {
    fn singles(&self, node: &Node, seen: &mut Vec<String>) -> Singles {
        match node {
            Node::Keyword(word) => Singles {
                atoms: BTreeSet::from([Atom::Keyword(word.clone())]),
                complete: true,
            },
            Node::Type(name) => {
                if let Some((_, kinds)) = KINDS.iter().find(|(kind, _)| kind == name) {
                    return Singles {
                        atoms: kinds.iter().map(|&k| Atom::Kind(k)).collect(),
                        complete: true,
                    };
                }
                self.follow(name, &self.syntaxes, seen)
            }
            Node::PropertyRef(name) => self.follow(name, &self.properties, seen),
            Node::Opaque | Node::All => Singles {
                atoms: BTreeSet::new(),
                complete: false,
            },
            Node::Alt(items) => {
                let mut out = Singles {
                    atoms: BTreeSet::new(),
                    complete: true,
                };
                for item in items {
                    let s = self.singles(item, seen);
                    out.atoms.extend(s.atoms);
                    out.complete &= s.complete;
                }
                out
            }
            // Each operand of `||` may stand alone.
            Node::AnyOrder(items) => {
                let mut atoms = BTreeSet::new();
                for item in items {
                    atoms.extend(self.singles(item, seen).atoms);
                }
                Singles {
                    atoms,
                    complete: false,
                }
            }
            Node::Seq(items) => {
                let required: Vec<&Node> = items
                    .iter()
                    .filter(|item| !matches!(item, Node::Repeat(_, 0, _)))
                    .collect();
                match required.as_slice() {
                    [only] => Singles {
                        atoms: self.singles(only, seen).atoms,
                        complete: false,
                    },
                    [] => {
                        let mut atoms = BTreeSet::new();
                        for item in items {
                            atoms.extend(self.singles(item, seen).atoms);
                        }
                        Singles {
                            atoms,
                            complete: false,
                        }
                    }
                    _ => Singles {
                        atoms: BTreeSet::new(),
                        complete: false,
                    },
                }
            }
            Node::Repeat(inner, _, max) => {
                let s = self.singles(inner, seen);
                Singles {
                    atoms: s.atoms,
                    complete: s.complete && *max == 1,
                }
            }
        }
    }

    fn follow(&self, name: &str, table: &Map<String, Value>, seen: &mut Vec<String>) -> Singles {
        let syntax = table.get(name).and_then(|v| v["syntax"].as_str());
        match syntax {
            Some(syntax) if !seen.iter().any(|s| s == name) => {
                seen.push(name.to_owned());
                let out = self.singles(&Parser::parse(syntax), seen);
                seen.pop();
                out
            }
            _ => Singles {
                atoms: BTreeSet::new(),
                complete: false,
            },
        }
    }
}

fn camel(name: &str) -> String {
    let mut out = String::new();
    for part in name.split('-').filter(|p| !p.is_empty()) {
        let mut chars = part.chars();
        if let Some(first) = chars.next() {
            out.extend(first.to_uppercase());
            out.push_str(chars.as_str());
        }
    }
    assert!(
        out.chars().next().is_some_and(|c| c.is_ascii_alphabetic()),
        "`{name}` does not start with a letter"
    );
    out
}

fn snake(name: &str) -> String {
    name.trim_start_matches('-').replace('-', "_")
}

fn check_ident(ident: &str, what: &str) {
    assert!(
        !RUST_KEYWORDS.contains(&ident),
        "{what} `{ident}` is a Rust keyword"
    );
}

fn load(file: &str) -> Map<String, Value> {
    let path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join(DATA)
        .join(file);
    let text = std::fs::read_to_string(&path).expect("read MDN data");
    match serde_json::from_str(&text).expect("parse MDN data") {
        Value::Object(map) => map,
        _ => panic!("{file} is not an object"),
    }
}

fn is_standard(value: &Value) -> bool {
    value["status"].as_str() == Some("standard")
}

/// Longest chain of shorthands above each property, so that a shorthand is
/// written before the longhands it sets.
fn ranks(properties: &BTreeMap<&str, &Value>) -> BTreeMap<String, u32> {
    let mut parents: BTreeMap<&str, Vec<&str>> = BTreeMap::new();
    for (&name, value) in properties {
        if let Some(longhands) = value["computed"].as_array() {
            for longhand in longhands.iter().filter_map(Value::as_str) {
                parents.entry(longhand).or_default().push(name);
            }
        }
    }
    properties
        .keys()
        .map(|&name| (name.to_owned(), depth(name, &parents, 0)))
        .collect()
}

fn depth(name: &str, parents: &BTreeMap<&str, Vec<&str>>, guard: u32) -> u32 {
    assert!(guard < 16, "shorthand cycle at {name}");
    parents.get(name).map_or(0, |ps| {
        ps.iter()
            .map(|p| 1 + depth(p, parents, guard + 1))
            .max()
            .unwrap_or(0)
    })
}

#[expect(
    clippy::too_many_lines,
    reason = "one section of the generated file after another"
)]
fn generate() -> (String, String) {
    let all_properties = load("properties.json");
    let syntaxes = load("syntaxes.json");
    let selectors = load("selectors.json");
    let data = Data {
        properties: all_properties.clone(),
        syntaxes: syntaxes.clone(),
    };
    let properties: BTreeMap<&str, &Value> = all_properties
        .iter()
        .filter(|(name, value)| is_standard(value) && !name.starts_with('-'))
        .map(|(name, value)| (name.as_str(), value))
        .collect();
    let ranks = ranks(&properties);

    let mut out = String::from(
        "// @generated by tests/generate.rs from data/mdn-data-2.37.2. Do not edit by hand.\n\n\
         use crate::style::Style;\n",
    );
    let mut single_kinds = BTreeSet::new();

    out.push_str("/// A standard CSS property.\n");
    out.push_str("#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]\n");
    out.push_str("#[non_exhaustive]\npub enum Property {\n");
    let mut type_names: BTreeSet<String> = TYPE_NAMES.iter().map(|s| (*s).to_owned()).collect();
    for &name in properties.keys() {
        let _ = writeln!(out, "    /// `{name}`.\n    {},", camel(name));
    }
    out.push_str("}\n\nimpl Property {\n");
    out.push_str("    /// The name as written in a stylesheet.\n    #[must_use]\n");
    out.push_str("    #[expect(clippy::too_many_lines, reason = \"one arm per property\")]\n");
    out.push_str("    pub fn name(self) -> &'static str {\n        match self {\n");
    for &name in properties.keys() {
        let _ = writeln!(out, "            Self::{} => \"{name}\",", camel(name));
    }
    out.push_str("        }\n    }\n\n");
    out.push_str(
        "    /// How many shorthands stand above this property, so that a shorthand is\n    \
         /// written before the longhands it sets.\n",
    );
    out.push_str("    #[expect(clippy::too_many_lines, reason = \"one arm per longhand\")]\n");
    out.push_str("    pub(crate) fn rank(self) -> u32 {\n        match self {\n");
    let mut by_rank: BTreeMap<u32, Vec<&str>> = BTreeMap::new();
    for &name in properties.keys() {
        by_rank.entry(ranks[name]).or_default().push(name);
    }
    let last = *by_rank.keys().last().expect("properties");
    for (rank, names) in &by_rank {
        if *rank == last {
            let _ = writeln!(out, "            _ => {rank},");
        } else {
            let arms: Vec<String> = names
                .iter()
                .map(|n| format!("Self::{}", camel(n)))
                .collect();
            let _ = writeln!(
                out,
                "            {} => {rank},",
                arms.join("\n            | ")
            );
        }
    }
    out.push_str("        }\n    }\n}\n\n");

    let mut methods = String::from("impl Style {\n");
    let mut enums = String::from(
        "// @generated by tests/generate.rs from data/mdn-data-2.37.2. Do not edit by hand.\n\n\
         use std::fmt;\n\n\
         use super::{Angle, Color, Length, Percentage, Quoted, Raw, Time, Url};\n\n",
    );
    let mut method_names = BTreeSet::new();
    for (&name, value) in &properties {
        let syntax = value["syntax"].as_str().expect("syntax");
        let mut seen = vec![name.to_owned()];
        let singles = data.singles(&Parser::parse(syntax), &mut seen);
        let keywords: Vec<&str> = singles
            .atoms
            .iter()
            .filter_map(|a| match a {
                Atom::Keyword(k) => Some(k.as_str()),
                Atom::Kind(_) => None,
            })
            .collect();
        let kinds: Vec<Kind> = singles
            .atoms
            .iter()
            .filter_map(|a| match a {
                Atom::Kind(k) => Some(*k),
                Atom::Keyword(_) => None,
            })
            .collect();
        let raw = !singles.complete;
        let method = snake(name);
        check_ident(&method, "property method");
        assert!(
            !STYLE_METHODS.contains(&method.as_str()),
            "`{method}` shadows a Style method"
        );
        assert!(
            method_names.insert(method.clone()),
            "two properties map to `{method}`"
        );
        let url = value["mdn_url"].as_str().unwrap_or("");
        let unchecked = if raw {
            "\n    ///\n    /// The single-value forms are typed. Any other value goes through \
             [`Raw`](crate::value::Raw), which is passed to the browser unchecked."
        } else {
            ""
        };
        let property = camel(name);

        let arg = if keywords.is_empty() && !raw && kinds.len() == 1 {
            if !matches!(kinds[0], Kind::Number | Kind::Integer) {
                single_kinds.insert(kinds[0].type_name());
            }
            kinds[0].type_name().to_owned()
        } else {
            let ty = property.clone();
            check_ident(&ty, "value type");
            assert!(type_names.insert(ty.clone()), "value type `{ty}` collides");
            let mut variants = BTreeSet::new();
            let _ = writeln!(enums, "/// A value of [`{name}`]({url}).");
            enums.push_str("#[derive(Debug, Clone, PartialEq)]\n#[non_exhaustive]\n");
            let _ = writeln!(enums, "pub enum {ty} {{");
            for keyword in &keywords {
                let variant = camel(keyword);
                assert!(variants.insert(variant.clone()), "{ty}::{variant} twice");
                let _ = writeln!(enums, "    /// `{keyword}`.\n    {variant},");
            }
            for kind in &kinds {
                let variant = kind.variant();
                assert!(variants.insert(variant.to_owned()), "{ty}::{variant} twice");
                let _ = writeln!(
                    enums,
                    "    /// {}\n    {variant}({}),",
                    kind.doc(),
                    kind.type_name()
                );
            }
            if raw {
                assert!(variants.insert("Raw".to_owned()), "{ty}::Raw twice");
                enums.push_str(
                    "    /// Any other value, passed through unchecked.\n    Raw(Raw),\n",
                );
            }
            enums.push_str("}\n\n");
            let _ = writeln!(enums, "impl fmt::Display for {ty} {{");
            enums.push_str("    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {\n");
            enums.push_str("        match self {\n");
            for keyword in &keywords {
                let _ = writeln!(
                    enums,
                    "            Self::{} => f.write_str(\"{keyword}\"),",
                    camel(keyword)
                );
            }
            for kind in &kinds {
                let _ = writeln!(
                    enums,
                    "            Self::{}(v) => v.fmt(f),",
                    kind.variant()
                );
            }
            if raw {
                enums.push_str("            Self::Raw(v) => v.fmt(f),\n");
            }
            enums.push_str("        }\n    }\n}\n\n");
            let has_integer = kinds.contains(&Kind::Integer);
            for kind in &kinds {
                let _ = writeln!(
                    enums,
                    "impl From<{t}> for {ty} {{\n    fn from(v: {t}) -> Self {{\n        Self::{v}(v)\n    }}\n}}\n",
                    t = kind.type_name(),
                    v = kind.variant()
                );
                if *kind == Kind::Number && !has_integer {
                    let _ = writeln!(
                        enums,
                        "impl From<i32> for {ty} {{\n    fn from(v: i32) -> Self {{\n        Self::Number(f64::from(v))\n    }}\n}}\n"
                    );
                }
            }
            if raw {
                let _ = writeln!(
                    enums,
                    "impl From<Raw> for {ty} {{\n    fn from(v: Raw) -> Self {{\n        Self::Raw(v)\n    }}\n}}\n"
                );
            }
            format!("crate::value::{ty}")
        };
        let _ = writeln!(
            methods,
            "    /// Sets [`{name}`]({url}).{unchecked}\n    #[must_use]\n    \
             pub fn {method}(self, value: impl Into<{arg}>) -> Self {{\n        \
             self.with(Property::{property}, value.into().to_string())\n    }}\n"
        );
    }
    methods.push_str("}\n\n");
    let imports: Vec<&str> = single_kinds.into_iter().collect();
    let _ = writeln!(out, "use crate::value::{{{}}};\n", imports.join(", "));
    out.push_str(&methods);

    let mut classes = Vec::new();
    let mut elements = Vec::new();
    for (name, value) in &selectors {
        if !is_standard(value) || name.ends_with(')') || name.starts_with("::-") {
            continue;
        }
        if let Some(el) = name.strip_prefix("::") {
            elements.push(el.to_owned());
        } else if let Some(class) = name.strip_prefix(':')
            && !SKIPPED_PSEUDO_CLASSES.contains(&name.as_str())
        {
            classes.push(class.to_owned());
        }
    }
    for (ty, prefix, names) in [
        ("PseudoClass", ":", &classes),
        ("PseudoElement", "::", &elements),
    ] {
        let what = if prefix == ":" {
            "pseudo-class"
        } else {
            "pseudo-element"
        };
        let _ = writeln!(out, "/// A standard {what} without arguments.");
        out.push_str("#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]\n");
        let _ = writeln!(out, "#[non_exhaustive]\npub enum {ty} {{");
        for name in names {
            let _ = writeln!(out, "    /// `{prefix}{name}`.\n    {},", camel(name));
        }
        let _ = writeln!(out, "}}\n\nimpl {ty} {{");
        out.push_str("    /// The name without its colons.\n    #[must_use]\n");
        out.push_str("    pub fn name(self) -> &'static str {\n        match self {\n");
        for name in names {
            let _ = writeln!(out, "            Self::{} => \"{name}\",", camel(name));
        }
        out.push_str("        }\n    }\n}\n\n");
    }

    let named = syntaxes["named-color"]["syntax"]
        .as_str()
        .expect("named-color");
    enums.push_str("impl Color {\n");
    for color in named.split('|').map(str::trim) {
        let _ = writeln!(
            enums,
            "    /// The named color `{color}`.\n    pub const {}: Self = Self::named(\"{color}\");",
            snake(color).to_uppercase()
        );
    }
    enums.push_str("}\n");
    (out, enums)
}

#[test]
fn generated_code_matches_the_data() {
    let (properties, values) = generate();
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let overwrite = std::env::var("HYPERSIG_GENERATE").as_deref() == Ok("overwrite");
    for (file, text) in [
        ("src/generated.rs", properties),
        ("src/value/generated.rs", values),
    ] {
        let path = root.join(file);
        if overwrite {
            std::fs::write(&path, text).expect("write generated code");
            continue;
        }
        let on_disk = std::fs::read_to_string(&path).unwrap_or_default();
        assert!(
            on_disk == text,
            "{} is out of date; rerun with HYPERSIG_GENERATE=overwrite",
            path.display()
        );
    }
}
