//! The page on one-way bindings: text, attributes, visibility, and classes
//! that follow the signals.

use crate::Site;
use crate::demo::{self, Demo};
use crate::doc::{Doc, Learn};
use crate::lang::L;
use crate::learn::snippet::{Snippet, show};
use crate::prose::{Part, heading, lead, para, parts, table_of, title};

pub fn page(site: Site) -> Part {
    parts(vec![
        title(Doc::Learn(Learn::Bindings).title(site)),
        lead(site, Doc::Learn(Learn::Bindings).leads()),
        overview(site),
        first_html(site),
        text_and_attributes(site),
        visibility(site),
        classes(site),
        example(site),
    ])
}

fn overview(site: Site) -> Part {
    parts([
        para(
            site,
            L {
                en: "An element can show a value that changes in the browser. Each binding takes an expression over the signals, such as `progress.to_text()` or `!busy`, and the runtime writes the expression's new value into the element whenever a signal it reads changes. The binding is rendered with the initial values first, so the HTML shows what the runtime would compute. [Expressions](learn/expressions/) covers what an expression can compute.",
                ja: "ブラウザで変わる値を要素に表示できます。束縛はそれぞれ `progress.to_text()` や `!busy` のようなシグナルを使った式を受け取り、式が読むシグナルが変わるたびに、ランタイムが式の新しい値を要素に書き込みます。束縛は最初に初期値で描画されるので、HTML にはランタイムが計算するのと同じ値が入っています。式で計算できることは [式](learn/expressions/) で説明します。",
            },
        ),
        show(Snippet::Bindings),
        table_of(
            site,
            L {
                en: &["Method", "Takes", "Keeps in step"],
                ja: &["メソッド", "受け取るもの", "合わせ続けるもの"],
            },
            L {
                en: &[
                    &[
                        "`bind_text`",
                        "`impl IntoExpr<S, String>`",
                        "The element's text, which it replaces",
                    ],
                    &[
                        "`bind_attr`",
                        "An `Attr` and `impl IntoExpr<S, String>`",
                        "The attribute's value",
                    ],
                    &[
                        "`attr_when`",
                        "An `Attr` and `impl IntoExpr<S, bool>`",
                        "Whether a boolean attribute such as `disabled` is present",
                    ],
                    &[
                        "`disabled`",
                        "`impl IntoExpr<S, bool>`",
                        "`disabled`, as `attr_when(attr::DISABLED, ...)` does",
                    ],
                    &[
                        "`show`",
                        "`impl IntoExpr<S, bool>`",
                        "Whether the element is shown",
                    ],
                    &[
                        "`html::when`",
                        "`impl IntoExpr<S, bool>` and an `El<S>`",
                        "Whether the element is in the document",
                    ],
                    &[
                        "`class_when`",
                        "A `Class` and `impl IntoExpr<S, bool>`",
                        "Whether the element has the class",
                    ],
                ],
                ja: &[
                    &[
                        "`bind_text`",
                        "`impl IntoExpr<S, String>`",
                        "要素のテキスト。式の値で置き換えます",
                    ],
                    &[
                        "`bind_attr`",
                        "`Attr` と `impl IntoExpr<S, String>`",
                        "属性の値",
                    ],
                    &[
                        "`attr_when`",
                        "`Attr` と `impl IntoExpr<S, bool>`",
                        "`disabled` のような真偽値の属性があるかどうか",
                    ],
                    &[
                        "`disabled`",
                        "`impl IntoExpr<S, bool>`",
                        "`disabled`。`attr_when(attr::DISABLED, ...)` と同じです",
                    ],
                    &[
                        "`show`",
                        "`impl IntoExpr<S, bool>`",
                        "要素を表示するかどうか",
                    ],
                    &[
                        "`html::when`",
                        "`impl IntoExpr<S, bool>` と `El<S>`",
                        "要素が文書にあるかどうか",
                    ],
                    &[
                        "`class_when`",
                        "`Class` と `impl IntoExpr<S, bool>`",
                        "要素がクラスを持つかどうか",
                    ],
                ],
            },
        ),
        para(
            site,
            L {
                en: "`css_when`, `css_match`, and `bind_style` do the same for the element's CSS, which [Styling](learn/styling/) covers. A binding goes one way, from the signals to the element. A text field that writes back into a signal uses `bind`, which [Binding form controls](learn/inputs/) covers.",
                ja: "`css_when`、`css_match`、`bind_style` は要素の CSS について同じことをします。これは [スタイル](learn/styling/) で説明します。束縛はシグナルから要素への一方向です。入力した値をシグナルに書き戻すテキスト欄には `bind` を使います。これは [入力欄の束縛](learn/inputs/) で説明します。",
            },
        ),
    ])
}

