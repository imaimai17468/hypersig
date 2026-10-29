//! The page on styling elements with typed CSS.

use crate::Site;
use crate::demo::{self, Demo};
use crate::doc::{Doc, Learn};
use crate::lang::L;
use crate::learn::snippet::{Snippet, show};
use crate::prose::{Part, bullets, heading, lead, para, parts, subheading, title};

pub fn page(site: Site) -> Part {
    parts(vec![
        title(Doc::Learn(Learn::Styling).title(site)),
        lead(site, Doc::Learn(Learn::Styling).leads()),
        atomic(site),
        typed(site),
        following(site),
        unchecked(site),
        next(site),
    ])
}

fn atomic(site: Site) -> Part {
    parts([
        heading(
            site,
            L {
                en: "Atomic styles",
                ja: "アトミックなスタイル",
            },
        ),
        para(
            site,
            L {
                en: "[`Style`](api:hypersig/style/struct.Style.html) has one method per standard CSS property, generated from MDN's CSS data. `El::css(self, style: Style) -> Self` adds each declaration as one class named after the declaration, as in the atomic CSS of Panda CSS and Tailwind CSS. The page writes the rules its elements use into `<head>`, and a patch that contains new elements includes their rules.",
                ja: "[`Style`](api:hypersig/style/struct.Style.html) には、標準の CSS プロパティごとに 1 つずつメソッドがあり、MDN の CSS データから生成しています。`El::css(self, style: Style) -> Self` は、Panda CSS や Tailwind CSS のアトミック CSS と同じく、宣言ごとにその宣言にちなんだ名前のクラスを 1 つ付けます。ページは要素が使うルールを `<head>` に書き、新しい要素を含む差分にはその要素のルールも含まれます。",
            },
        ),
        show(Snippet::Styles),
        para(
            site,
            L {
                en: "A later `css` call wins over an earlier one for the same property, and the class names come from the declarations, so two elements with one declaration share one rule.",
                ja: "同じプロパティについては、あとの `css` の呼び出しが前の呼び出しより優先されます。クラス名は宣言から決まるので、同じ宣言を持つ 2 つの要素は 1 つのルールを共有します。",
            },
        ),
        bullets(
            site,
            &[
                L {
                    en: "`on(PseudoClass::Hover, ...)` and `on(Media::Md, ...)` apply a style under a condition. Conditions cover pseudo-classes and pseudo-elements, media features, container sizes, and the state of a marked ancestor or sibling (`group` and `peer`). [Conditions](style/conditions/) lists them.",
                    ja: "`on(PseudoClass::Hover, ...)` や `on(Media::Md, ...)` は条件付きでスタイルを適用します。条件には、疑似クラスと疑似要素、メディア特性、コンテナのサイズ、印を付けた祖先や兄弟の状態（`group` と `peer`）があります。一覧は [条件](style/conditions/) にあります。",
                },
                L {
                    en: "A `Token` is a CSS custom property with a typed value. `Page::theme` sets tokens for the page, for example under `Media::Dark`. [Tokens](style/tokens/) covers them.",
                    ja: "`Token` は型の付いた値を持つ CSS カスタムプロパティです。`Page::theme` はたとえば `Media::Dark` のもとでのトークンをページに設定します。詳しくは [トークン](style/tokens/) で説明します。",
                },
                L {
                    en: "`Global` holds rules for elements by tag with a small reset, `@font-face`, and `@keyframes`, and `Page::global` adds them to the page. `hypersig::style::pattern` has layouts such as `stack` and `grid`.",
                    ja: "`Global` はタグで指定する要素のルールを、小さなリセット、`@font-face`、`@keyframes` と一緒に持ち、`Page::global` がそれをページに加えます。`hypersig::style::pattern` には `stack` や `grid` のようなレイアウトがあります。",
                },
            ],
        ),
    ])
}

fn typed(site: Site) -> Part {
    parts([
        heading(
            site,
            L {
                en: "What the types check",
                ja: "型が検査するもの",
            },
        ),
        para(
            site,
            L {
                en: "Each property method takes the values that property accepts: a keyword of its own enum, or a value of a type in [`style::value`](api:hypersig/style/value/index.html), such as a length or a color. A misspelled keyword is an unknown variant:",
                ja: "プロパティのメソッドはそれぞれ、そのプロパティが受け付ける値を受け取ります。プロパティ専用の列挙型のキーワードか、長さや色のような [`style::value`](api:hypersig/style/value/index.html) の型の値です。キーワードを書き間違えると、存在しないバリアントになります。",
            },
        ),
        show(Snippet::StyleKeyword),
        para(
            site,
            L {
                en: "A value of the wrong type, such as a color for a width, does not convert into the property's type:",
                ja: "幅に色を渡すように型の違う値は、プロパティの型に変換できません。",
            },
        ),
        show(Snippet::WrongValue),
    ])
}

