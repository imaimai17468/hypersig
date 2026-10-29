//! The page on binding form controls to signals.

use crate::Site;
use crate::doc::{Doc, Learn};
use crate::lang::L;
use crate::learn::snippet::{Snippet, show};
use crate::prose::{Part, bullets, heading, lead, para, parts, table_of, title};

pub fn page(site: Site) -> Part {
    parts(vec![
        title(Doc::Learn(Learn::BindingInputs).title(site)),
        lead(site, Doc::Learn(Learn::BindingInputs).leads()),
        heading(
            site,
            L {
                en: "Binding a control to a signal",
                ja: "入力欄をシグナルに束縛する",
            },
        ),
        para(
            site,
            L {
                en: "[`El::bind`](api:hypersig/html/struct.El.html#method.bind), `fn bind<T: Bindable, R: Writable>(self, signal: Sig<S, T, R>) -> Self`, ties an `<input>`, a `<textarea>`, or a `<select>` to a signal in both directions. What the visitor enters is written to the signal, and a statement or a patch that changes the signal changes the control. The signal can be a field of the page or a field of a list item.",
                ja: "[`El::bind`](api:hypersig/html/struct.El.html#method.bind)（`fn bind<T: Bindable, R: Writable>(self, signal: Sig<S, T, R>) -> Self`）は、`<input>`、`<textarea>`、`<select>` をシグナルと双方向に結び付けます。訪問者が入力した値はシグナルに書き込まれ、文や差分がシグナルを変えると入力欄も変わります。シグナルはページのフィールドでも、リスト項目のフィールドでもかまいません。",
            },
        ),
        show(Snippet::StateInputsBind),
        para(
            site,
            L {
                en: "`render` writes each initial value into its control: the `value` of a text, number, or date field, the text of a `<textarea>`, and `checked` for a checkbox. A `None` writes no `value`, so the field starts empty.",
                ja: "`render` は各初期値を入力欄に書き込みます。テキスト欄、数値欄、日付欄では `value`、`<textarea>` では中のテキスト、チェックボックスでは `checked` です。`None` のときは `value` を書かないので、欄は空で始まります。",
            },
        ),
        heading(
            site,
            L {
                en: "The types a control binds to",
                ja: "束縛できる型",
            },
        ),
        para(
            site,
            L {
                en: "`T` must implement [`Bindable`](api:hypersig/html/trait.Bindable.html), which is sealed. The type decides how the runtime reads the control's text and when it writes the signal:",
                ja: "`T` は [`Bindable`](api:hypersig/html/trait.Bindable.html) を実装している必要があり、このトレイトは封印されています。型によって、ランタイムが入力欄の文字をどう読み、いつシグナルに書き込むかが決まります。",
            },
        ),
        table_of(
            site,
            L {
                en: &["Type", "Control", "Writes the signal on"],
                ja: &["型", "入力欄", "シグナルに書き込むとき"],
            },
            L {
                en: &[
                    &[
                        "`String`",
                        "text field, `<textarea>`, `<select>`",
                        "each `input`",
                    ],
                    &["`bool`", "checkbox, as `checked`", "`change`"],
                    &["`i32`, `i64`", "number or range field", "each edit"],
                    &["`f64`", "number or range field", "each edit"],
                    &["`CalendarDate`", "date field", "each edit"],
                    &[
                        "`Option` of `String`, `i32`, `i64`, `f64`, or `CalendarDate`",
                        "as the type it holds",
                        "as the type it holds",
                    ],
                ],
                ja: &[
                    &[
                        "`String`",
                        "テキスト欄、`<textarea>`、`<select>`",
                        "`input` のたび",
                    ],
                    &["`bool`", "チェックボックスの `checked`", "`change`"],
                    &["`i32`、`i64`", "数値欄、範囲欄", "編集のたび"],
                    &["`f64`", "数値欄、範囲欄", "編集のたび"],
                    &["`CalendarDate`", "日付欄", "編集のたび"],
                    &[
                        "`String`、`i32`、`i64`、`f64`、`CalendarDate` の `Option`",
                        "中の型と同じ",
                        "中の型と同じ",
                    ],
                ],
            },
        ),
        bullets(
            site,
            &[
                L {
                    en: "An `Option` stands for an empty field in both directions: `None` empties the field, and emptying the field sets `None`.",
                    ja: "`Option` では、空の欄が双方向に `None` を表します。`None` は欄を空にし、欄を空にすると `None` になります。",
                },
                L {
                    en: "A number field whose text is not a number, and a date field whose text is not a `YYYY-MM-DD` date, leave the signal as it was. An emptied field bound to a plain number or date leaves it as it was too, because the type holds no empty value.",
                    ja: "数値欄の文字が数でないとき、日付欄の文字が `YYYY-MM-DD` の日付でないときは、シグナルは元の値のままです。`Option` でない数値や日付に束縛した欄を空にしたときも、その型には空の値がないので元の値のままです。",
                },
                L {
                    en: "`bind` does not set the `type` attribute. A checkbox needs `attr(attr::TYPE, \"checkbox\")`, and a number or date field its own `type`, as the sample sets them.",
                    ja: "`bind` は `type` 属性を設定しません。チェックボックスには `attr(attr::TYPE, \"checkbox\")` が、数値欄や日付欄にはそれぞれの `type` が必要です。サンプルではこれらを設定しています。",
                },
            ],
        ),
        para(
            site,
            L {
                en: "Any other type is a compile error that names the types `bind` takes. An enum goes to radio buttons or a `<select>`, through the two methods below. A `u64` and an `Option<bool>` are refused the same way, the second because a checkbox has no empty state to stand for `None`:",
                ja: "それ以外の型は、`bind` が受け取る型を示すコンパイルエラーになります。列挙型は下の 2 つのメソッドでラジオボタンか `<select>` に束縛します。`u64` と `Option<bool>` も同じように拒否されます。後者は、チェックボックスに `None` を表す空の状態がないからです。",
            },
        ),
        show(Snippet::StateInputsBindEnum),
        heading(
            site,
            L {
                en: "Radio buttons",
                ja: "ラジオボタン",
            },
        ),
        para(
            site,
            L {
                en: "[`El::bind_radio`](api:hypersig/html/struct.El.html#method.bind_radio), `fn bind_radio<T: Serialize + PartialEq, R: Writable>(self, signal: Sig<S, T, R>, value: &T) -> Self`, makes an `<input>` a radio button that sets `signal` to `value` when the visitor checks it, and that is checked while `signal` equals `value`. It sets `type=\"radio\"` itself, and `T` can be any serializable type, such as an enum:",
                ja: "[`El::bind_radio`](api:hypersig/html/struct.El.html#method.bind_radio)（`fn bind_radio<T: Serialize + PartialEq, R: Writable>(self, signal: Sig<S, T, R>, value: &T) -> Self`）は、`<input>` を、訪問者がチェックすると `signal` を `value` にし、`signal` が `value` に等しい間はチェックされたラジオボタンにします。`type=\"radio\"` はこのメソッドが設定し、`T` には列挙型のようなシリアライズできる型を使えます。",
            },
        ),
        show(Snippet::StateInputsRadio),
        heading(
            site,
            L {
                en: "A select of values",
                ja: "値を選ぶ select",
            },
        ),
        para(
            site,
            L {
                en: "[`El::bind_select`](api:hypersig/html/struct.El.html#method.bind_select), `fn bind_select<T: Serialize + PartialEq, R: Writable, L: Into<String>>(self, signal: Sig<S, T, R>, options: impl IntoIterator<Item = (T, L)>) -> Self`, fills a `<select>` with one `<option>` per value and label and binds it to `signal`. The option equal to the signal starts selected, and a choice writes its value to the signal on `change`:",
                ja: "[`El::bind_select`](api:hypersig/html/struct.El.html#method.bind_select)（`fn bind_select<T: Serialize + PartialEq, R: Writable, L: Into<String>>(self, signal: Sig<S, T, R>, options: impl IntoIterator<Item = (T, L)>) -> Self`）は、値とラベルの組ごとに `<option>` を 1 つ作って `<select>` を埋め、`signal` に束縛します。シグナルと等しい選択肢が最初に選ばれた状態になり、選び直すと `change` のときにその値がシグナルに書き込まれます。",
            },
        ),
        show(Snippet::StateInputsSelect),
        heading(
            site,
            L {
                en: "What panics when the page is built",
                ja: "ページを組み立てるときの panic",
            },
        ),
        para(
            site,
            L {
                en: "The element type is not part of `El<S>`, so these mistakes compile and panic when the element is built:",
                ja: "要素の種類は `El<S>` の型に含まれないので、次の誤りはコンパイルが通り、要素を組み立てるときに panic します。",
            },
        ),
        bullets(
            site,
            &[
                L {
                    en: "`bind` on an element other than `<input>`, `<textarea>`, and `<select>`, `bind_radio` on one other than `<input>`, and `bind_select` on one other than `<select>`.",
                    ja: "`<input>`、`<textarea>`、`<select>` 以外への `bind`、`<input>` 以外への `bind_radio`、`<select>` 以外への `bind_select`。",
                },
                L {
                    en: "A second binding on one control, by any of the three methods, and `bind_radio` on an `<input>` already given a `type`.",
                    ja: "3 つのメソッドのどれかによる、1 つの入力欄への 2 回目の束縛と、すでに `type` を持つ `<input>` への `bind_radio`。",
                },
                L {
                    en: "A radio or option value that fails to serialize, or that holds an integer outside -2^53 to 2^53.",
                    ja: "シリアライズに失敗する値や、-2^53 から 2^53 の範囲を外れる整数を持つ、ラジオボタンや選択肢の値。",
                },
            ],
        ),
        heading(
            site,
            L {
                en: "Line breaks",
                ja: "改行",
            },
        ),
        para(
            site,
            L {
                en: "The browser shows each line break of a `<textarea>` as `\\n`, and an `<input>` drops line breaks. A signal that holds `\\r\\n` for a textarea, or a line break for an input, keeps it until the visitor edits the field, so its length differs from what the field shows, and an untouched field sends it back as it came. Text saved by a plain HTML form holds `\\r\\n`, because a browser submits a textarea's line breaks so. Replace `\\r\\n` with `\\n` where the text enters the page's signals to keep the two the same.",
                ja: "ブラウザは `<textarea>` の改行をすべて `\\n` として表示し、`<input>` は改行を取り除きます。textarea 用のシグナルが `\\r\\n` を持つ場合や、input 用のシグナルが改行を持つ場合、その文字は訪問者が欄を編集するまで残ります。そのため長さが欄の表示と食い違い、触れていない欄はその値をそのまま送り返します。通常の HTML フォームで保存したテキストは `\\r\\n` を含みます。ブラウザは textarea の改行をその形で送信するからです。両者をそろえるには、テキストをページのシグナルに入れるところで `\\r\\n` を `\\n` に置き換えます。",
            },
        ),
        heading(
            site,
            L {
                en: "While the visitor is typing",
                ja: "訪問者が入力している間",
            },
        ),
        para(
            site,
            L {
                en: "An action's answer can send back text the visitor typed before typing on, as when an autosave sends each pause. While the field has focus, such an answer leaves the field as the visitor has it and puts the field's text back in the signal, so the next call sends what the visitor sees. Any other value shows at once and keeps the caret, so a statement that empties the field after Enter, or picks a suggestion, takes effect while the visitor is still in the field.",
                ja: "自動保存が手を止めるたびに送る場合のように、アクションの応答が、訪問者が入力を続ける前に打った文字を送り返すことがあります。欄にフォーカスがある間にそうした応答が届くと、欄は訪問者の入力のまま残り、欄の文字がシグナルに戻ります。次の呼び出しでは画面に見えている文字が送られます。それ以外の値はすぐに表示され、カーソルの位置も保たれます。Enter のあとに欄を空にする文や候補を選ぶ文は、訪問者が欄にいる間に反映されます。",
            },
        ),
        heading(
            site,
            L {
                en: "Signals a control cannot write",
                ja: "入力欄が書き込めないシグナル",
            },
        ),
        para(
            site,
            L {
                en: "The three methods take `R: Writable`, so a computed signal and a read-only view from `read()` are compile errors, which [Computed signals](learn/computed-signals/) shows. A checkbox that only shows a derived state, such as \"all done\", uses `bind_checked`, a one-way binding that [Values that follow the signals](learn/bindings/) covers.",
                ja: "3 つのメソッドは `R: Writable` を求めるので、計算シグナルや `read()` で作った読み取り専用のビューを渡すとコンパイルエラーになります。[計算シグナル](learn/computed-signals/) で示します。「すべて完了」のように、導き出した状態を表示するだけのチェックボックスには、一方向の束縛である `bind_checked` を使います。[シグナルに追従する値](learn/bindings/) で説明します。",
            },
        ),
        heading(
            site,
            L {
                en: "A coffee order",
                ja: "コーヒーの注文",
            },
        ),
        para(
            site,
            L {
                en: "Each control of the order writes one field, and the total is an expression over two of them:",
                ja: "注文の入力欄はそれぞれ 1 つのフィールドに書き込み、合計はそのうち 2 つを使った式です。",
            },
        ),
        show(Snippet::StateInputsExample),
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
                en: "[Computed signals](learn/computed-signals/) keeps a field equal to an expression, such as that total. [Forms](learn/forms/) sends a form to the server and shows its validation errors.",
                ja: "[計算シグナル](learn/computed-signals/) ではこの合計のような式の値をフィールドに保ち続けます。[フォーム](learn/forms/) ではフォームをサーバーに送り、検証エラーを表示します。",
            },
        ),
    ])
}
