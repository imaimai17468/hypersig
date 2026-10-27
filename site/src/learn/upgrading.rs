//! The page on moving an application to a newer release: each breaking
//! change of a public signature, shown as the code after it.

use super::snippet::{Snippet, show_change};
use crate::Site;
use crate::doc::{Doc, Learn};
use crate::lang::L;
use crate::prose::{Part, bullets, heading, lead, para, parts, title};

pub fn upgrading(site: Site) -> Part {
    parts([
        title(Doc::Learn(Learn::Upgrading).title(site)),
        lead(site, Doc::Learn(Learn::Upgrading).leads()),
        para(
            site,
            L {
                en: "The releases that changed a public signature, 0.5 through 0.10, each raised the minor version, so `hypersig = \"0.7\"` in `Cargo.toml` stays on 0.7 until you change it. Every crate of the workspace is released at one version, so raise `hypersig`, `hypersig_ui`, `hypersig_i18n`, and `cargo install hypersig_cli` together. The samples below are the code after each change: a line marked `-` is what the older release accepted, and the line marked `+` replaces it. The compiler stops at each line that still has the old form, so work through its errors from the top. [CHANGELOG.md](https://github.com/imaimai17468/hypersig/blob/main/CHANGELOG.md) lists every change of every release, including the ones that need nothing from you.",
                ja: "公開しているシグネチャを変えた 0.5 から 0.10 までのリリースでは、そのたびにマイナーバージョンを上げています。そのため、`Cargo.toml` に `hypersig = \"0.7\"` と書いたプロジェクトは、書き換えるまで 0.7 のままです。ワークスペースのクレートはすべて同じバージョンで公開するので、`hypersig`、`hypersig_ui`、`hypersig_i18n` と `cargo install hypersig_cli` は一緒に上げてください。下のサンプルは変更後のコードです。`-` の付いた行は前のリリースで書けた形で、`+` の付いた行がそれに置き換わります。古い形が残る行ではコンパイルが止まるので、エラーを上から順に直してください。リリースごとの変更は、手を入れなくてよいものも含めて [CHANGELOG.md](https://github.com/imaimai17468/hypersig/blob/main/CHANGELOG.md) に載っています。",
            },
        ),
        to_0_10(site),
        to_0_9(site),
        to_0_8(site),
        to_0_7(site),
        to_0_6(site),
        to_0_5(site),
    ])
}

fn to_0_10(site: Site) -> Part {
    parts([
        heading(
            site,
            L {
                en: "From 0.9 to 0.10",
                ja: "0.9 から 0.10 へ",
            },
        ),
        para(
            site,
            L {
                en: "A route parameter that holds any text is a [`PathSegment`](api:hypersig/struct.PathSegment.html) in place of a `String`. It refuses an empty value, `.`, and `..`, which a link cannot carry. `PathSegment::new` refuses one at compile time in a `const`, and `PathSegment::parse` returns an error for text read from data. A handler takes the `PathSegment`, and `as_str` gives its text.",
                ja: "任意の文字列を持つルートのパラメータは、`String` の代わりに [`PathSegment`](api:hypersig/struct.PathSegment.html) で受け取るようになりました。`PathSegment` は、リンクに載せられない空の値、`.`、`..` を拒否します。`const` の中の `PathSegment::new` はこれらをコンパイル時に拒否し、データから読んだ文字列には `PathSegment::parse` がエラーを返します。ハンドラは `PathSegment` を受け取り、文字列は `as_str` で取り出せます。",
            },
        ),
        show_change(Snippet::Upgrade010),
        para(
            site,
            L {
                en: "The other changes that need an edit:",
                ja: "ほかに書き換えが必要な変更は次のとおりです。",
            },
        ),
        bullets(
            site,
            &[
                L {
                    en: "`Rest::new` takes a `&'static str`. Build a `Rest` from text read at run time with `Rest::parse`, which refuses an empty rest and a `.` or `..` part.",
                    ja: "`Rest::new` は `&'static str` を受け取ります。実行時に読んだ文字列からは `Rest::parse` で作ってください。`Rest::parse` は空の値と、`.` や `..` の部分を拒否します。",
                },
                L {
                    en: "A signal field that a link reads as a segment becomes a `PathSegment`, and so does a `String` sort key of `hypersig_ui`'s `Table`.",
                    ja: "リンクがセグメントとして読むシグナルのフィールドは `PathSegment` にしてください。`hypersig_ui` の `Table` の `String` のソートキーも同じです。",
                },
                L {
                    en: "A `#[derive(Segment)]` variant renamed to an empty name, `.`, or `..` fails to compile.",
                    ja: "`#[derive(Segment)]` のバリアントを空の名前、`.`、`..` にリネームすると、コンパイルが通りません。",
                },
            ],
        ),
    ])
}