fn following(site: Site) -> Part {
    parts([
        heading(
            site,
            L {
                en: "Styles that follow the signals",
                ja: "シグナルに追従するスタイル",
            },
        ),
        para(
            site,
            L {
                en: "`css_when(style, cond)` adds a style while a signal condition holds, and `css_match(value, cases)` picks one style per value of a signal. While its condition holds, a style of `css_when` overrides the element's own declarations for the same property. Two styles switched on at once that set the same property are not checked against each other, and the one whose rule comes later in the stylesheet wins, whatever the order of the calls. `css_match` applies at most one case at a time, a value with no case adds nothing, and nothing checks that the cases cover every value. Where the value is known on the server, a function that returns a `Style` from a `match` is checked for every value by the compiler.",
                ja: "`css_when(style, cond)` はシグナルの条件が成り立つ間だけスタイルを付け、`css_match(value, cases)` はシグナルの値ごとにスタイルを 1 つ選びます。条件が成り立つ間、`css_when` のスタイルは同じプロパティについて要素自身の宣言より優先されます。同時に有効になった 2 つのスタイルが同じプロパティを設定しても互いに検査されず、スタイルシートであとに来るルールが勝ちます。呼び出しの順序には左右されません。`css_match` が一度に適用するケースは 1 つまでで、どのケースにも当たらない値は何も加えません。ケースがすべての値を網羅しているかどうかは検査しません。値がサーバーで分かる場合は、`match` で `Style` を返す関数にすれば、すべての値をコンパイラが検査します。",
            },
        ),
        para(
            site,
            L {
                en: "The badge below takes its color from `css_match` over an enum, and the green comes from a token with a dark value.",
                ja: "下のバッジは、列挙型に対する `css_match` で色が決まります。緑はダークモード用の値を持つトークンの色です。",
            },
        ),
        demo::show(site, Demo::Status),
        para(
            site,
            L {
                en: "`bind_style(property, value)` keeps one property equal to a `String` expression, for a value that no fixed set holds, such as a width in percent. The string is not checked against the property, and a value the browser rejects leaves the property as the element's classes and stylesheets set it.",
                ja: "`bind_style(property, value)` は、パーセントで表す幅のように決まった候補のない値について、1 つのプロパティを `String` の式の値に合わせ続けます。文字列がプロパティに合うかどうかは検査しません。ブラウザが受け付けない値なら、プロパティは要素のクラスとスタイルシートが設定したままになります。",
            },
        ),
        show(Snippet::BindStyle),
    ])
}

fn unchecked(site: Site) -> Part {
    parts([
        heading(
            site,
            L {
                en: "Values the types do not check",
                ja: "型が検査しない値",
            },
        ),
        para(
            site,
            L {
                en: "A value with several parts, such as `margin: 0 auto`, a shadow, or a `calc()`, goes through `raw(...)`, which is passed to the browser unchecked, and so is a value outside a property's range, such as a negative width, which the browser drops. `raw` panics when the text could end more than its own declaration: where it holds `;`, `{`, `}`, `<`, a backslash, `/*`, or a control character, or leaves a quote or a parenthesis open.",
                ja: "`margin: 0 auto`、影、`calc()` のように複数の部分からなる値は `raw(...)` で書き、検査されないままブラウザに渡ります。負の幅のように、プロパティの範囲を外れた値も検査されず、ブラウザが捨てます。`raw` は、テキストが自分の宣言より先まで終わらせてしまう場合にパニックします。`;`、`{`、`}`、`<`、バックスラッシュ、`/*`、制御文字を含む場合と、引用符か括弧が閉じていない場合です。",
            },
        ),
        subheading(
            site,
            L {
                en: "An existing stylesheet",
                ja: "既存のスタイルシート",
            },
        ),
        para(
            site,
            L {
                en: "A page that loads an existing stylesheet keeps it. `Page::style` writes a [`Stylesheet`](api:hypersig/html/struct.Stylesheet.html) into a `<style>` of `<head>`, and `Stylesheet::class(\"name\")` checks at compile time that the stylesheet has a selector for the class. Its rules sit outside the cascade layers of `El::css`, so they override those classes wherever both set a property. `Page::stylesheet` adds a `<link rel=\"stylesheet\">` for a file from an `Asset`, and its classes come from `Class::new`, which checks nothing against the file.",
                ja: "既存のスタイルシートを読み込むページは、そのまま使い続けられます。`Page::style` は [`Stylesheet`](api:hypersig/html/struct.Stylesheet.html) を `<head>` の `<style>` に書き、`Stylesheet::class(\"name\")` は、そのスタイルシートにクラスのセレクタがあることをコンパイル時に検査します。そのルールは `El::css` のカスケードレイヤーの外にあるので、両方が同じプロパティを設定する場所ではこちらが優先されます。`Page::stylesheet` は `Asset` のファイルを読む `<link rel=\"stylesheet\">` を加えます。そのクラスは `Class::new` で作るので、ファイルとは照合しません。",
            },
        ),
        show(Snippet::Unchecked),
        para(
            site,
            L {
                en: "`Stylesheet::new` fails to compile in a `const` when the CSS holds `</style`, in any case, which would end the `<style>` element.",
                ja: "`const` の中の `Stylesheet::new` は、大文字と小文字を問わず CSS が `</style` を含むとコンパイルエラーになります。`<style>` 要素を終わらせてしまうからです。",
            },
        ),
    ])
}

fn next(site: Site) -> Part {
    para(
        site,
        L {
            en: "The [`style`](https://github.com/imaimai17468/hypersig/tree/main/examples/style) example in the repository puts each feature on one page, and the [`ui`](https://github.com/imaimai17468/hypersig/tree/main/examples/ui) example shows the components of `hypersig_ui`, which are built with these features. The [Style](style/) documentation covers `hypersig::style` in full. Next, [The signal struct](learn/signals/) covers the state the conditions read.",
            ja: "リポジトリの [`style`](https://github.com/imaimai17468/hypersig/tree/main/examples/style) の例はそれぞれの機能を 1 つのページにまとめています。[`ui`](https://github.com/imaimai17468/hypersig/tree/main/examples/ui) の例は、これらの機能を使って作った `hypersig_ui` のコンポーネントを示します。`hypersig::style` のすべては [Style のドキュメント](style/) で説明しています。次の [シグナル構造体](learn/signals/) では、条件が読む状態を説明します。",
        },
    )
}
