//! The signals of a page as one flat list of nodes, which the server-side
//! evaluator in `ssr.rs` reads in place of a `serde_json::Value` tree.
//!
//! A `Value` built by `serde_json::to_value` allocates a map, a key, and a
//! string for every object and string in the signals, and frees them all
//! after the page is written. A `Tape` holds the same JSON in two buffers:
//! the nodes in document order, and the text of every string and key.
//! `TapeSerializer` follows the rules of `serde_json`'s value serializer, so
//! a value reads the same from a `Tape` as from the `Value` it replaces,
//! except that an object keeps its fields in the order they were
//! serialized, where a `Value` sorts them.

use std::fmt::Display;

use serde::Serialize;
use serde::ser::{
    self, Impossible, SerializeMap, SerializeSeq, SerializeStruct, SerializeStructVariant,
    SerializeTuple, SerializeTupleStruct, SerializeTupleVariant,
};
use serde_json::{Number, Value};

use crate::exact;

static NULL: Value = Value::Null;

/// One node of a `Tape`. An array is followed by its items, and an object by
/// a key and a value for each of its entries. `end` is the index just past
/// the last node of the array or object.
enum Node {
    Null,
    Bool(bool),
    Number(Number),
    Str {
        start: usize,
        end: usize,
    },
    Array {
        len: usize,
        end: usize,
    },
    Object {
        len: usize,
        end: usize,
        /// Whether no key appears twice, so a lookup stops at the first match.
        unique: bool,
    },
}

/// Which of the two compound nodes `Tape::open` pushes.
#[derive(Clone, Copy)]
enum Container {
    Array,
    Object,
}

/// The most entries `Tape::close` compares pairwise to find a repeated key.
/// A larger object is read as if it may repeat one.
const UNIQUE_CHECK_LIMIT: usize = 16;

/// A JSON value written as a flat list of nodes.
pub(crate) struct Tape {
    nodes: Vec<Node>,
    text: String,
}

impl Tape {
    /// The JSON that `serde_json::to_value(value)` would build, or an error
    /// for an integer outside -2^53 to 2^53, which the browser would round.
    pub(crate) fn new(value: &impl Serialize) -> Result<Tape, serde_json::Error> {
        let mut tape = Tape {
            nodes: Vec::new(),
            text: String::new(),
        };
        value.serialize(TapeSerializer { tape: &mut tape })?;
        Ok(tape)
    }

    pub(crate) fn root(&self) -> Ref<'_> {
        Ref::Tape(self, 0)
    }

    /// A copy with the value at the dotted `path` set to `next`, as
    /// `ssr::set_path` sets it in a `Value`.
    pub(crate) fn replaced(&self, path: &str, next: &Value) -> Tape {
        let path: Vec<&str> = if path.is_empty() {
            Vec::new()
        } else {
            path.split('.').collect()
        };
        Tape::new(&Replaced {
            at: self.root(),
            path: &path,
            next,
        })
        .expect("a tape and a JSON value serialize")
    }

    fn str(&self, start: usize, end: usize) -> &str {
        &self.text[start..end]
    }

    /// The index just past the node at `index` and everything inside it.
    fn skip(&self, index: usize) -> usize {
        match self.nodes[index] {
            Node::Array { end, .. } | Node::Object { end, .. } => end,
            _ => index + 1,
        }
    }

    fn push_str(&mut self, text: &str) {
        let start = self.text.len();
        self.text.push_str(text);
        self.nodes.push(Node::Str {
            start,
            end: self.text.len(),
        });
    }

    /// Pushes the head of an array or an object, whose `len` and `end`
    /// `close` fills in.
    fn open(&mut self, container: Container) -> usize {
        self.nodes.push(match container {
            Container::Object => Node::Object {
                len: 0,
                end: 0,
                unique: false,
            },
            Container::Array => Node::Array { len: 0, end: 0 },
        });
        self.nodes.len() - 1
    }

    fn close(&mut self, head: usize, count: usize) {
        let end = self.nodes.len();
        let distinct = matches!(self.nodes[head], Node::Object { .. })
            && count <= UNIQUE_CHECK_LIMIT
            && self.keys_distinct(head, count);
        match &mut self.nodes[head] {
            Node::Array { len, end: at } => {
                *len = count;
                *at = end;
            }
            Node::Object {
                len,
                end: at,
                unique,
            } => {
                *len = count;
                *at = end;
                *unique = distinct;
            }
            _ => unreachable!("`open` pushed an array or an object at `head`"),
        }
    }

    /// The value of the entry for `key` in the object at `head`, which has
    /// `len` entries.
    fn field(&self, head: usize, len: usize, unique: bool, key: &str) -> Option<Ref<'_>> {
        // A key that appears twice reads as its last value, as
        // `JSON.parse` and `serde_json::Map` both keep it.
        let key = key.as_bytes();
        let mut found = None;
        let mut next = head + 1;
        for _ in 0..len {
            // Keys are short, so comparing them byte by byte here is faster
            // than the call to `memcmp` that slice equality makes.
            if let Node::Str { start, end } = self.nodes[next]
                && end - start == key.len()
                && self.text.as_bytes()[start..end]
                    .iter()
                    .zip(key)
                    .all(|(a, b)| a == b)
            {
                found = Some(Ref::Tape(self, next + 1));
                if unique {
                    break;
                }
            }
            next = self.skip(next + 1);
        }
        found
    }

    /// The text of the key node at `index`.
    fn key(&self, index: usize) -> &[u8] {
        let Node::Str { start, end } = self.nodes[index] else {
            unreachable!("`TapeSerializer` writes a string key before each value");
        };
        &self.text.as_bytes()[start..end]
    }

    /// Whether the `count` keys of the object at `head` differ from each other.
    fn keys_distinct(&self, head: usize, count: usize) -> bool {
        let mut keys = [0; UNIQUE_CHECK_LIMIT];
        let mut next = head + 1;
        for key in keys.iter_mut().take(count) {
            *key = next;
            next = self.skip(next + 1);
        }
        let keys = &keys[..count];
        keys.iter().enumerate().all(|(at, &key)| {
            keys[at + 1..]
                .iter()
                .all(|&other| self.key(key) != self.key(other))
        })
    }
}

