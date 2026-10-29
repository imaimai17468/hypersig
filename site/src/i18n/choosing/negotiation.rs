//! The i18n page on `negotiate`: how it matches an `Accept-Language` value
//! against the locales, and using it in a handler of your own.

use crate::Site;
use crate::code::{self, Lang};
use crate::doc::{Doc, I18nPage};
use crate::lang::L;
use crate::learn::snippet::{Snippet, show};
use crate::prose::{Part, bullets, heading, lead, para, parts, table_of};

pub fn page(site: Site, head: Part) -> Part {
    parts(vec![
        head,
        lead(site, Doc::I18n(I18nPage::Negotiation).leads()),
        negotiate(site),
        rules(site),
        handler(site),
        unchecked(site),
        next(site),
    ])
}

/// The signature of `negotiate`.
fn negotiate(site: Site) -> Part {
    parts([
        heading(
            site,
            L {
                en: "negotiate",
                ja: "negotiate",
            },
        ),
        code::block(
            Lang::Rust,
            None,
            "pub fn negotiate<L: Locale>(accept_language: &str) -> L;",
        ),
        para(
            site,
            L {
                en: "A browser sends the languages its user reads in the `Accept-Language` header, in order of preference, as in `Accept-Language: ja-JP,ja;q=0.9,en;q=0.8`. [`negotiate`](api:hypersig_i18n/fn.negotiate.html) reads that value and returns the variant of `L` that matches it best, or the first variant, the default language, where none matches.",
                ja: "ブラウザは、利用者が読む言語を好みの順に `Accept-Language` ヘッダーで送ります。`Accept-Language: ja-JP,ja;q=0.9,en;q=0.8` のような形です。[`negotiate`](api:hypersig_i18n/fn.negotiate.html) はこの値を読み、`L` のバリアントから最も合うものを返します。どれにも合わなければ、最初のバリアント、つまり既定の言語を返します。",
            },
        ),
        para(
            site,
            L {
                en: "The value is text the visitor's browser chose, so the types check nothing about it. They check the result instead: it is always a variant of `L`, so a handler that matches on it covers every language the site has, and a language the site lacks never comes out.",
                ja: "値は訪問者のブラウザが決めた文字列なので、型はその中身を検査しません。代わりに型が保証するのは結果です。結果は必ず `L` のバリアントなので、結果に対する `match` はサイトのすべての言語を扱い、サイトにない言語が返ることはありません。",
            },
        ),
    ])
}