fn to_0_9(site: Site) -> Part {
    parts([
        heading(
            site,
            L {
                en: "From 0.8 to 0.9",
                ja: "0.8 から 0.9 へ",
            },
        ),
        para(
            site,
            L {
                en: "The URLs a page loads and its language are typed values now. [`Page::script`](api:hypersig/struct.Page.html#method.script), `stylesheet`, `icon`, and `runtime_src` take an [`Asset`](api:hypersig/struct.Asset.html), and [`Page::lang`](api:hypersig/struct.Page.html#method.lang) and the `hreflang` of `Page::alternate` take a [`LanguageTag`](api:hypersig/struct.LanguageTag.html). Both check their text when the constant is built, and an `Asset` path starting with `/` is written under the page's `Base`.",
                ja: "ページが読み込む URL とページの言語は、型の付いた値で渡すようになりました。[`Page::script`](api:hypersig/struct.Page.html#method.script)、`stylesheet`、`icon`、`runtime_src` は [`Asset`](api:hypersig/struct.Asset.html) を、[`Page::lang`](api:hypersig/struct.Page.html#method.lang) と `Page::alternate` の `hreflang` は [`LanguageTag`](api:hypersig/struct.LanguageTag.html) を受け取ります。どちらも定数を作るときに文字列を検査し、`/` で始まる `Asset` のパスはページの `Base` の下に書かれます。",
            },
        ),
        show_change(Snippet::Upgrade09),
        para(
            site,
            L {
                en: "The other changes that need an edit:",
                ja: "ほかに書き換えが必要な変更は次のとおりです。",
            },
        ),
        bullets(
            site,
            &[
                L {
                    en: "`Locale::ALL` is gone. `Locale` requires `Variants`, which `#[derive(Segment)]` already writes, and the first variant is the default.",
                    ja: "`Locale::ALL` はなくなりました。`Locale` は `Variants` を求めます。`Variants` は `#[derive(Segment)]` が書くもので、最初のバリアントが既定のロケールです。",
                },
                L {
                    en: "`hypersig_ui::Tabs::new` and `Listbox::new` take one closure over every variant in place of `.tab(..)` and `.option(..)`, and `Accordion::exclusive` takes an `Id`.",
                    ja: "`hypersig_ui::Tabs::new` と `Listbox::new` は `.tab(..)` と `.option(..)` の代わりに全バリアントを受け取るクロージャを 1 つ受け取り、`Accordion::exclusive` は `Id` を受け取ります。",
                },
                L {
                    en: "`UploadedFile::file_name` and `StreamedFile::file_name` return `Option<FileName>`. Call `base_name` for a name to store a file under.",
                    ja: "`UploadedFile::file_name` と `StreamedFile::file_name` は `Option<FileName>` を返します。ファイルを保存するときの名前には `base_name` を使ってください。",
                },
                L {
                    en: "`hypersig_ui::Chart` has two type parameters. Code that named the third drops it.",
                    ja: "`hypersig_ui::Chart` の型引数は 2 つになりました。3 つ目を書いていたコードはそれを消してください。",
                },
                L {
                    en: "A test that builds a GET action request by hand adds the `hypersig-call` header, or uses `Action::request`, which adds it.",
                    ja: "GET のアクションのリクエストをテストで手作りしている場合は、`hypersig-call` のヘッダーを付けるか、ヘッダーを付ける `Action::request` を使ってください。",
                },
                L {
                    en: "Two elements with one `Id::with_key` id, a `Page::computed` signal named twice or read before it is computed, a form rule that reads `Expr::now`, and a list item read outside its list now panic while the page is built.",
                    ja: "同じ `Id::with_key` の id を持つ 2 つの要素、2 回指定したか計算前に読んだ `Page::computed` のシグナル、`Expr::now` を読むフォームの規則、リストの外で読んだリストの項目は、ページを組み立てるときにパニックするようになりました。",
                },
            ],
        ),
    ])
}

