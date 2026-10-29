//! The page on the signal struct and `Sig`.

use crate::Site;
use crate::doc::{Doc, Learn};
use crate::lang::L;
use crate::learn::snippet::{Snippet, show};
use crate::prose::{Part, bullets, heading, lead, para, parts, title};

pub fn page(site: Site) -> Part {
    parts(vec![
        title(Doc::Learn(Learn::Signals).title(site)),
        lead(site, Doc::Learn(Learn::Signals).leads()),
        heading(
            site,
            L {
                en: "The signal struct",
                ja: "シグナル構造体",
            },
        ),
        para(
            site,
            L {
                en: "Derive [`Signals`](api:hypersig/trait.Signals.html) next to serde's `Serialize` and `Deserialize`. The derive adds one associated function per field, named after the field, that returns a [`Sig`](api:hypersig/struct.Sig.html) for it. `Sig<Counter, i32>` reads: an `i32` inside the signal struct `Counter`.",
                ja: "serde の `Serialize` と `Deserialize` と並べて [`Signals`](api:hypersig/trait.Signals.html) を derive します。この derive はフィールドごとにそのフィールドと同じ名前の関連関数を追加し、その関数はフィールドを指す [`Sig`](api:hypersig/struct.Sig.html) を返します。`Sig<Counter, i32>` は「シグナル構造体 `Counter` の中の `i32`」と読みます。",
            },
        ),
        show(Snippet::SignalPaths),
        para(
            site,
            L {
                en: "The trait is `pub trait Signals: Serialize + DeserializeOwned + Send + 'static`, and the type is `pub struct Sig<S, T, R = Root>`. `path()` returns the field's name. A `Sig` is `Copy`, so one value is used in as many expressions and statements as the page needs.",
                ja: "トレイトは `pub trait Signals: Serialize + DeserializeOwned + Send + 'static`、型は `pub struct Sig<S, T, R = Root>` です。`path()` はフィールド名を返します。`Sig` は `Copy` なので、1 つの値をページに必要なだけの式や文で使えます。",
            },
        ),
        heading(
            site,
            L {
                en: "A misspelled field",
                ja: "フィールド名の打ち間違い",
            },
        ),
        para(
            site,
            L {
                en: "The browser keeps a copy of the struct, and every expression, statement, and patch names its fields through a `Sig`. A misspelled field has no function to call, so it fails to compile:",
                ja: "ブラウザは構造体の写しを持ち、式も文も差分も `Sig` を通してそのフィールドを指します。打ち間違えたフィールド名には呼び出せる関数がないので、次のようにコンパイルが通りません。",
            },
        ),
        show(Snippet::WrongField),
        heading(
            site,
            L {
                en: "Signals of another page",
                ja: "別のページのシグナル",
            },
        ),
        para(
            site,
            L {
                en: "The `S` in `Sig<S, T>` stops a signal of one page from being used on another. An element of a `Counter` page is an `El<Counter>`, and it accepts only expressions over `Counter`:",
                ja: "`Sig<S, T>` の `S` は、あるページのシグナルを別のページで使うことを防ぎます。`Counter` のページの要素は `El<Counter>` で、`Counter` のシグナルを使った式しか受け取りません。",
            },
        ),
        show(Snippet::StateOtherPage),
        heading(
            site,
            L {
                en: "What the derive accepts",
                ja: "derive できる構造体",
            },
        ),
        para(
            site,
            L {
                en: "The derive takes a struct with named fields and no type parameters. An enum, a union, a tuple struct, a unit struct, and a generic struct are compile errors. A `#[serde(...)]` attribute on the struct or on a field is a compile error too, because a rename or a skip would make the value the browser holds differ from the field the `Sig` names:",
                ja: "derive できるのは、名前付きのフィールドを持ち、型パラメータのない構造体です。列挙型、共用体、タプル構造体、ユニット構造体、ジェネリックな構造体はコンパイルエラーになります。構造体やフィールドに付けた `#[serde(...)]` もコンパイルエラーになります。名前の変更や省略があると、ブラウザが持つ値が `Sig` の指すフィールドと一致しなくなるからです。",
            },
        ),
        show(Snippet::StateSerdeAttribute),
        para(
            site,
            L {
                en: "The type of a field keeps its own serde attributes, such as `#[serde(rename_all = \"kebab-case\")]` on an enum. The one attribute the derive reads on a field is `#[signals(computed)]`, which [Computed signals](learn/computed-signals/) covers.",
                ja: "フィールドの型には列挙型の `#[serde(rename_all = \"kebab-case\")]` のように、その型自身の serde 属性を付けられます。derive がフィールドから読む属性は `#[signals(computed)]` の 1 つだけで、[計算シグナル](learn/computed-signals/) で説明します。",
            },
        ),
        heading(
            site,
            L {
                en: "The first render",
                ja: "最初の描画",
            },
        ),
        para(
            site,
            L {
                en: "[`Page::new`](api:hypersig/struct.Page.html#method.new) takes the initial values, and [`Page::render`](api:hypersig/struct.Page.html#method.render) evaluates every binding with them. Text, visibility, classes, attributes, form values, and list items are in the HTML before the runtime loads, and the runtime uses those elements as they are rather than building them again. The browser fetches nothing to show the page, because the initial values are part of the HTML.",
                ja: "[`Page::new`](api:hypersig/struct.Page.html#method.new) が初期値を受け取り、[`Page::render`](api:hypersig/struct.Page.html#method.render) がその値ですべての束縛を評価します。テキスト、表示と非表示、クラス、属性、フォームの値、リストの項目は、ランタイムを読み込む前から HTML に入っています。ランタイムはそれらの要素を作り直さず、そのまま使います。初期値は HTML の一部なので、ブラウザはページを表示するために何も取得しません。",
            },
        ),
        show(Snippet::StateFirstRender),
        heading(
            site,
            L {
                en: "What the types leave unchecked",
                ja: "型では検査しないこと",
            },
        ),
        bullets(
            site,
            &[
                L {
                    en: "An integer outside -2^53 to 2^53 in an `i64` or `u64` field makes `Page::render` panic, because the browser keeps every number as a 64-bit float and would round it.",
                    ja: "`i64` や `u64` のフィールドが -2^53 から 2^53 の範囲を外れる整数を持つと、`Page::render` が panic します。ブラウザはすべての数を 64 ビットの浮動小数点数として持つので、その整数は丸められてしまうからです。",
                },
                L {
                    en: "A field type with a hand-written `Serialize` that returns an error makes `Page::render` panic. One whose `Serialize` and `Deserialize` disagree makes [`Page::value_of`](api:hypersig/struct.Page.html#method.value_of) panic on that field.",
                    ja: "手書きの `Serialize` がエラーを返すフィールドの型があると、`Page::render` が panic します。`Serialize` と `Deserialize` の結果が食い違う型では、そのフィールドに対する [`Page::value_of`](api:hypersig/struct.Page.html#method.value_of) が panic します。",
                },
                L {
                    en: "The server does not receive the signals back. An action gets a signal's value only when its call passes the signal as an argument, which [Actions](learn/actions/) covers.",
                    ja: "シグナルはサーバーへ送り返されません。アクションがシグナルの値を受け取るのは、呼び出しがそのシグナルを引数として渡したときだけです。詳しくは [アクション](learn/actions/) で説明します。",
                },
            ],
        ),
        heading(
            site,
            L {
                en: "Next",
                ja: "次に読むページ",
            },
        ),
        para(
            site,
            L {
                en: "[Signals inside signals](learn/nested-signals/) puts one signal struct inside another. [Expressions](learn/expressions/) computes values from the signals, and [Statements](learn/statements/) changes them.",
                ja: "[入れ子のシグナル](learn/nested-signals/) ではシグナル構造体の中に別のシグナル構造体を置きます。[式](learn/expressions/) ではシグナルから値を計算し、[文](learn/statements/) ではシグナルを変更します。",
            },
        ),
    ])
}
