//! The page on forms: the `Form<T>` field, binding its controls, the
//! messages, the dirty state, and submitting.

use crate::Site;
use crate::demo::{self, Demo};
use crate::doc::{Doc, Learn};
use crate::lang::L;
use crate::learn::snippet::{Snippet, show};
use crate::prose::{Part, heading, lead, note, para, parts, table_of, title};

/// The article.
pub fn page(site: Site) -> Part {
    parts([
        title(Doc::Learn(Learn::Forms).title(site)),
        lead(site, Doc::Learn(Learn::Forms).leads()),
        the_form(site),
        controls(site),
        messages(site),
        submitting(site),
        other_controls(site),
        own_components(site),
        unchecked(site),
        next(site),
    ])
}

/// `Form<T>`, the demo, and the page that holds one.
fn the_form(site: Site) -> Part {
    parts([
        heading(
            site,
            L {
                en: "A form on the page",
                ja: "ページに置くフォーム",
            },
        ),
        para(
            site,
            L {
                en: "A field of type [`Form<T>`](api:hypersig/struct.Form.html) holds a form's values, the values it started from, which fields the visitor has left, and the messages the server sent. `T` is a signal struct of the values that implements [`Validate`](api:hypersig/trait.Validate.html), whose `rules` adds a check per field: an expression that must be true, and the message shown while it is false. [Validating a form](learn/form-validation/) covers the rules, and this page covers the form around them.",
                ja: "型が [`Form<T>`](api:hypersig/struct.Form.html) のフィールドは、フォームの値、開始時の値、訪問者が離れたフィールド、サーバーが送ったメッセージを持ちます。`T` は値を持つシグナルの構造体で、[`Validate`](api:hypersig/trait.Validate.html) を実装します。`Validate` の `rules` はフィールドごとに検査を追加します。検査は真であるべき式と、偽のあいだ表示するメッセージの組です。ルールは [フォームの検証](learn/form-validation/) で説明し、このページではルールの周りのフォームを説明します。",
            },
        ),
        para(
            site,
            L {
                en: "In the demo below, a message appears once you leave a field, and pressing the button shows every message at once. The button runs its statement only when every rule passes, and Clear appears as soon as a value differs from the start. This site has no server, so the demo sets a signal where an application calls an action.",
                ja: "下のデモでは、フィールドを離れるとメッセージが出て、ボタンを押すとすべてのメッセージが一度に出ます。ボタンの文は、すべてのルールが通るときだけ実行されます。値が開始時と変わると、クリアのボタンが出ます。このサイトにはサーバーがないので、デモはアプリケーションがアクションを呼ぶところでシグナルを書き換えています。",
            },
        ),
        demo::show(site, Demo::Signup),
        para(
            site,
            L {
                en: "`Form::new(value: T) -> Form<T>` starts a form from `value`, untouched and not yet submitted, and needs `T: Clone` because it keeps a copy as the starting values. The page below holds one form of two fields and checks the email address.",
                ja: "`Form::new(value: T) -> Form<T>` は、`value` から始まり、まだ触れられても送信されてもいないフォームを作ります。開始時の値として写しを持つので、`T: Clone` が必要です。下のページは 2 つのフィールドを持つフォームを 1 つ置き、メールアドレスを検査します。",
            },
        ),
        show(Snippet::Forms),
        para(
            site,
            L {
                en: "The form keeps five things: the values, the values it started from, the fields the visitor has left, whether the visitor has submitted it, and the messages from the server. The values and the starting values are `T`, so each needs `Serialize` and `Deserialize` like any signal. The methods in the next sections read and change the other three.",
                ja: "フォームは 5 つのものを持ちます。値、開始時の値、訪問者が離れたフィールド、訪問者が送信したかどうか、サーバーからのメッセージです。値と開始時の値は `T` なので、ほかのシグナルと同じく `Serialize` と `Deserialize` が必要です。残りの 3 つは、次の節からのメソッドで読み書きします。",
            },
        ),
    ])
}

