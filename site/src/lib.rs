//! The hypersig documentation site, rendered to static HTML files.
//!
//! [`render`] builds every page with hypersig and `hypersig_ui`, and
//! [`check_links`] confirms that each link inside the site reaches a file
//! that [`render`] writes. The binary in `main.rs` writes the files for
//! GitHub Pages.

#![expect(
    clippy::too_many_lines,
    reason = "a page's content function holds its text in each language, so its length follows the page's text"
)]

mod code;
mod components;
mod demo;
mod doc;
mod home;
mod landing;
mod lang;
mod layout;
mod learn;
mod prose;
mod reference;
mod style;

use std::collections::BTreeSet;
use std::path::Path;

use hypersig_i18n::Locale as _;

pub use doc::Doc;
pub use lang::Lang;
use lang::{L, Text};

/// Where the site is served from.
#[derive(Debug, Clone, Copy)]
pub struct Site {
    base: &'static str,
    runtime: &'static str,
    lang: Lang,
}

impl Site {
    /// A site served under `base`, such as `/hypersig` for a GitHub Pages
    /// project site, or `""` at the root of a host.
    ///
    /// # Panics
    ///
    /// Panics if `base` is not empty and does not start with `/`, or ends
    /// with `/`.
    #[must_use]
    pub fn new(base: &'static str) -> Self {
        assert!(
            base.is_empty() || (base.starts_with('/') && !base.ends_with('/')),
            "a base path is empty or starts with `/` and does not end with one"
        );
        // `Page::runtime_src` keeps the URL for as long as the page lives,
        // and the generator builds one `Site` per run.
        let runtime = String::leak(format!("{base}/{RUNTIME_FILE}"));
        Self {
            base,
            runtime,
            lang: Lang::En,
        }
    }

    /// The same site, rendering its pages in `lang`.
    #[must_use]
    pub fn with_lang(self, lang: Lang) -> Self {
        Self { lang, ..self }
    }

    /// The language the pages are rendered in.
    #[must_use]
    pub fn lang(self) -> Lang {
        self.lang
    }

    /// `text` in the language the pages are rendered in.
    fn t<T>(self, text: L<T>) -> T {
        self.lang.pick(text)
    }

    /// `text` in the language the pages are rendered in, translated or not.
    fn text<'a, T: ?Sized>(self, text: impl Into<Text<'a, T>>) -> &'a T {
        text.into().get(self.lang)
    }

    /// The URL of `path`, a file inside the site without a leading `/` that
    /// every language shares, such as the runtime or the rustdoc output.
    fn url(self, path: &str) -> String {
        format!("{}/{path}", self.base)
    }

    /// The URL of the page at `path`, a page's directory inside the site, in
    /// the language the pages are rendered in.
    fn page_url(self, path: &str) -> String {
        format!("{}/{}{path}", self.base, self.lang.dir())
    }

    /// The URL the pages load the runtime from.
    fn runtime(self) -> &'static str {
        self.runtime
    }
}

/// A file of the site.
#[derive(Debug)]
pub struct File {
    /// The path inside the output directory, without a leading `/`.
    pub path: String,
    /// The file's contents.
    pub contents: String,
}

/// The file the pages load the runtime from.
const RUNTIME_FILE: &str = "hypersig.js";

/// The directory the rustdoc output is copied into, inside the site.
pub const API_DIR: &str = "api";

/// Every file of the site: the pages, the live demos, and the runtime.
#[must_use]
pub fn render(site: Site) -> Vec<File> {
    let mut files = Vec::new();
    for &lang in Lang::ALL {
        let site = site.with_lang(lang);
        files.extend(Doc::ALL.iter().map(|&doc| File {
            path: format!("{}{}index.html", lang.dir(), doc.path()),
            contents: doc.render(site),
        }));
        files.extend(demo::Demo::ALL.iter().map(|&demo| File {
            path: format!("{}{}index.html", lang.dir(), demo.path()),
            contents: demo.render(site),
        }));
    }
    files.push(File {
        path: RUNTIME_FILE.to_owned(),
        contents: hypersig::RUNTIME_JS.to_owned(),
    });
    files
}

