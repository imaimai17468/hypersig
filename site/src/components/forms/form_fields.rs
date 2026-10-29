//! The page on the form field variants bound to a field of a `Form`: the
//! `in_form` constructors, the `*_in_form` functions, and `submit_button`.

use crate::Site;
use crate::code::{self, Lang};
use crate::demo::{self, Demo};
use crate::doc::{Component, Doc};
use crate::lang::L;
use crate::learn::snippet::{Snippet, show};
use crate::prose::{Part, bullets, heading, lead, para, parts, table_of};

pub fn page(site: Site, head: Part) -> Part {
    parts(vec![
        head,
        lead(site, Doc::Components(Component::FormFields).leads()),
        demo::show(site, Demo::Signup),
        accessibility(site),
        variants(site),
        rendered(site),
        wrong_field(site),
        unchecked(site),
        example(site),
        next(site),
    ])
}

fn accessibility(site: Site) -> Part {
    parts([
        heading(
            site,
            L {
                en: "Accessibility",
                ja: "アクセシビリティ",
            },
        ),
        para(
            site,
            L {
                en: "Each variant is the control of its own page with the form's message for the field added under it. The message sits in an `aria-live=\"polite\"` region, the control points at it with `aria-describedby`, and `aria-invalid` is `true` while it is not empty, so a screen reader reads the message after the control's name and says the value is invalid. The message waits until the visitor leaves the control, so it does not interrupt their typing. The keys are those of each control's page.",
                ja: "どの版もそれぞれのページのコントロールの下に、そのフィールドに対するフォームのメッセージを加えたものです。メッセージは `aria-live=\"polite\"` の領域にあり、コントロールは `aria-describedby` でそれを指し、メッセージが空でない間は `aria-invalid` が `true` になります。そのため、スクリーンリーダーはコントロールの名前の後にメッセージを読み上げ、値が不正であることを伝えます。メッセージは訪問者がコントロールから離れるまで表示されないので、入力の途中で割り込みません。キー操作は各コントロールのページのとおりです。",
            },
        ),
    ])
}

