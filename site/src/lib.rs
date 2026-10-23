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
mod i18n;
mod landing;
mod lang;
mod layout;
mod learn;
mod prose;
mod reference;
mod style;
mod use_cases;

use std::collections::BTreeMap;
use std::fmt::Write as _;
use std::path::Path;

use hypersig::{Origin, PageResponse, Rest, Route, SafeUrl, Url};
use hypersig_i18n::Locale as _;

pub use doc::Doc;
pub use lang::Lang;
use lang::{L, Text};

/// Where the site is served from.
#[derive(Debug, Clone, Copy)]
pub struct Site {
    base: &'static str,
    runtime: &'static str,
    icon: &'static str,
    star_rating: &'static str,
    lang: Lang,
    origin: Option<Origin>,
}

// The site is written as files and nothing serves these routes, so the
// generated function goes unused. `PAGE` names a page's directory inside the
// site and builds the `Url` its canonical link takes.
hypersig::routes! {
    fn pages(Router<()>);
    const PAGE: Route<Rest> = "/{*path}" => unserved;
}

#[expect(dead_code, reason = "the site is written as files, not served")]
async fn unserved((): (), _path: Rest) -> PageResponse {
    PageResponse::not_found()
}

/// `url`, which the site wrote for one of its own files or pages, as the
/// `href` or `src` it becomes.
///
/// The site writes these URLs from its base path and its page paths, so a
/// URL that is not a `SafeUrl` is a bug in how the site builds them.
fn own(url: impl Into<String>) -> SafeUrl {
    SafeUrl::parse(url).expect("the site's own URLs have no scheme")
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
        let icon = String::leak(format!("{base}/{ICON_FILE}"));
        let star_rating = String::leak(format!("{base}/{STAR_RATING_FILE}"));
        Self {
            base,
            runtime,
            icon,
            star_rating,
            lang: Lang::En,
            origin: None,
        }
    }

    /// The same site, served from `origin`, such as
    /// `https://imaimai17468.github.io`, so that each page names its
    /// canonical URL. The site's base path goes under `origin`.
    #[must_use]
    pub fn with_origin(self, origin: Origin) -> Self {
        let origin = if self.base.is_empty() {
            origin
        } else {
            origin.under(self.base)
        };
        Self {
            origin: Some(origin),
            ..self
        }
    }

    /// The absolute URL of the page at `path` in the language the pages are
    /// rendered in, when the site knows its origin.
    fn canonical(self, path: &str) -> Option<(Origin, Url)> {
        self.origin.map(|origin| {
            let url = PAGE.url(Rest::new(format!("{}{path}", self.lang.dir())));
            (origin, url)
        })
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

    /// The URL of the hypersig icon, which the pages use as their tab icon.
    fn icon(self) -> &'static str {
        self.icon
    }

    /// The URL of the script that defines the custom element of the rating demo.
    fn star_rating(self) -> &'static str {
        self.star_rating
    }
}

/// A file of the site.
#[derive(Debug)]
pub struct File {
    /// The path inside the output directory, without a leading `/`.
    pub path: String,
    /// The file's contents.
    pub contents: Vec<u8>,
}

/// The file the pages load the runtime from.
const RUNTIME_FILE: &str = "hypersig.js";

/// The file of the hypersig icon.
const ICON_FILE: &str = "icon.svg";

/// The script that defines `<star-rating>`, which the rating demo loads.
const STAR_RATING_FILE: &str = "star-rating.js";

/// The list of the site's pages that search engines read.
const SITEMAP_FILE: &str = "sitemap.xml";

/// The index of the guide that coding agents read, as llmstxt.org describes
/// it, one in each language's directory.
const LLMS_FILE: &str = "llms.txt";

/// The file Google Search Console reads to confirm that the owner of the
/// site added it, under the name Search Console gave for
/// `https://imaimai17468.github.io/hypersig/`.
const SEARCH_CONSOLE_FILE: &str = "google925053ed05204fb8.html";

