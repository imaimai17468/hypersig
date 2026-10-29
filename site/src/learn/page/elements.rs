//! The page on elements, their attributes, and the ids they point at.

use crate::Site;
use crate::doc::{Doc, Learn};
use crate::lang::L;
use crate::learn::snippet::{Snippet, show};
use crate::prose::{Part, heading, lead, para, parts, table_of, title};

pub fn page(site: Site) -> Part {
    parts(vec![
        title(Doc::Learn(Learn::Elements).title(site)),
        lead(site, Doc::Learn(Learn::Elements).leads()),
        elements(site),
        children(site),
        attributes(site),
        refused(site),
        ids(site),
        urls(site),
        example(site),
    ])
}

fn elements(site: Site) -> Part {
    parts([
        heading(
            site,
            L {
                en: "Elements",
                ja: "要素",
            },
        ),
        para(
            site,
            L {
                en: "The [`html`](api:hypersig/html/index.html) module has one function per common tag, such as `div()`, `p()`, and `button()`. Each has the signature `fn div<S>() -> El<S>` and returns an [`El<S>`](api:hypersig/html/struct.El.html), where `S` is the page's signal struct. The methods of `El` take the element and return it, so a tree is one expression, and `render` returns its HTML.",
                ja: "[`html`](api:hypersig/html/index.html) モジュールには `div()`、`p()`、`button()` のように、よく使うタグごとの関数があります。シグネチャはどれも `fn div<S>() -> El<S>` の形で、[`El<S>`](api:hypersig/html/struct.El.html) を返します。`S` はページのシグナル構造体です。`El` のメソッドは要素を受け取って要素を返すので、木全体を 1 つの式で書けます。`render` はその HTML を返します。",
            },
        ),
        show(Snippet::Elements),
        para(
            site,
            L {
                en: "`div::<Profile>()` names the page's signal struct because nothing else in this code tells the compiler which page the element belongs to. An element passed to `Page::new(...).child(...)`, or to a function that takes an `El<Profile>`, gets its `S` from there, which is why the Quick start writes `div()`. The children of an element take its `S` the same way, so only the outermost element of a tree written on its own needs the `::<Profile>`.",
                ja: "`div::<Profile>()` とページのシグナル構造体を書いているのは、このコードにはこの要素がどのページのものかをコンパイラに伝えるものがほかにないからです。`Page::new(...).child(...)` や `El<Profile>` を受け取る関数に渡す要素は、渡し先から `S` が推論されます。クイックスタートが `div()` と書けるのはそのためです。子要素も親から同じように `S` を受け取るので、単独で書いた木でも `::<Profile>` が必要なのはいちばん外側の要素だけです。",
            },
        ),
        heading(
            site,
            L {
                en: "An element belongs to one page",
                ja: "要素は 1 つのページに属する",
            },
        ),
        para(
            site,
            L {
                en: "`El::child` has the signature `fn child(self, child: impl IntoNode<S>) -> Self`. [`IntoNode<S>`](api:hypersig/html/trait.IntoNode.html) is implemented for an `El<S>`, a `Node<S>`, and text, all with the same `S`. An element built for another page's signals carries that page's `S`, so it cannot sit inside this page, and neither can the signals and actions it reads:",
                ja: "`El::child` のシグネチャは `fn child(self, child: impl IntoNode<S>) -> Self` です。[`IntoNode<S>`](api:hypersig/html/trait.IntoNode.html) を実装しているのは、同じ `S` を持つ `El<S>`、`Node<S>`、テキストです。別のページのシグナル用に作った要素はそのページの `S` を持つので、このページの中には置けません。その要素が読むシグナルやアクションも同様です。",
            },
        ),
        show(Snippet::OtherPage),
    ])
}