fn to_0_8(site: Site) -> Part {
    parts([
        heading(
            site,
            L {
                en: "From 0.7 to 0.8",
                ja: "0.7 から 0.8 へ",
            },
        ),
        para(
            site,
            L {
                en: "One [`Page::framing`](api:hypersig/struct.Page.html#method.framing) call sets which sites may show the page in a frame, in place of `framed_by` and `framed_anywhere`. The attributes that hold URLs or CSS left `html::attr`, so each has a method that takes the checked type: [`El::srcset`](api:hypersig/html/struct.El.html#method.srcset) a `SafeUrl` and a `Candidate` per image, [`El::poster`](api:hypersig/html/struct.El.html#method.poster) a `SafeUrl`, and `El::css` or `El::bind_style` the style. `el(tag!(\"link\"))` no longer compiles, and [`Page::stylesheet`](api:hypersig/struct.Page.html#method.stylesheet) and [`Page::icon`](api:hypersig/struct.Page.html#method.icon) write the page's `<link>` elements.",
                ja: "ページを枠に表示してよいサイトは、`framed_by` と `framed_anywhere` の代わりに 1 回の [`Page::framing`](api:hypersig/struct.Page.html#method.framing) で決めます。URL や CSS を値に持つ属性は `html::attr` からなくなり、検査済みの型を受け取るメソッドで書きます。[`El::srcset`](api:hypersig/html/struct.El.html#method.srcset) は画像ごとに `SafeUrl` と `Candidate` を、[`El::poster`](api:hypersig/html/struct.El.html#method.poster) は `SafeUrl` を受け取り、スタイルは `El::css` か `El::bind_style` で書きます。`el(tag!(\"link\"))` はコンパイルが通らなくなりました。ページの `<link>` は [`Page::stylesheet`](api:hypersig/struct.Page.html#method.stylesheet) と [`Page::icon`](api:hypersig/struct.Page.html#method.icon) が書きます。",
            },
        ),
        show_change(Snippet::Upgrade08),
        para(
            site,
            L {
                en: "0.8.2 changes no signature, but several mistakes that used to do nothing visible now panic while the page is built, so a page that panics after the upgrade had one of them:",
                ja: "0.8.2 はシグネチャを変えていませんが、これまで目に見える結果を残さなかった次の間違いが、ページを組み立てるときにパニックするようになりました。アップグレードのあとでパニックするページには、このどれかがあります。",
            },
        ),
        bullets(
            site,
            &[
                L {
                    en: "A second `show`, `bind_text`, `id`, `node_ref`, `foreign`, or binding on one element, which used to replace the first.",
                    ja: "1 つの要素に 2 回目の `show`、`bind_text`、`id`、`node_ref`、`foreign`、束縛を付けること。これまでは 2 回目が 1 回目を置き換えていました。",
                },
                L {
                    en: "One static attribute set twice, through `attr` or a method such as `role`. The browser keeps only the first.",
                    ja: "`attr` や `role` のようなメソッドで、同じ静的な属性を 2 回付けること。ブラウザは 1 つ目しか使いません。",
                },
                L {
                    en: "A binding on an element the runtime does not bind, such as `bind` on a `<div>`.",
                    ja: "`<div>` に付けた `bind` のように、ランタイムが束縛しない要素への束縛。",
                },
                L {
                    en: "An `on_interval` of 0, or an `on_interval`, `on_timeout`, or `on_debounced` longer than 2^31 - 1 ms, which the browser runs at once.",
                    ja: "0 の `on_interval` と、2^31 - 1 ミリ秒より長い `on_interval`、`on_timeout`、`on_debounced`。ブラウザはこれらをすぐに実行します。",
                },
            ],
        ),
    ])
}

fn to_0_7(site: Site) -> Part {
    parts([
        heading(
            site,
            L {
                en: "From 0.6 to 0.7",
                ja: "0.6 から 0.7 へ",
            },
        ),
        para(
            site,
            L {
                en: "A field that [`Page::computed`](api:hypersig/struct.Page.html#method.computed) keeps equal to an expression is marked `#[signals(computed)]` in the struct, and its `Sig` has no `set`, `toggle`, list statement, `bind`, or `patch`. In 0.6 a write to it compiled, and the written value stayed on screen until a signal the expression reads changed. Write one of those signals instead, as the button below now empties `quantity`.",
                ja: "[`Page::computed`](api:hypersig/struct.Page.html#method.computed) が式と等しく保つフィールドには、構造体の中で `#[signals(computed)]` を付けます。そのフィールドの `Sig` には `set`、`toggle`、リストの文、`bind`、`patch` がありません。0.6 では書き込みのコードもコンパイルが通り、式が読むシグナルが次に変わるまで、書き込んだ値が画面に残っていました。下のボタンが `quantity` を 0 にするように、式が読むシグナルのほうに書き込んでください。",
            },
        ),
        show_change(Snippet::Upgrade07),
    ])
}