/// A link inside the site that names no file of the site.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BrokenLink {
    /// The page the link is on.
    pub page: String,
    /// The link's target, as written in the page.
    pub target: String,
}

/// The links in `files` that point inside `site` at a file `files` does not
/// hold.
///
/// A link into [`API_DIR`] is checked by `api`, which receives the path after
/// `api/` and the fragment after `#`, if any. Pass a function that returns
/// `true` to leave those links unchecked.
#[must_use]
pub fn check_links(
    site: Site,
    files: &[File],
    api: impl Fn(&str, Option<&str>) -> bool,
) -> Vec<BrokenLink> {
    let known: BTreeSet<&str> = files.iter().map(|file| file.path.as_str()).collect();
    let api_prefix = format!("{API_DIR}/");
    let mut broken = Vec::new();
    for file in files.iter().filter(|file| {
        Path::new(&file.path)
            .extension()
            .is_some_and(|ext| ext == "html")
    }) {
        for target in links(&file.contents) {
            let Some(inside) = target
                .strip_prefix(site.base)
                .and_then(|t| t.strip_prefix('/'))
            else {
                continue;
            };
            let (path, fragment) = match inside.split_once('#') {
                Some((path, fragment)) => (path, Some(fragment)),
                None => (inside, None),
            };
            let found = if let Some(item) = path.strip_prefix(&api_prefix) {
                api(item, fragment)
            } else if path.is_empty() || path.ends_with('/') {
                known.contains(format!("{path}index.html").as_str())
            } else {
                known.contains(path)
            };
            if !found {
                broken.push(BrokenLink {
                    page: file.path.clone(),
                    target: target.to_owned(),
                });
            }
        }
    }
    broken
}

/// The values of the `href` and `src` attributes in `html` that start with
/// `/`, which are the links inside the site.
///
/// Only the inside of each tag is searched, since text holds no `<` of its
/// own: the renderer escapes it.
fn links(html: &str) -> impl Iterator<Item = &str> {
    html.split('<')
        .filter_map(|rest| rest.find('>').map(|end| &rest[..end]))
        .flat_map(|tag| {
            [" href=\"/", " src=\"/"]
                .into_iter()
                .filter_map(move |marker| {
                    let start = tag.find(marker)? + marker.len() - 1;
                    tag[start..].find('"').map(|end| &tag[start..start + end])
                })
        })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_link_inside_the_site_reaches_a_page() {
        for base in ["/hypersig", ""] {
            let site = Site::new(base);
            let files = render(site);
            let broken = check_links(site, &files, |_, _| true);
            assert!(
                broken.is_empty(),
                "broken links under {base:?}: {broken:#?}"
            );
        }
    }

    #[test]
    fn a_link_to_a_missing_page_is_reported() {
        let site = Site::new("/hypersig");
        let files = vec![File {
            path: "index.html".to_owned(),
            contents: r#"<a href="/hypersig/learn/">Learn</a><a href="/hypersig/">Home</a><a href="https://example.com/">out</a><code>href="/hypersig/x"</code>"#.to_owned(),
        }];
        let broken = check_links(site, &files, |_, _| true);
        assert_eq!(
            broken,
            [BrokenLink {
                page: "index.html".to_owned(),
                target: "/hypersig/learn/".to_owned(),
            }]
        );
    }

    #[test]
    fn every_page_has_its_title_and_loads_the_runtime_from_the_base() {
        let site = Site::new("/hypersig");
        for &doc in Doc::ALL {
            let html = doc.render(site);
            assert!(html.contains(r#"src="/hypersig/hypersig.js""#), "{doc:?}");
            assert!(
                html.contains(&format!("<title>{}", doc.title(site))),
                "{doc:?}"
            );
        }
    }
}