fn variants(site: Site) -> Part {
    parts([
        heading(
            site,
            L {
                en: "The variants",
                ja: "フォーム用の版",
            },
        ),
        para(
            site,
            L {
                en: "Each takes the form's signal, a `Sig<S, Form<T>>`, and the field of `T`, a `Sig<T, U>`, in place of the plain signal:",
                ja: "どの版も単独のシグナルの代わりに、フォームのシグナル `Sig<S, Form<T>>` と、`T` のフィールド `Sig<T, U>` を受け取ります。",
            },
        ),
        table_of(
            site,
            L {
                en: &["Variant", "Field type", "The control"],
                ja: &["版", "フィールドの型", "コントロール"],
            },
            L {
                en: &[
                    &[
                        "[`TextField::in_form`](api:hypersig_ui/struct.TextField.html#method.in_form)",
                        "`String`",
                        "[Text field](components/text-field/)",
                    ],
                    &[
                        "[`NumberField::in_form`](api:hypersig_ui/struct.NumberField.html#method.in_form)",
                        "`i32` or `f64`",
                        "[Number field](components/number-field/)",
                    ],
                    &[
                        "[`DateField::in_form`](api:hypersig_ui/struct.DateField.html#method.in_form)",
                        "`CalendarDate` or `Option<CalendarDate>`",
                        "[Date field](components/date-field/)",
                    ],
                    &[
                        "[`checkbox_in_form`](api:hypersig_ui/fn.checkbox_in_form.html)",
                        "`bool`",
                        "[Checkbox](components/checkbox/)",
                    ],
                    &[
                        "[`switch_in_form`](api:hypersig_ui/fn.switch_in_form.html)",
                        "`bool`",
                        "[Switch](components/switch/)",
                    ],
                    &[
                        "[`radio_group_in_form`](api:hypersig_ui/fn.radio_group_in_form.html)",
                        "any `Serialize + PartialEq`",
                        "[Radio group](components/radio-group/)",
                    ],
                    &[
                        "[`select_field_in_form`](api:hypersig_ui/fn.select_field_in_form.html)",
                        "any `Serialize + PartialEq`",
                        "[Select](components/select/)",
                    ],
                ],
                ja: &[
                    &[
                        "[`TextField::in_form`](api:hypersig_ui/struct.TextField.html#method.in_form)",
                        "`String`",
                        "[テキストフィールド](components/text-field/)",
                    ],
                    &[
                        "[`NumberField::in_form`](api:hypersig_ui/struct.NumberField.html#method.in_form)",
                        "`i32` か `f64`",
                        "[数値フィールド](components/number-field/)",
                    ],
                    &[
                        "[`DateField::in_form`](api:hypersig_ui/struct.DateField.html#method.in_form)",
                        "`CalendarDate` か `Option<CalendarDate>`",
                        "[日付フィールド](components/date-field/)",
                    ],
                    &[
                        "[`checkbox_in_form`](api:hypersig_ui/fn.checkbox_in_form.html)",
                        "`bool`",
                        "[チェックボックス](components/checkbox/)",
                    ],
                    &[
                        "[`switch_in_form`](api:hypersig_ui/fn.switch_in_form.html)",
                        "`bool`",
                        "[スイッチ](components/switch/)",
                    ],
                    &[
                        "[`radio_group_in_form`](api:hypersig_ui/fn.radio_group_in_form.html)",
                        "`Serialize + PartialEq` を満たす任意の型",
                        "[ラジオグループ](components/radio-group/)",
                    ],
                    &[
                        "[`select_field_in_form`](api:hypersig_ui/fn.select_field_in_form.html)",
                        "`Serialize + PartialEq` を満たす任意の型",
                        "[セレクト](components/select/)",
                    ],
                ],
            },
        ),
        code::block(
            Lang::Rust,
            None,
            "impl<S> TextField<S> {\n    \
             pub fn in_form<T: Validate>(label: impl Into<String>, form: Sig<S, Form<T>>, field: Sig<T, String>) -> Self\n}\n\
             impl<S, T: Numeric> NumberField<S, T> {\n    \
             pub fn in_form<V: Validate>(label: impl Into<String>, form: Sig<S, Form<V>>, field: Sig<V, T>) -> Self\n}\n\
             impl<S, T: Dated> DateField<S, T> {\n    \
             pub fn in_form<V: Validate>(label: impl Into<String>, form: Sig<S, Form<V>>, field: Sig<V, T>) -> Self\n}\n\n\
             pub fn checkbox_in_form<S, T: Validate>(text: impl Into<String>, form: Sig<S, Form<T>>, field: Sig<T, bool>) -> El<S>\n\
             pub fn switch_in_form<S, T: Validate>(text: impl Into<String>, form: Sig<S, Form<T>>, field: Sig<T, bool>) -> El<S>\n\
             pub fn radio_group_in_form<S, T: Validate, U: Serialize + PartialEq, L: Into<String>>(\n    \
             title: impl Into<String>, form: Sig<S, Form<T>>, field: Sig<T, U>, options: impl IntoIterator<Item = (U, L)>,\n) -> El<S>\n\
             pub fn select_field_in_form<S, T: Validate, U: Serialize + PartialEq, L: Into<String>>(\n    \
             title: impl Into<String>, form: Sig<S, Form<T>>, field: Sig<T, U>, options: impl IntoIterator<Item = (U, L)>,\n) -> El<S>",
        ),
        para(
            site,
            L {
                en: "Each one binds the control to `form.field(field)`, shows `form.error(field)` under it, and runs `form.touch(field)` when the visitor leaves it. `form.error` shows the first failing rule's message once the field is touched, and submitting the form touches every field with a rule, which is why the message waits. Where no rule fails, a message the server sent with `Sig::reject` shows until the next submit. The three `in_form` constructors return the builder, so `kind`, `min`, `description`, and the other methods still apply.",
                ja: "どの版もコントロールを `form.field(field)` に結び付け、その下に `form.error(field)` を表示し、訪問者が離れると `form.touch(field)` を実行します。`form.error` は、フィールドが操作済みになると最初に失敗したルールのメッセージを表示し、フォームを送信するとルールのあるフィールドがすべて操作済みになります。そのため、メッセージは待ってから表示されます。どのルールも失敗していないときは、サーバーが `Sig::reject` で送ったメッセージが次の送信まで表示されます。3 つの `in_form` コンストラクタはビルダーを返すので、`kind`、`min`、`description` などのメソッドもそのまま使えます。",
            },
        ),
        para(
            site,
            L {
                en: "A form is sent by a [`submit_button`](api:hypersig_ui/fn.submit_button.html) inside it, which writes `type=\"submit\"`, as [Button](components/button/) shows. Any other `button` in the form is `type=\"button\"` and does not send it.",
                ja: "フォームはその中の [`submit_button`](api:hypersig_ui/fn.submit_button.html) で送信します。[ボタン](components/button/)にあるとおり、`submit_button` は `type=\"submit\"` を書き出します。フォームの中のほかの `button` は `type=\"button\"` なので、フォームを送信しません。",
            },
        ),
    ])
}

