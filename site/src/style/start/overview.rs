//! The Style overview: what `hypersig::style` is, what the compiler checks
//! in a style, how it compares with other CSS tools, and the benchmark.

use crate::Site;
use crate::code::{self, Lang};
use crate::doc::{Doc, StylePage};
use crate::lang::{L, Text};
use crate::learn::snippet::{Snippet, show};
use crate::prose::{Part, heading, lead, para, parts, table_of};

pub fn page(site: Site, head: Part) -> Part {
    parts(vec![
        head,
        lead(site, Doc::Style(StylePage::Overview).leads()),
        heading(
            site,
            L {
                en: "A style is a Rust value",
                ja: "スタイルは Rust の値",
            },
        ),
        para(
            site,
            L {
                en: "[`Style`](api:hypersig/style/struct.Style.html) has one method per standard CSS property and per anchor positioning property. The list of properties, the keywords of each property, the pseudo-classes, the pseudo-elements, and the named colors are generated from MDN's CSS data (`mdn-data` 2.37.2). [`El::css`](api:hypersig/html/struct.El.html#method.css) adds a style to an element:",
                ja: "[`Style`](api:hypersig/style/struct.Style.html) には、標準の CSS プロパティとアンカー配置のプロパティごとに 1 つずつメソッドがあります。プロパティの一覧、各プロパティのキーワード、擬似クラス、擬似要素、名前付きの色は、MDN の CSS データ（`mdn-data` 2.37.2）から生成しています。要素にスタイルを付けるには [`El::css`](api:hypersig/html/struct.El.html#method.css) を使います。",
            },
        ),
        show(Snippet::StyleBasics),
        para(
            site,
            L {
                en: "The browser applies these declarations to the button:",
                ja: "ブラウザはボタンに次の宣言を適用します。",
            },
        ),
        code::block(Lang::Html, None, BASICS_CSS),
        para(
            site,
            L {
                en: "Each declaration becomes one class, as in the atomic CSS of Panda CSS and Tailwind CSS. The page writes the rule of each class once into `<head>`, so two elements with the same declaration share one rule, and the stylesheet grows with the number of distinct declarations, not the number of elements. A patch that adds elements carries the rules they need. [Applying a style](style/applying/) covers how styles combine.",
                ja: "宣言は 1 つずつクラスになります。Panda CSS や Tailwind CSS の atomic CSS と同じ方式です。ページは各クラスのルールを `<head>` に 1 回だけ書くので、同じ宣言を持つ 2 つの要素は 1 つのルールを共有し、スタイルシートの大きさは要素の数ではなく、異なる宣言の数で決まります。要素を追加するパッチには、その要素が使うルールも含まれます。スタイルの組み合わせ方は[スタイルを付ける](style/applying/)で説明します。",
            },
        ),
        heading(
            site,
            L {
                en: "What the compiler checks",
                ja: "コンパイラが検査すること",
            },
        ),
        para(
            site,
            L {
                en: "A misspelled property has no method, and a misspelled keyword has no variant, so both are E0599. A value of the wrong type has no conversion into the property's value type, so it is E0277:",
                ja: "綴りを間違えたプロパティにはメソッドがなく、綴りを間違えたキーワードにはバリアントがないので、どちらも E0599 になります。型の違う値はプロパティの値の型に変換できないので、E0277 になります。",
            },
        ),
        show(Snippet::WrongValue),
        para(
            site,
            L {
                en: "The checks stop at the value's type. The table lists what is left to the browser, and the page that covers each case.",
                ja: "検査されるのは値の型までです。ブラウザに任される部分と、それぞれを説明するページを表にまとめます。",
            },
        ),
        table_of(
            site,
            L {
                en: &["Not checked", "When it is wrong", "Page"],
                ja: &["検査されないこと", "間違っていたとき", "ページ"],
            },
            L {
                en: &[
                    &[
                        "A number's range, such as a negative width",
                        "The browser drops the declaration",
                        "[Lengths and units](style/lengths/)",
                    ],
                    &[
                        "A value written with `raw`",
                        "The browser drops the declaration if it rejects the value",
                        "[Unchecked values](style/unchecked/)",
                    ],
                    &[
                        "Whether the file of a `url()` exists",
                        "The image or font does not load, and the console shows the 404",
                        "[URLs and strings](style/urls-and-strings/)",
                    ],
                    &[
                        "Two styles switched on at once that set one property",
                        "The rule later in the stylesheet wins",
                        "[Styles switched by a signal](style/toggled/)",
                    ],
                ],
                ja: &[
                    &[
                        "負の幅のような数値の範囲",
                        "ブラウザはその宣言を捨てます",
                        "[長さと単位](style/lengths/)",
                    ],
                    &[
                        "`raw` で書いた値",
                        "ブラウザが値を受け付けなければ、その宣言は捨てられます",
                        "[検査しない値](style/unchecked/)",
                    ],
                    &[
                        "`url()` のファイルがあるかどうか",
                        "画像やフォントが読み込まれず、コンソールに 404 が表示されます",
                        "[URL と文字列](style/urls-and-strings/)",
                    ],
                    &[
                        "同時に有効になった 2 つのスタイルが同じプロパティを設定すること",
                        "スタイルシートで後にあるルールが勝ちます",
                        "[シグナルで切り替えるスタイル](style/toggled/)",
                    ],
                ],
            },
        ),
        heading(
            site,
            L {
                en: "Compared with other CSS tools",
                ja: "ほかの CSS のツールとの比較",
            },
        ),
        para(
            site,
            L {
                en: "Tailwind CSS, Panda CSS, StyleX, and vanilla-extract each have a build step that writes the stylesheet, which the benchmark below times. `hypersig::style` has no build step: `Page::render` writes the rules of the classes the page uses, and a patch carries the rules of the elements it adds. A style is written in the same Rust code as the element it styles, so a misspelled property fails `cargo build` along with the rest of the page.",
                ja: "Tailwind CSS、Panda CSS、StyleX、vanilla-extract には、スタイルシートを書き出すビルドの手順があり、下のベンチマークではその時間も測っています。`hypersig::style` にはビルドの手順がありません。`Page::render` がページの使うクラスのルールを書き、パッチは追加する要素のルールを含みます。スタイルは対象の要素と同じ Rust のコードに書くので、プロパティの綴りを間違えると、ページのほかの部分と一緒に `cargo build` が失敗します。",
            },
        ),
        heading(
            site,
            L {
                en: "Benchmarks",
                ja: "ベンチマーク",
            },
        ),
        parts(benchmarks(site)),
        heading(
            site,
            L {
                en: "The pages of this section",
                ja: "このセクションのページ",
            },
        ),
        table_of(
            site,
            L {
                en: &["Group", "Covers"],
                ja: &["グループ", "内容"],
            },
            L {
                en: &[
                    &[
                        "[Values](style/values/)",
                        "The property methods and the types of lengths, colors, angles, times, URLs, strings, and the CSS-wide keywords",
                    ],
                    &[
                        "[Conditions](style/pseudo-classes/)",
                        "Pseudo-classes, pseudo-elements, media and container queries, group and peer, and styles switched by a signal",
                    ],
                    &[
                        "[Tokens and themes](style/tokens/)",
                        "Named values as CSS custom properties, themes, and dark mode",
                    ],
                    &[
                        "[Global styles](style/global/)",
                        "Rules by tag, the reset, and fonts",
                    ],
                    &[
                        "[Layout](style/stacks/)",
                        "Stacks, grids, and the other layout patterns",
                    ],
                    &[
                        "[Animation](style/animation/)",
                        "Keyframes, transitions, page transitions, and reduced motion",
                    ],
                    &[
                        "[Opting out of type checks](style/unchecked/)",
                        "`raw` and the other ways an unchecked string reaches the CSS",
                    ],
                ],
                ja: &[
                    &[
                        "[値](style/values/)",
                        "プロパティのメソッドと、長さ、色、角度、時間、URL、文字列、CSS 全体のキーワードの型",
                    ],
                    &[
                        "[条件](style/pseudo-classes/)",
                        "擬似クラス、擬似要素、メディアクエリとコンテナクエリ、グループとピア、シグナルで切り替えるスタイル",
                    ],
                    &[
                        "[トークンとテーマ](style/tokens/)",
                        "CSS カスタムプロパティとして書く名前付きの値、テーマ、ダークモード",
                    ],
                    &[
                        "[グローバルスタイル](style/global/)",
                        "タグ単位のルール、リセット、フォント",
                    ],
                    &[
                        "[レイアウト](style/stacks/)",
                        "スタック、グリッド、そのほかのレイアウトのパターン",
                    ],
                    &[
                        "[アニメーション](style/animation/)",
                        "キーフレーム、トランジション、ページ遷移、動きを減らす設定",
                    ],
                    &[
                        "[型の検査を外す](style/unchecked/)",
                        "`raw` と、検査されない文字列が CSS に入るそのほかの経路",
                    ],
                ],
            },
        ),
        para(
            site,
            L {
                en: "Next, [Applying a style](style/applying/) covers `El::css`, `Style::merge`, and which declaration wins when two set one property.",
                ja: "次の[スタイルを付ける](style/applying/)では `El::css` と `Style::merge`、2 つの宣言が同じプロパティを設定したときにどちらが勝つかを説明します。",
            },
        ),
    ])
}

