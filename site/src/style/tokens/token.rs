//! The page on tokens, typed values written as CSS custom properties.

use crate::Site;
use crate::code::{self, Lang};
use crate::demo::{self, Demo};
use crate::doc::{Doc, StylePage};
use crate::lang::L;
use crate::learn::snippet::{Snippet, show};
use crate::prose::{Part, bullets, heading, lead, para, parts};

pub fn page(site: Site, head: Part) -> Part {
    parts(vec![
        head,
        lead(site, Doc::Style(StylePage::Tokens).leads()),
        heading(
            site,
            L {
                en: "A named value",
                ja: "名前の付いた値",
            },
        ),
        para(
            site,
            L {
                en: "[`Token<T>`](api:hypersig/style/struct.Token.html) is a value of type `T` with a name, written as the custom property `--<name>`. [`Token::new`](api:hypersig/style/struct.Token.html#method.new), `fn new(name: &'static str, default: T) -> Token<T>`, creates one, and [`Token::var`](api:hypersig/style/struct.Token.html#method.var), `fn var(&self) -> T`, reads it as `var(--<name>, <default>)`. The result of `var` is a `T`, so it can be passed to any property method that takes a `T`:",
                ja: "[`Token<T>`](api:hypersig/style/struct.Token.html) は名前の付いた `T` 型の値で、カスタムプロパティ `--<name>` として書かれます。[`Token::new`](api:hypersig/style/struct.Token.html#method.new)（`fn new(name: &'static str, default: T) -> Token<T>`）でトークンを作り、[`Token::var`](api:hypersig/style/struct.Token.html#method.var)（`fn var(&self) -> T`）で `var(--<name>, <default>)` として読みます。`var` の結果は `T` なので、`T` を受け取るプロパティのメソッドならどれにでも渡せます。",
            },
        ),
        show(Snippet::Tokens),
        para(
            site,
            L {
                en: "Writing `.buy` for the button, the browser applies the CSS below. Nothing sets `--brand` or `--space` yet, so `var()` falls back to the defaults:",
                ja: "ボタンを `.buy` と書くと、ブラウザが適用するのは次の CSS です。`--brand` も `--space` もまだ何も設定していないので、`var()` は既定値を使います。",
            },
        ),
        code::block(
            Lang::Html,
            None,
            ".buy { background-color: var(--brand, #2563eb); padding: var(--space, 0.5rem) }\n\
             .buy:hover { border-color: var(--brand, #2563eb) }",
        ),
        para(
            site,
            L {
                en: "The value can be set in three places. A [theme](style/themes/) sets it for the whole page, `Style::token` for one element and those inside it, and `El::bind_token` from a signal.",
                ja: "値を設定する場所は 3 つあります。[テーマ](style/themes/)はページ全体に、`Style::token` は 1 つの要素とその内側の要素に、`El::bind_token` はシグナルから値を設定します。",
            },
        ),
        heading(
            site,
            L {
                en: "The types a token can hold",
                ja: "トークンが持てる型",
            },
        ),
        para(
            site,
            L {
                en: "`T` implements [`TokenValue`](api:hypersig/style/trait.TokenValue.html), which is implemented for the value types whose value can be a `var()`: `Color`, `Length`, `Percentage`, `Time`, and `Angle`. A token of a color read where a length is expected is a compile error:",
                ja: "`T` は [`TokenValue`](api:hypersig/style/trait.TokenValue.html) を実装する型です。`TokenValue` は、値を `var()` にできる値の型、つまり `Color`、`Length`、`Percentage`、`Time`、`Angle` に実装されています。色のトークンを長さを受け取る場所で読むとコンパイルエラーになります。",
            },
        ),
        show(Snippet::TokenType),
        para(
            site,
            L {
                en: "A token of any other type, such as a keyword of `font-family`, is a compile error too:",
                ja: "`font-family` のキーワードのように、ほかの型のトークンもコンパイルエラーになります。",
            },
        ),
        show(Snippet::TokenValue),
        heading(
            site,
            L {
                en: "The name",
                ja: "名前",
            },
        ),
        para(
            site,
            L {
                en: "The name is written into the stylesheet as it is, so `Token::new` panics on an empty name or one with a character other than an ASCII letter, a digit, `-`, or `_`, which a custom property name would need escaped. A token is usually built in a function that returns it, so a test that calls the function finds the panic:",
                ja: "名前はそのままスタイルシートに書かれます。そのため、名前が空のときや、ASCII の英字、数字、`-`、`_` 以外の文字を含むときは `Token::new` がパニックします。そうした文字はカスタムプロパティの名前ではエスケープが必要になるからです。トークンはたいていそれを返す関数の中で作るので、その関数を呼ぶテストでパニックを見つけられます。",
            },
        ),
        show(Snippet::TokenName),
        heading(
            site,
            L {
                en: "A value for one element",
                ja: "1 つの要素に付ける値",
            },
        ),
        para(
            site,
            L {
                en: "[`Style::token`](api:hypersig/style/struct.Style.html#method.token), `fn token<T: TokenValue>(self, token: &Token<T>, value: impl Into<T>) -> Style`, sets the custom property on one element. Custom properties are inherited, so the elements inside it read the new value, in place of the default and the page's theme:",
                ja: "[`Style::token`](api:hypersig/style/struct.Style.html#method.token)（`fn token<T: TokenValue>(self, token: &Token<T>, value: impl Into<T>) -> Style`）は、1 つの要素にカスタムプロパティを設定します。カスタムプロパティは継承されるので、その内側の要素は既定値やページのテーマの値の代わりに新しい値を読みます。",
            },
        ),
        show(Snippet::TokenScope),
        code::block(
            Lang::Html,
            None,
            ".danger-zone { --brand: #dc2626 }\n\
             .action { background-color: var(--brand, #2563eb); padding: 0.5rem }",
        ),
        para(
            site,
            L {
                en: "Both buttons have the same style. The one inside `.danger-zone` is red, and the other is blue.",
                ja: "2 つのボタンのスタイルは同じです。`.danger-zone` の中のボタンは赤になり、もう 1 つは青になります。",
            },
        ),
        heading(
            site,
            L {
                en: "A value from a signal",
                ja: "シグナルから決める値",
            },
        ),
        para(
            site,
            L {
                en: "[`El::bind_token`](api:hypersig/html/struct.El.html#method.bind_token), `fn bind_token<T: TokenValue>(self, token: &Token<T>, value: impl IntoExpr<S, String>) -> Self`, keeps the custom property on the element equal to a string expression over the signals. The value is written in the element's `style` attribute and rewritten in the browser each time the expression changes:",
                ja: "[`El::bind_token`](api:hypersig/html/struct.El.html#method.bind_token)（`fn bind_token<T: TokenValue>(self, token: &Token<T>, value: impl IntoExpr<S, String>) -> Self`）は、要素のカスタムプロパティを、シグナルから作る文字列の式の値と常に同じにします。値は要素の `style` 属性に書かれ、式の値が変わるたびにブラウザで書き直されます。",
            },
        ),
        show(Snippet::BindToken),
        para(
            site,
            L {
                en: "The swatches and the button below read the same token. `bind_token` sets that token's value from the radio group's signal:",
                ja: "下の色見本とボタンは同じトークンを読みます。このトークンの値は、`bind_token` がラジオグループのシグナルから設定します。",
            },
        ),
        demo::show(site, Demo::Accent),
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
                    en: "Two tokens with the same name are one custom property to the browser, even with different types. Nothing checks that each name is used once.",
                    ja: "同じ名前の 2 つのトークンは、型が違っていてもブラウザにとっては 1 つのカスタムプロパティです。名前が一度しか使われていないかどうかは検査しません。",
                },
                L {
                    en: "The string of `bind_token` is not checked, as with `El::bind_style`. A value the property cannot use, such as `banana` for a color, does not fall back to the token's default: the property is then unset, and an inherited property such as `color` takes its parent's value.",
                    ja: "`bind_token` の文字列は `El::bind_style` と同じく検査しません。色に `banana` を渡すように、プロパティが使えない値を渡しても、トークンの既定値は使われません。そのプロパティは設定されていない状態になり、`color` のように継承されるプロパティは親の値になります。",
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
                en: "[Themes](style/themes/) sets tokens for the whole page, under media conditions too. The components of `hypersig_ui` read the tokens of its [`token`](api:hypersig_ui/token/index.html) module, which [Theming](components/theming/) covers.",
                ja: "[テーマ](style/themes/)ではメディアの条件の下も含めて、ページ全体のトークンを設定します。`hypersig_ui` のコンポーネントは同じクレートの [`token`](api:hypersig_ui/token/index.html) モジュールのトークンを読みます。このモジュールは Components の[テーマ](components/theming/)のページで説明しています。",
            },
        ),
    ])
}
