//! Runs `cargo hypersig new` and `cargo hypersig add` for each host and
//! checks the projects they write, through `cargo check`, and runs
//! `cargo hypersig elements` on a manifest and checks a page that uses the
//! module it prints, through `cargo clippy`.
//!
//! The checks build against this repository's hypersig, as
//! `crates/hypersig_cli/stage.sh` does, and download the host's crates
//! (`worker`, `vercel_runtime`) from crates.io on the first run.

use std::path::{Path, PathBuf};
use std::process::{Command, Output};

const BIN: &str = env!("CARGO_BIN_EXE_cargo-hypersig");
const ROOT: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/../..");

/// The project of the quick start: `cargo new counter`, the dependencies its
/// page lists, and its program.
const QUICK_START_MANIFEST: &str = r#"[package]
name = "counter"
version = "0.1.0"
edition = "2024"

[dependencies]
hypersig = "0.4"
axum = "0.8"
serde = { version = "1", features = ["derive"] }
tokio = { version = "1", features = ["macros", "rt-multi-thread", "net"] }
"#;

const QUICK_START: &str = include_str!("../../../site/examples/quick_start.rs");

/// A fresh directory for one test, under the target directory, which the
/// workspace excludes so that the projects in it are packages of their own.
fn scratch(name: &str) -> PathBuf {
    let dir = Path::new(env!("CARGO_TARGET_TMPDIR"))
        .join("hypersig_cli")
        .join(name);
    // Keeps `target`, so a second run of the tests checks quickly.
    if let Ok(entries) = std::fs::read_dir(&dir) {
        for entry in entries.map(Result::unwrap) {
            if entry.file_name() != "target" {
                let path = entry.path();
                if path.is_dir() {
                    std::fs::remove_dir_all(path).unwrap();
                } else {
                    std::fs::remove_file(path).unwrap();
                }
            }
        }
    }
    std::fs::create_dir_all(&dir).unwrap();
    dir
}

fn hypersig(dir: &Path, args: &[&str]) -> Output {
    Command::new(BIN)
        .arg("hypersig")
        .args(args)
        .current_dir(dir)
        .output()
        .unwrap()
}

fn succeeds(output: &Output) -> String {
    let stdout = String::from_utf8(output.stdout.clone()).unwrap();
    assert!(
        output.status.success(),
        "stdout:\n{stdout}\nstderr:\n{}",
        String::from_utf8_lossy(&output.stderr)
    );
    stdout
}

fn read(path: impl AsRef<Path>) -> String {
    let path = path.as_ref();
    std::fs::read_to_string(path).unwrap_or_else(|error| panic!("{}: {error}", path.display()))
}

/// Every file under `dir`, by its path relative to `dir`.
fn files(dir: &Path) -> Vec<String> {
    fn walk(dir: &Path, base: &Path, out: &mut Vec<String>) {
        for entry in std::fs::read_dir(dir).unwrap().map(Result::unwrap) {
            let path = entry.path();
            if path.is_dir() {
                if entry.file_name() != "target" {
                    walk(&path, base, out);
                }
            } else {
                out.push(
                    path.strip_prefix(base)
                        .unwrap()
                        .to_str()
                        .unwrap()
                        .to_owned(),
                );
            }
        }
    }
    let mut out = Vec::new();
    walk(dir, dir, &mut out);
    out.sort();
    out
}

/// The Cargo that runs the tests, which sets `CARGO` for them.
fn cargo() -> String {
    std::env::var("CARGO").unwrap_or_else(|_| "cargo".to_owned())
}

/// Checks the project at `dir` against this repository's hypersig, in a
/// target directory that the projects of one host share.
fn check(dir: &Path, target: &Path, args: &[&str]) {
    std::fs::create_dir_all(dir.join(".cargo")).unwrap();
    std::fs::write(
        dir.join(".cargo/config.toml"),
        format!(
            "[patch.crates-io]\nhypersig = {{ path = \"{}\" }}\n",
            Path::new(ROOT).join("crates/hypersig").display()
        ),
    )
    .unwrap();
    let status = Command::new(cargo())
        .arg("check")
        .args(args)
        .arg("--quiet")
        .env("CARGO_TARGET_DIR", target)
        .current_dir(dir)
        .status()
        .unwrap();
    assert!(status.success(), "cargo check failed in {}", dir.display());
}

