//! `cargo hypersig add`: moves the project in a directory to a host's shape.
//!
//! It does what the "existing project" section of each deploy page asks for:
//! it edits `Cargo.toml`, writes the files the host reads, appends to the
//! ignore files, and moves the binary's source where the host looks for it.
//! The Rust code it leaves alone, because each project builds its `Router`
//! differently, and prints the change to make instead.

use std::path::Path;

use toml_edit::{Array, ArrayOfTables, DocumentMut, InlineTable, Item, Table, TableLike, Value};

use crate::host::{self, Host, Kind};
use crate::{Error, diff};

/// A change to the Rust code that `add` prints and leaves to the reader.
struct Step {
    text: &'static str,
    /// The code to write, from `steps/`, which the deploy pages show too.
    code: &'static str,
}

/// A file `add` moves: the binary's source, to where the host builds it from.
struct Move {
    from: &'static str,
    to: &'static str,
}

const SERVER_STEPS: &[Step] = &[Step {
    text: "In `main`, replace the `TcpListener::bind` line and the `axum::serve` line below it with the lines below, which listen on every interface at the port in `PORT`, and add the `shutdown` function after `main`. Without `shutdown`, the server, which runs as the container's first process, does not stop on Ctrl+C or on the SIGTERM a host sends, and `docker stop` waits 10 seconds and then kills it.",
    code: include_str!("../steps/server.rs"),
}];

const CLOUDFLARE_STEPS: &[Step] = &[
    Step {
        text: "In `src/lib.rs`, replace `#[tokio::main] async fn main` with a `fetch` function that builds the same `Router` as `main` did and hands it the request, and add the two `use` lines it needs. For the quick start counter, it is:",
        code: include_str!("../steps/cloudflare.rs"),
    },
    Step {
        text: "Change the code that uses tokio, which a worker does not have. A stream that waits with `tokio::time::sleep` waits with `worker::Delay`, wrapped in `worker::send::SendFuture`, and a value in the router's state does not outlast the isolate. Then `npx wrangler dev` runs the project.",
        code: "",
    },
];

const VERCEL_STEPS: &[Step] = &[
    Step {
        text: "In `api/app.rs`, add the two `use` lines below, change the return type of `main` to `Result<(), vercel_runtime::Error>`, and replace the listener and the `axum::serve` line with the call to `vercel_runtime::run`. For the quick start counter, `main` becomes:",
        code: include_str!("../steps/vercel.rs"),
    },
    Step {
        text: "Then `cargo run` serves the function at http://localhost:3000, and `npx vercel deploy --prod` publishes it.",
        code: "",
    },
];

fn steps(host: Host) -> &'static [Step] {
    match host {
        Host::Server => SERVER_STEPS,
        Host::Cloudflare => CLOUDFLARE_STEPS,
        Host::Vercel => VERCEL_STEPS,
    }
}

fn moves(host: Host) -> Option<Move> {
    match host {
        Host::Server => None,
        Host::Cloudflare => Some(Move {
            from: "src/main.rs",
            to: "src/lib.rs",
        }),
        Host::Vercel => Some(Move {
            from: "src/main.rs",
            to: "api/app.rs",
        }),
    }
}

/// `code` without the lines that start with `# `, which a step file holds so
/// that it compiles as a doctest, as rustdoc leaves them out.
pub fn shown(code: &str) -> String {
    code.lines()
        .filter(|line| !(line.starts_with("# ") || *line == "#"))
        .flat_map(|line| [line, "\n"])
        .collect()
}