/// A JSON value the evaluator reads: a `serde_json::Value`, or a node of a `Tape`.
#[derive(Clone, Copy)]
pub(crate) enum Ref<'a> {
    Json(&'a Value),
    Tape(&'a Tape, usize),
}

impl<'a> From<&'a Value> for Ref<'a> {
    fn from(value: &'a Value) -> Self {
        Ref::Json(value)
    }
}

/// What a `Ref` holds, with an array's or an object's contents left to
/// `Ref::items` and `Ref::entries`.
pub(crate) enum View<'a> {
    Null,
    Bool(bool),
    Number(&'a Number),
    Str(&'a str),
    Array,
    Object,
}

impl<'a> Ref<'a> {
    pub(crate) const NULL: Ref<'static> = Ref::Json(&NULL);

    pub(crate) fn view(self) -> View<'a> {
        match self {
            Ref::Json(value) => match value {
                Value::Null => View::Null,
                Value::Bool(b) => View::Bool(*b),
                Value::Number(n) => View::Number(n),
                Value::String(s) => View::Str(s),
                Value::Array(_) => View::Array,
                Value::Object(_) => View::Object,
            },
            Ref::Tape(tape, index) => match &tape.nodes[index] {
                Node::Null => View::Null,
                Node::Bool(b) => View::Bool(*b),
                Node::Number(n) => View::Number(n),
                Node::Str { start, end } => View::Str(tape.str(*start, *end)),
                Node::Array { .. } => View::Array,
                Node::Object { .. } => View::Object,
            },
        }
    }

    pub(crate) fn is_null(self) -> bool {
        matches!(self.view(), View::Null)
    }

    pub(crate) fn as_f64(self) -> Option<f64> {
        match self.view() {
            View::Number(n) => n.as_f64(),
            _ => None,
        }
    }

    pub(crate) fn as_i64(self) -> Option<i64> {
        match self.view() {
            View::Number(n) => n.as_i64(),
            _ => None,
        }
    }

    pub(crate) fn as_str(self) -> Option<&'a str> {
        match self.view() {
            View::Str(s) => Some(s),
            _ => None,
        }
    }

    /// The items of an array, or `None` for any other value.
    pub(crate) fn items(self) -> Option<Items<'a>> {
        match self {
            Ref::Json(value) => Some(Items::Json(value.as_array()?.iter())),
            Ref::Tape(tape, index) => match tape.nodes[index] {
                Node::Array { len, .. } => Some(Items::Tape {
                    tape,
                    next: index + 1,
                    left: len,
                }),
                _ => None,
            },
        }
    }

    /// The entries of an object, or `None` for any other value.
    pub(crate) fn entries(self) -> Option<Entries<'a>> {
        match self {
            Ref::Json(value) => Some(Entries::Json(value.as_object()?.iter())),
            Ref::Tape(tape, index) => match tape.nodes[index] {
                Node::Object { len, .. } => Some(Entries::Tape {
                    tape,
                    next: index + 1,
                    left: len,
                }),
                _ => None,
            },
        }
    }

    /// The number of items of an array or entries of an object.
    pub(crate) fn len(self) -> Option<usize> {
        match self {
            Ref::Json(Value::Array(items)) => Some(items.len()),
            Ref::Json(Value::Object(map)) => Some(map.len()),
            Ref::Tape(tape, index) => match tape.nodes[index] {
                Node::Array { len, .. } | Node::Object { len, .. } => Some(len),
                _ => None,
            },
            Ref::Json(_) => None,
        }
    }

    /// The value of an object's entry for `key`.
    pub(crate) fn field(self, key: &str) -> Option<Ref<'a>> {
        match self {
            Ref::Json(value) => value.as_object()?.get(key).map(Ref::Json),
            Ref::Tape(tape, index) => match tape.nodes[index] {
                Node::Object { len, unique, .. } => tape.field(index, len, unique, key),
                _ => None,
            },
        }
    }

    /// The value at a dotted path of object keys and array indices.
    pub(crate) fn get_path(self, path: &str) -> Option<Ref<'a>> {
        if path.is_empty() {
            return Some(self);
        }
        let mut current = self;
        let mut rest = path;
        while let Some(dot) = rest.bytes().position(|byte| byte == b'.') {
            current = current.child(&rest[..dot])?;
            rest = &rest[dot + 1..];
        }
        current.child(rest)
    }

    /// An object's entry for `segment`, or an array's item at the index `segment` names.
    fn child(self, segment: &str) -> Option<Ref<'a>> {
        match self {
            Ref::Json(Value::Object(map)) => map.get(segment).map(Ref::Json),
            Ref::Json(Value::Array(items)) => {
                items.get(segment.parse::<usize>().ok()?).map(Ref::Json)
            }
            Ref::Json(_) => None,
            Ref::Tape(tape, index) => match tape.nodes[index] {
                Node::Object { len, unique, .. } => tape.field(index, len, unique, segment),
                Node::Array { len, .. } => {
                    let at = segment.parse::<usize>().ok().filter(|at| *at < len)?;
                    let next = (0..at).fold(index + 1, |next, _| tape.skip(next));
                    Some(Ref::Tape(tape, next))
                }
                _ => None,
            },
        }
    }

    pub(crate) fn to_value(self) -> Value {
        match self {
            Ref::Json(value) => value.clone(),
            Ref::Tape(..) => serde_json::to_value(self).expect("a tape serializes"),
        }
    }
}