fn children(site: Site) -> Part {
    parts([
        heading(
            site,
            L {
                en: "Text and children",
                ja: "テキストと子要素",
            },
        ),
        para(
            site,
            L {
                en: "`text` appends static text, and `child` appends an element or a string. `children` appends each item of an iterator, and an `Option` counts as zero or one child. The renderer writes `&`, `<`, and `>` in text as `&amp;`, `&lt;`, and `&gt;`, so text from a visitor shows as text.",
                ja: "`text` は固定のテキストを、`child` は要素か文字列を末尾に追加します。`children` はイテレータの各要素を追加し、`Option` は 0 個か 1 個の子として数えます。テキストの `&`、`<`、`>` は `&amp;`、`&lt;`、`&gt;` として書き出されるので、訪問者が入力したテキストもテキストとして表示されます。",
            },
        ),
        show(Snippet::Children),
        para(
            site,
            L {
                en: "The types do not check which elements may sit inside which. One case panics when `child` runs: a `p` given a child at which the HTML parser ends a paragraph, such as a `div`, a `ul`, or another `p`, outside a `button`, a `table`, or a template, because the browser would end the `p` there and move the element after it. Write such a `p` as a `div`. Any other nesting the HTML standard forbids is written as given, and the browser repairs it as it parses.",
                ja: "どの要素の中にどの要素を置けるかは、型では検査しません。`child` の実行時にパニックするのは 1 つの場合だけです。`p` に、HTML パーサが段落を閉じる要素（`div`、`ul`、別の `p` など）を子として渡したときで、`button`、`table`、テンプレートの中は除きます。ブラウザはそこで `p` を閉じ、その要素を `p` の後ろに移すからです。そのような `p` は `div` で書いてください。HTML 標準が禁じるほかの入れ子はそのまま書き出され、ブラウザが解析するときに修復します。",
            },
        ),
    ])
}

fn attributes(site: Site) -> Part {
    parts([
        heading(
            site,
            L {
                en: "Attributes",
                ja: "属性",
            },
        ),
        para(
            site,
            L {
                en: "`attr` has the signature `fn attr(self, name: Attr, value: impl Into<String>) -> Self` and sets a plain attribute, such as `.attr(attr::PLACEHOLDER, \"Name\")`. The name is an [`Attr`](api:hypersig/html/struct.Attr.html). The module [`html::attr`](api:hypersig/html/attr/index.html) holds one for each attribute of HTML, SVG, and ARIA, so a misspelled name stops the build. A name outside those lists, such as `data-lat` or an attribute of a custom element, is written `hypersig::attr!(\"data-lat\")`, which checks it at compile time.",
                ja: "`attr` のシグネチャは `fn attr(self, name: Attr, value: impl Into<String>) -> Self` で、`.attr(attr::PLACEHOLDER, \"Name\")` のように普通の属性を設定します。名前は [`Attr`](api:hypersig/html/struct.Attr.html) で受け取ります。HTML、SVG、ARIA の属性はモジュール [`html::attr`](api:hypersig/html/attr/index.html) に 1 つずつ定数があるので、名前を書き間違えるとビルドが止まります。`data-lat` やカスタム要素の属性のように一覧にない名前は `hypersig::attr!(\"data-lat\")` と書き、コンパイル時に検査されます。",
            },
        ),
        para(
            site,
            L {
                en: "A tag with no function of its own, such as an SVG element or a custom element, is `el(hypersig::tag!(\"geo-map\"))`. [`el`](api:hypersig/html/fn.el.html) takes a [`Tag`](api:hypersig/html/struct.Tag.html), and `tag!` checks at compile time that the name is letters, digits, `-`, `_`, `.`, and `:`, starting with a letter, and that it is not `script`, `style`, `base`, or `link`, in any case. The first two run their text as code, `base` moves every relative URL of the page to another origin, and `link` loads from the host its `href` names. `Page::stylesheet` and `Page::icon` write the `<link>`s a page needs, as [Styling](learn/styling/) and [The page and its head](learn/page/) show.",
                ja: "SVG の要素やカスタム要素のように専用の関数がないタグは、`el(hypersig::tag!(\"geo-map\"))` と書きます。[`el`](api:hypersig/html/fn.el.html) は [`Tag`](api:hypersig/html/struct.Tag.html) を受け取り、`tag!` は名前が英字で始まり、英字、数字、`-`、`_`、`.`、`:` だけでできていること、そして大文字と小文字を問わず `script`、`style`、`base`、`link` でないことをコンパイル時に検査します。前の 2 つは中のテキストをコードとして実行し、`base` はページの相対 URL をすべて別のオリジンに向け、`link` は `href` が指すホストから読み込むからです。ページに必要な `<link>` は `Page::stylesheet` と `Page::icon` が書きます。[スタイル](learn/styling/) と [ページと head](learn/page/) で説明します。",
            },
        ),
        show(Snippet::Attrs),
        para(
            site,
            L {
                en: "The value is any string, and nothing checks it against the attribute or the attribute against the tag, so `type=\"emial\"` or a `placeholder` on a `div` is written as given: the browser shows an `<input>` with an unknown `type` as a text field, and a `placeholder` on a `div` does nothing. Setting the same attribute twice on one element panics, from `attr` or from a method that sets it, such as `El::role`, because the browser reads the first of two and ignores the second.",
                ja: "値は任意の文字列で、値が属性に合うかどうかも、属性がタグに合うかどうかも検査しません。`type=\"emial\"` や `div` の `placeholder` はそのまま書き出されます。ブラウザは未知の `type` を持つ `<input>` をテキスト欄として表示し、`div` の `placeholder` は何の働きもしません。1 つの要素に同じ属性を 2 回設定するとパニックします。`attr` でも、`El::role` のようにその属性を設定するメソッドでも同じです。ブラウザは 2 つのうち最初の 1 つだけを読み、2 つ目を無視するからです。",
            },
        ),
    ])
}