/// The methods that bind a control to a field, and the compile error for a field of another struct.
fn controls(site: Site) -> Part {
    parts([
        heading(
            site,
            L {
                en: "Binding a control",
                ja: "コントロールを束縛する",
            },
        ),
        para(
            site,
            L {
                en: "`Join::signup()` is a `Sig<Join, Form<Signup>>`, and the methods below exist only on a `Sig` whose type is a `Form<T>`. Each takes a field as `Sig<T, U>`, the function the `Signals` derive writes on `T`, such as `Signup::email()`.",
                ja: "`Join::signup()` は `Sig<Join, Form<Signup>>` で、下のメソッドは型が `Form<T>` の `Sig` にだけあります。どれもフィールドを `Sig<T, U>` で受け取ります。これは `Signals` の derive が `T` に書く関数で、`Signup::email()` のようなものです。",
            },
        ),
        table_of(
            site,
            L {
                en: &["Method", "Returns", "What it is"],
                ja: &["メソッド", "戻り値", "内容"],
            },
            L {
                en: &[
                    &[
                        "`field(f)`",
                        "`Sig<S, U>`",
                        "The field `f` of the values, for `bind` or `bind_text`",
                    ],
                    &[
                        "`value()`",
                        "`Sig<S, T>`",
                        "All the values, for the call that sends them",
                    ],
                    &[
                        "`touch(f)`",
                        "`Stmt<S>`",
                        "Marks `f` as left by the visitor. Put it on `on_blur`",
                    ],
                    &[
                        "`is_touched(f)`",
                        "`Expr<S, bool>`",
                        "Whether the visitor has left `f`, or has submitted the form where `f` has a rule",
                    ],
                ],
                ja: &[
                    &[
                        "`field(f)`",
                        "`Sig<S, U>`",
                        "値のフィールド `f`。`bind` や `bind_text` に渡します",
                    ],
                    &[
                        "`value()`",
                        "`Sig<S, T>`",
                        "すべての値。値を送る呼び出しに渡します",
                    ],
                    &[
                        "`touch(f)`",
                        "`Stmt<S>`",
                        "訪問者が `f` を離れたことにします。`on_blur` に付けます",
                    ],
                    &[
                        "`is_touched(f)`",
                        "`Expr<S, bool>`",
                        "訪問者が `f` を離れたか、`f` にルールがあってフォームを送信したか",
                    ],
                ],
            },
        ),
        para(
            site,
            L {
                en: "The input of the page above binds `signup.field(email)` and runs `signup.touch(email)` through [`El::on_blur`](api:hypersig/html/struct.El.html#method.on_blur), so its message waits until the visitor moves to another control.",
                ja: "上のページの入力欄は `signup.field(email)` を束縛し、[`El::on_blur`](api:hypersig/html/struct.El.html#method.on_blur) で `signup.touch(email)` を実行します。そのため、メッセージは訪問者がほかのコントロールへ移るまで出ません。",
            },
        ),
        para(
            site,
            L {
                en: "A field belongs to the struct of the form's values, so a field of another struct does not compile, even one with the same name and type:",
                ja: "フィールドはフォームの値の構造体に属するので、ほかの構造体のフィールドは、名前と型が同じでもコンパイルできません。",
            },
        ),
        show(Snippet::FormWrongField),
    ])
}

