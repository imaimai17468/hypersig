//! `cargo hypersig`: creates a hypersig project for a host, moves the
//! project in the current directory to one, or writes the Rust names of a
//! library's custom elements.
//!
//! The templates are compiled into the binary, so a project made with one
//! release of this crate always holds the same files, and the build commands
//! in them (the `Dockerfile`'s `RUN`, the `[build]` of `wrangler.toml`, the
//! `buildCommand` of `vercel.json`) are the ones crates.io published.

mod add;
mod dev;
mod diff;
mod elements;
mod host;
mod new;

use std::fmt;
use std::path::PathBuf;
use std::process::ExitCode;

use host::Host;

/// The `Hosts:` lines of the usage, which the refusal of a `new` without
/// `--host` prints too.
macro_rules! hosts {
    () => {
        "\
Hosts:
  server       A container image, for a host such as Fly.io, Cloud Run, or Render
  cloudflare   Cloudflare Workers
  vercel       A Vercel Function"
    };
}

const USAGE: &str = concat!(
    "\
Usage:
  cargo hypersig new <path> --host <host>   Create a project from the host's template
  cargo hypersig add <host> [--force]       Move the project here to the host
  cargo hypersig dev [<build options>] [-- <args>]
                                            Build and run the server, and again after each
                                            change; in a debug build, the open pages reload
                                            from the new one
  cargo hypersig elements <custom-elements.json>
                                            Print a Rust module of the custom elements,
                                            attributes, and events that a library's
                                            Custom Elements Manifest declares

",
    hosts!(),
    "

`add` writes no file that is already there unless it is given --force.
`dev` passes <build options>, such as -p or --bin, to `cargo build`, and <args> to the server.
`elements` prints the module, which a redirect such as `> src/shoelace.rs` saves."
);

/// A failure to report to the person running the command, as one message.
#[derive(Debug)]
pub struct Error(String);

impl Error {
    pub fn new(message: impl Into<String>) -> Self {
        Self(message.into())
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

impl std::error::Error for Error {}

enum Command {
    New { path: PathBuf, host: Host },
    Add { host: Host, force: bool },
    Elements { manifest: PathBuf },
    Help,
    Version,
}

fn parse(mut args: impl Iterator<Item = String>) -> Result<Command, Error> {
    let mut positional = Vec::new();
    let mut host = None;
    let mut force = false;
    while let Some(arg) = args.next() {
        match arg.as_str() {
            "-h" | "--help" => return Ok(Command::Help),
            "-V" | "--version" => return Ok(Command::Version),
            "--force" => force = true,
            "--host" => {
                let name = args.next().ok_or_else(|| {
                    Error::new("--host needs a host: server, cloudflare, or vercel")
                })?;
                host = Some(name.parse()?);
            }
            _ => match arg.strip_prefix("--host=") {
                Some(name) => host = Some(name.parse()?),
                None if arg.starts_with('-') => {
                    return Err(Error::new(format!("unknown option `{arg}`\n\n{USAGE}")));
                }
                None => positional.push(arg),
            },
        }
    }
    let mut positional = positional.into_iter();
    let command = match positional.next().as_deref() {
        Some("new") => {
            if force {
                return Err(Error::new(
                    "`new` writes only into a directory that does not exist, so it takes no --force",
                ));
            }
            let path = positional
                .next()
                .ok_or_else(|| Error::new(format!("`new` needs the project's path\n\n{USAGE}")))?;
            let host = host.ok_or_else(|| {
                Error::new(format!(
                    "`new` needs --host, which picks the template's files and the crate's shape\n\n{}\n\nFor example:\n  cargo hypersig new {path} --host server",
                    hosts!()
                ))
            })?;
            Command::New {
                path: path.into(),
                host,
            }
        }
        Some("add") => {
            if host.is_some() {
                return Err(Error::new(
                    "`add` takes the host as its argument: cargo hypersig add <host>",
                ));
            }
            let host = positional
                .next()
                .ok_or_else(|| Error::new(format!("`add` needs a host\n\n{USAGE}")))?
                .parse()?;
            Command::Add { host, force }
        }
        Some("elements") => {
            if force || host.is_some() {
                return Err(Error::new(
                    "`elements` prints the module and takes only the manifest's path: cargo hypersig elements <custom-elements.json>",
                ));
            }
            let manifest = positional.next().ok_or_else(|| {
                Error::new(format!(
                    "`elements` needs the path of a custom-elements.json\n\n{USAGE}"
                ))
            })?;
            Command::Elements {
                manifest: manifest.into(),
            }
        }
        Some(other) => return Err(Error::new(format!("unknown command `{other}`\n\n{USAGE}"))),
        None => return Ok(Command::Help),
    };
    if let Some(extra) = positional.next() {
        return Err(Error::new(format!(
            "unexpected argument `{extra}`\n\n{USAGE}"
        )));
    }
    Ok(command)
}

fn run() -> Result<(), Error> {
    let mut args = std::env::args().skip(1).peekable();
    // Cargo runs `cargo hypersig new ...` as `cargo-hypersig hypersig new ...`.
    if args.peek().map(String::as_str) == Some("hypersig") {
        args.next();
    }
    // The arguments of `dev` belong to `cargo build` and the server, so the
    // options of `new` and `add` are not read from them.
    if args.peek().map(String::as_str) == Some("dev") {
        args.next();
        let mut build: Vec<String> = Vec::new();
        let mut program = Vec::new();
        let mut after = false;
        for arg in args {
            if after {
                program.push(arg);
            } else if arg == "--" {
                after = true;
            } else if arg == "-h" || arg == "--help" {
                println!("{USAGE}");
                return Ok(());
            } else {
                build.push(arg);
            }
        }
        return dev::run(&build, &program);
    }
    match parse(args)? {
        Command::New { path, host } => new::run(&path, host),
        Command::Add { host, force } => {
            let dir = std::env::current_dir().map_err(|error| {
                Error::new(format!("cannot read the current directory: {error}"))
            })?;
            add::run(&dir, host, force)
        }
        Command::Elements { manifest } => elements::run(&manifest),
        Command::Help => {
            println!("{USAGE}");
            Ok(())
        }
        Command::Version => {
            println!("cargo-hypersig {}", env!("CARGO_PKG_VERSION"));
            Ok(())
        }
    }
}

fn main() -> ExitCode {
    match run() {
        Ok(()) => ExitCode::SUCCESS,
        Err(error) => {
            eprintln!("error: {error}");
            ExitCode::FAILURE
        }
    }
}