/// The picture a link preview shows for a page in `lang`, and its contents.
///
/// `assets/og-en.svg` and `assets/og-ja.svg` are the sources of the PNG files,
/// which headless Chrome draws at 1200×630, since link previews do not show
/// SVG.
fn og_image(lang: Lang) -> (&'static str, &'static [u8]) {
    match lang {
        Lang::En => ("og-en.png", include_bytes!("../../assets/og-en.png")),
        Lang::Ja => ("og-ja.png", include_bytes!("../../assets/og-ja.png")),
    }
}

/// The directory the rustdoc output is copied into, inside the site.
pub const API_DIR: &str = "api";

/// Every file of the site.
///
/// These are the pages, the live demos and the script of the rating demo,
/// the runtime, the icon, the link preview pictures, each language's
/// `llms.txt`, the Search Console file, and, for a site that knows its
/// origin, the sitemap.
#[must_use]
pub fn render(site: Site) -> Vec<File> {
    let mut files = Vec::new();
    for &lang in Lang::ALL {
        let site = site.with_lang(lang);
        files.extend(Doc::ALL.iter().map(|&doc| File {
            path: format!("{}{}index.html", lang.dir(), doc.path()),
            contents: doc.render(site).into_bytes(),
        }));
        files.extend(demo::Demo::ALL.iter().map(|&demo| File {
            path: format!("{}{}index.html", lang.dir(), demo.path()),
            contents: demo.render(site).into_bytes(),
        }));
        files.push(File {
            path: format!("{}{LLMS_FILE}", lang.dir()),
            contents: llms(site).into_bytes(),
        });
        let (path, contents) = og_image(lang);
        files.push(File {
            path: path.to_owned(),
            contents: contents.to_vec(),
        });
    }
    files.push(File {
        path: RUNTIME_FILE.to_owned(),
        contents: hypersig::RUNTIME_JS.as_bytes().to_vec(),
    });
    files.push(File {
        path: STAR_RATING_FILE.to_owned(),
        contents: demo::STAR_RATING_JS.as_bytes().to_vec(),
    });
    files.push(File {
        path: ICON_FILE.to_owned(),
        contents: include_bytes!("../../assets/hypersig.svg").to_vec(),
    });
    files.push(File {
        path: SEARCH_CONSOLE_FILE.to_owned(),
        contents: format!("google-site-verification: {SEARCH_CONSOLE_FILE}").into_bytes(),
    });
    if let Some(sitemap) = sitemap(site) {
        files.push(File {
            path: SITEMAP_FILE.to_owned(),
            contents: sitemap.into_bytes(),
        });
    }
    files
}

/// The absolute URL of the page at `path` in each language, English first.
fn absolute_urls(site: Site, path: &str) -> Option<Vec<(Lang, String)>> {
    Lang::ALL
        .iter()
        .map(|&lang| {
            let (origin, url) = site.with_lang(lang).canonical(path)?;
            Some((lang, format!("{origin}{url}")))
        })
        .collect()
}

/// The sitemap of every page in every language, each naming its
/// translations, or `None` for a site that does not know its origin, since a
/// sitemap holds absolute URLs. The demos are left out, as their pages ask
/// not to be indexed.
fn sitemap(site: Site) -> Option<String> {
    let mut xml = String::from(concat!(
        r#"<?xml version="1.0" encoding="UTF-8"?>"#,
        "\n",
        r#"<urlset xmlns="http://www.sitemaps.org/schemas/sitemap/0.9" xmlns:xhtml="http://www.w3.org/1999/xhtml">"#,
        "\n",
    ));
    for &doc in Doc::ALL {
        let urls = absolute_urls(site, &doc.path())?;
        for (_, loc) in &urls {
            writeln!(xml, "  <url>\n    <loc>{loc}</loc>").ok()?;
            for (lang, href) in &urls {
                writeln!(
                    xml,
                    r#"    <xhtml:link rel="alternate" hreflang="{}" href="{href}"/>"#,
                    lang.tag()
                )
                .ok()?;
            }
            xml.push_str("  </url>\n");
        }
    }
    xml.push_str("</urlset>\n");
    Some(xml)
}