fn rendered(site: Site) -> Part {
    parts([
        heading(
            site,
            L {
                en: "What they render",
                ja: "描画される HTML",
            },
        ),
        para(
            site,
            L {
                en: "`TextField::in_form(\"Email\", Join::signup(), Signup::email())` renders as follows before the field is touched, without the ids from the signal's path and the `class`:",
                ja: "`TextField::in_form(\"Email\", Join::signup(), Signup::email())` は、フィールドが操作済みになる前は、シグナルのパスから作られる id と `class` を除くと次のように描画されます。",
            },
        ),
        code::block(
            Lang::Html,
            None,
            "<div>\n  <label for=\"…\">Email</label>\n  <input id=\"…\" type=\"text\" aria-describedby=\"…\" aria-invalid=\"false\" value=\"\">\n  <p id=\"…\" aria-live=\"polite\"></p>\n</div>",
        ),
        para(
            site,
            L {
                en: "ARIA allows `aria-invalid` on a radio group but not on a plain group, so `radio_group_in_form` gives the `<fieldset>` `role=\"radiogroup\"`, names it by its legend through `aria-labelledby`, puts `aria-describedby` and `aria-invalid` on it, and the message inside it. The field is marked touched when focus leaves the group, not while the arrow keys move between its radios:",
                ja: "ARIA では `aria-invalid` を radio group には付けられますが、ただの group には付けられません。そのため、`radio_group_in_form` は `<fieldset>` に `role=\"radiogroup\"` を与え、`aria-labelledby` で凡例を名前にし、`aria-describedby` と `aria-invalid` を付け、メッセージを `<fieldset>` の中に置きます。フィールドが操作済みになるのはフォーカスがグループから離れたときで、矢印キーでラジオボタン間を移る間ではありません。",
            },
        ),
        code::block(
            Lang::Html,
            None,
            "<fieldset role=\"radiogroup\" aria-describedby=\"…\" aria-labelledby=\"…\" aria-invalid=\"false\">\n  <legend id=\"…\">Plan</legend>\n  <label>…Hobby</label>\n  <label>…Pro</label>\n  <p id=\"…\" aria-live=\"polite\"></p>\n</fieldset>",
        ),
    ])
}

fn wrong_field(site: Site) -> Part {
    parts([
        heading(
            site,
            L {
                en: "A field of another form",
                ja: "別のフォームのフィールド",
            },
        ),
        para(
            site,
            L {
                en: "The field's first type parameter is the form's value type `T`, so a field of another struct fails to compile with E0308, even where the two structs have a field of the same name:",
                ja: "フィールドの 1 つ目の型パラメーターはフォームの値の型 `T` なので、別の構造体のフィールドは、同じ名前のフィールドがあっても E0308 でコンパイルできません。",
            },
        ),
        show(Snippet::UiInFormOtherStruct),
        para(
            site,
            L {
                en: "The form's value type has to implement [`Validate`](api:hypersig/trait.Validate.html), and the field's type has to match the control: a `bool` field in `TextField::in_form` fails with E0308, and a `String` field in `NumberField::in_form` with E0277, as on the control's own page.",
                ja: "フォームの値の型は [`Validate`](api:hypersig/trait.Validate.html) を実装していなければならず、フィールドの型はコントロールに合っていなければなりません。各コントロールのページと同じく、`TextField::in_form` に `bool` のフィールドを渡すと E0308、`NumberField::in_form` に `String` のフィールドを渡すと E0277 になります。",
            },
        ),
    ])
}

fn unchecked(site: Site) -> Part {
    parts([
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
                    en: "A field that no rule of `Validate::rules` checks never has a message, and its control is never invalid.",
                    ja: "`Validate::rules` のどのルールも検査しないフィールドにはメッセージが出ず、そのコントロールが不正な状態になることもありません。",
                },
                L {
                    en: "The `<form>` from `Sig::form` has `novalidate`, so the browser does not check `required`, `min_length`, `min`, `max`, or `type=\"email\"` before the submit. They still set the on-screen keyboard and what a screen reader announces; write each check as a rule too.",
                    ja: "`Sig::form` の `<form>` は `novalidate` を持つので、ブラウザは送信前に `required`、`min_length`、`min`、`max`、`type=\"email\"` を検査しません。これらは画面のキーボードやスクリーンリーダーの読み上げには効くので、検査はそれぞれルールとしても書きます。",
                },
                L {
                    en: "The rules run in the browser, so the server checks the value it receives again, as [Validating a form](learn/form-validation/) shows.",
                    ja: "ルールはブラウザで実行されるので、[フォームの検証](learn/form-validation/)にあるとおり、サーバーは受け取った値をもう一度検査します。",
                },
                L {
                    en: "Two controls bound to one field of one form get the same ids, and the second one's message points at the first.",
                    ja: "1 つのフォームの同じフィールドに 2 つのコントロールを結び付けると同じ id になり、2 つ目のメッセージは 1 つ目を指します。",
                },
            ],
        ),
    ])
}

fn example(site: Site) -> Part {
    parts([
        heading(
            site,
            L {
                en: "Example: a sign-up form",
                ja: "例: 登録フォーム",
            },
        ),
        para(
            site,
            L {
                en: "Every variant on one form, sent to an action by its submit button:",
                ja: "すべての版を 1 つのフォームに置き、送信ボタンでアクションに送ります。",
            },
        ),
        show(Snippet::UiFormFields),
    ])
}

fn next(site: Site) -> Part {
    parts([
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
                en: "[Forms](learn/forms/) covers `Form<T>`, `form.field`, and `form.touch`, and [Validating a form](learn/form-validation/) writes the rules and checks the value again on the server.",
                ja: "`Form<T>`、`form.field`、`form.touch` は[フォーム](learn/forms/)で説明しています。ルールの書き方とサーバーでの再検査は[フォームの検証](learn/form-validation/)で説明しています。",
            },
        ),
    ])
}