fn first_html(site: Site) -> Part {
    parts([
        heading(
            site,
            L {
                en: "What the first HTML holds",
                ja: "最初の HTML に入るもの",
            },
        ),
        para(
            site,
            L {
                en: "`Page::render` evaluates each binding with the initial values and writes the result into the element. The upload above, rendered while nothing uploads:",
                ja: "`Page::render` は初期値で各束縛を評価し、その結果を要素に書き込みます。上のアップロードを、何もアップロードしていない状態で描画します。",
            },
        ),
        show(Snippet::BindingsRender),
        para(
            site,
            L {
                en: "The HTML holds what a visitor sees before the runtime loads: the text `0 %`, the label `Nothing to cancel`, a disabled button, and a hidden paragraph.",
                ja: "HTML には、ランタイムが読み込まれる前に訪問者が見るものが入っています。テキスト `0 %`、ラベル `Nothing to cancel`、無効になったボタン、隠れた段落です。",
            },
        ),
    ])
}

fn text_and_attributes(site: Site) -> Part {
    parts([
        heading(
            site,
            L {
                en: "Text and attributes",
                ja: "テキストと属性",
            },
        ),
        para(
            site,
            L {
                en: "`bind_text` replaces the element's text. `bind_attr` keeps an attribute equal to a string expression, and `attr_when` adds a boolean attribute such as `disabled` while a condition holds. Both take the attribute's name as an `Attr`, so they refuse the same names `attr` refuses, as [Elements and attributes](learn/elements/) lists. A number is turned into text with `to_text` first, because each of them takes the type it writes, and a number where a `String` is expected is a compile error.",
                ja: "`bind_text` は要素のテキストを置き換えます。`bind_attr` は属性を文字列の式の値に合わせ続け、`attr_when` は条件が成り立つ間だけ `disabled` のような真偽値の属性を付けます。どちらも属性の名前を `Attr` で受け取るので、`attr` と同じ名前を拒否します。拒否する名前は [要素と属性](learn/elements/) にあります。数値は先に `to_text` でテキストにします。どのメソッドも書き込む値の型を受け取るので、`String` を受け取る場所に数値を渡すとコンパイルエラーになるからです。",
            },
        ),
        para(
            site,
            L {
                en: "`disabled(when)` is `attr_when(attr::DISABLED, when)`. The browser skips a disabled control in the tab order, ignores clicks on it, and leaves it out of a submitted form, on `<button>`, `<input>`, `<select>`, `<textarea>`, `<fieldset>`, `<option>`, and `<optgroup>`. Nothing checks the tag, and on another element the attribute changes nothing. A second `bind_text` on one element panics, because the text is one expression.",
                ja: "`disabled(when)` は `attr_when(attr::DISABLED, when)` と同じです。ブラウザは無効なコントロールをタブ順から外し、クリックを無視し、送信するフォームから除きます。これが働くのは `<button>`、`<input>`、`<select>`、`<textarea>`、`<fieldset>`、`<option>`、`<optgroup>` です。タグは検査しないので、ほかの要素に付けても何も変わりません。1 つの要素に `bind_text` を 2 回呼ぶとパニックします。テキストは 1 つの式だからです。",
            },
        ),
    ])
}