fn refused(site: Site) -> Part {
    parts([
        heading(
            site,
            L {
                en: "Names an Attr cannot hold",
                ja: "Attr にできない名前",
            },
        ),
        para(
            site,
            L {
                en: "Neither the constants nor `attr!` give an `Attr` for the names below, ignoring case, so `attr`, `bind_attr`, and `attr_when` cannot write them. `attr!` with one of them fails to compile with the rule it breaks:",
                ja: "次の名前には定数でも `attr!` でも `Attr` を作れないので、`attr`、`bind_attr`、`attr_when` では書けません。大文字と小文字は区別しません。`attr!` にこれらの名前を渡すと、破った規則を示してコンパイルエラーになります。",
            },
        ),
        show(Snippet::AttrScript),
        table_of(
            site,
            L {
                en: &["Name", "Written instead by"],
                ja: &["名前", "代わりに書く方法"],
            },
            L {
                en: &[
                    &[
                        "`id` and `class`",
                        "`El::id` and `El::class`, below and in [Values that follow the signals](learn/bindings/)",
                    ],
                    &[
                        "An attribute whose value names another element by its id, such as `for`, `aria-describedby`, `aria-controls`, and `commandfor`",
                        "The methods that take an [`Id`](api:hypersig/html/struct.Id.html), such as `label_for(&EMAIL)` and `aria_describedby(&[&HINT])`, so a reference cannot be spelled apart from its target",
                    ],
                    &[
                        "`role`",
                        "`El::role`, from a [`Role`](api:hypersig/html/enum.Role.html), so a misspelled role stops the build too",
                    ],
                    &[
                        "A name starting with `data-hs-`",
                        "Nothing: hypersig reserves these names for its own methods",
                    ],
                    &[
                        "`href`, `src`, `poster`, and `srcset`",
                        "`El::href`, `El::src`, `El::poster`, and `El::srcset`, from a `Link` or a [`SafeUrl`](api:hypersig/struct.SafeUrl.html), a URL whose scheme cannot run a script",
                    ],
                    &[
                        "A name starting with `on`, such as `onclick`",
                        "The `on_*` methods, such as `on_click`, which attach a statement in its place",
                    ],
                    &[
                        "`srcdoc`, `action`, `formaction`, `xlink:href`, `data`, `http-equiv`, `ping`, `imagesrcset`, `background`, `manifest`, `codebase`, `archive`, `classid`, and `longdesc`",
                        "Nothing: they load a URL or a document and have no method of their own. A form sends its values through an action call, as [Forms](learn/forms/) shows",
                    ],
                    &[
                        "`style`",
                        "`El::css` and `El::bind_style`, which write the element's CSS",
                    ],
                    &[
                        "`attributeName`",
                        "Nothing: with the value `href`, an SVG animation would write into a link",
                    ],
                    &[
                        "A name holding a character other than a letter, a digit, `-`, `_`, `.`, and `:`",
                        "Nothing: such a character would end the name and start another attribute",
                    ],
                ],
                ja: &[
                    &[
                        "`id` と `class`",
                        "`El::id` と `El::class`。下の節と [シグナルに追従する値](learn/bindings/) で説明します",
                    ],
                    &[
                        "`for`、`aria-describedby`、`aria-controls`、`commandfor` のように、値がほかの要素を id で指す属性",
                        "`label_for(&EMAIL)` や `aria_describedby(&[&HINT])` など、[`Id`](api:hypersig/html/struct.Id.html) を受け取るメソッド。指す先の要素と同じ値から設定するので、参照と参照先の綴りが食い違うことはありません",
                    ],
                    &[
                        "`role`",
                        "[`Role`](api:hypersig/html/enum.Role.html) を受け取る `El::role`。ロールの書き間違いもビルドで止まります",
                    ],
                    &[
                        "`data-hs-` で始まる名前",
                        "ありません。hypersig が自分のメソッドのために予約している名前です",
                    ],
                    &[
                        "`href`、`src`、`poster`、`srcset`",
                        "`El::href`、`El::src`、`El::poster`、`El::srcset`。`Link` か、スクリプトを実行できないスキームの URL を持つ [`SafeUrl`](api:hypersig/struct.SafeUrl.html) から設定します",
                    ],
                    &[
                        "`onclick` のように `on` で始まる名前",
                        "`on_click` などの `on_*` のメソッド。属性の代わりに文を付けます",
                    ],
                    &[
                        "`srcdoc`、`action`、`formaction`、`xlink:href`、`data`、`http-equiv`、`ping`、`imagesrcset`、`background`、`manifest`、`codebase`、`archive`、`classid`、`longdesc`",
                        "ありません。どれも URL か文書を読み込む属性で、設定するメソッドはありません。フォームの値は [フォーム](learn/forms/) のとおりアクションの呼び出しで送ります",
                    ],
                    &["`style`", "要素の CSS を書く `El::css` と `El::bind_style`"],
                    &[
                        "`attributeName`",
                        "ありません。値が `href` だと、SVG のアニメーションがリンクに書き込むことになるからです",
                    ],
                    &[
                        "英字、数字、`-`、`_`、`.`、`:` 以外の文字を含む名前",
                        "ありません。そうした文字は名前を終わらせ、別の属性を始めてしまうからです",
                    ],
                ],
            },
        ),
    ])
}

