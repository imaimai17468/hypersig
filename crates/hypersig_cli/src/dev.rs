//! `cargo hypersig dev`: builds the project's server, runs it, and builds
//! and runs it again each time a file it is built from changes.
//!
//! blaze writes the binary, and `cargo check` decides whether the code
//! compiles. After an edit, blaze compiles only the functions that changed,
//! so the wait is the check's.
//!
//! A page that a debug build of hypersig serves keeps a stream to the server
//! open and reloads once a new server answers in its place, so the browser
//! shows the new build without a hand on the reload button. The reloaded
//! page keeps the signals the visitor changed, where its route and its
//! signal struct's fields are the same.

use std::collections::{BTreeMap, BTreeSet};
use std::ffi::OsString;
use std::io::{BufRead as _, BufReader};
use std::path::{Path, PathBuf};
use std::process::{Child, Command, Stdio};
use std::time::{Duration, SystemTime};

use serde_json::Value;

use crate::Error;

/// How often the files are read for a change. Reading them costs one
/// `stat` per file, so a short interval costs little next to a build.
const POLL: Duration = Duration::from_millis(50);

/// How long to wait after a change for the rest of an editor's writes. A
/// write that comes later still starts another build, as any change does.
const SETTLE: Duration = Duration::from_millis(50);

/// Builds with `cargo build` and `build_args`, runs the binary with
/// `program_args`, and repeats both after each change until interrupted.
pub fn run(build_args: &[String], program_args: &[String]) -> Result<(), Error> {
    let dir = std::env::current_dir()
        .map_err(|error| Error::new(format!("cannot read the current directory: {error}")))?;
    if build_args
        .iter()
        .any(|arg| arg == "--release" || arg == "-r" || arg.starts_with("--profile"))
    {
        eprintln!(
            "warning: only a debug build serves the stream a page reloads from, so pages will not reload"
        );
    }
    blaze_installed()?;
    let before = snapshot(&dir, None);
    let mut built = build(build_args)?;
    let mut seen = as_built(&before, snapshot(&dir, built.as_ref()));
    eprintln!("{}", watching(&dir, before.len(), seen.len()));
    let mut server = match &built {
        Some(built) => Some(start(&dir, &built.binary, program_args, "Started")?),
        None => None,
    };
    loop {
        std::thread::sleep(POLL);
        if let Some(child) = &mut server
            && let Ok(Some(status)) = child.try_wait()
        {
            eprintln!("The server stopped with {status}; it starts again after the next change.");
            server = None;
        }
        if snapshot(&dir, built.as_ref()) == seen {
            continue;
        }
        std::thread::sleep(SETTLE);
        let before = snapshot(&dir, built.as_ref());
        // A build that fails leaves the server of the last build running
        // until the next change, after Cargo has printed the errors.
        if let Some(next) = build(build_args)? {
            // A change that compiles to the same binary, such as one to a
            // file the binary is not built from, leaves the server running.
            let same = built.as_ref().is_some_and(|built| {
                built.binary == next.binary && built.modified == next.modified
            });
            if !same || server.is_none() {
                if let Some(mut child) = server.take() {
                    stop(&mut child);
                }
                server = Some(start(&dir, &next.binary, program_args, "Restarted")?);
            }
            built = Some(next);
        }
        seen = as_built(&before, snapshot(&dir, built.as_ref()));
    }
}

/// The binary a build wrote, and the files it was built from.
struct Built {
    binary: PathBuf,
    modified: Option<SystemTime>,
    sources: BTreeSet<PathBuf>,
}

/// The install line `dev` prints where blaze is missing. blaze's own
/// installer puts `blaze` on the path.
const INSTALL_BLAZE: &str =
    "curl -fsSL https://raw.githubusercontent.com/indexable-inc/blaze/main/install.sh | sh";

fn cargo() -> OsString {
    std::env::var_os("CARGO").unwrap_or_else(|| OsString::from("cargo"))
}

/// Fails where `blaze` does not run, before the first build.
///
/// # Errors
///
/// Fails with the install line where no `blaze` command runs.
fn blaze_installed() -> Result<(), Error> {
    let runs = Command::new("blaze")
        .arg("--version")
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()
        .is_ok_and(|status| status.success());
    if runs {
        Ok(())
    } else {
        Err(Error::new(format!(
            "`cargo hypersig dev` builds with blaze, which is not installed; install it with\n\n    {INSTALL_BLAZE}\n\nand run `cargo hypersig dev` again"
        )))
    }
}