fn visibility(site: Site) -> Part {
    parts([
        heading(
            site,
            L {
                en: "Showing and removing an element",
                ja: "要素を表示する、取り除く",
            },
        ),
        para(
            site,
            L {
                en: "`show` hides the element while a condition is false. `when(cond, el)` removes it instead, so each time it returns its inputs start empty and its `on_load`, the statement that runs when an element enters the document, runs again. `show` suits an element that keeps what the visitor typed while hidden, and `when` one that should start over.",
                ja: "`show` は条件が偽の間、要素を隠します。`when(cond, el)` は要素を取り除くので、要素が戻るたびに入力欄は空から始まり、`on_load`（要素が文書に入ったときに実行する文）がもう一度実行されます。隠している間も入力を保ちたい要素には `show` を、毎回最初から始めたい要素には `when` を使います。",
            },
        ),
        show(Snippet::When),
        para(
            site,
            L {
                en: "`when` returns a [`Node<S>`](api:hypersig/html/struct.Node.html), which `child` takes like an element, and the first HTML holds the element only where the condition is true when the page renders. Both take a `bool` expression, so a signal of another type is a compile error:",
                ja: "`when` は [`Node<S>`](api:hypersig/html/struct.Node.html) を返し、`child` は要素と同じようにこれを受け取ります。最初の HTML に要素が入るのは、ページを描画する時点で条件が真のときだけです。どちらも `bool` の式を受け取るので、ほかの型のシグナルを渡すとコンパイルエラーになります。",
            },
        ),
        show(Snippet::ShowType),
        para(
            site,
            L {
                en: "A second `show` on one element panics, because the element has one condition: join the two with `and`.",
                ja: "1 つの要素に `show` を 2 回呼ぶとパニックします。要素の条件は 1 つなので、2 つの条件は `and` でつないでください。",
            },
        ),
    ])
}

fn classes(site: Site) -> Part {
    parts([
        heading(
            site,
            L {
                en: "Classes",
                ja: "クラス",
            },
        ),
        para(
            site,
            L {
                en: "`class` adds a static class and `class_when` adds one while a condition holds. Both take a [`Class`](api:hypersig/html/struct.Class.html), declared once as a `const`, so a misspelled class at a use site is an unknown name to the compiler. `Class::new` fails to compile in a `const` when the name is empty or holds whitespace.",
                ja: "`class` は固定のクラスを付け、`class_when` は条件が成り立つ間だけクラスを付けます。どちらも `const` で 1 度だけ宣言した [`Class`](api:hypersig/html/struct.Class.html) を受け取るので、使う場所でクラス名を書き間違えると、コンパイラにとって未知の名前になります。`const` の中の `Class::new` は、名前が空か空白を含むとコンパイルエラーになります。",
            },
        ),
        show(Snippet::Classes),
        para(
            site,
            L {
                en: "A class from `Class::new` is not checked against any stylesheet, which suits a stylesheet the page loads from a URL, and a class that no rule selects changes nothing. A class from `Stylesheet::class` is checked against the CSS at compile time, as [Styling](learn/styling/) shows.",
                ja: "`Class::new` のクラスはどのスタイルシートとも照合しません。URL から読み込むスタイルシートにはこれが合います。どのルールも選択しないクラスは何も変えません。`Stylesheet::class` のクラスはコンパイル時に CSS と照合されます。これは [スタイル](learn/styling/) で説明します。",
            },
        ),
    ])
}

fn example(site: Site) -> Part {
    parts([
        heading(
            site,
            L {
                en: "Try it here",
                ja: "ここで試す",
            },
        ),
        para(
            site,
            L {
                en: "The demo binds a text field to `name` and shows one of two paragraphs depending on whether it is empty.",
                ja: "このデモはテキスト欄を `name` に束縛し、空かどうかで 2 つの段落のどちらかを表示します。",
            },
        ),
        demo::show(site, Demo::Greeting),
        para(
            site,
            L {
                en: "Next, [Components](learn/components/) puts bindings like these in functions that any page can use. [The signal struct](learn/signals/) covers the signals the expressions read.",
                ja: "次の [コンポーネント](learn/components/) では、こうした束縛を、どのページでも使える関数にまとめます。式が読むシグナルは [シグナル構造体](learn/signals/) で説明します。",
            },
        ),
    ])
}