fn ids(site: Site) -> Part {
    parts([
        heading(
            site,
            L {
                en: "Ids and the attributes that point at them",
                ja: "id と、id を指す属性",
            },
        ),
        para(
            site,
            L {
                en: "`El::id` takes an [`Id`](api:hypersig/html/struct.Id.html), which you declare once, such as `const EMAIL: Id = Id::new(\"email\")`. `Id::new` fails to compile in a `const` when the name is empty or holds whitespace. The element and each attribute that points at it read the same value, so the two cannot be spelled apart. `El::role` takes a [`Role`](api:hypersig/html/enum.Role.html), the WAI-ARIA role a screen reader announces the element by.",
                ja: "`El::id` は、`const EMAIL: Id = Id::new(\"email\")` のように 1 度だけ宣言した [`Id`](api:hypersig/html/struct.Id.html) を受け取ります。`const` の中の `Id::new` は、名前が空か空白を含むとコンパイルエラーになります。要素と、その要素を指す各属性が同じ値を読むので、綴りが食い違うことはありません。`El::role` は [`Role`](api:hypersig/html/enum.Role.html) を受け取ります。これはスクリーンリーダーが要素を読み上げるときの WAI-ARIA のロールです。",
            },
        ),
        show(Snippet::Ids),
        table_of(
            site,
            L {
                en: &["Method", "Attribute it writes"],
                ja: &["メソッド", "書く属性"],
            },
            L {
                en: &[
                    &["`label_for(&Id)`", "`for` on a `<label>` or an `<output>`"],
                    &[
                        "`aria_labelledby(&[&Id])`, `aria_describedby`, `aria_controls`, `aria_owns`, `aria_details`, `aria_flowto`, `aria_errormessage`",
                        "The `aria-*` attribute of the same name, a list of ids separated by spaces",
                    ],
                    &["`aria_activedescendant(&Id)`", "`aria-activedescendant`"],
                    &[
                        "`command_for(&Id)`, `popover_target(&Id)`",
                        "`commandfor`, `popovertarget`",
                    ],
                    &[
                        "`datalist(&Id)`, `form_owner(&Id)`",
                        "`list` on an `<input>`, `form` on a control",
                    ],
                    &[
                        "`headers(&[&Id])`, `itemref(&[&Id])`",
                        "`headers` on a table cell, `itemref`",
                    ],
                ],
                ja: &[
                    &["`label_for(&Id)`", "`<label>` か `<output>` の `for`"],
                    &[
                        "`aria_labelledby(&[&Id])`、`aria_describedby`、`aria_controls`、`aria_owns`、`aria_details`、`aria_flowto`、`aria_errormessage`",
                        "同じ名前の `aria-*` 属性。空白で区切った id の並びです",
                    ],
                    &["`aria_activedescendant(&Id)`", "`aria-activedescendant`"],
                    &[
                        "`command_for(&Id)`、`popover_target(&Id)`",
                        "`commandfor`、`popovertarget`",
                    ],
                    &[
                        "`datalist(&Id)`、`form_owner(&Id)`",
                        "`<input>` の `list`、コントロールの `form`",
                    ],
                    &[
                        "`headers(&[&Id])`、`itemref(&[&Id])`",
                        "表のセルの `headers`、`itemref`",
                    ],
                ],
            },
        ),
        para(
            site,
            L {
                en: "`Id::with_key(key)` names one of several elements, such as a row: `ROW.with_key(7)` is `row-7`. `Id::from_text` builds an id from text at run time, such as the anchor of a heading.",
                ja: "`Id::with_key(key)` は、行のように複数ある要素の 1 つに名前を付けます。`ROW.with_key(7)` は `row-7` です。`Id::from_text` は、見出しのアンカーのように実行時にテキストから id を作ります。",
            },
        ),
        para(
            site,
            L {
                en: "Nothing checks that the page holds the element a reference names. The browser ignores a reference to an id no element carries, so the control reads with no label or description. Three mistakes panic in `Page::render`: two elements carrying one id from `Id::new`, with or without the keys of `with_key`; an element inside a list template with an id, which every item would repeat; and a second `El::id` call on one element, which panics where it is called. Two ids alike where either comes from `Id::from_text` render, because the text may come from data visitors stored, and an anchor or a label then reaches the first of them. A page handler that panics answers nothing, so the browser reports an empty response.",
                ja: "参照が指す要素がページにあるかどうかは検査しません。どの要素も持たない id への参照はブラウザに無視されるので、そのコントロールにはラベルも説明もない状態になります。次の 3 つの間違いはパニックします。`Id::new` から作った同じ id（`with_key` のキーの有無は問いません）を 2 つの要素が持つ場合と、リストのテンプレートの中の要素に id を付けた場合（すべての項目が同じ id を繰り返します）は、`Page::render` でパニックします。1 つの要素に `El::id` を 2 回呼んだ場合は、呼んだ場所でパニックします。どちらかが `Id::from_text` から作った id なら、同じ id でも描画できます。テキストは訪問者が保存したデータから来ることがあるからで、そのときアンカーやラベルは最初の要素を指します。ページのハンドラがパニックすると応答が返らないので、ブラウザは空の応答として報告します。",
            },
        ),
    ])
}