/// The matching rules, as a table of inputs and results.
fn rules(site: Site) -> Part {
    parts([
        heading(
            site,
            L {
                en: "How it matches",
                ja: "照合の規則",
            },
        ),
        bullets(
            site,
            &[
                L {
                    en: "It tries the ranges from the highest `q` to the lowest, and in the order written where two have the same `q`. A range without `q` has `q=1`. A range with `q=0`, or with a `q` that is not a number, is skipped.",
                    ja: "範囲は `q` の大きい順に試し、`q` が同じものは書かれた順に試します。`q` のない範囲は `q=1` です。`q=0` の範囲と、`q` が数値でない範囲は飛ばします。",
                },
                L {
                    en: "A range matches a tag equal to it, ignoring case. Then it drops its last `-subtag`, one at a time, and matches a tag equal to what is left, so `ja-JP` matches `ja`.",
                    ja: "範囲は大文字と小文字を区別せずに、範囲と等しいタグに一致します。次に、後ろの `-subtag` を 1 つずつ削り、残りと等しいタグに一致します。そのため `ja-JP` は `ja` に一致します。",
                },
                L {
                    en: "Last, a range matches a tag that starts with the range and a `-`, so `pt` matches `pt-BR`. This uses the range as written, so `pt-PT` does not match `pt-BR`.",
                    ja: "最後に、範囲と `-` で始まるタグに一致します。そのため `pt` は `pt-BR` に一致します。ここでは書かれたままの範囲を使うので、`pt-PT` は `pt-BR` に一致しません。",
                },
                L {
                    en: "`*` gives the default at once, so ranges after it are not tried. An empty header, and a header with no match, give the default too.",
                    ja: "`*` はその場で既定の言語を返すので、後ろの範囲は試しません。空のヘッダーと、どれにも一致しないヘッダーも既定の言語になります。",
                },
            ],
        ),
        para(
            site,
            L {
                en: "For an enum of `en`, `ja`, and `pt-BR`, declared in that order:",
                ja: "`en`、`ja`、`pt-BR` をこの順に宣言した enum では、次のようになります。",
            },
        ),
        table_of(
            site,
            L {
                en: &["Accept-Language", "Result", "Why"],
                ja: &["Accept-Language", "結果", "理由"],
            },
            L {
                en: &[
                    &["`ja`", "`Ja`", "Equal"],
                    &["`JA`", "`Ja`", "Equal, ignoring case"],
                    &["`ja-JP`", "`Ja`", "`ja-JP` without `-JP` is `ja`"],
                    &["`pt-br`", "`PtBr`", "Equal, ignoring case"],
                    &["`pt`", "`PtBr`", "`pt-BR` starts with `pt-`"],
                    &[
                        "`pt-PT`",
                        "`En`",
                        "No tag is `pt-PT` or `pt`, or starts with `pt-PT-`",
                    ],
                    &[
                        "`ja-JP,ja;q=0.9,en;q=0.8`",
                        "`Ja`",
                        "`ja-JP` has the highest `q`",
                    ],
                    &["`en;q=0.5, ja`", "`Ja`", "`ja` has `q=1`"],
                    &[
                        "`en;q=0.8, ja;q=0.8`",
                        "`En`",
                        "Same `q`, so the written order",
                    ],
                    &["`fr, ja;q=0.5`", "`Ja`", "`fr` matches nothing"],
                    &["`ja;q=0, pt`", "`PtBr`", "`q=0` is skipped"],
                    &[
                        "`ja;q=nope`",
                        "`En`",
                        "A `q` that is not a number is skipped",
                    ],
                    &["`ja, *`", "`Ja`", "`ja` comes before `*`"],
                    &["`*, ja`", "`En`", "`*` gives the default"],
                    &["`fr`", "`En`", "No match"],
                    &["(empty)", "`En`", "No range"],
                ],
                ja: &[
                    &["`ja`", "`Ja`", "等しい"],
                    &["`JA`", "`Ja`", "大文字と小文字を区別せずに等しい"],
                    &["`ja-JP`", "`Ja`", "`ja-JP` から `-JP` を削ると `ja`"],
                    &["`pt-br`", "`PtBr`", "大文字と小文字を区別せずに等しい"],
                    &["`pt`", "`PtBr`", "`pt-BR` は `pt-` で始まる"],
                    &[
                        "`pt-PT`",
                        "`En`",
                        "`pt-PT` と `pt` に等しいタグも、`pt-PT-` で始まるタグもない",
                    ],
                    &[
                        "`ja-JP,ja;q=0.9,en;q=0.8`",
                        "`Ja`",
                        "`ja-JP` の `q` が最も大きい",
                    ],
                    &["`en;q=0.5, ja`", "`Ja`", "`ja` は `q=1`"],
                    &["`en;q=0.8, ja;q=0.8`", "`En`", "`q` が同じなので書かれた順"],
                    &["`fr, ja;q=0.5`", "`Ja`", "`fr` はどれにも一致しない"],
                    &["`ja;q=0, pt`", "`PtBr`", "`q=0` は飛ばす"],
                    &["`ja;q=nope`", "`En`", "数値でない `q` は飛ばす"],
                    &["`ja, *`", "`Ja`", "`ja` が `*` より前"],
                    &["`*, ja`", "`En`", "`*` は既定の言語"],
                    &["`fr`", "`En`", "一致しない"],
                    &["（空）", "`En`", "範囲がない"],
                ],
            },
        ),
        para(
            site,
            L {
                en: "Each row is one assertion of this sample:",
                ja: "表の各行は次のサンプルの 1 つのアサーションに当たります。",
            },
        ),
        show(Snippet::I18nNegotiate),
    ])
}