impl Serialize for Ref<'_> {
    fn serialize<S: ser::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self.view() {
            View::Null => serializer.serialize_unit(),
            View::Bool(b) => serializer.serialize_bool(b),
            View::Number(n) => n.serialize(serializer),
            View::Str(s) => serializer.serialize_str(s),
            View::Array => serializer.collect_seq(self.items().into_iter().flatten()),
            View::Object => serializer.collect_map(self.entries().into_iter().flatten()),
        }
    }
}

pub(crate) enum Items<'a> {
    Json(std::slice::Iter<'a, Value>),
    Tape {
        tape: &'a Tape,
        next: usize,
        left: usize,
    },
}

impl<'a> Iterator for Items<'a> {
    type Item = Ref<'a>;

    fn next(&mut self) -> Option<Ref<'a>> {
        match self {
            Items::Json(items) => items.next().map(Ref::Json),
            Items::Tape { tape, next, left } => {
                if *left == 0 {
                    return None;
                }
                let item = Ref::Tape(tape, *next);
                *next = tape.skip(*next);
                *left -= 1;
                Some(item)
            }
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        match self {
            Items::Json(items) => items.size_hint(),
            Items::Tape { left, .. } => (*left, Some(*left)),
        }
    }
}

pub(crate) enum Entries<'a> {
    Json(serde_json::map::Iter<'a>),
    Tape {
        tape: &'a Tape,
        next: usize,
        left: usize,
    },
}

impl<'a> Iterator for Entries<'a> {
    type Item = (&'a str, Ref<'a>);

    fn next(&mut self) -> Option<(&'a str, Ref<'a>)> {
        match self {
            Entries::Json(entries) => entries
                .next()
                .map(|(key, value)| (key.as_str(), Ref::Json(value))),
            Entries::Tape { tape, next, left } => {
                if *left == 0 {
                    return None;
                }
                let Node::Str { start, end } = tape.nodes[*next] else {
                    unreachable!("`TapeSerializer` writes a string key before each value");
                };
                let value = Ref::Tape(tape, *next + 1);
                *next = tape.skip(*next + 1);
                *left -= 1;
                Some((tape.str(start, end), value))
            }
        }
    }
}