/// Runs `cargo check` and `blaze build` at once, and finds the one
/// binary the build wrote, or `None` where the check failed.
///
/// rustc's check decides what is an error, so a mistake that blaze would
/// compile still stops here. Where blaze fails on code that checks, Cargo
/// builds it instead.
///
/// # Errors
///
/// Fails where Cargo cannot run, or the package has no binary or several,
/// which no edit to the code changes.
fn build(build_args: &[String]) -> Result<Option<Built>, Error> {
    let dirs = Dirs::read(build_args)?;
    let mut check = Command::new(cargo())
        .arg("check")
        .arg("--message-format=json-render-diagnostics")
        .args(build_args)
        .stdout(Stdio::piped())
        .spawn()
        .map_err(|error| Error::new(format!("cannot run cargo: {error}")))?;
    // blaze's own lines repeat what the check prints, so they are dropped.
    let mut blaze = Command::new("blaze")
        .arg("build")
        .args(build_args)
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()
        .map_err(|error| Error::new(format!("cannot run blaze: {error}")))?;
    let binaries = read_cargo(&mut check, |line| checked_binary_of(line, &dirs))?;
    let checked = check
        .wait()
        .map_err(|error| Error::new(format!("cannot wait for cargo: {error}")))?;
    let blazed = blaze
        .wait()
        .map_err(|error| Error::new(format!("cannot wait for blaze: {error}")))?;
    if !checked.success() {
        return Ok(None);
    }
    let binary = one_binary(binaries)?;
    if !blazed.success() {
        eprintln!("blaze did not build it, so Cargo builds it");
        return build_with_cargo(build_args);
    }
    Ok(Some(built(binary)))
}

/// Runs `cargo build` and finds the one binary it wrote, or `None` where
/// the build failed.
///
/// # Errors
///
/// Fails where Cargo cannot run, or the build wrote no binary or several.
fn build_with_cargo(build_args: &[String]) -> Result<Option<Built>, Error> {
    let mut child = Command::new(cargo())
        .arg("build")
        .arg("--message-format=json-render-diagnostics")
        .args(build_args)
        .stdout(Stdio::piped())
        .spawn()
        .map_err(|error| Error::new(format!("cannot run cargo: {error}")))?;
    let binaries = read_cargo(&mut child, binary_of)?;
    let status = child
        .wait()
        .map_err(|error| Error::new(format!("cannot wait for cargo: {error}")))?;
    if !status.success() {
        return Ok(None);
    }
    Ok(Some(built(one_binary(binaries)?)))
}

/// The binaries `binary` finds in the lines of Cargo's JSON output.
fn read_cargo(
    child: &mut Child,
    binary: impl Fn(&str) -> Option<PathBuf>,
) -> Result<Vec<PathBuf>, Error> {
    let stdout = child.stdout.take().expect("stdout is piped");
    let mut binaries = Vec::new();
    for line in BufReader::new(stdout).lines() {
        let line =
            line.map_err(|error| Error::new(format!("cannot read cargo's output: {error}")))?;
        binaries.extend(binary(&line));
    }
    Ok(binaries)
}

/// The one binary of `binaries`.
///
/// # Errors
///
/// Fails where there is none or several.
fn one_binary(binaries: Vec<PathBuf>) -> Result<PathBuf, Error> {
    match <[PathBuf; 1]>::try_from(binaries) {
        Ok([binary]) => Ok(binary),
        Err(binaries) if binaries.is_empty() => Err(Error::new(
            "the build has no binary to run; a Cloudflare project runs under `npx wrangler dev`",
        )),
        Err(binaries) => {
            let names: Vec<String> = binaries
                .iter()
                .filter_map(|binary| binary.file_name())
                .map(|name| format!("`{}`", name.to_string_lossy()))
                .collect();
            Err(Error::new(format!(
                "the build has {}; name one with --bin or -p",
                names.join(", ")
            )))
        }
    }
}

/// The binary at `binary`, and the files it was built from.
fn built(binary: PathBuf) -> Built {
    let sources = std::fs::read_to_string(dep_info_of(&binary))
        .map(|text| parse_dep_info(&text))
        .unwrap_or_default();
    Built {
        modified: modified(&binary),
        binary,
        sources,
    }
}

/// The path of the binary that a line of Cargo's JSON output reports, if it
/// reports one.
fn binary_of(line: &str) -> Option<PathBuf> {
    let message: Value = serde_json::from_str(line).ok()?;
    if message["reason"] != "compiler-artifact" {
        return None;
    }
    let is_bin = message["target"]["kind"]
        .as_array()?
        .iter()
        .any(|kind| kind == "bin");
    let executable = message["executable"].as_str()?;
    is_bin.then(|| PathBuf::from(executable))
}