fn to_0_6(site: Site) -> Part {
    parts([
        heading(
            site,
            L {
                en: "From 0.5 to 0.6",
                ja: "0.5 から 0.6 へ",
            },
        ),
        para(
            site,
            L {
                en: "An action, a stream, and an upload are lines of [`routes!`](api:hypersig/macro.routes.html), each with its handler, so `Action::post`, `StreamAction::new`, `Upload::post`, and the router's `.action(ADD, add)` are gone. Move each constant into the block that serves its page and delete the registration. A `NodeRef` or `DialogRef` is declared with [`node_ref!`](api:hypersig/macro.node_ref.html) or [`dialog_ref!`](api:hypersig/macro.dialog_ref.html) at module level, which names it after the constant's path. The methods of `Expr` take `&self`, so a `.clone()` written to call one can go, and a method passed as a function, such as `Expr::and`, becomes a closure.",
                ja: "アクション、ストリーム、アップロードは [`routes!`](api:hypersig/macro.routes.html) の行になり、それぞれのハンドラと一緒に宣言します。そのため `Action::post`、`StreamAction::new`、`Upload::post` と、ルーターの `.action(ADD, add)` はなくなりました。定数はページを配信するブロックに移し、登録の行は消してください。`NodeRef` と `DialogRef` はモジュールの直下で [`node_ref!`](api:hypersig/macro.node_ref.html) か [`dialog_ref!`](api:hypersig/macro.dialog_ref.html) を使って宣言し、マクロが定数のパスから名前を付けます。`Expr` のメソッドは `&self` を受け取るようになったので、呼び出しのために書いた `.clone()` は消せます。`Expr::and` のようにメソッドを関数として渡すコードは、クロージャに書き換えてください。",
            },
        ),
        show_change(Snippet::Upgrade06),
        para(
            site,
            L {
                en: "`hypersig_ui`'s `Tooltip::new`, `Menu::new`, `Dialog::new`, and `Combobox::new` take the `NodeRef` or `DialogRef` in place of a name.",
                ja: "`hypersig_ui` の `Tooltip::new`、`Menu::new`、`Dialog::new`、`Combobox::new` は、名前の文字列の代わりに `NodeRef` か `DialogRef` を受け取ります。",
            },
        ),
    ])
}

fn to_0_5(site: Site) -> Part {
    parts([
        heading(
            site,
            L {
                en: "From 0.4 to 0.5",
                ja: "0.4 から 0.5 へ",
            },
        ),
        para(
            site,
            L {
                en: "Attribute names, roles, ids, and tag names are types. `attr`, `bind_attr`, and `attr_when` take an `Attr`: a constant of [`html::attr`](api:hypersig/html/attr/index.html), or [`attr!`](api:hypersig/macro.attr.html) for a name the list lacks, such as a `data-` attribute. The role is [`El::role`](api:hypersig/html/struct.El.html#method.role). `El::id` takes an [`Id`](api:hypersig/html/struct.Id.html), and an attribute that points at an element, such as `for` or `aria-describedby`, has a method that takes the same `Id`, so the two cannot be spelled apart. `el` takes the [`Tag`](api:hypersig/html/struct.Tag.html) that [`tag!`](api:hypersig/macro.tag.html) checks at compile time. Code that `cargo hypersig elements` wrote for 0.4 is written again by the 0.5 command.",
                ja: "属性名、ロール、id、タグ名は型になりました。`attr`、`bind_attr`、`attr_when` は `Attr` を受け取ります。[`html::attr`](api:hypersig/html/attr/index.html) の定数を使い、`data-` 属性のように一覧にない名前は [`attr!`](api:hypersig/macro.attr.html) で作ります。ロールは [`El::role`](api:hypersig/html/struct.El.html#method.role) で付けます。`El::id` は [`Id`](api:hypersig/html/struct.Id.html) を受け取り、`for` や `aria-describedby` のように要素を指す属性には同じ `Id` を受け取るメソッドがあるので、2 か所の綴りが食い違うことはありません。`el` は、[`tag!`](api:hypersig/macro.tag.html) がコンパイル時に検査した [`Tag`](api:hypersig/html/struct.Tag.html) を受け取ります。0.4 の `cargo hypersig elements` が書いたコードは、0.5 のコマンドで書き直してください。",
            },
        ),
        show_change(Snippet::Upgrade05),
    ])
}