/// What `add` writes, found before anything is written, so that a refusal
/// leaves the project as it was.
struct Plan {
    manifest: Option<String>,
    writes: Vec<(&'static str, String)>,
    appends: Vec<(&'static str, String, Vec<&'static str>)>,
    moved: Option<Move>,
    notes: Vec<String>,
}

pub fn run(dir: &Path, host: Host, force: bool) -> Result<(), Error> {
    let plan = plan(dir, host, force)?;
    if let Some(after) = &plan.manifest {
        let before = read(&dir.join("Cargo.toml"))?;
        println!("Cargo.toml:\n{}", diff::lines(&before, after));
    }
    apply(dir, &plan)?;
    report(host, &plan);
    Ok(())
}

fn plan(dir: &Path, host: Host, force: bool) -> Result<Plan, Error> {
    let manifest_path = dir.join("Cargo.toml");
    if !manifest_path.exists() {
        return Err(Error::new(
            "there is no Cargo.toml here; run `cargo hypersig add` in the package's directory",
        ));
    }
    let before = read(&manifest_path)?;
    let mut manifest: DocumentMut = before
        .parse()
        .map_err(|error| Error::new(format!("Cargo.toml does not parse: {error}")))?;
    let package = manifest
        .get("package")
        .and_then(|package| package.get("name"))
        .and_then(Item::as_str)
        .ok_or_else(|| {
            Error::new(
                "Cargo.toml has no [package] with a name; run `cargo hypersig add` in the package's directory, not at a workspace's root",
            )
        })?
        .to_owned();
    let template: DocumentMut = host
        .manifest(&package)
        .parse()
        .expect("the template's Cargo.toml parses");
    let mut notes = Vec::new();
    edit(host, &mut manifest, &template, &mut notes);
    let after = manifest.to_string();

    let mut writes = Vec::new();
    let mut appends = Vec::new();
    let mut taken = Vec::new();
    for file in host.files() {
        let dest = dir.join(file.path);
        match file.kind {
            Kind::Config => {
                if dest.exists() && !force {
                    taken.push(file.path);
                }
                writes.push((file.path, host::fill(file.text, &package)));
            }
            Kind::Ignore => {
                let current = if dest.exists() {
                    read(&dest)?
                } else {
                    String::new()
                };
                if let Some((text, missing)) = append(current, file.text) {
                    appends.push((file.path, text, missing));
                }
            }
            Kind::Manifest | Kind::Source => {}
        }
    }
    if !taken.is_empty() {
        return Err(Error::new(format!(
            "{} already {}; run `cargo hypersig add {host} --force` to replace {}",
            taken
                .iter()
                .map(|path| format!("`{path}`"))
                .collect::<Vec<_>>()
                .join(", "),
            if taken.len() == 1 { "exists" } else { "exist" },
            if taken.len() == 1 { "it" } else { "them" },
        )));
    }

    let moved = plan_move(dir, host, &mut notes);

    Ok(Plan {
        manifest: (after != before).then_some(after),
        writes,
        appends,
        moved,
        notes,
    })
}

/// `current` with the lines of `wanted` it lacks added at its end, and those
/// lines, or `None` where it has them all.
fn append(mut current: String, wanted: &'static str) -> Option<(String, Vec<&'static str>)> {
    let missing: Vec<&str> = wanted
        .lines()
        .filter(|line| !current.lines().any(|have| have.trim() == *line))
        .collect();
    if missing.is_empty() {
        return None;
    }
    if !current.is_empty() && !current.ends_with('\n') {
        current.push('\n');
    }
    for line in &missing {
        current.push_str(line);
        current.push('\n');
    }
    Some((current, missing))
}

/// The host's move, where its source is there and its destination is not.
/// Where either fails, the note says what to do instead.
fn plan_move(dir: &Path, host: Host, notes: &mut Vec<String>) -> Option<Move> {
    let moved = moves(host).and_then(|step| {
        let from = dir.join(step.from);
        let to = dir.join(step.to);
        if !from.exists() {
            notes.push(format!(
                "There is no `{}` to move, so move the code that builds the `Router` to `{}` yourself.",
                step.from, step.to
            ));
            None
        } else if to.exists() {
            notes.push(format!(
                "`{}` already exists, so `{}` stays where it is; merge it into `{}` yourself.",
                step.to, step.from, step.to
            ));
            None
        } else {
            Some(step)
        }
    });
    if host == Host::Vercel && moved.is_some() && has_modules(&dir.join("src")) {
        notes.push(
            "`src/` still holds Rust files. A `mod` in `api/app.rs` reads its file from `api/`, so move them there too, or give each `mod` a `#[path = \"../src/...\"]`."
                .to_owned(),
        );
    }
    moved
}

fn apply(dir: &Path, plan: &Plan) -> Result<(), Error> {
    if let Some(after) = &plan.manifest {
        write(&dir.join("Cargo.toml"), after)?;
    }
    for (path, text) in &plan.writes {
        write(&dir.join(path), text)?;
    }
    for (path, text, _) in &plan.appends {
        write(&dir.join(path), text)?;
    }
    if let Some(step) = &plan.moved {
        let to = dir.join(step.to);
        if let Some(parent) = to.parent() {
            std::fs::create_dir_all(parent).map_err(|error| {
                Error::new(format!("cannot create `{}`: {error}", parent.display()))
            })?;
        }
        let from = dir.join(step.from);
        std::fs::rename(&from, &to).map_err(|error| {
            Error::new(format!(
                "cannot move `{}` to `{}`: {error}",
                step.from, step.to
            ))
        })?;
        // Leaves no empty `src/` behind once its one file has moved. A
        // directory that still holds a file is kept.
        if let Some(parent) = from.parent() {
            let _ = std::fs::remove_dir(parent);
        }
    }
    Ok(())
}

fn report(host: Host, plan: &Plan) {
    println!("Changed the project for {host}:");
    if plan.manifest.is_some() {
        println!("  changed   Cargo.toml");
    }
    for (path, _) in &plan.writes {
        println!("  wrote     {path}");
    }
    for (path, _, lines) in &plan.appends {
        println!("  appended  {} to {path}", lines.join(", "));
    }
    if let Some(step) = &plan.moved {
        println!("  moved     {} to {}", step.from, step.to);
    }
    println!("\nChange by hand:");
    let mut number = 0;
    for note in &plan.notes {
        number += 1;
        println!("\n{number}. {note}");
    }
    for step in steps(host) {
        number += 1;
        println!("\n{number}. {}", step.text);
        if !step.code.is_empty() {
            println!("\n{}", indent(&shown(step.code)));
        }
    }
}

/// Edits `manifest` as the host's page asks, taking each dependency it adds
/// from the template's `Cargo.toml`, so that both name the same versions.
fn edit(host: Host, manifest: &mut DocumentMut, template: &DocumentMut, notes: &mut Vec<String>) {
    match host {
        Host::Server => {
            let tokio = dependency(manifest, template, "tokio");
            add_feature(tokio, "signal");
        }
        Host::Cloudflare => {
            let package = manifest
                .get("package")
                .and_then(Item::as_table)
                .and_then(Table::position);
            let lib = manifest.entry("lib").or_insert_with(|| {
                let mut lib = Table::new();
                // Puts `[lib]` after `[package]`, where the template has it:
                // tables print in the order of their positions.
                lib.set_position(package);
                Item::Table(lib)
            });
            lib["crate-type"] = toml_edit::value(Array::from_iter(["cdylib"]));
            let axum = dependency(manifest, template, "axum");
            set(axum, "default-features", Value::from(false));
            dependency(manifest, template, "tower-service");
            dependency(manifest, template, "worker");
            if dependencies(manifest).remove("tokio").is_none() {
                notes.push("Cargo.toml has no tokio to remove.".to_owned());
            }
        }
        Host::Vercel => {
            dependency(manifest, template, "tower");
            dependency(manifest, template, "vercel_runtime");
            let bins = manifest
                .entry("bin")
                .or_insert_with(|| Item::ArrayOfTables(ArrayOfTables::new()));
            if let Some(bins) = bins.as_array_of_tables_mut() {
                let has_app = bins
                    .iter()
                    .any(|bin| bin.get("name").and_then(Item::as_str) == Some("app"));
                if !has_app {
                    let mut app = Table::new();
                    app["name"] = toml_edit::value("app");
                    app["path"] = toml_edit::value("api/app.rs");
                    bins.push(app);
                }
            } else {
                notes.push(
                    "`bin` in Cargo.toml is not a list of `[[bin]]` tables; add one named `app` with `path = \"api/app.rs\"`."
                        .to_owned(),
                );
            }
        }
    }
}

fn dependencies(manifest: &mut DocumentMut) -> &mut dyn TableLike {
    manifest
        .entry("dependencies")
        .or_insert_with(|| Item::Table(Table::new()))
        .as_table_like_mut()
        .expect("`dependencies` is a table")
}

/// The dependency `name` of `manifest`, added from the template where the
/// manifest does not have it.
fn dependency<'a>(
    manifest: &'a mut DocumentMut,
    template: &DocumentMut,
    name: &str,
) -> &'a mut Item {
    let from_template = || {
        let mut item = template["dependencies"][name].clone();
        if let Some(value) = item.as_value_mut() {
            value.decor_mut().clear();
        }
        item
    };
    dependencies(manifest)
        .entry(name)
        .or_insert_with(from_template)
}