/// The value at `at` with the value at `path` inside it replaced by `next`,
/// written by `Serialize` so that `Tape::new` copies it into a new tape.
struct Replaced<'a> {
    at: Ref<'a>,
    path: &'a [&'a str],
    next: &'a Value,
}

impl Serialize for Replaced<'_> {
    fn serialize<S: ser::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let Some((head, rest)) = self.path.split_first() else {
            return self.next.serialize(serializer);
        };
        let inner = |at| Replaced {
            at,
            path: rest,
            next: self.next,
        };
        match self.at.view() {
            View::Array => {
                let items = self.at.items().into_iter().flatten();
                match head.parse::<usize>() {
                    Ok(index) if index < self.at.items().map_or(0, Iterator::count) => {
                        let mut seq = serializer.serialize_seq(None)?;
                        for (at, item) in items.enumerate() {
                            if at == index {
                                seq.serialize_element(&inner(item))?;
                            } else {
                                seq.serialize_element(&item)?;
                            }
                        }
                        seq.end()
                    }
                    _ => serializer.collect_seq(items),
                }
            }
            View::Object => {
                let mut map = serializer.serialize_map(None)?;
                let mut found = false;
                for (key, value) in self.at.entries().into_iter().flatten() {
                    if key == *head {
                        found = true;
                        map.serialize_entry(key, &inner(value))?;
                    } else {
                        map.serialize_entry(key, &value)?;
                    }
                }
                if !found {
                    map.serialize_entry(head, &inner(Ref::NULL))?;
                }
                map.end()
            }
            _ => {
                let mut map = serializer.serialize_map(Some(1))?;
                map.serialize_entry(head, &inner(Ref::NULL))?;
                map.end()
            }
        }
    }
}

fn error(message: impl Display) -> serde_json::Error {
    ser::Error::custom(message)
}

/// Writes a value into a `Tape` as `serde_json::value::Serializer` builds a `Value`.
struct TapeSerializer<'t> {
    tape: &'t mut Tape,
}

/// An array or an object being written, whose head `close` completes.
struct Compound<'t> {
    tape: &'t mut Tape,
    head: usize,
    count: usize,
    /// The head of the object that wraps an enum variant's contents, closed after `head`.
    variant: Option<usize>,
}

impl<'t> Compound<'t> {
    fn new(tape: &'t mut Tape, container: Container, variant: Option<&'static str>) -> Self {
        let variant = variant.map(|name| {
            let head = tape.open(Container::Object);
            tape.push_str(name);
            head
        });
        let head = tape.open(container);
        Compound {
            tape,
            head,
            count: 0,
            variant,
        }
    }

    fn value<T: ?Sized + Serialize>(&mut self, value: &T) -> Result<(), serde_json::Error> {
        value.serialize(TapeSerializer { tape: self.tape })
    }

    fn finish(self) {
        self.tape.close(self.head, self.count);
        if let Some(head) = self.variant {
            self.tape.close(head, 1);
        }
    }
}

impl<'t> ser::Serializer for TapeSerializer<'t> {
    type Ok = ();
    type Error = serde_json::Error;
    type SerializeSeq = Compound<'t>;
    type SerializeTuple = Compound<'t>;
    type SerializeTupleStruct = Compound<'t>;
    type SerializeTupleVariant = Compound<'t>;
    type SerializeMap = Compound<'t>;
    type SerializeStruct = Compound<'t>;
    type SerializeStructVariant = Compound<'t>;

    fn serialize_bool(self, v: bool) -> Result<(), serde_json::Error> {
        self.tape.nodes.push(Node::Bool(v));
        Ok(())
    }

    fn serialize_i8(self, v: i8) -> Result<(), serde_json::Error> {
        self.serialize_i64(v.into())
    }

    fn serialize_i16(self, v: i16) -> Result<(), serde_json::Error> {
        self.serialize_i64(v.into())
    }

    fn serialize_i32(self, v: i32) -> Result<(), serde_json::Error> {
        self.serialize_i64(v.into())
    }

    fn serialize_i64(self, v: i64) -> Result<(), serde_json::Error> {
        if !exact::holds_i64(v) {
            return Err(error(exact::message(v)));
        }
        self.tape.nodes.push(Node::Number(v.into()));
        Ok(())
    }

