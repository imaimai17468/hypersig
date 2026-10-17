//! The landing page of hypersig, the framework.

use hypersig_ui::ButtonVariant;

use crate::Site;
use crate::demo::{self, Demo};
use crate::doc::{Doc, Library};
use crate::landing::{self, features, hero, titled, two_cards};
use crate::prose::{Part, para, table_of};

pub fn render(site: Site) -> String {
    landing::page(
        site,
        Doc::Home,
        [
            hero(
                site,
                "A web framework on axum",
                "The page, its behavior, and the server, all in Rust",
                "`cargo build` builds it, with no Node and no wasm, and a mismatch between the page and the server is a compile error.",
                &[
                    ("Get started", "learn/", ButtonVariant::Default),
                    ("Reference", "reference/", ButtonVariant::Outline),
                ],
            ),
            features(
                site,
                &[
                    (
                        "No browser build",
                        "`cargo build` builds it, with no Node and no wasm. The crate embeds a prebuilt runtime of about 14 KB gzipped.",
                    ),
                    (
                        "Typed from page to server",
                        "Signal names, expressions, actions, and links come from Rust types. A link whose page has no handler, or whose path parameters or query do not fit the page, is a compile error, as is any other mismatch between the page and the server.",
                    ),
                    (
                        "Optimistic updates for free",
                        "A statement changes the page in the browser, and an action call sent with it reaches the server after. Changing the page before the server answers takes no extra code.",
                    ),
                ],
            ),
            titled(
                "Styles and components, too",
                [
                    para(
                        site,
                        "hypersig is the framework. Two crates in the same repository add typed styles and a set of accessible components, each with docs of its own.",
                    ),
                    two_cards(
                        site,
                        [
                            (
                                Library::Style,
                                "hypersig_style",
                                "Typed atomic CSS: every property a method and every keyword a variant, with tokens, themes, and conditions. Reached as `hypersig::style`.",
                            ),
                            (
                                Library::Components,
                                "hypersig_ui",
                                "Accessible components styled after shadcn/ui, on native elements and the WAI-ARIA Authoring Practices.",
                            ),
                        ],
                    ),
                ],
            ),
            titled(
                "Write the page in Rust",
                [
                    para(
                        site,
                        "The signal struct holds the page's state. `Counter::count()` is a typed reference to its field, and `count.set(count + 1)` is a statement the browser runs on a click without asking the server. The page renders on the server with the count already in place, so it shows before the runtime loads.",
                    ),
                    demo::show(site, Demo::Counter),
                ],
            ),
            comparison(site),
            benchmarks(site),
        ],
    )
}

/// The README's comparison with other frameworks.
fn comparison(site: Site) -> Part {
    titled(
        "How it compares",
        [
            para(
                site,
                "Adding a page to a Rust server leaves two choices today. Leptos type-checks what passes between the page and the server, and needs wasm and its build. htmx needs no build, but the URLs and expressions written in its attributes are not type-checked.",
            ),
            table_of(
                site,
                &[
                    "",
                    "hypersig",
                    "Leptos",
                    "React",
                    "Next.js",
                    "TanStack Start",
                    "Nuxt",
                    "htmx",
                ],
                &[
                    &[
                        "Page written in",
                        "Rust",
                        "Rust",
                        "TypeScript",
                        "TypeScript",
                        "TypeScript",
                        "TypeScript and Vue templates",
                        "HTML attributes",
                    ],
                    &[
                        "Server written in",
                        "Rust",
                        "Rust",
                        "Any",
                        "TypeScript",
                        "TypeScript",
                        "TypeScript",
                        "Any",
                    ],
                    &[
                        "No browser build",
                        "Yes",
                        "No (wasm and cargo-leptos)",
                        "No (Vite)",
                        "No (Node bundler)",
                        "No (Vite)",
                        "No (Vite)",
                        "Yes",
                    ],
                    &[
                        "Request types checked at compile time",
                        "Yes",
                        "Yes",
                        "No",
                        "Yes",
                        "Yes",
                        "No",
                        "No",
                    ],
                    &[
                        "Links checked against routes at compile time",
                        "Yes",
                        "No (`href` takes a string)",
                        "No (no router)",
                        "Opt-in (`typedRoutes`)",
                        "Yes",
                        "Experimental (`typedPages`)",
                        "No",
                    ],
                    &[
                        "State kept in the browser",
                        "Yes",
                        "Yes",
                        "Yes",
                        "Yes",
                        "Yes",
                        "Yes",
                        "No",
                    ],
                ],
            ),
        ],
    )
}

/// The README's benchmark, with its numbers unchanged.
fn benchmarks(site: Site) -> Part {
    titled(
        "Benchmarks",
        [
            para(
                site,
                "A 2000-item TodoMVC that stores every add, toggle, and delete on the server. Medians of 3 runs in Chrome 155 on an Apple M4 Max, with the CPU slowed 4x and 50 ms added to every request. The apps and the harness are in [`bench/compare`](https://github.com/imaimai17468/hypersig/tree/main/bench/compare).",
            ),
            table_of(
                site,
                &[
                    "",
                    "hypersig",
                    "Leptos",
                    "React",
                    "Next.js",
                    "TanStack Start",
                    "Nuxt",
                    "htmx",
                ],
                &[
                    &[
                        "JS (gzip)",
                        "13.4 KB",
                        "140.5 KB",
                        "67.1 KB",
                        "133.3 KB",
                        "119.5 KB",
                        "59.2 KB",
                        "16.5 KB",
                    ],
                    &[
                        "First Contentful Paint",
                        "179 ms",
                        "190 ms",
                        "184 ms",
                        "198 ms",
                        "188 ms",
                        "172 ms",
                        "235 ms",
                    ],
                    &[
                        "Total Blocking Time",
                        "0 ms",
                        "27 ms",
                        "2 ms",
                        "5 ms",
                        "0 ms",
                        "25 ms",
                        "233 ms",
                    ],
                    &[
                        "Interaction to Next Paint",
                        "24 ms",
                        "16 ms",
                        "24 ms",
                        "16 ms",
                        "24 ms",
                        "16 ms",
                        "16 ms",
                    ],
                ],
            ),
        ],
    )
}