/// Sets `key` on a dependency, turning `name = "1"` into
/// `name = { version = "1", key = value }` first.
fn set(dependency: &mut Item, key: &str, value: Value) {
    if let Some(version) = dependency.as_str() {
        let mut table = InlineTable::new();
        table.insert("version", Value::from(version));
        *dependency = Item::Value(Value::InlineTable(table));
    }
    if let Some(table) = dependency.as_table_like_mut() {
        table.insert(key, Item::Value(value));
    }
}

fn add_feature(dependency: &mut Item, feature: &str) {
    let has = dependency
        .get("features")
        .and_then(Item::as_array)
        .is_some_and(|features| features.iter().any(|have| have.as_str() == Some(feature)));
    if has {
        return;
    }
    if dependency
        .get("features")
        .and_then(Item::as_array)
        .is_none()
    {
        set(dependency, "features", Value::Array(Array::new()));
    }
    if let Some(features) = dependency.get_mut("features").and_then(Item::as_array_mut) {
        features.push(feature);
    }
}

/// Whether `dir` holds a `.rs` file besides `main.rs`, which `add` moves.
fn has_modules(dir: &Path) -> bool {
    std::fs::read_dir(dir).is_ok_and(|entries| {
        entries.filter_map(Result::ok).any(|entry| {
            let path = entry.path();
            path.extension().is_some_and(|ext| ext == "rs") && !path.ends_with("main.rs")
                || path.is_dir()
        })
    })
}

fn indent(code: &str) -> String {
    code.lines()
        .map(|line| {
            if line.is_empty() {
                String::new()
            } else {
                format!("    {line}")
            }
        })
        .collect::<Vec<_>>()
        .join("\n")
}

fn read(path: &Path) -> Result<String, Error> {
    std::fs::read_to_string(path)
        .map_err(|error| Error::new(format!("cannot read `{}`: {error}", path.display())))
}

fn write(path: &Path, text: &str) -> Result<(), Error> {
    std::fs::write(path, text)
        .map_err(|error| Error::new(format!("cannot write `{}`: {error}", path.display())))
}
