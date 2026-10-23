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
    check_name(package)?;
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
    // The commands of the host's deploy page, after `cd`.
    let next = match host {
        Host::Server => format!(
            "  docker build -t {package} .\n  docker run --rm -p 3000:3000 {package}    # http://localhost:3000"
        ),
        Host::Cloudflare => "  npx wrangler dev                          # http://localhost:8787\n  npx wrangler login\n  npx wrangler deploy".to_owned(),
        Host::Vercel => "  cargo run                                 # http://localhost:3000\n  npx vercel login\n  npx vercel deploy --prod".to_owned(),
    };
    println!("{next}");
    Ok(())
}

/// Refuses a name that Cargo refuses as a package name, before any file is
/// written, so that a mistyped name leaves nothing behind.
fn check_name(name: &str) -> Result<(), Error> {
    let starts_well = name.starts_with(|c: char| c.is_ascii_alphabetic() || c == '_');
    let rest_well = name
        .chars()
        .all(|c| c.is_ascii_alphanumeric() || c == '-' || c == '_');
    if starts_well && rest_well {
        Ok(())
    } else {
        Err(Error::new(format!(
            "`{name}` cannot name a package; use ASCII letters, digits, `-`, and `_`, starting with a letter"
        )))
    }
}

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