/// The `llms.txt` of the pages in the site's language: the framework's
/// description, then one line per page of each sidebar with the page's lead.
/// The URLs are absolute where the site knows its origin, since an agent
/// fetches them from the file alone, and start at the site's base otherwise.
///
/// The Learn section comes first. The reference, the components, the styles,
/// and the i18n pages go under `Optional`, which an agent may skip.
fn llms(site: Site) -> String {
    let mut text = format!(
        "# hypersig\n\n> {}\n\n{}\n",
        site.t(Doc::Home.leads()),
        site.t(L {
            en: "Each page's Rust samples are compiled by the repository's tests. The pages follow the main branch, which can be ahead of the release on crates.io, so confirm an API in the source or on docs.rs of the version a project uses.",
            ja: "各ページの Rust のサンプルは、リポジトリのテストでコンパイルされています。ページはメインブランチに合わせて書かれていて、crates.io の最新リリースより先に進んでいることがあります。API は、プロジェクトが使うバージョンのソースか docs.rs で確かめてください。",
        }),
    );
    let list = |text: &mut String, docs: &[Doc]| {
        for &doc in docs {
            let url = match site.canonical(&doc.path()) {
                Some((origin, url)) => format!("{origin}{url}"),
                None => doc.url(site),
            };
            let lead = without_links(site.t(doc.leads()));
            let _ = writeln!(text, "- [{}]({url}): {lead}", doc.title(site));
        }
    };
    for group in doc::LEARN {
        let _ = write!(text, "\n## {}\n\n", site.t(group.title));
        list(&mut text, group.docs);
    }
    text.push_str("\n## Optional\n\n");
    for groups in [doc::REFERENCE, doc::COMPONENTS, doc::STYLE, doc::I18N] {
        for group in groups {
            list(&mut text, group.docs);
        }
    }
    text
}

/// `text` with each `[label](target)` replaced by its label. A lead's
/// targets are relative to the site, which an agent reading `llms.txt` has no
/// base for, and the other marks are Markdown already.
fn without_links(text: &str) -> String {
    let mut out = String::with_capacity(text.len());
    let mut rest = text;
    while let Some(open) = rest.find('[') {
        let link = rest[open..].find("](").and_then(|middle| {
            let close = rest[open + middle..].find(')')?;
            Some((middle, middle + close))
        });
        let Some((middle, close)) = link else { break };
        out.push_str(&rest[..open]);
        out.push_str(&rest[open + 1..open + middle]);
        rest = &rest[open + close + 1..];
    }
    out.push_str(rest);
    out
}

/// A link inside the site that names no file of the site, or no section of the
/// page it names.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BrokenLink {
    /// The page the link is on.
    pub page: String,
    /// The link's target, as written in the page.
    pub target: String,
}