/// `error`, `is_dirty`, `is_field_dirty`, `is_submitted`, and `reset`.
fn messages(site: Site) -> Part {
    parts([
        heading(
            site,
            L {
                en: "Messages, changes, and reset",
                ja: "メッセージ、変更の検出、リセット",
            },
        ),
        para(
            site,
            L {
                en: "`error(field)` returns an `Expr<S, String>`: the message of the first failing rule once the visitor has left the field, which `touch` records, or has submitted the form. Where no rule fails, it is the message the server sent for the field, and otherwise `\"\"`. Bind it with `bind_text`, as the `<span>` above does.",
                ja: "`error(field)` は `Expr<S, String>` を返します。訪問者がそのフィールドを離れたか（`touch` が記録します）、フォームを送信したあとは、失敗した最初のルールのメッセージです。失敗するルールがなければ、サーバーがそのフィールドに送ったメッセージで、それもなければ `\"\"` です。上の `<span>` のように `bind_text` で束縛します。",
            },
        ),
        table_of(
            site,
            L {
                en: &["Method", "Returns", "What it is"],
                ja: &["メソッド", "戻り値", "内容"],
            },
            L {
                en: &[
                    &[
                        "`is_dirty()`",
                        "`Expr<S, bool>`",
                        "Whether the values differ from the values the form started from, compared as JSON as `Expr::equals` compares",
                    ],
                    &[
                        "`is_field_dirty(f)`",
                        "`Expr<S, bool>`",
                        "Whether `f` differs from its starting value",
                    ],
                    &[
                        "`is_submitted()`",
                        "`Expr<S, bool>`",
                        "Whether the visitor has submitted the form since it started or was reset",
                    ],
                    &[
                        "`reset()`",
                        "`Stmt<S>`",
                        "Puts the starting values back, and forgets the fields the visitor has left, the submit, and the messages from the server",
                    ],
                ],
                ja: &[
                    &[
                        "`is_dirty()`",
                        "`Expr<S, bool>`",
                        "値が開始時の値と違うか。`Expr::equals` と同じく JSON で比べます",
                    ],
                    &[
                        "`is_field_dirty(f)`",
                        "`Expr<S, bool>`",
                        "`f` が開始時の値と違うか",
                    ],
                    &[
                        "`is_submitted()`",
                        "`Expr<S, bool>`",
                        "開始時かリセット後に、訪問者がフォームを送信したか",
                    ],
                    &[
                        "`reset()`",
                        "`Stmt<S>`",
                        "開始時の値に戻し、訪問者が離れたフィールド、送信したこと、サーバーからのメッセージを消します",
                    ],
                ],
            },
        ),
        para(
            site,
            L {
                en: "The Clear button of the page above shows while `is_dirty()` is true and runs `reset()` on click. It carries `type=\"button\"`, because a `<button>` inside a `<form>` without a `type` is a submit button in HTML, and pressing it would submit the form after the reset.",
                ja: "上のページのクリアのボタンは、`is_dirty()` が真のあいだ表示され、クリックで `reset()` を実行します。このボタンには `type=\"button\"` を付けています。HTML では、`<form>` の中にある `type` のない `<button>` は送信ボタンなので、押すとリセットのあとにフォームが送信されるからです。",
            },
        ),
    ])
}

/// `form(then)` and the statement it runs on submit.
fn submitting(site: Site) -> Part {
    parts([
        heading(
            site,
            L {
                en: "Submitting",
                ja: "送信",
            },
        ),
        para(
            site,
            L {
                en: "`form(then: impl IntoStmt<S>) -> El<S>` returns a `<form novalidate>` whose `submit` runs four steps in order. It touches every field with a rule, marks the form submitted, empties the messages from the server, and runs `then` only when every rule passes. `then` is usually the call that sends `value()` to an action, as `JOIN.call_with(signup.value())` does above. The browser submits the form when the visitor presses a submit button, or presses Enter in a text input of a form that has one.",
                ja: "`form(then: impl IntoStmt<S>) -> El<S>` は `<form novalidate>` を返し、その `submit` は 4 つの手順を順に実行します。ルールのあるフィールドをすべて触れたことにし、フォームを送信済みにし、サーバーからのメッセージを空にし、すべてのルールが通るときだけ `then` を実行します。`then` は、上の `JOIN.call_with(signup.value())` のように、`value()` をアクションに送る呼び出しにするのが普通です。ブラウザは、訪問者が送信ボタンを押したときと、送信ボタンのあるフォームのテキストの入力欄で Enter を押したときに、フォームを送信します。",
            },
        ),
        para(
            site,
            L {
                en: "The browser's own checks are off because `required` and `type=\"email\"` would stop the submit event before the rules run, so write those checks as rules. The call sends `T` alone, so the action is declared as `Action<Join, Signup>` and its handler receives a `Signup`.",
                ja: "ブラウザ自身の検査を止めているのは、`required` や `type=\"email\"` がルールより先に送信のイベントを止めてしまうからです。そうした検査はルールとして書いてください。呼び出しが送るのは `T` だけなので、アクションは `Action<Join, Signup>` と宣言し、ハンドラは `Signup` を受け取ります。",
            },
        ),
    ])
}

