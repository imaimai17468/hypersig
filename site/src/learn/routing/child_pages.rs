//! The page on routes declared under another route with `in PARENT`.

use crate::Site;
use crate::doc::{Doc, Learn};
use crate::lang::L;
use crate::learn::snippet::{Snippet, show};
use crate::prose::{Part, bullets, heading, lead, para, parts, title};

pub fn page(site: Site) -> Part {
    parts([
        title(Doc::Learn(Learn::ChildPages).title(site)),
        lead(site, Doc::Learn(Learn::ChildPages).leads()),
        declaring(site),
        compile_errors(site),
        unchecked(site),
        example(site),
        next(site),
    ])
}

/// `in PARENT` and what the child repeats.
fn declaring(site: Site) -> Part {
    parts([
        heading(
            site,
            L {
                en: "Declaring a page under another",
                ja: "ページの下にページを宣言する",
            },
        ),
        para(
            site,
            L {
                en: "`in PARENT` after the path declares a route under the page of an earlier line. Its path is the parent's path followed by `/` and more segments. Its parameters are the parent's, in the same order, followed by its own, so a link to the child needs the parent's values too. Its search parameters are its own and do not come from the parent.",
                ja: "パスのあとに `in PARENT` と書くと、それより前の行のページの下にルートを宣言します。子のパスは親のパスに `/` とセグメントを続けたものです。子のパラメータは親のパラメータを同じ順に並べ、そのあとに自分のパラメータを続けたものです。そのため、子へのリンクには親の値も必要です。子の検索パラメータは子のもので、親からは受け継ぎません。",
            },
        ),
        show(Snippet::ChildRoutes),
        para(
            site,
            L {
                en: "The trait [`Extends<P>`](api:hypersig/trait.Extends.html) holds that rule: a child of a route with parameters `P` needs parameters that implement `Extends<P>`, which a tuple does when it starts with `P`.",
                ja: "この規則はトレイト [`Extends<P>`](api:hypersig/trait.Extends.html) で表します。パラメータが `P` のルートの子には `Extends<P>` を実装したパラメータが必要で、タプルは `P` で始まるときに `Extends<P>` を実装します。",
            },
        ),
    ])
}

/// The mistakes `in PARENT` refuses.
fn compile_errors(site: Site) -> Part {
    parts([
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
                en: "Parameters that do not start with the parent's fail to compile:",
                ja: "親のパラメータで始まらないパラメータは、コンパイルできません。",
            },
        ),
        show(Snippet::WrongChild),
        para(
            site,
            L {
                en: "A path that does not continue the parent's, such as `\"/members/{id}\"` under `\"/orgs/{org}\"`, is error E0080 with the message \"a child route's path is its parent's path followed by `/` and more segments\".",
                ja: "`\"/orgs/{org}\"` の下の `\"/members/{id}\"` のように親のパスの続きになっていないパスは、エラー E0080 になります。メッセージは \"a child route's path is its parent's path followed by `/` and more segments\" です。",
            },
        ),
    ])
}

/// What `in PARENT` leaves to the program.
fn unchecked(site: Site) -> Part {
    parts([
        heading(
            site,
            L {
                en: "What stays unchecked",
                ja: "検査されないこと",
            },
        ),
        bullets(
            site,
            &[
                L {
                    en: "A parent after `in` is a page itself, so a path prefix with no page of its own cannot be one.",
                    ja: "`in` のあとに書く親はそれ自体がページなので、自分のページを持たないパスの接頭辞は親にできません。",
                },
                L {
                    en: "`in` changes what compiles and nothing else. The router serves the child as a page of its own, and a request for the child runs only the child's handler, so a check the parent's handler makes, such as whether the org exists, is made again in the child's.",
                    ja: "`in` が変えるのは、何がコンパイルできるかだけです。ルーターは子を独立したページとして配信し、子へのリクエストでは子のハンドラだけが動きます。そのため、組織があるかどうかのように親のハンドラがする検査は、子のハンドラでもう一度します。",
                },
            ],
        ),
    ])
}

/// An org with members and a member's tabs.
fn example(site: Site) -> Part {
    parts([
        heading(
            site,
            L {
                en: "A complete example",
                ja: "完全な例",
            },
        ),
        para(
            site,
            L {
                en: "An org lists its members, each member's page lies under the org, and the member's tabs lie under the member, with the tab as an enum segment. The org page reads the org from its signals and each id from the list item, and the member page writes the tab's URL on the server.",
                ja: "組織のページはメンバーを一覧にし、各メンバーのページは組織の下に、メンバーのタブはメンバーの下にあります。タブは列挙型のセグメントです。組織のページは組織をシグナルから、各 id をリストの項目から読み、メンバーのページはタブの URL をサーバーで書きます。",
            },
        ),
        show(Snippet::OrgPages),
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
            &[
                L {
                    en: "[Search parameters](learn/search-params/) adds a typed query string to a route.",
                    ja: "[検索パラメータ](learn/search-params/) ではルートに型の付いたクエリ文字列を加えます。",
                },
                L {
                    en: "[Error pages](learn/error-pages/) shows a page for a path under `/{lang}` that no route serves, in the language of the path.",
                    ja: "[エラーページ](learn/error-pages/) では`/{lang}` の下でどのルートも配信しないパスに、パスの言語でページを出します。",
                },
            ],
        ),
    ])
}
