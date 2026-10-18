//! Writes the documentation site to a directory, for GitHub Pages.
//!
//! ```sh
//! cargo run -p site -- --out target/site --base /hypersig --rustdoc target/doc
//! ```
//!
//! `--rustdoc` copies the output of `cargo doc` into `api/` and checks each
//! link into it. Without it, those links are left unchecked.
//!
//! `--origin https://imaimai17468.github.io` gives each page a canonical URL
//! and an `og:url`, joined from the origin, the base path, and the page's
//! directory. A local preview leaves it out, and its pages have neither.

use std::path::{Path, PathBuf};
use std::process::ExitCode;
use std::{env, fs, io};

use hypersig::Origin;
use site::{API_DIR, File, Site, check_links, render};

struct Args {
    out: PathBuf,
    base: String,
    rustdoc: Option<PathBuf>,
    origin: Option<String>,
}

fn parse_args() -> Result<Args, String> {
    let mut args = Args {
        out: PathBuf::from("target/site"),
        base: "/hypersig".to_owned(),
        rustdoc: None,
        origin: None,
    };
    let mut rest = env::args().skip(1);
    while let Some(flag) = rest.next() {
        let mut value = || rest.next().ok_or(format!("{flag} takes a value"));
        match flag.as_str() {
            "--out" => args.out = PathBuf::from(value()?),
            "--base" => args.base = value()?,
            "--rustdoc" => args.rustdoc = Some(PathBuf::from(value()?)),
            "--origin" => args.origin = Some(value()?),
            _ => return Err(format!("unknown argument {flag}")),
        }
    }
    Ok(args)
}

fn main() -> ExitCode {
    let args = match parse_args() {
        Ok(args) => args,
        Err(message) => {
            eprintln!(
                "{message}\nusage: site [--out DIR] [--base /PATH] [--rustdoc DIR] [--origin https://HOST]"
            );
            return ExitCode::FAILURE;
        }
    };
    match run(&args) {
        Ok(true) => ExitCode::SUCCESS,
        Ok(false) => ExitCode::FAILURE,
        Err(error) => {
            eprintln!("{error}");
            ExitCode::FAILURE
        }
    }
}

/// Writes the site and reports whether every link inside it resolves.
fn run(args: &Args) -> io::Result<bool> {
    let mut site = Site::new(String::leak(args.base.clone()));
    if let Some(origin) = &args.origin {
        let origin = Origin::parse(String::leak(origin.clone()))
            .map_err(|error| io::Error::new(io::ErrorKind::InvalidInput, error))?;
        site = site.with_origin(origin);
    }
    let files = render(site);
    if args.out.exists() {
        fs::remove_dir_all(&args.out)?;
    }
    for File { path, contents } in &files {
        let target = args.out.join(path);
        if let Some(parent) = target.parent() {
            fs::create_dir_all(parent)?;
        }
        fs::write(target, contents)?;
    }
    let api = args.out.join(API_DIR);
    if let Some(rustdoc) = &args.rustdoc {
        copy_dir(rustdoc, &api)?;
    }
    let broken = check_links(site, &files, |item, fragment| {
        args.rustdoc.is_none() || rustdoc_has(&api, item, fragment)
    });
    for link in &broken {
        eprintln!("{}: broken link to {}", link.page, link.target);
    }
    println!("wrote {} files to {}", files.len(), args.out.display());
    Ok(broken.is_empty())
}

/// Whether the rustdoc page `item` exists under `api`, holding an element
/// with the id `fragment` if one is given.
fn rustdoc_has(api: &Path, item: &str, fragment: Option<&str>) -> bool {
    let Ok(html) = fs::read_to_string(api.join(item)) else {
        return false;
    };
    fragment.is_none_or(|id| html.contains(&format!("id=\"{id}\"")))
}

fn copy_dir(from: &Path, to: &Path) -> io::Result<()> {
    fs::create_dir_all(to)?;
    for entry in fs::read_dir(from)? {
        let entry = entry?;
        let target = to.join(entry.file_name());
        if entry.file_type()?.is_dir() {
            copy_dir(&entry.path(), &target)?;
        } else {
            fs::copy(entry.path(), target)?;
        }
    }
    Ok(())
}