    fn serialize_i128(self, v: i128) -> Result<(), serde_json::Error> {
        if let Ok(v) = u64::try_from(v) {
            self.serialize_u64(v)
        } else if let Ok(v) = i64::try_from(v) {
            self.serialize_i64(v)
        } else {
            Err(error("number out of range"))
        }
    }

    fn serialize_u8(self, v: u8) -> Result<(), serde_json::Error> {
        self.serialize_u64(v.into())
    }

    fn serialize_u16(self, v: u16) -> Result<(), serde_json::Error> {
        self.serialize_u64(v.into())
    }

    fn serialize_u32(self, v: u32) -> Result<(), serde_json::Error> {
        self.serialize_u64(v.into())
    }

    fn serialize_u64(self, v: u64) -> Result<(), serde_json::Error> {
        if !exact::holds_u64(v) {
            return Err(error(exact::message(v)));
        }
        self.tape.nodes.push(Node::Number(v.into()));
        Ok(())
    }

    fn serialize_u128(self, v: u128) -> Result<(), serde_json::Error> {
        let v = u64::try_from(v).map_err(error)?;
        self.serialize_u64(v)
    }

    fn serialize_f32(self, v: f32) -> Result<(), serde_json::Error> {
        self.tape.nodes.push(match Value::from(v) {
            Value::Number(n) => Node::Number(n),
            _ => Node::Null,
        });
        Ok(())
    }

    fn serialize_f64(self, v: f64) -> Result<(), serde_json::Error> {
        self.tape.nodes.push(match Value::from(v) {
            Value::Number(n) => Node::Number(n),
            _ => Node::Null,
        });
        Ok(())
    }

    fn serialize_char(self, v: char) -> Result<(), serde_json::Error> {
        self.tape.push_str(v.encode_utf8(&mut [0; 4]));
        Ok(())
    }

    fn serialize_str(self, v: &str) -> Result<(), serde_json::Error> {
        self.tape.push_str(v);
        Ok(())
    }

    fn serialize_bytes(self, v: &[u8]) -> Result<(), serde_json::Error> {
        let head = self.tape.open(Container::Array);
        for byte in v {
            self.tape.nodes.push(Node::Number((*byte).into()));
        }
        self.tape.close(head, v.len());
        Ok(())
    }

    fn serialize_none(self) -> Result<(), serde_json::Error> {
        self.serialize_unit()
    }

    fn serialize_some<T: ?Sized + Serialize>(self, value: &T) -> Result<(), serde_json::Error> {
        value.serialize(self)
    }

    fn serialize_unit(self) -> Result<(), serde_json::Error> {
        self.tape.nodes.push(Node::Null);
        Ok(())
    }

    fn serialize_unit_struct(self, _name: &'static str) -> Result<(), serde_json::Error> {
        self.serialize_unit()
    }

    fn serialize_unit_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
    ) -> Result<(), serde_json::Error> {
        self.serialize_str(variant)
    }

    fn serialize_newtype_struct<T: ?Sized + Serialize>(
        self,
        _name: &'static str,
        value: &T,
    ) -> Result<(), serde_json::Error> {
        value.serialize(self)
    }

    fn serialize_newtype_variant<T: ?Sized + Serialize>(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
        value: &T,
    ) -> Result<(), serde_json::Error> {
        let head = self.tape.open(Container::Object);
        self.tape.push_str(variant);
        value.serialize(TapeSerializer { tape: self.tape })?;
        self.tape.close(head, 1);
        Ok(())
    }

    fn serialize_seq(self, _len: Option<usize>) -> Result<Compound<'t>, serde_json::Error> {
        Ok(Compound::new(self.tape, Container::Array, None))
    }

    fn serialize_tuple(self, _len: usize) -> Result<Compound<'t>, serde_json::Error> {
        Ok(Compound::new(self.tape, Container::Array, None))
    }

    fn serialize_tuple_struct(
        self,
        _name: &'static str,
        _len: usize,
    ) -> Result<Compound<'t>, serde_json::Error> {
        Ok(Compound::new(self.tape, Container::Array, None))
    }

    fn serialize_tuple_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
        _len: usize,
    ) -> Result<Compound<'t>, serde_json::Error> {
        Ok(Compound::new(self.tape, Container::Array, Some(variant)))
    }

    fn serialize_map(self, _len: Option<usize>) -> Result<Compound<'t>, serde_json::Error> {
        Ok(Compound::new(self.tape, Container::Object, None))
    }

    fn serialize_struct(
        self,
        _name: &'static str,
        _len: usize,
    ) -> Result<Compound<'t>, serde_json::Error> {
        Ok(Compound::new(self.tape, Container::Object, None))
    }

    fn serialize_struct_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
        _len: usize,
    ) -> Result<Compound<'t>, serde_json::Error> {
        Ok(Compound::new(self.tape, Container::Object, Some(variant)))
    }
}

