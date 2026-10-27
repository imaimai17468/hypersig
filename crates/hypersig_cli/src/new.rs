//! `cargo hypersig new`: writes a host's template into a new directory.

use std::path::Path;
use std::process::{Command, Stdio};

use crate::Error;
use crate::host::Host;

pub fn run(path: &Path, host: Host) -> Result<(), Error> {
    let package = path
        .file_name()
        .and_then(|name| name.to_str())
        .ok_or_else(|| Error::new(format!("`{}` does not end in a name", path.display())))?;
    check_name(package, host)?;
    if path.exists() {
        return Err(Error::new(format!(
            "`{}` already exists; `cargo hypersig add {host}` moves a project that is there",
            path.display()
        )));
    }
    for file in host.files() {
        let dest = path.join(file.path);
        if let Some(parent) = dest.parent() {
            std::fs::create_dir_all(parent).map_err(|error| {
                Error::new(format!("cannot create `{}`: {error}", parent.display()))
            })?;
        }
        std::fs::write(&dest, host.fill(file.text, package))
            .map_err(|error| Error::new(format!("cannot write `{}`: {error}", dest.display())))?;
    }
    let git = init_git(path);
    println!(
        "Created `{package}` from the {host} template at {}.",
        path.display()
    );
    if !git {
        println!("It is not a Git repository; run `git init` there to make it one.");
    }
    println!("\nNext:\n  cd {}", path.display());
    // The command that serves the project on this machine, then the
    // commands of the host's deploy page, after `cd`.
    let next = match host {
        Host::Server => format!(
            "  cargo hypersig dev                        # http://localhost:3000\n  docker build -t {package} .\n  docker run --rm -p 3000:3000 {package}    # http://localhost:3000"
        ),
        Host::Cloudflare => "  npx wrangler dev                          # http://localhost:8787\n  npx wrangler login\n  npx wrangler deploy".to_owned(),
        Host::Vercel => "  cargo run                                 # http://localhost:3000\n  npx vercel login\n  npx vercel deploy --prod".to_owned(),
    };
    println!("{next}");
    Ok(())
}

/// Refuses a name that Cargo refuses for the package or its binary, or
/// that the host's tool refuses, before any file is written, so that a
/// mistyped name leaves nothing behind.
///
/// The name becomes the Docker image of the server template, whose
/// repository names are lowercase, and the Worker of the Cloudflare
/// template, whose names hold no `_`.
fn check_name(name: &str, host: Host) -> Result<(), Error> {
    let starts_well = name.starts_with(|c: char| c.is_ascii_lowercase() || c == '_');
    let rest_well = name
        .chars()
        .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '-' || c == '_');
    if !(starts_well && rest_well) {
        return Err(Error::new(format!(
            "`{name}` cannot name a project; use lowercase ASCII letters, digits, `-`, and `_`, starting with a letter"
        )));
    }
    if host == Host::Cloudflare && name.contains('_') {
        return Err(Error::new(format!(
            "`{name}` cannot name a Worker, which holds no `_`; use `{}`",
            name.replace('_', "-")
        )));
    }
    if RESERVED.contains(&name) || crate::elements::KEYWORDS.contains(&name) {
        return Err(Error::new(format!(
            "`{name}` cannot name a package: Cargo refuses a Rust keyword, `test`, and the names of the directories under `target/debug`"
        )));
    }
    Ok(())
}

/// Names Cargo refuses for a package or its binary besides the keywords:
/// the directories of `target/<profile>/`, the name of the built-in test
/// crate, and the path keywords a raw identifier cannot spell.
const RESERVED: [&str; 8] = [
    "build",
    "deps",
    "examples",
    "incremental",
    "test",
    "self",
    "super",
    "crate",
];

/// Makes the project a Git repository, as `cargo new` does, unless it sits
/// in one already. Returns whether the project is in a repository.
fn init_git(path: &Path) -> bool {
    let quiet = |command: &mut Command| {
        command
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .status()
            .is_ok_and(|status| status.success())
    };
    quiet(
        Command::new("git")
            .arg("-C")
            .arg(path)
            .args(["rev-parse", "--is-inside-work-tree"]),
    ) || quiet(Command::new("git").arg("init").arg("--quiet").arg(path))
}

#[cfg(test)]
mod tests {
    use super::check_name;
    use crate::host::Host;

    fn refuses(name: &str, host: Host) -> bool {
        check_name(name, host).is_err()
    }

    #[test]
    fn refuses_a_name_cargo_or_the_host_refuses() {
        for name in [
            "build", "deps", "test", "fn", "yield", "self", "MyApp", "1app", "a.b",
        ] {
            assert!(refuses(name, Host::Server), "{name}");
        }
        assert!(refuses("my_app", Host::Cloudflare));
        for name in ["my-app", "my_app", "app2", "builder"] {
            assert!(!refuses(name, Host::Server), "{name}");
        }
        assert!(!refuses("my-app", Host::Cloudflare));
    }
}
