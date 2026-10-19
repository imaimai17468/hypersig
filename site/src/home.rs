//! The landing page of hypersig, the framework.

use hypersig_ui::ButtonVariant;

use crate::Site;
use crate::demo::{self, Demo};
use crate::doc::{Doc, Library};
use crate::landing::{self, features, hero, library_cards, titled};
use crate::lang::{L, Text};
use crate::prose::{Part, para, table_of};

pub fn render(site: Site) -> String {
    landing::page(
        site,
        Doc::Home,
        [
            hero(
                site,
                site.t(L {
                    en: "A web framework on axum",
                    ja: "axum の上で動く Web フレームワーク",
                }),
                site.t(L {
                    en: "The page, its behavior, and the server, all in Rust",
                    ja: "画面もその動きもサーバーも、すべて Rust で",
                }),
                site.t(L {
                    en: "`cargo build` builds it, with no Node and no wasm, and a mismatch between the page and the server is a compile error.",
                    ja: "`cargo build` だけでビルドでき、Node も wasm も要りません。画面とサーバーの食い違いはコンパイルエラーになります。",
                }),
                &[
                    (
                        site.t(L {
                            en: "Get started",
                            ja: "はじめる",
                        }),
                        "learn/",
                        ButtonVariant::Default,
                    ),
                    (
                        site.t(L {
                            en: "Why hypersig",
                            ja: "hypersig を選ぶ理由",
                        }),
                        "learn/why/",
                        ButtonVariant::Outline,
                    ),
                    (
                        site.t(L {
                            en: "Reference",
                            ja: "リファレンス",
                        }),
                        "reference/",
                        ButtonVariant::Ghost,
                    ),
                ],
            ),
            features(
                site,
                &[
                    (
                        site.t(L {
                            en: "No browser build",
                            ja: "ブラウザ向けのビルドが不要",
                        }),
                        site.t(L {
                            en: "`cargo build` builds it, with no Node and no wasm. The crate embeds a prebuilt runtime of about 14 KB gzipped.",
                            ja: "`cargo build` だけでビルドでき、Node も wasm も要りません。gzip で約 14 KB のランタイムをビルド済みの状態でクレートに埋め込んでいます。",
                        }),
                    ),
                    (
                        site.t(L {
                            en: "Typed from page to server",
                            ja: "画面からサーバーまで型で検査",
                        }),
                        site.t(L {
                            en: "Signal names, expressions, actions, and links come from Rust types. A link whose page has no handler, or whose path parameters or query do not fit the page, is a compile error, as is any other mismatch between the page and the server.",
                            ja: "シグナルの名前、式、アクション、リンクは Rust の型から作られます。ハンドラのないページへのリンクや、パスのパラメータやクエリがページに合わないリンクは、画面とサーバーのほかの食い違いと同じくコンパイルエラーになります。",
                        }),
                    ),
                    (
                        site.t(L {
                            en: "Optimistic updates with no extra code",
                            ja: "楽観的更新に追加のコードが要らない",
                        }),
                        site.t(L {
                            en: "A statement changes the page in the browser, and an action call sent with it reaches the server after. Changing the page before the server answers takes no extra code.",
                            ja: "文はブラウザの中で画面を変え、一緒に送ったアクションの呼び出しはそのあとでサーバーに届きます。サーバーの応答を待たずに画面を変える処理を、追加のコードなしで書けます。",
                        }),
                    ),
                ],
            ),
            titled(
                site,
                L {
                    en: "Styles, components, and translations, too",
                    ja: "スタイル、コンポーネント、翻訳も",
                },
                [
                    para(
                        site,
                        L {
                            en: "hypersig is the framework. Three crates in the same repository add typed styles, a set of accessible components, and typed translations, each with docs of its own.",
                            ja: "hypersig はフレームワーク本体です。同じリポジトリにある 3 つのクレートが、型付きのスタイル、アクセシブルなコンポーネント、型で検査する翻訳を足します。それぞれにドキュメントがあります。",
                        },
                    ),
                    library_cards(
                        site,
                        [
                            (
                                Library::Style,
                                "hypersig_style",
                                site.t(L {
                                    en: "Typed atomic CSS: every property a method and every keyword a variant, with tokens, themes, and conditions. Reached as `hypersig::style`.",
                                    ja: "型付きの atomic CSS です。プロパティはすべてメソッド、キーワードはすべてバリアントで、トークン、テーマ、条件も書けます。`hypersig::style` から使います。",
                                }),
                            ),
                            (
                                Library::Components,
                                "hypersig_ui",
                                site.t(L {
                                    en: "Accessible components styled after shadcn/ui, built on native elements and following the WAI-ARIA Authoring Practices.",
                                    ja: "shadcn/ui の見た目を持つアクセシブルなコンポーネントです。ネイティブ要素を使い、WAI-ARIA Authoring Practices に沿って作っています。",
                                }),
                            ),
                            (
                                Library::I18n,
                                "hypersig_i18n",
                                site.t(L {
                                    en: "Typed translations: a text without one of its translations fails to compile, and the language is part of each page's route.",
                                    ja: "型で検査する翻訳です。どれかの訳が欠けた文はコンパイルエラーになり、言語は各ページのルートの一部です。",
                                }),
                            ),
                        ],
                    ),
                ],
            ),
            titled(
                site,
                L {
                    en: "Write the page in Rust",
                    ja: "ページを Rust で書く",
                },
                [
                    para(
                        site,
                        L {
                            en: "The signal struct holds the page's state. `Counter::count()` is a typed reference to its field, and `count.set(count + 1)` is a statement the browser runs on a click without asking the server. The server renders the page with the count already filled in, so the page shows before the runtime loads.",
                            ja: "ページの状態はシグナル構造体が持ちます。`Counter::count()` はそのフィールドへの型付きの参照で、`count.set(count + 1)` はクリックのたびにブラウザがサーバーに問い合わせずに実行する文です。サーバーがカウントを入れた状態でページをレンダリングするので、ページはランタイムを読み込む前から表示されます。",
                        },
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
    let header = &[
        "",
        "hypersig",
        "Leptos",
        "React",
        "Next.js",
        "TanStack Start",
        "Nuxt",
        "htmx",
    ];
    titled(
        site,
        L {
            en: "How it compares",
            ja: "フレームワークの比較",
        },
        [
            para(
                site,
                L {
                    en: "There are two ways today to add a page to a Rust server. Leptos type-checks what passes between the page and the server, and needs wasm and its build. htmx needs no build, but the URLs and expressions written in its attributes are not type-checked.",
                    ja: "Rust のサーバーに画面を付けるには、今は 2 通りの選び方があります。Leptos なら画面とサーバーの間で型が検査される代わりに、wasm とそのビルドが要ります。htmx ならビルドは要りませんが、属性に書く URL や式は型で検査されません。",
                },
            ),
            table_of(
                site,
                L {
                    en: header,
                    ja: header,
                },
                L {
                    en: &[
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
                    ja: &[
                        &[
                            "画面を書く言語",
                            "Rust",
                            "Rust",
                            "TypeScript",
                            "TypeScript",
                            "TypeScript",
                            "TypeScript と Vue のテンプレート",
                            "HTML の属性",
                        ],
                        &[
                            "サーバーを書く言語",
                            "Rust",
                            "Rust",
                            "任意",
                            "TypeScript",
                            "TypeScript",
                            "TypeScript",
                            "任意",
                        ],
                        &[
                            "ブラウザ向けのビルドが不要",
                            "はい",
                            "いいえ（wasm と cargo-leptos）",
                            "いいえ（Vite）",
                            "いいえ（Node のバンドラー）",
                            "いいえ（Vite）",
                            "いいえ（Vite）",
                            "はい",
                        ],
                        &[
                            "送る値の型をコンパイル時に検査",
                            "はい",
                            "はい",
                            "いいえ",
                            "はい",
                            "はい",
                            "いいえ",
                            "いいえ",
                        ],
                        &[
                            "リンクの行き先をコンパイル時に検査",
                            "はい",
                            "いいえ（`href` は文字列）",
                            "いいえ（ルーターなし）",
                            "設定で有効化（`typedRoutes`）",
                            "はい",
                            "実験的（`typedPages`）",
                            "いいえ",
                        ],
                        &[
                            "ブラウザの中で状態を持てる",
                            "はい",
                            "はい",
                            "はい",
                            "はい",
                            "はい",
                            "はい",
                            "いいえ",
                        ],
                    ],
                },
            ),
        ],
    )
}

/// The README's benchmark, with its numbers unchanged.
fn benchmarks(site: Site) -> Part {
    let header = &[
        "",
        "hypersig",
        "Leptos",
        "React",
        "Next.js",
        "TanStack Start",
        "Nuxt",
        "htmx",
    ];
    titled(
        site,
        L {
            en: "Benchmarks",
            ja: "ベンチマーク",
        },
        [
            para(
                site,
                L {
                    en: "A 2000-item TodoMVC that stores every add, toggle, and delete on the server. Medians of 3 runs in Chrome 155 on an Apple M4 Max, with the CPU slowed 4x and 50 ms added to every request. The apps and the harness are in [`bench/compare`](https://github.com/imaimai17468/hypersig/tree/main/bench/compare).",
                    ja: "追加、チェック、削除をサーバーに保存する 2000 件の TodoMVC を、Apple M4 Max の Chrome 155 で 3 回測った中央値です。CPU を 4 倍に減速し、通信のたびに 50 ms の遅延を足しています。測ったアプリと計測の仕組みは [`bench/compare`](https://github.com/imaimai17468/hypersig/tree/main/bench/compare) にあります。",
                },
            ),
            table_of(
                site,
                L {
                    en: header,
                    ja: header,
                },
                // The numbers read the same in both languages, and the first
                // cell of each row is picked with `site.t`.
                Text::Chosen::<[&[&str; 8]]>(&[
                    &[
                        site.t(L {
                            en: "JS (gzip)",
                            ja: "JS（gzip）",
                        }),
                        "19.2 KB",
                        "140.5 KB",
                        "67.1 KB",
                        "133.3 KB",
                        "119.5 KB",
                        "59.2 KB",
                        "16.5 KB",
                    ],
                    &[
                        "First Contentful Paint",
                        "186 ms",
                        "203 ms",
                        "196 ms",
                        "198 ms",
                        "193 ms",
                        "177 ms",
                        "236 ms",
                    ],
                    &[
                        "Total Blocking Time",
                        "0 ms",
                        "26 ms",
                        "5 ms",
                        "10 ms",
                        "0 ms",
                        "29 ms",
                        "268 ms",
                    ],
                    &[
                        "Interaction to Next Paint",
                        "24 ms",
                        "24 ms",
                        "24 ms",
                        "24 ms",
                        "24 ms",
                        "16 ms",
                        "16 ms",
                    ],
                ]),
            ),
        ],
    )
}
