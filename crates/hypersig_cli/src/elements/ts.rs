//! A reader of the TypeScript types that a manifest writes as text, which
//! reads enough of the grammar to map primitives, their unions and arrays,
//! and object literals of them to Rust.

/// A TypeScript type, reduced to what decides its Rust type.
#[derive(Debug, PartialEq)]
pub enum Ty {
    /// `string`, `number`, `boolean`, or a literal of one of them.
    Prim(Prim),
    /// `null`, `undefined`, or `void`.
    Nullish,
    /// `T[]` or `Array<T>`.
    Array(Box<Ty>),
    /// `A | B`.
    Union(Vec<Ty>),
    /// `{ a: A; b?: B }`.
    Object(Vec<Field>),
    /// A named type, such as `HTMLElement`, or a form this reader leaves
    /// unread, such as an intersection.
    Other,
}

/// The primitive a literal or a keyword type stands for.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Prim {
    String,
    Number,
    Boolean,
}

/// A property of an object literal type.
#[derive(Debug, PartialEq)]
pub struct Field {
    pub name: String,
    /// Written `name?:`, so the property may be missing.
    pub optional: bool,
    pub ty: Ty,
    /// The type as the manifest writes it, with each run of whitespace made one space.
    pub text: String,
}

/// The type `text` writes, or `None` where it is not a type this reader reads.
pub fn parse(text: &str) -> Option<Ty> {
    let mut parser = Parser {
        chars: text.chars().collect(),
        at: 0,
    };
    let ty = parser.union()?;
    parser.skip_space();
    (parser.at == parser.chars.len()).then_some(ty)
}

/// `text` with each run of whitespace, line breaks included, made one space.
pub fn one_line(text: &str) -> String {
    text.split_whitespace().collect::<Vec<_>>().join(" ")
}

/// The Rust type a value of `ty` deserializes into, where one exists.
///
/// A number is `f64`, because a JavaScript number may hold a fraction.
pub fn rust_type(ty: &Ty) -> Option<String> {
    match ty {
        Ty::Prim(prim) => Some(prim_type(*prim).to_owned()),
        Ty::Array(item) => rust_type(item).map(|item| format!("Vec<{item}>")),
        Ty::Union(items) => {
            let nullable = items.contains(&Ty::Nullish);
            let rest: Vec<&Ty> = items.iter().filter(|item| **item != Ty::Nullish).collect();
            let inner = match rest.as_slice() {
                [one] => rust_type(one)?,
                [Ty::Prim(first), others @ ..]
                    if others.iter().all(|other| **other == Ty::Prim(*first)) =>
                {
                    prim_type(*first).to_owned()
                }
                // No member but `null`, or members of different types.
                _ => return None,
            };
            Some(if nullable { optional(&inner) } else { inner })
        }
        Ty::Nullish | Ty::Object(_) | Ty::Other => None,
    }
}

/// `Option<inner>`, or `inner` where it is an `Option` already.
pub fn optional(inner: &str) -> String {
    if inner.starts_with("Option<") {
        inner.to_owned()
    } else {
        format!("Option<{inner}>")
    }
}

fn prim_type(prim: Prim) -> &'static str {
    match prim {
        Prim::String => "String",
        Prim::Number => "f64",
        Prim::Boolean => "bool",
    }
}

struct Parser {
    chars: Vec<char>,
    at: usize,
}

impl Parser {
    fn skip_space(&mut self) {
        while self.chars.get(self.at).is_some_and(|c| c.is_whitespace()) {
            self.at += 1;
        }
    }

    fn peek(&mut self) -> Option<char> {
        self.skip_space();
        self.chars.get(self.at).copied()
    }

    fn eat(&mut self, c: char) -> bool {
        if self.peek() == Some(c) {
            self.at += 1;
            true
        } else {
            false
        }
    }

    /// `A | B | C`, where a `|` may also come before the first member, as
    /// a type broken over lines writes it.
    fn union(&mut self) -> Option<Ty> {
        self.eat('|');
        let mut items = vec![self.intersection()?];
        while self.eat('|') {
            items.push(self.intersection()?);
        }
        Some(if items.len() == 1 {
            items.remove(0)
        } else {
            Ty::Union(items)
        })
    }

    fn intersection(&mut self) -> Option<Ty> {
        let first = self.postfix()?;
        if self.peek() != Some('&') {
            return Some(first);
        }
        while self.eat('&') {
            self.postfix()?;
        }
        Some(Ty::Other)
    }

    fn postfix(&mut self) -> Option<Ty> {
        let mut ty = self.primary()?;
        while self.peek() == Some('[') {
            self.at += 1;
            if !self.eat(']') {
                return None;
            }
            ty = Ty::Array(Box::new(ty));
        }
        Some(ty)
    }

    fn primary(&mut self) -> Option<Ty> {
        match self.peek()? {
            '(' => {
                self.at += 1;
                let ty = self.union()?;
                self.eat(')').then_some(ty)
            }
            '{' => {
                self.at += 1;
                self.object()
            }
            quote @ ('\'' | '"' | '`') => {
                self.at += 1;
                self.string(quote)?;
                Some(Ty::Prim(Prim::String))
            }
            c if c == '-' || c.is_ascii_digit() => {
                self.at += 1;
                while self
                    .chars
                    .get(self.at)
                    .is_some_and(|c| c.is_ascii_alphanumeric() || *c == '.' || *c == '_')
                {
                    self.at += 1;
                }
                Some(Ty::Prim(Prim::Number))
            }
            _ => self.named(),
        }
    }