/// The form variants of `hypersig_ui`'s controls.
fn other_controls(site: Site) -> Part {
    parts([
        heading(
            site,
            L {
                en: "Other controls and your own components",
                ja: "テキスト以外のコントロールと自作のコンポーネント",
            },
        ),
        para(
            site,
            L {
                en: "`field`, `touch`, `error`, and `Rules::check` take a field of any type, so the plan in the demo is an enum and the terms are a `bool`. `hypersig_ui` has a form variant of each of its form controls. Each binds the control to the field, touches it when the visitor leaves, and shows the message under the control, tied to it by `aria-describedby` and `aria-invalid`.",
                ja: "`field`、`touch`、`error`、`Rules::check` はどの型のフィールドも受け取るので、デモのプランは列挙型、規約への同意は `bool` です。`hypersig_ui` のフォーム用のコントロールには、それぞれフォーム版があります。どれもコントロールをフィールドに束縛し、離れたときにそのフィールドを触れたことにし、メッセージをコントロールの下に出して `aria-describedby` と `aria-invalid` で結び付けます。",
            },
        ),
        table_of(
            site,
            L {
                en: &["Control", "Type of the field"],
                ja: &["コントロール", "フィールドの型"],
            },
            L {
                en: &[
                    &[
                        "[`TextField::in_form`](api:hypersig_ui/struct.TextField.html#method.in_form)",
                        "`String`",
                    ],
                    &[
                        "[`NumberField::in_form`](api:hypersig_ui/struct.NumberField.html#method.in_form)",
                        "`i32` or `f64`",
                    ],
                    &[
                        "[`DateField::in_form`](api:hypersig_ui/struct.DateField.html#method.in_form)",
                        "`CalendarDate` or `Option<CalendarDate>`",
                    ],
                    &[
                        "[`select_field_in_form`](api:hypersig_ui/fn.select_field_in_form.html), [`radio_group_in_form`](api:hypersig_ui/fn.radio_group_in_form.html)",
                        "Any `U: Serialize + PartialEq`, one value per option",
                    ],
                    &[
                        "[`checkbox_in_form`](api:hypersig_ui/fn.checkbox_in_form.html), [`switch_in_form`](api:hypersig_ui/fn.switch_in_form.html)",
                        "`bool`",
                    ],
                ],
                ja: &[
                    &[
                        "[`TextField::in_form`](api:hypersig_ui/struct.TextField.html#method.in_form)",
                        "`String`",
                    ],
                    &[
                        "[`NumberField::in_form`](api:hypersig_ui/struct.NumberField.html#method.in_form)",
                        "`i32` か `f64`",
                    ],
                    &[
                        "[`DateField::in_form`](api:hypersig_ui/struct.DateField.html#method.in_form)",
                        "`CalendarDate` か `Option<CalendarDate>`",
                    ],
                    &[
                        "[`select_field_in_form`](api:hypersig_ui/fn.select_field_in_form.html)、[`radio_group_in_form`](api:hypersig_ui/fn.radio_group_in_form.html)",
                        "`U: Serialize + PartialEq` を満たす任意の型。選択肢ごとに 1 つの値",
                    ],
                    &[
                        "[`checkbox_in_form`](api:hypersig_ui/fn.checkbox_in_form.html)、[`switch_in_form`](api:hypersig_ui/fn.switch_in_form.html)",
                        "`bool`",
                    ],
                ],
            },
        ),
        para(
            site,
            L {
                en: "The compiler checks the type in the second column: a checkbox given a `String` field does not compile.",
                ja: "2 列目の型はコンパイラが検査します。チェックボックスに `String` のフィールドを渡すとコンパイルできません。",
            },
        ),
    ])
}