/// Where Cargo writes the files a build keeps and the ones it builds them from.
#[derive(Debug, PartialEq)]
struct Dirs {
    /// The target directory, which holds the binary and its dep-info file.
    target: PathBuf,
    /// The build directory (`build.build-dir`), which holds the metadata files
    /// a check writes. It is the target directory unless set apart.
    build: PathBuf,
}

impl Dirs {
    /// The directories `cargo metadata` reports, with a `--target-dir` of
    /// `build_args` in place of the target directory, as the build takes it.
    ///
    /// # Errors
    ///
    /// Fails where `cargo metadata` cannot run or answers with no directories.
    fn read(build_args: &[String]) -> Result<Self, Error> {
        let mut command = Command::new(cargo());
        command.args(["metadata", "--format-version", "1", "--no-deps"]);
        if let Some(manifest) = value_of(build_args, "--manifest-path") {
            command.args(["--manifest-path", manifest]);
        }
        let output = command
            .stderr(Stdio::inherit())
            .output()
            .map_err(|error| Error::new(format!("cannot run cargo metadata: {error}")))?;
        let metadata: Value = serde_json::from_slice(&output.stdout)
            .map_err(|error| Error::new(format!("cannot read cargo metadata: {error}")))?;
        let dir = |key: &str| metadata[key].as_str().map(PathBuf::from);
        let target = dir("target_directory")
            .ok_or_else(|| Error::new("cargo metadata names no target directory"))?;
        let build = dir("build_directory").unwrap_or_else(|| target.clone());
        // Cargo reads a relative `--target-dir` from the directory it runs in.
        let given = value_of(build_args, "--target-dir").map(|given| {
            std::env::current_dir().map_or_else(|_| PathBuf::from(given), |dir| dir.join(given))
        });
        Ok(match given {
            // A build directory left unset follows the target directory.
            Some(given) if build == target => Self {
                target: given.clone(),
                build: given,
            },
            Some(given) => Self {
                target: given,
                build,
            },
            None => Self { target, build },
        })
    }
}

/// The value of `flag` in `args`, written as `flag value` or `flag=value`.
fn value_of<'a>(args: &'a [String], flag: &str) -> Option<&'a str> {
    args.iter().enumerate().find_map(|(at, arg)| {
        if arg == flag {
            args.get(at + 1).map(String::as_str)
        } else {
            arg.strip_prefix(flag)?.strip_prefix('=')
        }
    })
}

/// The path a build writes the binary to, for a line of `cargo check`'s
/// JSON output that reports a binary.
///
/// A check reports no executable, only the metadata file it wrote under the
/// profile's `deps` directory in the build directory, and a build puts the
/// binary in the same profile's directory of the target directory, under
/// the target's name.
fn checked_binary_of(line: &str, dirs: &Dirs) -> Option<PathBuf> {
    let message: Value = serde_json::from_str(line).ok()?;
    if message["reason"] != "compiler-artifact" {
        return None;
    }
    let is_bin = message["target"]["kind"]
        .as_array()?
        .iter()
        .any(|kind| kind == "bin");
    let name = message["target"]["name"].as_str()?;
    let metadata = Path::new(message["filenames"].get(0)?.as_str()?);
    let built = metadata.parent()?.parent()?;
    let profile = match built.strip_prefix(&dirs.build) {
        Ok(profile) => dirs.target.join(profile),
        Err(_) => built.to_owned(),
    };
    is_bin.then(|| profile.join(format!("{name}{}", std::env::consts::EXE_SUFFIX)))
}

/// The dep-info file Cargo writes beside `binary`, which lists the files it
/// was built from.
fn dep_info_of(binary: &Path) -> PathBuf {
    let name = match binary.extension() {
        Some(extension) if extension == "exe" => binary.file_stem(),
        _ => binary.file_name(),
    }
    .unwrap_or_default();
    let mut file = name.to_owned();
    file.push(".d");
    binary.with_file_name(file)
}