    /// The rest of a string literal opened by `quote`.
    fn string(&mut self, quote: char) -> Option<String> {
        let mut value = String::new();
        loop {
            let c = *self.chars.get(self.at)?;
            self.at += 1;
            match c {
                '\\' => {
                    value.push(*self.chars.get(self.at)?);
                    self.at += 1;
                }
                c if c == quote => return Some(value),
                c => value.push(c),
            }
        }
    }

    fn ident(&mut self) -> Option<String> {
        self.skip_space();
        let start = self.at;
        while self
            .chars
            .get(self.at)
            .is_some_and(|c| c.is_alphanumeric() || *c == '_' || *c == '$' || *c == '.')
        {
            self.at += 1;
        }
        (self.at > start).then(|| self.chars[start..self.at].iter().collect())
    }

    fn named(&mut self) -> Option<Ty> {
        let name = self.ident()?;
        match name.as_str() {
            "readonly" | "keyof" | "typeof" | "unique" => {
                let ty = self.postfix()?;
                return Some(if name == "readonly" { ty } else { Ty::Other });
            }
            _ => {}
        }
        let mut args = Vec::new();
        if self.eat('<') {
            args.push(self.union()?);
            while self.eat(',') {
                args.push(self.union()?);
            }
            if !self.eat('>') {
                return None;
            }
        }
        Some(match (name.as_str(), args.as_mut_slice()) {
            ("string" | "String", []) => Ty::Prim(Prim::String),
            ("number" | "Number", []) => Ty::Prim(Prim::Number),
            ("boolean" | "Boolean" | "true" | "false", []) => Ty::Prim(Prim::Boolean),
            ("null" | "undefined" | "void", []) => Ty::Nullish,
            ("Array" | "ReadonlyArray", [item]) => {
                Ty::Array(Box::new(std::mem::replace(item, Ty::Other)))
            }
            _ => Ty::Other,
        })
    }

    /// The rest of an object literal type after its `{`.
    fn object(&mut self) -> Option<Ty> {
        let mut fields = Vec::new();
        loop {
            if self.eat('}') {
                return Some(Ty::Object(fields));
            }
            let mut name = match self.peek()? {
                quote @ ('\'' | '"') => {
                    self.at += 1;
                    self.string(quote)?
                }
                _ => self.ident()?,
            };
            // `readonly name: T` declares `name`.
            if name == "readonly" && !matches!(self.peek()?, ':' | '?') {
                name = self.ident()?;
            }
            let optional = self.eat('?');
            if !self.eat(':') {
                return None;
            }
            self.skip_space();
            let start = self.at;
            let ty = self.union()?;
            let text = one_line(&self.chars[start..self.at].iter().collect::<String>());
            fields.push(Field {
                name,
                optional,
                ty,
                text,
            });
            if !self.eat(';') {
                self.eat(',');
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{Prim, Ty, parse, rust_type};

    fn rust(text: &str) -> Option<String> {
        rust_type(&parse(text).unwrap_or(Ty::Other))
    }

    #[test]
    fn maps_primitives_and_their_unions() {
        assert_eq!(rust("string").as_deref(), Some("String"));
        assert_eq!(rust("number").as_deref(), Some("f64"));
        assert_eq!(rust("true | false").as_deref(), Some("bool"));
        assert_eq!(
            rust("'small' | 'medium' | 'large'").as_deref(),
            Some("String")
        );
        assert_eq!(rust("\n | 'top'\n | 'bottom'").as_deref(), Some("String"));
        assert_eq!(rust("'_self' | string").as_deref(), Some("String"));
        assert_eq!(rust("string | null").as_deref(), Some("Option<String>"));
        assert_eq!(
            rust("number | undefined | null").as_deref(),
            Some("Option<f64>")
        );
        assert_eq!(rust("number[]").as_deref(), Some("Vec<f64>"));
        assert_eq!(rust("Array<string>").as_deref(), Some("Vec<String>"));
        assert_eq!(rust("readonly string[]").as_deref(), Some("Vec<String>"));
    }

    #[test]
    fn leaves_other_types_unmapped() {
        for text in [
            "number | string",
            "HTMLElement",
            "Element | Element[]",
            "(value: number) => string",
            "A & B",
            "null",
            "{ a: string }",
            "Map<string, number>",
        ] {
            assert_eq!(rust(text), None, "{text}");
        }
    }

    #[test]
    fn reads_an_object_literal() {
        let Some(Ty::Object(fields)) =
            parse("{ phase: 'start' | 'move' | 'end', value?: number; \"x-y\": string[] }")
        else {
            panic!("an object literal");
        };
        let read: Vec<_> = fields
            .iter()
            .map(|field| (field.name.as_str(), field.optional, field.text.as_str()))
            .collect();
        assert_eq!(
            read,
            [
                ("phase", false, "'start' | 'move' | 'end'"),
                ("value", true, "number"),
                ("x-y", false, "string[]"),
            ]
        );
        assert_eq!(fields[1].ty, Ty::Prim(Prim::Number));
    }

    #[test]
    fn refuses_what_it_cannot_read() {
        for text in [
            "{ a: string",
            "string |",
            "(string",
            "{ [key: string]: number }",
        ] {
            assert_eq!(parse(text), None, "{text}");
        }
    }
}