/// A component of its own that touches a field when focus leaves it.
fn own_components(site: Site) -> Part {
    parts([
        para(
            site,
            L {
                en: "A component you write yourself takes `form.field(f)` like any signal. [`Event::Blur`](api:hypersig/html/enum.Event.html#variant.Blur) runs when focus leaves the element and everything inside it, so `on_blur(form.touch(f))` on the element the component returns runs when focus leaves the component, and not while it moves between the controls inside. Put the message on an element of its own with `bind_text(form.error(f))`, give that element an [`Id`](api:hypersig/html/struct.Id.html), and pass the same `Id` to the control's `aria_describedby` so that a screen reader reads the message as the control's description.",
                ja: "自作のコンポーネントには、ほかのシグナルと同じように `form.field(f)` を渡します。[`Event::Blur`](api:hypersig/html/enum.Event.html#variant.Blur) は、フォーカスが要素とその中のすべてから外れたときに実行されます。そのため、コンポーネントが返す要素に `on_blur(form.touch(f))` を付けると、フォーカスがコンポーネントの外へ出たときに実行されます。中のコントロールどうしでフォーカスが移っても実行されません。メッセージは `bind_text(form.error(f))` を付けた別の要素に出し、その要素に [`Id`](api:hypersig/html/struct.Id.html) を付けて、同じ `Id` をコントロールの `aria_describedby` に渡してください。そうするとスクリーンリーダーがメッセージをコントロールの説明として読みます。",
            },
        ),
        show(Snippet::FormComponent),
    ])
}

/// What the types leave to the page's author.
fn unchecked(site: Site) -> Part {
    parts([
        heading(
            site,
            L {
                en: "What the compiler does not check",
                ja: "コンパイラが検査しないこと",
            },
        ),
        para(
            site,
            L {
                en: "A control without `on_blur(form.touch(f))` compiles, and its message appears only after the visitor submits the form. A `<form>` built with `html::form()` in place of `form(then)` compiles too, and its submit runs no rule. In both cases the page shows no error in the console.",
                ja: "`on_blur(form.touch(f))` のないコントロールもコンパイルでき、そのメッセージは訪問者がフォームを送信するまで出ません。`form(then)` の代わりに `html::form()` で作った `<form>` もコンパイルでき、送信してもルールは動きません。どちらの場合も、コンソールにエラーは出ません。",
            },
        ),
        note(
            site,
            L {
                en: "The rules on the page only decide what the visitor sees. A request can come from something other than the page, so the handler checks the same rules with `Form::validate` before it trusts the values, as the next page shows.",
                ja: "ページのルールが決めるのは、訪問者に見えるものだけです。リクエストはページ以外からも届くので、ハンドラは値を使う前に `Form::validate` で同じルールを検査します。その方法は次のページで説明します。",
            },
        ),
    ])
}

/// The links to the pages that come next.
fn next(site: Site) -> Part {
    para(
        site,
        L {
            en: "Next, [Validating a form](learn/form-validation/) writes the rules and checks them again in the handler. [Binding form controls](learn/inputs/) covers `bind` on inputs outside a form.",
            ja: "次の [フォームの検証](learn/form-validation/) ではルールを書き、ハンドラでもう一度検査します。フォームの外にある入力欄の `bind` は [入力欄の束縛](learn/inputs/) で説明します。",
        },
    )
}