/// The links in `files` that point inside `site` at a file `files` does not
/// hold, or at a fragment that the page they name has no `id` for.
///
/// A link into [`API_DIR`] is checked by `api`, which receives the path after
/// `api/` and the fragment after `#`, if any. Pass a function that returns
/// `true` to leave those links unchecked.
///
/// # Panics
///
/// Panics if a page of `files`, a file ending in `.html`, is not UTF-8.
#[must_use]
pub fn check_links(
    site: Site,
    files: &[File],
    api: impl Fn(&str, Option<&str>) -> bool,
) -> Vec<BrokenLink> {
    let known: BTreeMap<&str, &[u8]> = files
        .iter()
        .map(|file| (file.path.as_str(), file.contents.as_slice()))
        .collect();
    let api_prefix = format!("{API_DIR}/");
    let mut broken = Vec::new();
    for file in files.iter().filter(|file| {
        Path::new(&file.path)
            .extension()
            .is_some_and(|ext| ext == "html")
    }) {
        let html = str::from_utf8(&file.contents).expect("a page is UTF-8");
        for target in links(html) {
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
            } else {
                let file = if path.is_empty() || path.ends_with('/') {
                    known.get(format!("{path}index.html").as_str())
                } else {
                    known.get(path)
                };
                file.is_some_and(|contents| {
                    fragment.is_none_or(|id| {
                        str::from_utf8(contents)
                            .is_ok_and(|html| html.contains(&format!("id=\"{id}\"")))
                    })
                })
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
            contents: r#"<a href="/hypersig/learn/">Learn</a><a href="/hypersig/">Home</a><a href="https://example.com/">out</a><code>href="/hypersig/x"</code>"#.as_bytes().to_vec(),
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
    fn a_link_to_a_missing_section_is_reported() {
        let site = Site::new("/hypersig");
        let files = vec![
            File {
                path: "index.html".to_owned(),
                contents: r#"<a href="/hypersig/learn/#streams">here</a><a href="/hypersig/learn/#gone">gone</a>"#.as_bytes().to_vec(),
            },
            File {
                path: "learn/index.html".to_owned(),
                contents: r#"<h2 id="streams">Streams</h2>"#.as_bytes().to_vec(),
            },
        ];
        let broken = check_links(site, &files, |_, _| true);
        assert_eq!(
            broken,
            [BrokenLink {
                page: "index.html".to_owned(),
                target: "/hypersig/learn/#gone".to_owned(),
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
            assert!(!html.contains(r#"rel="canonical""#), "{doc:?}");
        }
    }

    #[test]
    fn a_site_that_knows_its_origin_names_each_page_under_its_base() {
        let site = Site::new("/hypersig")
            .with_origin(Origin::new("https://example.com"))
            .with_lang(Lang::Ja);
        let html = Doc::Learn(doc::Learn::Forms).render(site);
        let url = "https://example.com/hypersig/ja/learn/forms/";
        assert!(html.contains(&format!(r#"<link rel="canonical" href="{url}">"#)));
        assert!(html.contains(&format!(r#"<meta property="og:url" content="{url}">"#)));
        let home = Doc::Home.render(Site::new("").with_origin(Origin::new("https://example.com")));
        assert!(home.contains(r#"<link rel="canonical" href="https://example.com/">"#));
    }

    #[test]
    fn a_page_names_its_translations_and_its_picture_under_the_base() {
        let site = Site::new("/hypersig").with_origin(Origin::new("https://example.com"));
        let html = Doc::Learn(doc::Learn::Forms).render(site);
        for (lang, url) in [
            ("en", "https://example.com/hypersig/learn/forms/"),
            ("ja", "https://example.com/hypersig/ja/learn/forms/"),
            ("x-default", "https://example.com/hypersig/learn/forms/"),
        ] {
            assert!(
                html.contains(&format!(
                    r#"<link rel="alternate" hreflang="{lang}" href="{url}">"#
                )),
                "{lang}"
            );
        }
        assert!(html.contains(
            r#"<meta property="og:image" content="https://example.com/hypersig/og-en.png">"#
        ));
        assert!(html.contains(r#"<meta property="og:locale" content="en_US">"#));
        assert!(html.contains(r#"<meta property="og:type" content="article">"#));
        assert!(html.contains(r#"<meta name="description" content="A form"#));
    }

    #[test]
    fn a_lead_keeps_the_label_of_its_link() {
        assert_eq!(
            without_links("Read [`Page`](api:hypersig/struct.Page.html) and [x"),
            "Read `Page` and [x"
        );
    }

    #[test]
    fn llms_txt_lists_every_page_once_with_its_url() {
        let site = Site::new("/hypersig").with_origin(Origin::new("https://example.com"));
        let text = llms(site);
        assert!(text.starts_with("# hypersig\n\n> "));
        let listed = text.lines().filter(|line| line.starts_with("- [")).count();
        let pages = Doc::ALL
            .iter()
            .filter(|doc| !doc.groups().is_empty())
            .count();
        assert_eq!(listed, pages);
        assert!(text.contains("](https://example.com/hypersig/learn/sessions/): "));
        let ja = llms(site.with_lang(Lang::Ja));
        assert!(ja.contains("](https://example.com/hypersig/ja/learn/sessions/): "));
        assert!(llms(Site::new("/hypersig")).contains("](/hypersig/learn/sessions/): "));
    }

    #[test]
    fn the_sitemap_lists_every_page_in_every_language() {
        let site = Site::new("/hypersig").with_origin(Origin::new("https://example.com"));
        let xml = sitemap(site).expect("the site knows its origin");
        assert_eq!(
            xml.matches("<loc>").count(),
            Doc::ALL.len() * Lang::ALL.len()
        );
        assert!(xml.contains("<loc>https://example.com/hypersig/ja/</loc>"));
        assert!(sitemap(Site::new("/hypersig")).is_none());
    }
}
