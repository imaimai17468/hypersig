//! Code blocks, with the Rust source colored by a small tokenizer.

use std::ops::Range;

use hypersig::html::{Node, code, div, pre, span};
use hypersig::style::value::{
    BorderBottomStyle, BorderStyle, Color, FontWeight, LineHeight, OverflowX, px, rem,
};
use hypersig::style::{Media, Style, Theme, Token};
use hypersig_ui::token;

use crate::prose::{Docs, Part};

/// The language a code block is written in.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Lang {
    Rust,
    Toml,
    Shell,
    /// HTML, shown without colors.
    Html,
}

/// The kinds of token the tokenizer colors.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Kind {
    Plain,
    Keyword,
    Type,
    Function,
    Macro,
    Text,
    Number,
    Comment,
}

macro_rules! syntax {
    ($($kind:ident => $name:literal, $light:literal, $dark:literal;)*) => {
        fn color(kind: Kind) -> Option<Token<Color>> {
            match kind {
                Kind::Plain => None,
                $(Kind::$kind => Some(Token::new($name, Color::hex($light))),)*
            }
        }

        /// The dark values of the syntax colors, for the page theme.
        pub fn theme() -> Theme {
            let dark = Theme::new()$(.set(&Token::new($name, Color::hex($light)), Color::hex($dark)))*;
            Theme::new().on(Media::Dark, dark)
        }
    };
}

// GitHub's light and dark palettes, with the comment color darkened to keep
// 4.5:1 on the code background in the light theme.
syntax! {
    Keyword => "code-keyword", 0x00cf_222e, 0x00ff_7b72;
    Type => "code-type", 0x0095_3800, 0x00ff_a657;
    Function => "code-function", 0x0082_50df, 0x00d2_a8ff;
    Macro => "code-macro", 0x0005_50ae, 0x0079_c0ff;
    Text => "code-text", 0x000a_3069, 0x00a5_d6ff;
    Number => "code-number", 0x0005_50ae, 0x0079_c0ff;
    Comment => "code-comment", 0x0057_606a, 0x008b_949e;
}

/// A code block holding `source`, under a bar naming `file` if it is given.
pub fn block(lang: Lang, file: Option<&str>, source: &str) -> Part {
    let body = pre()
        .css(
            Style::new()
                .padding(rem(1))
                .overflow_x(OverflowX::Auto)
                .font_size(rem(0.8125))
                .line_height(LineHeight::Length(rem(1.375))),
        )
        .child(code().children(highlight(lang, source.trim_end())));
    div()
        .css(
            Style::new()
                .margin_top(rem(1.5))
                .border_width(px(1))
                .border_style(BorderStyle::Solid)
                .border_color(token::border().var())
                .border_radius(token::radius().var())
                .background_color(token::muted().var())
                .overflow_x(hypersig::style::value::OverflowX::Hidden),
        )
        .children(file.map(|file| {
            div()
                .css(
                    Style::new()
                        .padding_block(rem(0.5))
                        .padding_inline(rem(1))
                        .border_bottom_width(px(1))
                        .border_bottom_style(BorderBottomStyle::Solid)
                        .border_bottom_color(token::border().var())
                        .font_size(rem(0.75))
                        .font_weight(FontWeight::Number(500.0))
                        .color(token::muted_foreground().var())
                        .font_family(hypersig::style::value::FontFamily::Monospace),
                )
                .text(file)
        }))
        .child(body)
}

/// The Rust source of a file of this repository, without its `//!` header,
/// which describes the file for its readers in the repository.
pub fn without_header(source: &str) -> &str {
    let mut rest = source;
    while let Some(line_end) = rest.find('\n') {
        let line = &rest[..line_end];
        if line.starts_with("//!") || (line.is_empty() && rest.len() < source.len()) {
            rest = &rest[line_end + 1..];
        } else {
            break;
        }
    }
    rest
}

/// `source` as text nodes and colored spans.
fn highlight(lang: Lang, source: &str) -> Vec<Node<Docs>> {
    if lang == Lang::Html {
        return vec![source.into()];
    }
    tokens(lang, source)
        .into_iter()
        .map(|(kind, text)| match color(kind) {
            None => text.into(),
            Some(token) => span()
                .css(Style::new().color(token.var()))
                .text(text)
                .into(),
        })
        .collect()
}

const KEYWORDS: &[&str] = &[
    "as", "async", "await", "break", "const", "continue", "crate", "else", "enum", "false", "fn",
    "for", "if", "impl", "in", "let", "loop", "match", "mod", "move", "mut", "pub", "ref",
    "return", "self", "Self", "static", "struct", "super", "trait", "true", "type", "unsafe",
    "use", "where", "while", "dyn",
];

