---
description: The Pragmatic Rust Guidelines this codebase leans on, plus the lint and formatting gates. Applies to every Rust source file
globs: "**/*.rs,**/Cargo.toml"
alwaysApply: false
paths: "**/*.rs, **/Cargo.toml"
---

# Rust

The IDs below come from Microsoft's Pragmatic Rust Guidelines. The full text, with examples, is in `.claude/skills/rust-guidelines/references/pragmatic-rust-guidelines.md`. Search it by ID when the summary here does not settle a case.

## Gates

**The lint set lives in `[workspace.lints]` and every crate opts in with `[lints] workspace = true`** (M-STATIC-VERIFICATION). Clippy's `pedantic` and `cargo` groups are on, so `cargo clippy --workspace --all-targets -- -D warnings` fails on what they flag.

**Override a lint with `#[expect(..., reason = "...")]`, never a bare `#[allow]`** (M-LINT-OVERRIDE-EXPECT). `#[expect]` warns once the lint stops firing, so a stale override does not survive. Code a proc macro emits is the exception: it uses `#[allow]`, because whether the lint fires depends on the user's input.

**No `unsafe`.** The workspace forbids it. A change that needs it is a change to `Cargo.toml` and goes to the user first, together with M-UNSAFE.

## Types

**Use the strongest type at the earliest point** (M-STRONG-TYPES). The library exists to replace strings with types, so a public function that takes `&str` for a signal path, an element id, or an expression has lost the point. Construct those values only from generated code or from types that already carry the guarantee.

**Keep public types at one or two type parameters** (M-SIMPLE-ABSTRACTIONS). `El<S>`, `Sig<S, T>`, `Expr<S, T>`, and `Action<S, A>` are the ceiling. A third parameter needs a reason written into the pull request. `Sig<S, T, R = Root>` has one: `R` records whether the path starts at the page or at a list item, so that `patch` exists only on a page-rooted signal and an item field cannot be patched from the server. The default keeps it out of sight for page signals.

**Every public type implements `Debug`** (M-PUBLIC-DEBUG). `missing_debug_implementations` is on.

**Do not put `Arc`, `Box`, `Rc`, or `RefCell` in a public signature** (M-AVOID-WRAPPERS). Take `&T` or `T` and wrap inside.

**A public item has one public path** (M-SINGLE-ITEM-PATH). An item re-exported at the crate root lives in a private module, as `Page` lives in `page.rs` behind `pub use page::Page`. A public module such as `html` holds the items a reader reaches through its name.

**Third-party types in the public API are limited to `serde`'s traits, the `Stream` trait of `futures`, and, behind the `axum` feature, axum's `Router` and the traits and types a handler reads the request and writes the response with: `FromRequestParts`, `IntoResponse`, `IntoResponseParts`, `Parts`, `Response`, `StatusCode`, the `Bytes` a streaming upload yields, and the `Request<Body>` that `Action::request` builds** (M-DONT-LEAK-TYPES). A handler that reads a cookie or answers with its own error is written against axum's own traits, so wrapping them would only make the reader translate between two names for one thing. A chunk of an uploaded file is `axum::body::Bytes` because that is what the multipart parser hands over, and copying it into a `Vec<u8>` would cost a copy per chunk of a file read in chunks to save memory. `Action::request` returns `axum::http::Request<axum::body::Body>` because a test hands it to `tower::ServiceExt::oneshot` on the `Router`, which takes exactly that type. A stream's handler returns a `Stream` of `Patches`, and `PatchStream` names that bound, because `Stream` is the trait every async stream crate implements, so a wrapper would only make each handler convert its stream.

## Performance

**Rendering reads the signals and the expression trees by reference** (M-HOTPATH). `Page::render` runs once per request and writes each `each` template once per list item, so it borrows what it reads (`Ctx::get`, `Expr::as_json`, the `Cow` from `ssr::evaluate`) and allocates only for a value it produces. A claim that a change is faster carries the medians from `cargo bench -p hypersig --bench render` before and after, which report allocations alongside time.

## Errors and panics

**The library's errors are structs with a `Display`, an `Error` impl, and a `source` where one exists** (M-ERRORS-CANONICAL-STRUCTS). The examples are applications and may use whatever is shortest (M-APP-ERROR).

**A broken invariant panics, and bad input returns `Result`** (M-PANIC-ON-BUG). An action request whose arguments do not deserialize into the action's argument type is input and gets a 400. A `Sig` whose path is empty is a bug in generated code and panics.

## Macros

**Write a macro only after the language runs out** (M-MACRO-LAST-RESORT). `#[derive(Signals)]` exists because a field name cannot become a value any other way, `#[derive(Segment)]` because a variant name cannot, and `#[derive(Search)]` because the types of a struct's fields cannot be bounded any other way. `routes!` exists because a program's constants cannot be enumerated at compile time, so declaring each `Route` on the line that names its handler is the only way to keep a route without a handler from existing. `node_ref!` and `dialog_ref!` exist because a constant's identifier cannot become its name any other way, and that name keeps two constants from sharing one. Actions, fragments, and patches are plain types and traits, and they stay that way.

**A macro emits nothing the reader cannot predict from the input** (M-MACROS-DONT-LIE, M-PROC-IMPLIED-ITEMS). `#[derive(Signals)]` adds one `impl Signals`, whose `SHAPE` constant lists the fields' names and types, and one associated function per field, named after the field. `#[derive(Segment)]` adds `impl Segment`, the impl of its seal, and `impl Variants`, whose one constant lists the variants in declaration order. `#[derive(Search)]` adds one hidden `impl SearchFields` with a bound per field. None of them defines a new type, public or private. `routes!` emits the constants and the one function its input names, and nothing else. `node_ref!` and `dialog_ref!` emit the constant their input declares and a `const _` that reads it through `self::`, which fails to compile inside a function body.

**The logic lives in `hypersig_proc_impl` with snapshot tests, and `hypersig_proc` only forwards** (M-PROC-IMPL). Generated code names items through `::hypersig::_private` (M-MACRO-HELPERS, M-MACRO-MAIN-CRATE), and `hypersig` pins `hypersig_proc` with `=` (M-MACRO-VERSION-PIN).

**A compile error the library promises has a trybuild case under `crates/hypersig/tests/ui/fail/`.** Add the case in the same commit as the type that produces the error.

## Documentation

**Every public item has a summary sentence under 15 words, then `# Examples`, `# Errors`, and `# Panics` where they apply** (M-CANONICAL-DOCS, M-FIRST-DOC-SENTENCE). Every module has a `//!` header saying what it holds (M-MODULE-DOCS).