fn urls(site: Site) -> Part {
    parts([
        heading(
            site,
            L {
                en: "URLs",
                ja: "URL",
            },
        ),
        para(
            site,
            L {
                en: "`href`, `src`, and `poster` take an `impl IntoHref<S>`: a `Link` or a `Url` from a `Route`, which [Links and redirects](learn/links/) covers, or a [`SafeUrl`](api:hypersig/struct.SafeUrl.html) for an address the router does not serve, which [Values the types do not check](learn/unchecked-values/) covers. A fixed URL is written as a plain attribute, and one that reads signals follows them. `srcset` takes pairs of a `SafeUrl` and a [`Candidate`](api:hypersig/html/enum.Candidate.html), the width or the pixel density the image is drawn for.",
                ja: "`href`、`src`、`poster` は `impl IntoHref<S>` を受け取ります。`Route` から作る `Link` か `Url`（[リンクとリダイレクト](learn/links/) で説明します）か、ルーターが配信しないアドレスに使う [`SafeUrl`](api:hypersig/struct.SafeUrl.html)（[型で検査しない値](learn/unchecked-values/) で説明します）です。固定の URL は普通の属性として書き出され、シグナルを読む URL はシグナルに合わせて変わります。`srcset` は、`SafeUrl` と [`Candidate`](api:hypersig/html/enum.Candidate.html)（その画像を描く幅かピクセル密度）の組を受け取ります。",
            },
        ),
        para(
            site,
            L {
                en: "The `src` of an `<iframe>`, an `<embed>`, or a `<frame>` shows the page at that URL inside this one, so it comes from the program: `src` panics on those tags when the URL reads signals. A `Candidate::Density` that is not a finite number above 0 panics too.",
                ja: "`<iframe>`、`<embed>`、`<frame>` の `src` はその URL のページをこのページの中に表示します。そのため URL はプログラムが決めるもので、これらのタグに渡した URL がシグナルを読むと `src` はパニックします。0 より大きい有限の数でない `Candidate::Density` もパニックします。",
            },
        ),
    ])
}

