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
                    en: "A hypermedia web framework written entirely in Rust",
                    ja: "Rust だけで書くハイパーメディアの Web フレームワーク",
                }),
                site.t(L {
                    en: "Server-owned state, type-checked strings.",
                    ja: "サーバーが状態を、型が正しさを。",
                }),
                site.t(L {
                    en: "`cargo build` builds the whole application. There is no Node or wasm step, and a mismatch between the page and the server is a compile error.",
                    ja: "`cargo build` だけでビルドでき、Node も wasm も不要です。画面とサーバーの食い違いはコンパイルエラーになります。",
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
                            en: "Use cases",
                            ja: "ユースケース",
                        }),
                        "use-cases/",
                        ButtonVariant::Ghost,
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
                            en: "`cargo build` builds the whole application, with no Node or wasm step. The crate embeds a prebuilt runtime of about 20 KB gzipped.",
                            ja: "`cargo build` だけでビルドでき、Node も wasm も不要です。gzip で約 20 KB のランタイムをビルド済みの状態でクレートに埋め込んでいます。",
                        }),
                    ),
                    (
                        site.t(L {
                            en: "Typed from page to server",
                            ja: "画面からサーバーまで型で検査",
                        }),
                        site.t(L {
                            en: "Signal names, expressions, actions, and links come from Rust types. A link whose page has no handler, a link whose path parameters or query do not fit the page, and an action call whose arguments do not fit its handler are compile errors. Whether the router serves each action a page calls is left to a test.",
                            ja: "シグナルの名前、式、アクション、リンクは Rust の型から作られます。ハンドラのないページへのリンク、パスのパラメータやクエリがページに合わないリンク、引数がハンドラに合わないアクションの呼び出しは、コンパイルエラーになります。ページが呼ぶアクションをルーターが配信しているかどうかは、テストで確かめます。",
                        }),
                    ),
                    (
                        site.t(L {
                            en: "The server holds the state, with no hydration",
                            ja: "状態はサーバーが持ち、ハイドレーションはなし",
                        }),
                        site.t(L {
                            en: "The server renders the page with its state and its behavior written into the HTML. On load, the runtime reads the state from `data-hs-signals` and runs no component again, and what the server changes later reaches the browser as patches over server-sent events.",
                            ja: "サーバーは状態と動きを HTML に書き込んでページを描画します。読み込み時、ランタイムは `data-hs-signals` から状態を読むだけで、コンポーネントを実行し直しません。そのあとサーバーで変えた状態は、Server-Sent Events のパッチでブラウザに届きます。",
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
                            ja: "hypersig はフレームワーク本体です。同じリポジトリにある 3 つのクレートが、型付きのスタイル、アクセシブルなコンポーネント、型で検査する翻訳を追加します。それぞれにドキュメントがあります。",
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
        "Topcoat",
        "React",
        "Next.js",
        "TanStack Start",
        "Nuxt",
        "Datastar",
        "Leptos",
        "Dioxus",
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
                    en: "Topcoat and Datastar come closest to hypersig. Both keep signals in the browser and update the page from the server with no browser build, and Topcoat writes the browser's expressions in Rust too. Leptos and Dioxus write the page and the server in Rust as well, but compile the page to WebAssembly, which hydrates the server's HTML in the browser.",
                    ja: "hypersig に近いのは Topcoat と Datastar です。どちらもブラウザの中にシグナルを持ち、ブラウザ向けのビルドなしにサーバーから画面を更新します。Topcoat はブラウザで動く式も Rust で書きます。Leptos と Dioxus も画面とサーバーを Rust で書きますが、画面を WebAssembly にコンパイルし、ブラウザでサーバーの HTML をハイドレートします。",
                },
            ),
            table_of(
                site,
                L {
                    en: &["", "Topcoat and Datastar", "hypersig"],
                    ja: &["", "Topcoat と Datastar", "hypersig"],
                },
                L {
                    en: &[
                        &[
                            "Signal names and URLs (Datastar)",
                            "Strings in HTML attributes. A typo shows up when the browser runs it",
                            "Written from Rust values. A typo stops `cargo build`",
                        ],
                        &[
                            "Link parameters (Topcoat)",
                            "`href!` checks only the handler. A missing or extra path parameter panics when the page renders, and the request answers 500",
                            "`Route` takes its parameters as a typed value. A wrong one is a compile error",
                        ],
                        &[
                            "Adding to a list (Topcoat)",
                            "Expressions cannot add an item, so the list changes once the server answers and renders it again",
                            "The list is a signal, so the browser shows the change before the server answers",
                        ],
                    ],
                    ja: &[
                        &[
                            "シグナル名と URL（Datastar）",
                            "HTML の属性に文字列で書きます。打ち間違いはブラウザが実行するまで分かりません",
                            "Rust の値から書きます。打ち間違いがあると `cargo build` が止まります",
                        ],
                        &[
                            "リンクのパラメータ（Topcoat）",
                            "`href!` が検査するのはハンドラだけです。パスパラメータが足りないか多いと、ページを描くときにパニックし、リクエストは 500 を返します",
                            "`Route` がパラメータを型の付いた値で受け取ります。間違えるとコンパイルエラーになります",
                        ],
                        &[
                            "一覧への追加（Topcoat）",
                            "式から項目を追加できないので、サーバーが応答して描き直したときに一覧が変わります",
                            "一覧をシグナルに持つので、サーバーが応答する前にブラウザが変更を表示します",
                        ],
                    ],
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
                            "Rust",
                            "Rust",
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
                            "Rust",
                            "Rust",
                        ],
                        &[
                            "No browser build",
                            "Yes",
                            "Yes",
                            "No (Vite)",
                            "No (Node bundler)",
                            "No (Vite)",
                            "No (Vite)",
                            "Yes",
                            "No (`cargo leptos`, to Wasm)",
                            "No (`dx`, to Wasm)",
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
                            "Yes",
                            "Yes",
                        ],
                        &[
                            "Links checked against routes at compile time",
                            "Yes",
                            "Handler only (parameters at run time)",
                            "No (no router)",
                            "Opt-in (`typedRoutes`)",
                            "Yes",
                            "Experimental (`typedPages`)",
                            "No",
                            "No (`href` is a string)",
                            "Yes (a `Routable` enum value)",
                        ],
                        &[
                            "State kept in the browser",
                            "Yes",
                            "Yes",
                            "Yes",
                            "Yes",
                            "Yes",
                            "Yes",
                            "Yes",
                            "Yes",
                            "Yes",
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
                            "Rust",
                            "Rust",
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
                            "Rust",
                            "Rust",
                        ],
                        &[
                            "ブラウザ向けのビルドが不要",
                            "はい",
                            "はい",
                            "いいえ（Vite）",
                            "いいえ（Node のバンドラー）",
                            "いいえ（Vite）",
                            "いいえ（Vite）",
                            "はい",
                            "いいえ（`cargo leptos` で Wasm に）",
                            "いいえ（`dx` で Wasm に）",
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
                            "はい",
                            "はい",
                        ],
                        &[
                            "リンクの行き先をコンパイル時に検査",
                            "はい",
                            "ハンドラだけ（パラメータは実行時）",
                            "いいえ（ルーターなし）",
                            "設定で有効化（`typedRoutes`）",
                            "はい",
                            "実験的（`typedPages`）",
                            "いいえ",
                            "いいえ（`href` は文字列）",
                            "はい（`Routable` の列挙型の値）",
                        ],
                        &[
                            "ブラウザの中で状態を持てる",
                            "はい",
                            "はい",
                            "はい",
                            "はい",
                            "はい",
                            "はい",
                            "はい",
                            "はい",
                            "はい",
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
        "Topcoat",
        "React",
        "Next.js",
        "TanStack Start",
        "Nuxt",
        "Datastar",
        "Leptos",
        "Dioxus",
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
                    en: "A 2000-item TodoMVC that stores every add, toggle, and delete on the server. Medians of 10 runs in Chrome 155 on an Apple M4 Max, with the CPU slowed 4x and 50 ms added to every request, and the fastest and slowest of the 10 runs in parentheses. The apps and the harness are in [`bench/compare`](https://github.com/imaimai17468/hypersig/tree/main/bench/compare). Every app was measured at commit 7d031d0. Leptos and Dioxus load most of their code as WebAssembly, which the Wasm row counts apart from the JS row.",
                    ja: "追加、チェック、削除をサーバーに保存する 2000 件の TodoMVC を、Apple M4 Max の Chrome 155 で 10 回測った中央値です。CPU を 4 倍に減速し、通信のたびに 50 ms の遅延を加えています。括弧の中は 10 回のうち最も速い回と遅い回の値です。測ったアプリと計測の仕組みは [`bench/compare`](https://github.com/imaimai17468/hypersig/tree/main/bench/compare) にあります。どのアプリもコミット 7d031d0 で測りました。Leptos と Dioxus はコードの大半を WebAssembly で読み込むので、その大きさを JS の行とは別に Wasm の行に数えています。",
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
                Text::Chosen::<[&[&str; 10]]>(&[
                    &[
                        site.t(L {
                            en: "JS (gzip)",
                            ja: "JS（gzip）",
                        }),
                        "19.7 KB",
                        "14.8 KB",
                        "67.1 KB",
                        "133.3 KB",
                        "119.5 KB",
                        "59.2 KB",
                        "13.1 KB",
                        "5.5 KB",
                        "15.7 KB",
                    ],
                    &[
                        site.t(L {
                            en: "Wasm (gzip)",
                            ja: "Wasm（gzip）",
                        }),
                        "-",
                        "-",
                        "-",
                        "-",
                        "-",
                        "-",
                        "-",
                        "145.1 KB",
                        "441.4 KB",
                    ],
                    &[
                        "First Contentful Paint",
                        "239 ms (231–309)",
                        "221 ms (206–241)",
                        "209 ms (198–266)",
                        "240 ms (229–261)",
                        "237 ms (217–251)",
                        "236 ms (223–263)",
                        "217 ms (204–249)",
                        "232 ms (222–240)",
                        "213 ms (205–244)",
                    ],
                    &[
                        "Total Blocking Time",
                        "3 ms (0–37)",
                        "318 ms (311–351)",
                        "16 ms (11–23)",
                        "14 ms (9–16)",
                        "0 ms (0–0)",
                        "36 ms (29–42)",
                        "20 ms (17–23)",
                        "47 ms (42–57)",
                        "122 ms (114–126)",
                    ],
                    &[
                        "Interaction to Next Paint",
                        "24 ms (24–24)",
                        "24 ms (24–24)",
                        "24 ms (16–24)",
                        "24 ms (16–24)",
                        "24 ms (16–24)",
                        "24 ms (24–24)",
                        "16 ms (16–16)",
                        "24 ms (16–24)",
                        "72 ms (72–80)",
                    ],
                ]),
            ),
            para(
                site,
                L {
                    en: "The same page styled with `hypersig::style`, Tailwind CSS, Panda CSS, StyleX, and vanilla-extract is measured on the [Styling](style/) page.",
                    ja: "同じページを `hypersig::style`、Tailwind CSS、Panda CSS、StyleX、vanilla-extract でスタイルした計測は、[スタイル](style/) のページにあります。",
                },
            ),
        ],
    )
}