/// The files a dep-info file lists after its target, without the ones under
/// Cargo's home, which hold downloaded crates that do not change.
///
/// The file holds `target: source source ...` on its first line, with a space
/// inside a path written as `\ `.
fn parse_dep_info(text: &str) -> BTreeSet<PathBuf> {
    let home = cargo_home();
    let line = text.lines().next().unwrap_or_default();
    let Some((_, list)) = line.split_once(": ") else {
        return BTreeSet::new();
    };
    let mut paths = BTreeSet::new();
    let mut path = String::new();
    let mut chars = list.chars();
    while let Some(c) = chars.next() {
        match c {
            '\\' => match chars.next() {
                Some(' ') => path.push(' '),
                Some(next) => {
                    path.push('\\');
                    path.push(next);
                }
                None => path.push('\\'),
            },
            ' ' => {
                if !path.is_empty() {
                    paths.insert(PathBuf::from(std::mem::take(&mut path)));
                }
            }
            c => path.push(c),
        }
    }
    if !path.is_empty() {
        paths.insert(PathBuf::from(path));
    }
    paths.retain(|path| home.as_ref().is_none_or(|home| !path.starts_with(home)));
    paths
}

/// Cargo's home directory, where it keeps the crates it downloads.
fn cargo_home() -> Option<PathBuf> {
    std::env::var_os("CARGO_HOME")
        .map(PathBuf::from)
        .or_else(|| std::env::home_dir().map(|home| home.join(".cargo")))
}

/// The modification time of each file, or `None` where it cannot be read.
type Snapshot = BTreeMap<PathBuf, Option<SystemTime>>;

/// The modification time of each file the next build may read: each file
/// the last build was built from, and each Rust file and manifest under
/// `dir`, which covers a file added since and a build that has not yet
/// succeeded.
fn snapshot(dir: &Path, built: Option<&Built>) -> Snapshot {
    let mut files: Snapshot = built
        .iter()
        .flat_map(|built| &built.sources)
        .map(|path| (path.clone(), modified(path)))
        .collect();
    walk(dir, &mut files);
    files
}

/// The files of `after`, each at the time `before` read for it where
/// `before` holds it, which is the state the build read. A file saved while
/// the build ran then differs from this, and starts the next build.
fn as_built(before: &Snapshot, after: Snapshot) -> Snapshot {
    after
        .into_iter()
        .map(|(path, time)| {
            let time = before.get(&path).copied().unwrap_or(time);
            (path, time)
        })
        .collect()
}

/// Adds each Rust file and manifest under `dir` to `files`, skipping build
/// output, hidden directories, and `node_modules`.
fn walk(dir: &Path, files: &mut Snapshot) {
    let Ok(entries) = std::fs::read_dir(dir) else {
        return;
    };
    for entry in entries.flatten() {
        let path = entry.path();
        let name = entry.file_name();
        let name = name.to_string_lossy();
        let Ok(kind) = entry.file_type() else {
            continue;
        };
        if kind.is_dir() {
            if !(name.starts_with('.') || name == "target" || name == "node_modules") {
                walk(&path, files);
            }
        } else if name.ends_with(".rs") || name == "Cargo.toml" || name == "Cargo.lock" {
            let time = modified(&path);
            files.insert(path, time);
        }
    }
}

fn modified(path: &Path) -> Option<SystemTime> {
    std::fs::metadata(path)
        .and_then(|meta| meta.modified())
        .ok()
}

/// The line that names what `snapshot` reads, from the number of files the
/// walk of `dir` found and the number it reads with the build's sources.
fn watching(dir: &Path, walked: usize, watched: usize) -> String {
    let more = match watched.saturating_sub(walked) {
        0 => String::new(),
        1 => ", and 1 more file the build read".to_owned(),
        more => format!(", and {more} more files the build read"),
    };
    format!(
        "Watching the `.rs` files, `Cargo.toml`, and `Cargo.lock` under {}{more}",
        dir.display()
    )
}

/// Starts the server, which shares the terminal, so it prints there and
/// stops with the same Ctrl+C, and prints `verb` and the binary once it runs.
///
/// The line names no address, because the server picks its own and prints
/// it where it prints one.
fn start(dir: &Path, binary: &Path, program_args: &[String], verb: &str) -> Result<Child, Error> {
    let child = Command::new(binary)
        .args(program_args)
        .spawn()
        .map_err(|error| Error::new(format!("cannot run `{}`: {error}", binary.display())))?;
    let shown = binary.strip_prefix(dir).unwrap_or(binary);
    eprintln!("{verb} `{}`", shown.display());
    Ok(child)
}