fn example(site: Site) -> Part {
    parts([
        heading(
            site,
            L {
                en: "A team page",
                ja: "チームのページ",
            },
        ),
        para(
            site,
            L {
                en: "The page below lists the members of a team. Each card is a component, a function that returns an `El<Team>`, and is named by its heading through an id with the member's key. The photo comes from a `SafeUrl`, and the profile link from the route `MEMBER`, so a link with a parameter of the wrong type fails to compile.",
                ja: "次のページはチームのメンバーを一覧にします。カードはそれぞれコンポーネント（`El<Team>` を返す関数）で、メンバーのキーを付けた id を通じて見出しで名前が付きます。写真は `SafeUrl` から、プロフィールへのリンクはルート `MEMBER` から作るので、型の違うパラメータを渡すリンクはコンパイルエラーになります。",
            },
        ),
        show(Snippet::Team),
        para(
            site,
            L {
                en: "Next, [The page and its head](learn/page/) wraps a tree in a document, and [Values that follow the signals](learn/bindings/) makes its text and attributes change in the browser. [Components](learn/components/) covers functions like `card`.",
                ja: "次の [ページと head](learn/page/) では木を文書で包み、[シグナルに追従する値](learn/bindings/) ではテキストと属性をブラウザで変わるようにします。`card` のような関数は [コンポーネント](learn/components/) で説明します。",
            },
        ),
    ])
}