/// The declarations the button of `Snippet::StyleBasics` gets.
const BASICS_CSS: &str = "padding-block: 0.5rem;
padding-inline: 1rem;
border-radius: 6px;
background-color: #18181b;
color: white;
font-weight: 500;
cursor: pointer;";

/// The styling benchmark of `bench/style`, with the numbers of its
/// `results/latest.md`, for the Style overview and the landing page of the
/// Style docs.
pub fn benchmarks(site: Site) -> [Part; 4] {
    [
        para(
            site,
            L {
                en: "These figures come from the 2000-item TodoMVC page of [hypersig's framework benchmark](https://github.com/imaimai17468/hypersig/tree/main/bench/compare), styled six ways and rendered by hypersig with the same markup, so only the class names and the CSS differ. They were measured in Chrome 155 on an Apple M4 Max with the CPU slowed 4x. Browser values are medians of 5 runs, and the server render is the median of 50.",
                ja: "[hypersig のフレームワークのベンチマーク](https://github.com/imaimai17468/hypersig/tree/main/bench/compare) にある 2000 件の TodoMVC のページを、6 通りの方法でスタイルを付け、同じマークアップのまま hypersig で描画したものです。違うのはクラス名と CSS だけです。Apple M4 Max の Chrome 155 で、CPU を 4 倍に減速して測りました。ブラウザの値は 5 回、サーバーでの描画時間は 50 回の中央値です。",
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
                        ja: "サーバーでの描画時間",
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
        table_of(
            site,
            L {
                en: &["Where hypersig falls behind", "Why"],
                ja: &["hypersig が不利な点", "理由"],
            },
            L {
                en: &[
                    &[
                        "HTML size",
                        "Every element carries one class per declaration, and the class names are longer than those of the other tools. One declaration gets the same class name in the first render and in every later patch, because a patch renders its elements without reading the classes already on the page",
                    ],
                    &[
                        "Server render",
                        "`Page::render` writes those names into every item. The HTML before gzip is 1.21 MB for hypersig and 0.88 MB for Panda CSS, and the render times grow in the same order",
                    ],
                ],
                ja: &[
                    &[
                        "HTML の大きさ",
                        "要素はどれも宣言 1 つにつき 1 つのクラスを持ち、クラス名もほかのツールより長くなります。パッチはページにすでにあるクラスを読まずに要素を描画するので、同じ宣言は最初の描画でも後のパッチでも同じクラス名になります",
                    ],
                    &[
                        "サーバーでの描画時間",
                        "`Page::render` がその長い名前を項目ごとに書き出します。gzip 前の HTML は hypersig が 1.21 MB、Panda CSS が 0.88 MB で、描画時間も同じ順に長くなります",
                    ],
                ],
            },
        ),
        para(
            site,
            L {
                en: "The pages and the harness are in [`bench/style`](https://github.com/imaimai17468/hypersig/tree/main/bench/style).",
                ja: "ページと計測の仕組みは [`bench/style`](https://github.com/imaimai17468/hypersig/tree/main/bench/style) にあります。",
            },
        ),
    ]
}