/// Stops the server at once. A graceful shutdown would wait for the stream
/// each open page holds, which ends only when the server stops.
fn stop(child: &mut Child) {
    // An error means the process has already exited, which is the aim.
    let _ = child.kill();
    let _ = child.wait();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reads_the_binary_from_a_bin_artifact_only() {
        let bin = r#"{"reason":"compiler-artifact","target":{"kind":["bin"]},"executable":"/p/target/debug/app"}"#;
        let lib = r#"{"reason":"compiler-artifact","target":{"kind":["lib"]},"executable":null}"#;
        let done = r#"{"reason":"build-finished","success":true}"#;
        assert_eq!(binary_of(bin), Some(PathBuf::from("/p/target/debug/app")));
        assert_eq!(binary_of(lib), None);
        assert_eq!(binary_of(done), None);
        assert_eq!(binary_of("not json"), None);
    }

    #[test]
    fn puts_the_binary_of_a_checked_bin_above_its_metadata() {
        let bin = r#"{"reason":"compiler-artifact","target":{"kind":["bin"],"name":"my-app"},"filenames":["/p/target/debug/deps/libmy_app-0123.rmeta"],"executable":null}"#;
        let lib = r#"{"reason":"compiler-artifact","target":{"kind":["lib"],"name":"core"},"filenames":["/p/target/debug/deps/libcore-0123.rmeta"],"executable":null}"#;
        let expected = format!("/p/target/debug/my-app{}", std::env::consts::EXE_SUFFIX);
        let dirs = Dirs {
            target: PathBuf::from("/p/target"),
            build: PathBuf::from("/p/target"),
        };
        assert_eq!(checked_binary_of(bin, &dirs), Some(PathBuf::from(expected)));
        assert_eq!(checked_binary_of(lib, &dirs), None);
        assert_eq!(checked_binary_of("not json", &dirs), None);
    }

    #[test]
    fn puts_the_binary_of_a_checked_bin_in_the_target_directory_when_the_build_directory_is_apart()
    {
        let bin = r#"{"reason":"compiler-artifact","target":{"kind":["bin"],"name":"app"},"filenames":["/p/bdir/x86_64-unknown-linux-gnu/debug/deps/libapp-0123.rmeta"],"executable":null}"#;
        let dirs = Dirs {
            target: PathBuf::from("/p/target"),
            build: PathBuf::from("/p/bdir"),
        };
        let expected = format!(
            "/p/target/x86_64-unknown-linux-gnu/debug/app{}",
            std::env::consts::EXE_SUFFIX
        );
        assert_eq!(checked_binary_of(bin, &dirs), Some(PathBuf::from(expected)));
    }

    #[test]
    fn reads_a_flag_written_with_a_space_or_an_equals_sign() {
        let args = [
            "--target-dir".to_owned(),
            "t".to_owned(),
            "--manifest-path=m".to_owned(),
        ];
        assert_eq!(value_of(&args, "--target-dir"), Some("t"));
        assert_eq!(value_of(&args, "--manifest-path"), Some("m"));
        assert_eq!(value_of(&args, "--bin"), None);
    }

    #[test]
    fn keeps_the_time_a_build_read_for_a_file_saved_during_it() {
        let read = Some(SystemTime::UNIX_EPOCH);
        let saved = Some(SystemTime::UNIX_EPOCH + Duration::from_secs(1));
        let before = Snapshot::from([(PathBuf::from("src/main.rs"), read)]);
        let after = Snapshot::from([
            (PathBuf::from("src/main.rs"), saved),
            (PathBuf::from("assets/a.js"), read),
        ]);
        assert_eq!(
            as_built(&before, after),
            Snapshot::from([
                (PathBuf::from("assets/a.js"), read),
                (PathBuf::from("src/main.rs"), read),
            ])
        );
    }

    #[test]
    fn finds_the_dep_info_beside_the_binary() {
        assert_eq!(
            dep_info_of(Path::new("/p/target/debug/my.app")),
            PathBuf::from("/p/target/debug/my.app.d")
        );
        assert_eq!(
            dep_info_of(Path::new("/p/target/debug/app.exe")),
            PathBuf::from("/p/target/debug/app.d")
        );
    }

    #[test]
    fn reads_the_sources_of_a_dep_info_file_with_escaped_spaces() {
        let text = "/p/target/debug/app: /p/src/main.rs /p/my\\ dir/page.rs /p/assets/a.js\n";
        let paths: Vec<PathBuf> = parse_dep_info(text).into_iter().collect();
        assert_eq!(
            paths,
            [
                PathBuf::from("/p/assets/a.js"),
                PathBuf::from("/p/my dir/page.rs"),
                PathBuf::from("/p/src/main.rs"),
            ]
        );
    }
}