impl SerializeSeq for Compound<'_> {
    type Ok = ();
    type Error = serde_json::Error;

    fn serialize_element<T: ?Sized + Serialize>(
        &mut self,
        value: &T,
    ) -> Result<(), serde_json::Error> {
        self.count += 1;
        self.value(value)
    }

    fn end(self) -> Result<(), serde_json::Error> {
        self.finish();
        Ok(())
    }
}

impl SerializeTuple for Compound<'_> {
    type Ok = ();
    type Error = serde_json::Error;

    fn serialize_element<T: ?Sized + Serialize>(
        &mut self,
        value: &T,
    ) -> Result<(), serde_json::Error> {
        SerializeSeq::serialize_element(self, value)
    }

    fn end(self) -> Result<(), serde_json::Error> {
        self.finish();
        Ok(())
    }
}

impl SerializeTupleStruct for Compound<'_> {
    type Ok = ();
    type Error = serde_json::Error;

    fn serialize_field<T: ?Sized + Serialize>(
        &mut self,
        value: &T,
    ) -> Result<(), serde_json::Error> {
        SerializeSeq::serialize_element(self, value)
    }

    fn end(self) -> Result<(), serde_json::Error> {
        self.finish();
        Ok(())
    }
}

impl SerializeTupleVariant for Compound<'_> {
    type Ok = ();
    type Error = serde_json::Error;

    fn serialize_field<T: ?Sized + Serialize>(
        &mut self,
        value: &T,
    ) -> Result<(), serde_json::Error> {
        SerializeSeq::serialize_element(self, value)
    }

    fn end(self) -> Result<(), serde_json::Error> {
        self.finish();
        Ok(())
    }
}

impl SerializeMap for Compound<'_> {
    type Ok = ();
    type Error = serde_json::Error;

    fn serialize_key<T: ?Sized + Serialize>(&mut self, key: &T) -> Result<(), serde_json::Error> {
        self.count += 1;
        key.serialize(KeySerializer { tape: self.tape })
    }

    fn serialize_value<T: ?Sized + Serialize>(
        &mut self,
        value: &T,
    ) -> Result<(), serde_json::Error> {
        self.value(value)
    }

    fn end(self) -> Result<(), serde_json::Error> {
        self.finish();
        Ok(())
    }
}

impl SerializeStruct for Compound<'_> {
    type Ok = ();
    type Error = serde_json::Error;

    fn serialize_field<T: ?Sized + Serialize>(
        &mut self,
        key: &'static str,
        value: &T,
    ) -> Result<(), serde_json::Error> {
        self.count += 1;
        self.tape.push_str(key);
        self.value(value)
    }

    fn end(self) -> Result<(), serde_json::Error> {
        self.finish();
        Ok(())
    }
}

impl SerializeStructVariant for Compound<'_> {
    type Ok = ();
    type Error = serde_json::Error;

    fn serialize_field<T: ?Sized + Serialize>(
        &mut self,
        key: &'static str,
        value: &T,
    ) -> Result<(), serde_json::Error> {
        SerializeStruct::serialize_field(self, key, value)
    }

    fn end(self) -> Result<(), serde_json::Error> {
        self.finish();
        Ok(())
    }
}

/// Writes a map key as `serde_json`'s value serializer turns it into a
/// string: a string or a char as itself, a unit variant as its name, and a
/// bool or a number as the text `serde_json` writes for it.
struct KeySerializer<'t> {
    tape: &'t mut Tape,
}

impl KeySerializer<'_> {
    /// A number as `serde_json` writes it, which is also how it writes the
    /// number as a key.
    fn number(self, value: &impl Serialize) -> Result<(), serde_json::Error> {
        let text = serde_json::to_string(value)?;
        self.tape.push_str(&text);
        Ok(())
    }
}

fn key_must_be_a_string() -> serde_json::Error {
    error("key must be a string")
}

