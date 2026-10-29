//! The Components page on `badge`: a short label such as a status, drawn as
//! a small pill.

use crate::Site;
use crate::code::{self, Lang};
use crate::demo::{self, Demo};
use crate::doc::{Component, Doc};
use crate::lang::L;
use crate::learn::snippet::{Snippet, show};
use crate::prose::{Part, heading, lead, para, parts, table_of};

pub fn page(site: Site, head: Part) -> Part {
    parts(vec![
        head,
        lead(site, Doc::Components(Component::Badge).leads()),
        demo::show(site, Demo::Badge),
        accessibility(site),
        building(site),
        html(site),
        compile_errors(site),
        unchecked(site),
        example(site),
    ])
}

fn accessibility(site: Site) -> Part {
    parts([
        heading(
            site,
            L {
                en: "Accessibility",
                ja: "アクセシビリティ",
            },
        ),
        para(
            site,
            L {
                en: "A badge is a `<span>` and takes no role, so a screen reader reads its text where it stands in the page, as part of the sentence or the row around it. Its color is not read. Where the color carries the meaning, put the meaning in the text too, such as \"Failed\" rather than a red badge with no word in it. The badge takes no keys.",
                ja: "バッジは `<span>` でロールを持たないので、スクリーンリーダーはページの中で置かれた位置のテキストを、周りの文や行の一部として読み上げます。色は読み上げられません。色が意味を持つ場合は、文字のない赤いバッジで済ませず、「失敗」のように意味をテキストにも書いてください。バッジはキー操作を受けません。",
            },
        ),
    ])
}

fn building(site: Site) -> Part {
    parts([
        heading(
            site,
            L {
                en: "Building a badge",
                ja: "バッジを作る",
            },
        ),
        para(
            site,
            L {
                en: "[`badge`](api:hypersig_ui/fn.badge.html) takes a [`BadgeVariant`](api:hypersig_ui/enum.BadgeVariant.html) and returns an `El<S>`, so its text comes from `El::text` or `El::bind_text`, and `El::show` hides it:",
                ja: "[`badge`](api:hypersig_ui/fn.badge.html) は [`BadgeVariant`](api:hypersig_ui/enum.BadgeVariant.html) を受け取って `El<S>` を返すので、テキストは `El::text` か `El::bind_text` で入れ、`El::show` で隠せます。",
            },
        ),
        code::block(
            Lang::Rust,
            None,
            "pub fn badge<S>(variant: BadgeVariant) -> El<S>;

#[non_exhaustive]
pub enum BadgeVariant {
    Default,
    Secondary,
    Destructive,
    Outline,
}",
        ),
        table_of(
            site,
            L {
                en: &["`BadgeVariant`", "Look"],
                ja: &["`BadgeVariant`", "見た目"],
            },
            L {
                en: &[
                    &["`BadgeVariant::Default`", "Filled with `token::primary`"],
                    &[
                        "`BadgeVariant::Secondary`",
                        "Filled with `token::secondary`",
                    ],
                    &[
                        "`BadgeVariant::Destructive`",
                        "Filled with `token::destructive`",
                    ],
                    &[
                        "`BadgeVariant::Outline`",
                        "A border in `token::border` and no fill",
                    ],
                ],
                ja: &[
                    &[
                        "`BadgeVariant::Default`",
                        "`token::primary` で塗りつぶします",
                    ],
                    &[
                        "`BadgeVariant::Secondary`",
                        "`token::secondary` で塗りつぶします",
                    ],
                    &[
                        "`BadgeVariant::Destructive`",
                        "`token::destructive` で塗りつぶします",
                    ],
                    &[
                        "`BadgeVariant::Outline`",
                        "`token::border` の枠線だけで、塗りつぶしません",
                    ],
                ],
            },
        ),
        para(
            site,
            L {
                en: "The variant changes only the colors, and every variant renders the same `<span>`. The text stays on one line.",
                ja: "バリアントが変えるのは色だけで、どのバリアントでも同じ `<span>` が出力されます。テキストは折り返さず 1 行に収めます。",
            },
        ),
    ])
}

fn html(site: Site) -> Part {
    parts([
        heading(
            site,
            L {
                en: "The HTML",
                ja: "出力される HTML",
            },
        ),
        para(
            site,
            L {
                en: "The example at the end of this page renders this HTML, with the classes left out. The Failed badge is in the HTML too, hidden while `failed` is false, and is left out here. The browser shows it and hides Passing when the signal changes:",
                ja: "このページの最後の例は次の HTML を出力します（クラスは省いています）。Failed のバッジも HTML にありますが、`failed` が false の間は隠れているので、ここでは省いています。シグナルが変わると、ブラウザは Failed を表示し、Passing を隠します。",
            },
        ),
        code::block(
            Lang::Html,
            None,
            r"<div>
  <span>main </span>
  <span>Passing</span>
  <span>v0.14</span>
</div>",
        ),
    ])
}

fn compile_errors(site: Site) -> Part {
    parts([
        heading(
            site,
            L {
                en: "What fails to compile",
                ja: "コンパイルエラーになるもの",
            },
        ),
        para(
            site,
            L {
                en: "The look is one of four variants, so a string is E0308, mismatched types. A look the theme does not have cannot be named:",
                ja: "見た目は 4 つのバリアントのどれかなので、文字列を渡すと E0308（型の不一致）になります。テーマにない見た目を指定することはできません。",
            },
        ),
        show(Snippet::UiBadgeVariant),
        para(
            site,
            L {
                en: "`BadgeVariant` is `#[non_exhaustive]`, so a `match` on it outside the crate needs a `_` arm.",
                ja: "`BadgeVariant` は `#[non_exhaustive]` なので、クレートの外で `match` するときは `_` の腕が必要です。",
            },
        ),
    ])
}

fn unchecked(site: Site) -> Part {
    parts([
        heading(
            site,
            L {
                en: "What is not checked",
                ja: "検査されないこと",
            },
        ),
        para(
            site,
            L {
                en: "Nothing checks the text. A badge with no text renders as an empty pill that a screen reader skips, and a long text widens the badge instead of wrapping. A badge that only changes color when a signal changes tells a screen reader nothing, so change its text too, as the example does with two badges and `show`.",
                ja: "テキストは検査されません。テキストのないバッジは空の丸い枠として描かれ、スクリーンリーダーは読み飛ばします。長いテキストは折り返さず、バッジが横に広がります。シグナルが変わったときに色だけを変えるバッジは、スクリーンリーダーには何も伝えません。例のように 2 つのバッジと `show` を使って、テキストも変えてください。",
            },
        ),
    ])
}

fn example(site: Site) -> Part {
    parts([
        heading(
            site,
            L {
                en: "Example: the state of a build",
                ja: "例: ビルドの状態",
            },
        ),
        para(
            site,
            L {
                en: "One of two badges shows, depending on `failed`, and an outline badge shows the version:",
                ja: "`failed` に応じて 2 つのバッジのどちらかが表示され、枠線だけのバッジがバージョンを表示します。",
            },
        ),
        show(Snippet::UiBadge),
        para(
            site,
            L {
                en: "A badge whose colors are set by hand is on the [Colors](style/colors/) page of the Style section. A message with a title and a description is an [Alert](components/alert/).",
                ja: "色を自分で指定するバッジは、スタイルのセクションの [色](style/colors/) のページで説明しています。タイトルと説明を持つメッセージには [アラート](components/alert/) を使います。",
            },
        ),
    ])
}