/// `new` writes each file of `templates/$host` with its placeholders filled,
/// and the shared `AGENTS.md` and `CLAUDE.md`, and nothing else, and the
/// project checks.
fn new(host: &str, target: &Path, check_args: &[&str]) {
    let dir = scratch(&format!("new-{host}"));
    let package = format!("my-{host}");
    succeeds(&hypersig(&dir, &["new", &package, "--host", host]));
    let project = dir.join(&package);

    let template = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("templates")
        .join(host);
    let notes = read(template.join("AGENTS.host.md"));
    let mut expected: Vec<String> = files(&template)
        .into_iter()
        .filter(|path| path != "AGENTS.host.md")
        .map(|path| path.trim_end_matches(".template").to_owned())
        .chain(["AGENTS.md".to_owned(), "CLAUDE.md".to_owned()])
        .collect();
    expected.sort();
    assert_eq!(files(&project), expected);
    for path in &expected {
        let source = match path.as_str() {
            "Cargo.toml" => template.join("Cargo.toml.template"),
            "AGENTS.md" | "CLAUDE.md" => template.parent().unwrap().join(path),
            _ => template.join(path),
        };
        let want = read(source)
            .replace("{{project-name}}", &package)
            .replace("{{hypersig-version}}", env!("CARGO_PKG_VERSION"))
            .replace("{{host-notes}}", notes.trim_end());
        assert_eq!(read(project.join(path)), want, "{path}");
    }
    assert!(!read(project.join("AGENTS.md")).contains("{{"));
    assert!(
        read(project.join("Cargo.toml"))
            .contains(&format!("hypersig = \"{}\"", env!("CARGO_PKG_VERSION")))
    );

    check(&project, target, check_args);
}

/// The quick start counter in a directory of its own.
fn quick_start(name: &str) -> PathBuf {
    let dir = scratch(name);
    std::fs::create_dir_all(dir.join("src")).unwrap();
    std::fs::write(dir.join("Cargo.toml"), QUICK_START_MANIFEST).unwrap();
    std::fs::write(dir.join("src/main.rs"), QUICK_START).unwrap();
    std::fs::write(dir.join(".gitignore"), "/target\n").unwrap();
    dir
}

/// A step file without the lines rustdoc hides.
fn shown(code: &str) -> String {
    code.lines()
        .filter(|line| !(line.starts_with("# ") || *line == "#"))
        .flat_map(|line| [line, "\n"])
        .collect()
}

/// `source` with `main` and everything after it replaced by `code`.
fn replace_main(source: &str, code: &str) -> String {
    let at = source
        .find("#[tokio::main]")
        .expect("the quick start has a main");
    format!("{}{code}", &source[..at])
}

fn add_refuses_to_overwrite(dir: &Path, host: &str, taken: &str) {
    let before = read(dir.join("Cargo.toml"));
    let output = hypersig(dir, &["add", host]);
    assert!(!output.status.success());
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains(taken) && stderr.contains("--force"),
        "{stderr}"
    );
    assert_eq!(read(dir.join("Cargo.toml")), before);
}

