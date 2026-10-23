//! The three hosts and the template each one's projects start from.

use std::fmt;
use std::str::FromStr;

use crate::Error;

/// A host that a hypersig project can be deployed to.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Host {
    /// A container image, for a host such as Fly.io, Cloud Run, or Render.
    Server,
    /// Cloudflare Workers, which runs the crate compiled to wasm.
    Cloudflare,
    /// Vercel, which runs a binary under `api/` as a Vercel Function.
    Vercel,
}

/// What a file of a template is to `add`, which treats each kind differently.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Kind {
    /// `Cargo.toml`, which `add` edits in place.
    Manifest,
    /// The Rust code, which `add` leaves to the reader.
    Source,
    /// An ignore file, which `add` appends its missing lines to.
    Ignore,
    /// A file the host reads, which `add` writes as its `moved` text.
    Config,
    /// A file a coding agent reads, which `add` writes only where the
    /// project has none, since a project's own instructions outrank it.
    Guide,
}

/// One file of a template.
#[derive(Debug)]
pub struct File {
    /// Where the file goes in the project.
    pub path: &'static str,
    /// The text, with `{{project-name}}`, `{{hypersig-version}}`, and
    /// `{{host-notes}}` in it.
    pub text: &'static str,
    /// What `add` writes in place of `text`, which leaves out what only the
    /// template's own code uses.
    pub moved: &'static str,
    pub kind: Kind,
}

/// One file of `templates/$host`, read at compile time.
///
/// A template's manifest is `Cargo.toml.template` in the repository, because
/// `cargo package` leaves out a directory that holds a `Cargo.toml`, and it
/// is written to the project as `Cargo.toml`.
macro_rules! file {
    ($host:literal, "Cargo.toml") => {
        File {
            path: "Cargo.toml",
            text: include_str!(concat!("../templates/", $host, "/Cargo.toml.template")),
            moved: include_str!(concat!("../templates/", $host, "/Cargo.toml.template")),
            kind: Kind::Manifest,
        }
    };
    ($host:literal, $path:literal, $kind:ident) => {
        File {
            path: $path,
            text: include_str!(concat!("../templates/", $host, "/", $path)),
            moved: include_str!(concat!("../templates/", $host, "/", $path)),
            kind: Kind::$kind,
        }
    };
    // `AGENTS.md` is one text for every host, with the host's notes from
    // `templates/$host/AGENTS.host.md` in place of `{{host-notes}}`.
    ($host:literal, guide $path:literal) => {
        File {
            path: $path,
            text: include_str!(concat!("../templates/", $path)),
            moved: include_str!(concat!("../templates/", $path)),
            kind: Kind::Guide,
        }
    };
    ($host:literal, $path:literal, $kind:ident, moved = $moved:literal) => {
        File {
            path: $path,
            text: include_str!(concat!("../templates/", $host, "/", $path)),
            moved: include_str!(concat!("../steps/", $moved)),
            kind: Kind::$kind,
        }
    };
}

const SERVER: &[File] = &[
    file!("server", "Cargo.toml"),
    file!("server", "src/main.rs", Source),
    file!("server", "Dockerfile", Config),
    file!("server", ".dockerignore", Ignore),
    file!("server", ".gitignore", Ignore),
    file!("server", guide "AGENTS.md"),
    file!("server", guide "CLAUDE.md"),
];

const CLOUDFLARE: &[File] = &[
    file!("cloudflare", "Cargo.toml"),
    file!("cloudflare", "src/lib.rs", Source),
    // The template's worker saves its count in D1, which a project that
    // `add` moves has no use for.
    file!(
        "cloudflare",
        "wrangler.toml",
        Config,
        moved = "wrangler.toml"
    ),
    file!("cloudflare", "rust-toolchain.toml", Config),
    file!("cloudflare", ".gitignore", Ignore),
    file!("cloudflare", guide "AGENTS.md"),
    file!("cloudflare", guide "CLAUDE.md"),
];

const VERCEL: &[File] = &[
    file!("vercel", "Cargo.toml"),
    file!("vercel", "api/app.rs", Source),
    file!("vercel", "vercel.json", Config),
    file!("vercel", "rust-toolchain.toml", Config),
    file!("vercel", ".vercelignore", Ignore),
    file!("vercel", ".gitignore", Ignore),
    file!("vercel", guide "AGENTS.md"),
    file!("vercel", guide "CLAUDE.md"),
];

impl Host {
    pub const ALL: [Self; 3] = [Self::Server, Self::Cloudflare, Self::Vercel];

    /// The name the command line takes, which is also the template's
    /// directory under `templates/`.
    pub fn name(self) -> &'static str {
        match self {
            Self::Server => "server",
            Self::Cloudflare => "cloudflare",
            Self::Vercel => "vercel",
        }
    }

    pub fn files(self) -> &'static [File] {
        match self {
            Self::Server => SERVER,
            Self::Cloudflare => CLOUDFLARE,
            Self::Vercel => VERCEL,
        }
    }

    /// The template's `Cargo.toml` with its placeholders filled.
    pub fn manifest(self, package: &str) -> String {
        let file = self
            .files()
            .iter()
            .find(|file| file.kind == Kind::Manifest)
            .expect("every template lists its Cargo.toml");
        self.fill(file.text, package)
    }

    /// The paragraphs of `AGENTS.md` about this host.
    fn notes(self) -> &'static str {
        match self {
            Self::Server => include_str!("../templates/server/AGENTS.host.md"),
            Self::Cloudflare => include_str!("../templates/cloudflare/AGENTS.host.md"),
            Self::Vercel => include_str!("../templates/vercel/AGENTS.host.md"),
        }
    }

    /// `text` with the package's name, this CLI's version, and the host's
    /// notes in place of the placeholders. The CLI is released with hypersig
    /// at the same version, so its version is the hypersig the template was
    /// tested against.
    pub fn fill(self, text: &str, package: &str) -> String {
        text.replace("{{project-name}}", package)
            .replace("{{hypersig-version}}", env!("CARGO_PKG_VERSION"))
            .replace("{{host-notes}}", self.notes().trim_end())
    }
}

impl fmt::Display for Host {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.name())
    }
}

impl FromStr for Host {
    type Err = Error;

    fn from_str(name: &str) -> Result<Self, Error> {
        Self::ALL
            .into_iter()
            .find(|host| host.name() == name)
            .ok_or_else(|| {
                Error::new(format!(
                    "`{name}` is not a host; use server, cloudflare, or vercel"
                ))
            })
    }
}