/// `source` split into runs of one kind. Joined, the runs give back `source`.
fn tokens(lang: Lang, source: &str) -> Vec<(Kind, &str)> {
    let mut runs: Vec<(Kind, Range<usize>)> = Vec::new();
    let bytes = source.as_bytes();
    let mut i = 0;
    while i < bytes.len() {
        let rest = &source[i..];
        let c = bytes[i];
        let (kind, len) = if (lang == Lang::Rust && rest.starts_with("//"))
            || (lang != Lang::Rust && c == b'#')
        {
            (Kind::Comment, rest.find('\n').unwrap_or(rest.len()))
        } else if c == b'"' {
            (Kind::Text, string_len(rest))
        } else if lang == Lang::Rust && (rest.starts_with("r#\"") || rest.starts_with("r\"")) {
            let hashes = rest[1..].len() - rest[1..].trim_start_matches('#').len();
            let close = format!("\"{}", "#".repeat(hashes));
            let body = 1 + hashes + 1;
            (
                Kind::Text,
                rest[body..]
                    .find(&close)
                    .map_or(rest.len(), |end| body + end + close.len()),
            )
        } else if lang == Lang::Rust && c == b'\'' {
            char_or_lifetime(rest)
        } else if lang == Lang::Rust && rest.starts_with("#[") {
            (
                Kind::Macro,
                rest.find(']').map_or(rest.len(), |end| end + 1),
            )
        } else if c.is_ascii_digit() {
            let len = rest
                .find(|c: char| !(c.is_ascii_alphanumeric() || c == '_' || c == '.'))
                .unwrap_or(rest.len());
            (Kind::Number, len)
        } else if c.is_ascii_alphabetic() || c == b'_' {
            let len = rest
                .find(|c: char| !(c.is_ascii_alphanumeric() || c == '_'))
                .unwrap_or(rest.len());
            (word_kind(lang, &rest[..len], &rest[len..]), len)
        } else {
            (Kind::Plain, rest.chars().next().map_or(1, char::len_utf8))
        };
        let len = if kind == Kind::Macro && rest[len..].starts_with('!') {
            len + 1
        } else {
            len
        };
        match runs.last_mut() {
            Some((Kind::Plain, run)) if kind == Kind::Plain => run.end = i + len,
            _ => runs.push((kind, i..i + len)),
        }
        i += len;
    }
    runs.into_iter()
        .map(|(kind, run)| (kind, &source[run]))
        .collect()
}

/// The kind of the word `word`, followed in the source by `after`.
fn word_kind(lang: Lang, word: &str, after: &str) -> Kind {
    if lang == Lang::Toml {
        return if after.trim_start().starts_with('=') {
            Kind::Function
        } else {
            Kind::Plain
        };
    }
    if lang == Lang::Shell {
        return Kind::Plain;
    }
    if after.starts_with('!') && !after.starts_with("!=") {
        Kind::Macro
    } else if KEYWORDS.contains(&word) {
        Kind::Keyword
    } else if word.starts_with(|c: char| c.is_ascii_uppercase()) {
        Kind::Type
    } else if after.starts_with('(') || after.starts_with("::<") {
        Kind::Function
    } else {
        Kind::Plain
    }
}

/// The length of the string literal at the start of `text`, quotes included.
fn string_len(text: &str) -> usize {
    let mut escaped = false;
    for (at, c) in text.char_indices().skip(1) {
        match c {
            _ if escaped => escaped = false,
            '\\' => escaped = true,
            '"' => return at + 1,
            _ => {}
        }
    }
    text.len()
}

/// A character literal such as `'a'`, or a lifetime such as `'static`, at
/// the start of `text`.
fn char_or_lifetime(text: &str) -> (Kind, usize) {
    let mut chars = text.char_indices().skip(1);
    match (chars.next(), chars.next()) {
        (Some((_, '\\')), _) => (
            Kind::Text,
            text[2..].find('\'').map_or(text.len(), |end| end + 3),
        ),
        (Some(_), Some((at, '\''))) => (Kind::Text, at + 1),
        _ => {
            let len = text[1..]
                .find(|c: char| !(c.is_ascii_alphanumeric() || c == '_'))
                .map_or(text.len(), |end| end + 1);
            (Kind::Type, len)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn kinds(lang: Lang, source: &str) -> Vec<(Kind, &str)> {
        tokens(lang, source)
            .into_iter()
            .filter(|(kind, _)| *kind != Kind::Plain)
            .collect()
    }

    #[test]
    fn the_runs_give_back_the_source() {
        let source = include_str!("demo/counter.rs");
        let joined: String = tokens(Lang::Rust, source)
            .into_iter()
            .map(|(_, text)| text)
            .collect();
        assert_eq!(joined, source);
    }

    #[test]
    fn rust_words_are_told_apart() {
        assert_eq!(
            kinds(
                Lang::Rust,
                r#"let page: Page<S> = vec![1]; f("a\"b") // done"#
            ),
            [
                (Kind::Keyword, "let"),
                (Kind::Type, "Page"),
                (Kind::Type, "S"),
                (Kind::Macro, "vec!"),
                (Kind::Number, "1"),
                (Kind::Function, "f"),
                (Kind::Text, r#""a\"b""#),
                (Kind::Comment, "// done"),
            ]
        );
    }

    #[test]
    fn a_lifetime_is_not_a_character() {
        assert_eq!(
            kinds(Lang::Rust, "&'static str, 'x'"),
            [(Kind::Type, "'static"), (Kind::Text, "'x'")]
        );
    }

    #[test]
    fn a_raw_string_ends_at_its_hashes() {
        assert_eq!(
            kinds(Lang::Rust, r##"r#"a "b" c"# x"##),
            [(Kind::Text, r##"r#"a "b" c"#"##)]
        );
    }

    #[test]
    fn the_header_comment_is_left_out() {
        assert_eq!(
            without_header("//! A demo.\n//! More.\n\nuse a;\n"),
            "use a;\n"
        );
        assert_eq!(without_header("use a;\n"), "use a;\n");
    }
}
