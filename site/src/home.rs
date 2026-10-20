//! The landing page of hypersig, the framework.

use hypersig_ui::ButtonVariant;

use crate::Site;
use crate::demo::{self, Demo};
use crate::doc::{Doc, Library};
use crate::landing::{self, features, hero, library_cards, titled};
use crate::lang::{L, Text};
use crate::prose::{Part, para, subheading, table_of};

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
        "Topcoat",
        "React",
        "Next.js",
        "TanStack Start",
        "Nuxt",
        "Datastar",
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
                    en: "Topcoat and Datastar come closest to hypersig. Both keep signals in the browser and update the page from the server with no browser build, and Topcoat writes the browser's expressions in Rust too.",
                    ja: "hypersig に近いのは Topcoat と Datastar です。どちらもブラウザの中にシグナルを持ち、ブラウザ向けのビルドなしにサーバーから画面を更新します。Topcoat はブラウザで動く式も Rust で書きます。",
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
                            "式から項目を足せないので、サーバーが答えて描き直したときに一覧が変わります",
                            "一覧をシグナルに持つので、サーバーが答える前にブラウザが変更を表示します",
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
                            "Yes",
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
                            "Handler only (parameters at run time)",
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
                            "はい",
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
                            "ハンドラだけ（パラメータは実行時）",
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
                            "はい",
                        ],
                    ],
                },
            ),
        ],
    )
}

/// The README's benchmarks, with their numbers unchanged.
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
                        "14.8 KB",
                        "67.1 KB",
                        "133.3 KB",
                        "119.5 KB",
                        "59.2 KB",
                        "13.1 KB",
                    ],
                    &[
                        "First Contentful Paint",
                        "186 ms",
                        "165 ms",
                        "196 ms",
                        "198 ms",
                        "193 ms",
                        "177 ms",
                        "182 ms",
                    ],
                    &[
                        "Total Blocking Time",
                        "0 ms",
                        "313 ms",
                        "5 ms",
                        "10 ms",
                        "0 ms",
                        "29 ms",
                        "16 ms",
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
            subheading(
                site,
                L {
                    en: "Styling",
                    ja: "スタイル",
                },
            ),
            para(
                site,
                L {
                    en: "The same 2000-item page, styled six ways and rendered by hypersig with the same markup, so only the class names and the CSS differ. Measured in Chrome 155 on an Apple M4 Max with the CPU slowed 4x. Browser values are medians of 5 runs, and the server render is the median of 50.",
                    ja: "同じ 2000 件のページを 6 通りのやり方でスタイルし、同じマークアップで hypersig に描かせたものです。違うのはクラス名と CSS だけです。Apple M4 Max の Chrome 155 で、CPU を 4 倍に減速して測りました。ブラウザの値は 5 回、サーバーで描く時間は 50 回の中央値です。",
                },
            ),
            table_of(
                site,
                L {
                    en: &[
                        "",
                        "hypersig",
                        "Tailwind CSS",
                        "Panda CSS",
                        "StyleX",
                        "vanilla-extract",
                        "Hand-written CSS",
                    ],
                    ja: &[
                        "",
                        "hypersig",
                        "Tailwind CSS",
                        "Panda CSS",
                        "StyleX",
                        "vanilla-extract",
                        "手書きの CSS",
                    ],
                },
                Text::Chosen::<[&[&str; 7]]>(&[
                    &[
                        site.t(L {
                            en: "CSS (gzip)",
                            ja: "CSS（gzip）",
                        }),
                        "1.0 KB",
                        "1.6 KB",
                        "1.0 KB",
                        "1.0 KB",
                        "0.6 KB",
                        "0.8 KB",
                    ],
                    &[
                        site.t(L {
                            en: "HTML without CSS (gzip)",
                            ja: "CSS を除いた HTML（gzip）",
                        }),
                        "30.2 KB",
                        "29.1 KB",
                        "26.9 KB",
                        "26.8 KB",
                        "27.6 KB",
                        "27.3 KB",
                    ],
                    &[
                        site.t(L {
                            en: "Sent in all (gzip)",
                            ja: "送る合計（gzip）",
                        }),
                        "31.5 KB",
                        "30.7 KB",
                        "27.8 KB",
                        "27.8 KB",
                        "28.2 KB",
                        "28.1 KB",
                    ],
                    &[
                        site.t(L {
                            en: "Build of the CSS",
                            ja: "CSS のビルド",
                        }),
                        site.t(L {
                            en: "None",
                            ja: "なし",
                        }),
                        "268 ms",
                        "574 ms",
                        "182 ms",
                        "74 ms",
                        "3.1 ms",
                    ],
                    &[
                        site.t(L {
                            en: "Server render",
                            ja: "サーバーで描く時間",
                        }),
                        "1.7 ms",
                        "1.5 ms",
                        "1.4 ms",
                        "1.4 ms",
                        "0.8 ms",
                        "0.8 ms",
                    ],
                    &[
                        "First Contentful Paint",
                        "136 ms",
                        "140 ms",
                        "136 ms",
                        "144 ms",
                        "140 ms",
                        "140 ms",
                    ],
                    &[
                        site.t(L {
                            en: "Style and layout on load",
                            ja: "読み込み時のスタイル計算とレイアウト",
                        }),
                        "76 ms",
                        "74 ms",
                        "73 ms",
                        "81 ms",
                        "67 ms",
                        "68 ms",
                    ],
                    &[
                        site.t(L {
                            en: "Marking 2000 items done",
                            ja: "2000 件を完了にする時間",
                        }),
                        "27 ms",
                        "37 ms",
                        "26 ms",
                        "27 ms",
                        "24 ms",
                        "24 ms",
                    ],
                ]),
            ),
            para(
                site,
                L {
                    en: "hypersig's class names are long because each one is made from its declaration alone: the conditions, the property's full name, and the value, as in `font-size_24px`. A value with any character other than letters, digits, `.`, `#`, `%`, and `-`, such as a space, becomes a 64-bit hash written in 16 hex digits. The name has to come from the declaration alone because a server patch renders new elements without knowing the classes the page already has, and the browser adds the rules the patch brings to the page's own, so one declaration has to get the same name every time it is rendered. `Page::render` writes these names into every item, so the server render grows with them: across the six pages it grows in the same order as the HTML before gzip, which is 1.21 MB for hypersig and 0.88 MB for Panda CSS. The pages and the harness are in [`bench/style`](https://github.com/imaimai17468/hypersig/tree/main/bench/style).",
                    ja: "hypersig のクラス名が長いのは、宣言だけから名前を作っているからです。名前は条件、プロパティの正式な名前、値をつないだもので、`font-size_24px` のようになります。値に英数字と `.`、`#`、`%`、`-` 以外の文字（空白など）があると、値は 64 ビットのハッシュになり、16 桁の 16 進数で書かれます。宣言だけから作るのは、サーバーのパッチがページにすでにあるクラスを知らずに新しい要素を描き、ブラウザがパッチの持ってきたルールをページのルールに足すからです。そのため同じ宣言は、いつ描いても同じ名前になる必要があります。`Page::render` はこの名前を項目ごとに書き出すので、サーバーで描く時間も名前の分だけ延びます。6 つのページでは、gzip 前の HTML が大きい順に描く時間も長くなっています（hypersig は 1.21 MB、Panda CSS は 0.88 MB）。ページと計測の仕組みは [`bench/style`](https://github.com/imaimai17468/hypersig/tree/main/bench/style) にあります。",
                },
            ),
        ],
    )
}