/// `negotiate` in a handler of your own.
fn handler(site: Site) -> Part {
    parts([
        heading(
            site,
            L {
                en: "In a handler of your own",
                ja: "独自のハンドラで使う",
            },
        ),
        para(
            site,
            L {
                en: "[The redirect from /](i18n/redirect/) calls `negotiate` for `GET /`. Anywhere else, read the header from axum's `HeaderMap` and pass its text. A header that is not visible ASCII has no text, and `unwrap_or_default` turns it into an empty value, which gives the default. This handler sends the links a site had before it added languages, such as `/members/7`, to the member's page in the visitor's language:",
                ja: "[/ からのリダイレクト](i18n/redirect/) は `GET /` に対して `negotiate` を呼びます。それ以外の場所では、axum の `HeaderMap` からヘッダーを読み、その文字列を渡します。表示できる ASCII 以外を含むヘッダーは文字列にならず、`unwrap_or_default` で空の値になるので、既定の言語になります。次のハンドラは言語を追加する前のサイトにあった `/members/7` のようなリンクを、訪問者の言語のメンバーのページへ送ります。",
            },
        ),
        show(Snippet::I18nNegotiateHandler),
        para(
            site,
            L {
                en: "The response depends on `Accept-Language`, so it carries `Vary: Accept-Language`. Without it, a cache between the server and the browser could keep the redirect to `/ja/members/7` and send a visitor who reads English there.",
                ja: "レスポンスは `Accept-Language` によって変わるので、`Vary: Accept-Language` を付けます。これがないと、サーバーとブラウザの間のキャッシュが `/ja/members/7` へのリダイレクトを保存し、英語を読む訪問者もそこへ送ることがあります。",
            },
        ),
    ])
}

/// What `negotiate` leaves to the program.
fn unchecked(site: Site) -> Part {
    parts([
        heading(
            site,
            L {
                en: "What stays unchecked",
                ja: "検査しないこと",
            },
        ),
        bullets(
            site,
            &[
                L {
                    en: "`Accept-Language` is the browser's setting, not the visitor's choice on the site. A visitor who picked another language with a [language switcher](i18n/language-switcher/) is sent by `negotiate` back to the browser's language, so keep the visitor's choice in a cookie or a [session](learn/sessions/) and read it before the header.",
                    ja: "`Accept-Language` はブラウザの設定で、訪問者がサイトで選んだ言語ではありません。[言語を切り替えるリンク](i18n/language-switcher/) で別の言語を選んだ訪問者も、`negotiate` はブラウザの言語へ戻します。訪問者の選んだ言語は Cookie か [セッション](learn/sessions/) に保存し、ヘッダーより先に読んでください。",
                },
                L {
                    en: "`negotiate` panics if `L` has no variant. `#[derive(Segment)]` refuses an enum without one at compile time, so an enum that derives it never reaches the panic.",
                    ja: "`L` にバリアントが 1 つもないと `negotiate` はパニックします。`#[derive(Segment)]` はバリアントのない enum をコンパイル時に拒否するので、derive した enum がこのパニックに至ることはありません。",
                },
                L {
                    en: "Each call reads every variant's `tag`, so a variant whose segment is not shaped as a tag panics here as in [The language of the page](i18n/page-language/#a-segment-that-is-not-a-tag).",
                    ja: "呼び出すたびにすべてのバリアントの `tag` を読むので、タグの形をしていないセグメントのバリアントは、[ページの言語](i18n/page-language/#a-segment-that-is-not-a-tag) と同じくここでパニックします。",
                },
            ],
        ),
    ])
}

/// The pages after this one.
fn next(site: Site) -> Part {
    parts([
        heading(
            site,
            L {
                en: "Next",
                ja: "次に読むページ",
            },
        ),
        bullets(
            site,
            &[L {
                en: "[The redirect from /](i18n/redirect/) answers `GET /` with a redirect to the page `negotiate` picks.",
                ja: "[/ からのリダイレクト](i18n/redirect/) では `GET /` に対して `negotiate` が選んだページへのリダイレクトを返します。",
            }],
        ),
    ])
}