#[test]
fn server_projects_check() {
    let target = scratch("target-server").join("target");
    new("server", &target, &[]);

    let dir = quick_start("add-server");
    let stdout = succeeds(&hypersig(&dir, &["add", "server"]));
    assert!(stdout.contains("  wrote     Dockerfile"), "{stdout}");
    assert_eq!(
        read(dir.join("Cargo.toml")),
        QUICK_START_MANIFEST.replace(r#""net"] }"#, r#""net", "signal"] }"#)
    );
    assert_eq!(
        read(dir.join("Dockerfile")),
        include_str!("../templates/server/Dockerfile")
    );
    assert_eq!(read(dir.join(".dockerignore")), "target\n");
    assert_eq!(read(dir.join(".gitignore")), "/target\n");
    add_refuses_to_overwrite(&dir, "server", "Dockerfile");

    // The step: the listener and `axum::serve` in `main` become the step's
    // lines, and `shutdown` follows `main`.
    let step = shown(include_str!("../steps/server.rs"));
    let (body, shutdown) = step.split_once("\n\n").unwrap();
    let body: String = body.lines().flat_map(|line| ["    ", line, "\n"]).collect();
    let main = read(dir.join("src/main.rs")).replace(
        "    let listener = tokio::net::TcpListener::bind(\"127.0.0.1:3000\").await?;\n    axum::serve(listener, app).await\n}\n",
        &format!("{body}}}\n\n{shutdown}"),
    );
    assert!(main.contains("fn shutdown"));
    std::fs::write(dir.join("src/main.rs"), main).unwrap();
    check(&dir, &target, &[]);
}

#[test]
fn cloudflare_projects_check() {
    let target = scratch("target-cloudflare").join("target");
    let wasm = ["--target", "wasm32-unknown-unknown"];
    new("cloudflare", &target, &wasm);

    let dir = quick_start("add-cloudflare");
    let stdout = succeeds(&hypersig(&dir, &["add", "cloudflare"]));
    assert!(
        stdout.contains("  moved     src/main.rs to src/lib.rs"),
        "{stdout}"
    );
    assert_eq!(
        read(dir.join("Cargo.toml")),
        r#"[package]
name = "counter"
version = "0.1.0"
edition = "2024"

[lib]
crate-type = ["cdylib"]

[dependencies]
hypersig = "0.4"
axum = { version = "0.8", default-features = false }
serde = { version = "1", features = ["derive"] }
tower-service = "0.3"
worker = { version = "0.8", features = ["http", "axum", "d1"] }
"#
    );
    assert_eq!(
        read(dir.join("wrangler.toml")),
        include_str!("../steps/wrangler.toml").replace("{{project-name}}", "counter")
    );
    // The template's `wrangler.toml` is the one `add` writes and the D1
    // binding its worker saves the count in.
    assert!(
        include_str!("../templates/cloudflare/wrangler.toml")
            .starts_with(include_str!("../steps/wrangler.toml"))
    );
    assert_eq!(
        read(dir.join("rust-toolchain.toml")),
        include_str!("../templates/cloudflare/rust-toolchain.toml")
    );
    assert_eq!(
        read(dir.join(".gitignore")),
        "/target\n/build\n/.wrangler\n"
    );
    assert!(!dir.join("src/main.rs").exists());
    add_refuses_to_overwrite(&dir, "cloudflare", "wrangler.toml");

    let lib = replace_main(
        &read(dir.join("src/lib.rs")),
        include_str!("../steps/cloudflare.rs"),
    );
    std::fs::write(dir.join("src/lib.rs"), lib).unwrap();
    check(&dir, &target, &wasm);
}

#[test]
fn vercel_projects_check() {
    let target = scratch("target-vercel").join("target");
    new("vercel", &target, &[]);

    let dir = quick_start("add-vercel");
    let stdout = succeeds(&hypersig(&dir, &["add", "vercel"]));
    assert!(
        stdout.contains("  moved     src/main.rs to api/app.rs"),
        "{stdout}"
    );
    assert!(!stdout.contains("still holds"), "{stdout}");
    assert_eq!(
        read(dir.join("Cargo.toml")),
        format!(
            "{QUICK_START_MANIFEST}{}",
            r#"tower = "0.5"
vercel_runtime = { version = "2.4", features = ["axum"] }

[[bin]]
name = "app"
path = "api/app.rs"
"#
        )
    );
    assert_eq!(
        read(dir.join("vercel.json")),
        include_str!("../templates/vercel/vercel.json")
    );
    assert_eq!(read(dir.join(".vercelignore")), "/target\n");
    assert_eq!(read(dir.join(".gitignore")), "/target\n/.vercel\n");
    assert!(!dir.join("src").exists());
    add_refuses_to_overwrite(&dir, "vercel", "vercel.json");

    let app = replace_main(
        &read(dir.join("api/app.rs")),
        include_str!("../steps/vercel.rs"),
    );
    std::fs::write(dir.join("api/app.rs"), app).unwrap();
    check(&dir, &target, &[]);
}

#[test]
fn add_with_force_replaces_the_files() {
    let dir = quick_start("add-force");
    succeeds(&hypersig(&dir, &["add", "server"]));
    std::fs::write(dir.join("Dockerfile"), "FROM scratch\n").unwrap();
    succeeds(&hypersig(&dir, &["add", "server", "--force"]));
    assert_eq!(
        read(dir.join("Dockerfile")),
        include_str!("../templates/server/Dockerfile")
    );
    // The features already hold `signal`, so the second run leaves them.
    assert_eq!(read(dir.join("Cargo.toml")).matches("signal").count(), 1);
}

#[test]
fn add_writes_agents_md_only_where_there_is_none() {
    let dir = quick_start("add-guide");
    let stdout = succeeds(&hypersig(&dir, &["add", "server"]));
    assert!(stdout.contains("  wrote     AGENTS.md"), "{stdout}");
    assert_eq!(read(dir.join("CLAUDE.md")), "@AGENTS.md\n");

    let dir = quick_start("add-guide-kept");
    std::fs::write(dir.join("AGENTS.md"), "Our own rules.\n").unwrap();
    let stdout = succeeds(&hypersig(&dir, &["add", "server", "--force"]));
    assert!(!stdout.contains("  wrote     AGENTS.md"), "{stdout}");
    assert!(stdout.contains("`AGENTS.md` is already here"), "{stdout}");
    assert_eq!(read(dir.join("AGENTS.md")), "Our own rules.\n");
}

#[test]
fn new_refuses_a_directory_that_exists() {
    let dir = scratch("new-exists");
    std::fs::create_dir_all(dir.join("taken")).unwrap();
    let output = hypersig(&dir, &["new", "taken", "--host", "server"]);
    assert!(!output.status.success());
    assert_eq!(files(&dir.join("taken")), Vec::<String>::new());
}

/// The manifest under `tests/elements/` holds `sl-button` and `sl-rating`
/// from Shoelace 2.20.1, `wa-stepper` from Web Awesome 3.14.0, and
/// `md-elevated-button` with the classes it extends from Material Web 2.5.0,
/// each trimmed to a few attributes and stripped of descriptions, and
/// `test-picker`, written for this test, whose names and `detail` types
/// reach the cases the others leave out.
#[test]
fn elements_prints_a_module_that_a_page_checks_against() {
    let fixtures = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/elements");
    let dir = scratch("elements");
    let module = succeeds(&hypersig(
        &dir,
        &[
            "elements",
            fixtures.join("custom-elements.json").to_str().unwrap(),
        ],
    ));
    assert_eq!(
        module,
        read(fixtures.join("expected.rs")),
        "run the command on tests/elements/custom-elements.json and save its output as tests/elements/expected.rs, after reading the difference"
    );

    // The JavaScript page of the site shows the part for `sl-rating`.
    let shown = include_str!("../../../site/snippets/elements_module.rs");
    let (imports, part) = shown.split_once("\n\n").unwrap();
    assert!(module.contains(imports) && module.contains(part));

    std::fs::create_dir_all(dir.join("src")).unwrap();
    std::fs::write(
        dir.join("Cargo.toml"),
        "[package]\nname = \"elements\"\nversion = \"0.1.0\"\nedition = \"2024\"\n\n\
         [dependencies]\nhypersig = \"0.4\"\nserde = { version = \"1\", features = [\"derive\"] }\n\n\
         [lints.clippy]\npedantic = { level = \"warn\", priority = -1 }\n",
    )
    .unwrap();
    std::fs::write(dir.join("src/elements.rs"), &module).unwrap();
    std::fs::write(dir.join("src/main.rs"), read(fixtures.join("page.rs"))).unwrap();
    let target = dir.join("target");
    check(&dir, &target, &[]);

    let cargo = cargo();
    let status = Command::new(&cargo)
        .args(["clippy", "--quiet", "--", "-D", "warnings"])
        .env("CARGO_TARGET_DIR", &target)
        .current_dir(&dir)
        .status()
        .unwrap();
    assert!(status.success(), "cargo clippy failed in {}", dir.display());
    let status = Command::new(&cargo)
        .args(["fmt", "--check"])
        .current_dir(&dir)
        .status()
        .unwrap();
    assert!(status.success(), "the module is not as rustfmt writes it");

    let run = Command::new(&cargo)
        .args(["run", "--quiet"])
        .env("CARGO_TARGET_DIR", &target)
        .current_dir(&dir)
        .output()
        .unwrap();
    let html = succeeds(&run);
    for name in [
        "sl-hover",
        "sl-change",
        "wa-step-change",
        "previousName",
        "pick",
    ] {
        assert!(html.contains(name), "{name} in {html}");
    }
}

#[test]
fn elements_refuses_a_manifest_without_elements() {
    let dir = scratch("elements-empty");
    std::fs::write(
        dir.join("custom-elements.json"),
        r#"{"schemaVersion":"1.0.0","modules":[]}"#,
    )
    .unwrap();
    let output = hypersig(&dir, &["elements", "custom-elements.json"]);
    assert!(!output.status.success());
    assert!(
        String::from_utf8_lossy(&output.stderr).contains("no custom element"),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
}