impl ser::Serializer for KeySerializer<'_> {
    type Ok = ();
    type Error = serde_json::Error;
    type SerializeSeq = Impossible<(), serde_json::Error>;
    type SerializeTuple = Impossible<(), serde_json::Error>;
    type SerializeTupleStruct = Impossible<(), serde_json::Error>;
    type SerializeTupleVariant = Impossible<(), serde_json::Error>;
    type SerializeMap = Impossible<(), serde_json::Error>;
    type SerializeStruct = Impossible<(), serde_json::Error>;
    type SerializeStructVariant = Impossible<(), serde_json::Error>;

    fn serialize_bool(self, v: bool) -> Result<(), serde_json::Error> {
        self.tape.push_str(if v { "true" } else { "false" });
        Ok(())
    }

    fn serialize_i8(self, v: i8) -> Result<(), serde_json::Error> {
        self.number(&v)
    }

    fn serialize_i16(self, v: i16) -> Result<(), serde_json::Error> {
        self.number(&v)
    }

    fn serialize_i32(self, v: i32) -> Result<(), serde_json::Error> {
        self.number(&v)
    }

    fn serialize_i64(self, v: i64) -> Result<(), serde_json::Error> {
        self.number(&v)
    }

    fn serialize_i128(self, v: i128) -> Result<(), serde_json::Error> {
        self.number(&v)
    }

    fn serialize_u8(self, v: u8) -> Result<(), serde_json::Error> {
        self.number(&v)
    }

    fn serialize_u16(self, v: u16) -> Result<(), serde_json::Error> {
        self.number(&v)
    }

    fn serialize_u32(self, v: u32) -> Result<(), serde_json::Error> {
        self.number(&v)
    }

    fn serialize_u64(self, v: u64) -> Result<(), serde_json::Error> {
        self.number(&v)
    }

    fn serialize_u128(self, v: u128) -> Result<(), serde_json::Error> {
        self.number(&v)
    }

    fn serialize_f32(self, v: f32) -> Result<(), serde_json::Error> {
        if v.is_finite() {
            self.number(&v)
        } else {
            Err(error("float key must be finite (got NaN or +/-inf)"))
        }
    }

    fn serialize_f64(self, v: f64) -> Result<(), serde_json::Error> {
        if v.is_finite() {
            self.number(&v)
        } else {
            Err(error("float key must be finite (got NaN or +/-inf)"))
        }
    }

    fn serialize_char(self, v: char) -> Result<(), serde_json::Error> {
        self.tape.push_str(v.encode_utf8(&mut [0; 4]));
        Ok(())
    }

    fn serialize_str(self, v: &str) -> Result<(), serde_json::Error> {
        self.tape.push_str(v);
        Ok(())
    }

    fn serialize_bytes(self, _value: &[u8]) -> Result<(), serde_json::Error> {
        Err(key_must_be_a_string())
    }

    fn serialize_none(self) -> Result<(), serde_json::Error> {
        Err(key_must_be_a_string())
    }

    fn serialize_some<T: ?Sized + Serialize>(self, _value: &T) -> Result<(), serde_json::Error> {
        Err(key_must_be_a_string())
    }

    fn serialize_unit(self) -> Result<(), serde_json::Error> {
        Err(key_must_be_a_string())
    }

    fn serialize_unit_struct(self, _name: &'static str) -> Result<(), serde_json::Error> {
        Err(key_must_be_a_string())
    }

    fn serialize_unit_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
    ) -> Result<(), serde_json::Error> {
        self.tape.push_str(variant);
        Ok(())
    }

    fn serialize_newtype_struct<T: ?Sized + Serialize>(
        self,
        _name: &'static str,
        value: &T,
    ) -> Result<(), serde_json::Error> {
        value.serialize(self)
    }

    fn serialize_newtype_variant<T: ?Sized + Serialize>(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _value: &T,
    ) -> Result<(), serde_json::Error> {
        Err(key_must_be_a_string())
    }

    fn serialize_seq(
        self,
        _len: Option<usize>,
    ) -> Result<Impossible<(), serde_json::Error>, serde_json::Error> {
        Err(key_must_be_a_string())
    }

    fn serialize_tuple(
        self,
        _len: usize,
    ) -> Result<Impossible<(), serde_json::Error>, serde_json::Error> {
        Err(key_must_be_a_string())
    }

    fn serialize_tuple_struct(
        self,
        _name: &'static str,
        _len: usize,
    ) -> Result<Impossible<(), serde_json::Error>, serde_json::Error> {
        Err(key_must_be_a_string())
    }

    fn serialize_tuple_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> Result<Impossible<(), serde_json::Error>, serde_json::Error> {
        Err(key_must_be_a_string())
    }

    fn serialize_map(
        self,
        _len: Option<usize>,
    ) -> Result<Impossible<(), serde_json::Error>, serde_json::Error> {
        Err(key_must_be_a_string())
    }

    fn serialize_struct(
        self,
        _name: &'static str,
        _len: usize,
    ) -> Result<Impossible<(), serde_json::Error>, serde_json::Error> {
        Err(key_must_be_a_string())
    }

    fn serialize_struct_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> Result<Impossible<(), serde_json::Error>, serde_json::Error> {
        Err(key_must_be_a_string())
    }
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeMap;

    use serde::Serialize;
    use serde_json::{Value, json};

    use super::Tape;

    #[derive(Serialize)]
    enum Shape {
        Unit,
        Newtype(i32),
        Tuple(i32, String),
        Struct { side: f64 },
    }

    #[derive(Serialize)]
    struct Everything {
        flag: bool,
        small: i8,
        big: u64,
        negative: i64,
        wide: i128,
        float: f64,
        nan: f64,
        single: f32,
        letter: char,
        text: String,
        bytes: serde_bytes_like::Bytes,
        none: Option<i32>,
        some: Option<i32>,
        unit: (),
        tuple: (i32, &'static str),
        shapes: Vec<Shape>,
        by_number: BTreeMap<i32, &'static str>,
        by_bool: BTreeMap<bool, i32>,
        by_float: BTreeMap<FloatKey, i32>,
        nested: Vec<Vec<Value>>,
    }

    /// A `Serialize` that calls `serialize_bytes`, which `Vec<u8>` does not.
    mod serde_bytes_like {
        pub(super) struct Bytes(pub(super) &'static [u8]);

        impl serde::Serialize for Bytes {
            fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
                serializer.serialize_bytes(self.0)
            }
        }
    }

    #[derive(PartialEq, Eq, PartialOrd, Ord)]
    struct FloatKey(i32);

    impl Serialize for FloatKey {
        fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
            serializer.serialize_f64(f64::from(self.0) / 4.0)
        }
    }

    #[test]
    fn reads_as_the_value_serde_json_builds() {
        let value = Everything {
            flag: true,
            small: -3,
            big: 1 << 53,
            negative: -(1 << 53),
            wide: 1 << 40,
            float: 0.1,
            nan: f64::NAN,
            single: 0.1,
            letter: '日',
            text: "a\"b<&".to_owned(),
            bytes: serde_bytes_like::Bytes(&[0, 255]),
            none: None,
            some: Some(7),
            unit: (),
            tuple: (1, "x"),
            shapes: vec![
                Shape::Unit,
                Shape::Newtype(2),
                Shape::Tuple(3, "t".to_owned()),
                Shape::Struct { side: 1.5 },
            ],
            by_number: BTreeMap::from([(-1, "m"), (10, "t")]),
            by_bool: BTreeMap::from([(false, 0), (true, 1)]),
            by_float: BTreeMap::from([(FloatKey(1), 1), (FloatKey(6), 6)]),
            nested: vec![vec![], vec![json!({ "k": [null, 1e300] })]],
        };
        let tape = Tape::new(&value).expect("serializes");
        assert_eq!(
            tape.root().to_value(),
            serde_json::to_value(&value).expect("serializes")
        );
    }

    #[test]
    fn refuses_an_integer_the_browser_would_round() {
        for value in [
            json!(u64::MAX),
            json!([i64::MIN]),
            json!({ "k": (1_u64 << 53) + 1 }),
        ] {
            let error = Tape::new(&value).err().expect("refused");
            assert!(
                error.to_string().contains("outside -2^53 to 2^53"),
                "{error}"
            );
        }
    }

    #[test]
    fn replaces_a_path_as_set_path_does() {
        let root = json!({ "a": { "b": 1, "c": [1, 2] }, "n": 3 });
        let tape = Tape::new(&root).expect("serializes");
        for (path, next) in [
            ("a.b", json!("x")),
            ("a.c.1", json!(9)),
            ("a.c.5", json!(9)),
            ("a.c.x", json!(9)),
            ("a.d.e", json!(true)),
            ("n.m", json!(null)),
            ("", json!([1])),
        ] {
            assert_eq!(
                tape.replaced(path, &next).root().to_value(),
                crate::ssr::set_path(&root, path, next.clone()),
                "{path}"
            );
        }
    }

    #[test]
    fn reads_the_last_value_of_a_repeated_key() {
        struct Twice;

        impl Serialize for Twice {
            fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
                use serde::ser::SerializeMap;
                let mut map = serializer.serialize_map(None)?;
                map.serialize_entry("k", &1)?;
                map.serialize_entry("k", &2)?;
                map.end()
            }
        }

        let tape = Tape::new(&Twice).expect("serializes");
        let value = tape.root().get_path("k").expect("has k");
        assert_eq!(value.as_i64(), Some(2));
    }
}
