//! The page on forms: rules written once and checked on both sides, and
//! what the form tracks about the visitor.

use super::snippet::{Snippet, show};
use crate::Site;
use crate::demo::{self, Demo};
use crate::doc::{Doc, Learn};
use crate::lang::L;
use crate::prose::{Part, heading, lead, para, parts, title};

pub fn forms(site: Site) -> Part {
    parts([
        title(Doc::Learn(Learn::Forms).title(site)),
        lead(site, Doc::Learn(Learn::Forms).leads()),
        heading(
            site,
            L {
                en: "Rules",
                ja: "ルール",
            },
        ),
        para(
            site,
            L {
                en: "A field of type [`Form<T>`](api:hypersig/struct.Form.html) holds a form's values, the values it started from, which fields the visitor has left, and the messages the server sent. `T` is a signal struct of the values that implements [`Validate`](api:hypersig/trait.Validate.html), whose `rules` adds a check per field: an expression that must be true, and the message shown while it is false.",
                ja: "型が [`Form<T>`](api:hypersig/struct.Form.html) のフィールドは、フォームの値、開始時の値、訪問者が離れたフィールド、サーバーが送ったメッセージを持ちます。`T` は値を持つシグナルの構造体で、[`Validate`](api:hypersig/trait.Validate.html) を実装します。`Validate` の `rules` は、フィールドごとに検査を追加します。検査は、真であるべき式と、偽のあいだ表示するメッセージの組です。",
            },
        ),
        para(
            site,
            L {
                en: "In the demo below, a message appears once you leave a field, and pressing the button shows every message at once. The button runs its statement only when every rule passes, and Clear appears as soon as a value differs from the start.",
                ja: "下のデモでは、フィールドを離れるとメッセージが出て、ボタンを押すとすべてのメッセージが一度に出ます。ボタンの文は、すべてのルールが通るときだけ実行されます。値が開始時と変わると、クリアのボタンが出ます。",
            },
        ),
        demo::show(site, Demo::Signup),
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
                en: "`error(field)` shows a rule's message once the visitor has left the field, which `touch` records, or has submitted the form. `TextField::in_form` binds a text field to the form's value and attaches both `touch` and `error`.",
                ja: "`error(field)` は、訪問者がそのフィールドを離れたか（`touch` が記録します）、フォームを送信したあとに、ルールのメッセージを返します。`TextField::in_form` はテキストフィールドをフォームの値に束縛し、この 2 つを付けます。",
            },
        ),
        para(
            site,
            L {
                en: "`form(then)` returns a `<form novalidate>` that touches every field with a rule on submit and runs `then` only when every rule passes. The browser's own checks are off because they would stop the submit before the rules run. `is_dirty` compares the values with the values the form started from, and `reset` puts those back.",
                ja: "`form(then)` は `<form novalidate>` を返します。送信すると、ルールのあるフィールドをすべて触れたことにし、すべてのルールが通るときだけ `then` を実行します。ブラウザ自身の検査を止めているのは、それがルールより先に送信を止めてしまうからです。`is_dirty` は値を開始時の値と比べ、`reset` は開始時の値に戻します。",
            },
        ),
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
                en: "`field`, `touch`, `error`, and `Rules::check` take a field of any type, so the plan in the demo is an enum and the terms are a `bool`. `hypersig_ui` has a form variant of each of its form controls: [`select_field_in_form`](api:hypersig_ui/fn.select_field_in_form.html), [`radio_group_in_form`](api:hypersig_ui/fn.radio_group_in_form.html), [`checkbox_in_form`](api:hypersig_ui/fn.checkbox_in_form.html), and [`switch_in_form`](api:hypersig_ui/fn.switch_in_form.html). Like `TextField::in_form`, each binds the control to the field, touches it when the visitor leaves, and shows the message under the control, tied to it by `aria-describedby` and `aria-invalid`. A checkbox or a switch takes only a `bool` field, which the compiler checks.",
                ja: "`field`、`touch`、`error`、`Rules::check` はどの型のフィールドも受け取るので、デモのプランは列挙型、規約への同意は `bool` です。`hypersig_ui` のフォーム用のコントロールには、それぞれフォーム版があります。[`select_field_in_form`](api:hypersig_ui/fn.select_field_in_form.html)、[`radio_group_in_form`](api:hypersig_ui/fn.radio_group_in_form.html)、[`checkbox_in_form`](api:hypersig_ui/fn.checkbox_in_form.html)、[`switch_in_form`](api:hypersig_ui/fn.switch_in_form.html) です。どれも `TextField::in_form` と同じく、コントロールをフィールドに束縛し、離れたときに touch し、メッセージをコントロールの下に出して `aria-describedby` と `aria-invalid` で結び付けます。チェックボックスとスイッチは `bool` のフィールドしか受け取らず、コンパイラがそれを検査します。",
            },
        ),
        para(
            site,
            L {
                en: "A component you write yourself takes `form.field(f)` like any signal. [`Event::Blur`](api:hypersig/html/enum.Event.html#variant.Blur) runs when focus leaves the element and everything inside it, so `on_blur(form.touch(f))` on the element the component returns runs when focus leaves the component, and not while it moves between the controls inside. Put the message on an element of its own with `bind_text(form.error(f))`, give that element an [`Id`](api:hypersig/html/struct.Id.html), and pass the same `Id` to the control's `aria_describedby` so that a screen reader reads the message as the control's description.",
                ja: "自作のコンポーネントには、ほかのシグナルと同じように `form.field(f)` を渡します。[`Event::Blur`](api:hypersig/html/enum.Event.html#variant.Blur) は、フォーカスが要素とその中のすべてから外れたときに実行されます。そのため、コンポーネントが返す要素に `on_blur(form.touch(f))` を付けると、フォーカスがコンポーネントの外へ出たときに実行されます。中のコントロールどうしでフォーカスが移っても実行されません。メッセージは `bind_text(form.error(f))` を付けた別の要素に出し、その要素に [`Id`](api:hypersig/html/struct.Id.html) を付けて、同じ `Id` をコントロールの `aria_describedby` に渡してください。そうするとスクリーンリーダーが、メッセージをコントロールの説明として読みます。",
            },
        ),
        heading(
            site,
            L {
                en: "Checking on the server",
                ja: "サーバーでの検査",
            },
        ),
        para(
            site,
            L {
                en: "`rules` is generic over the page `S`, so the same code builds the expressions twice. On the page, the browser evaluates them as the visitor types. In the handler, `Form::validate` evaluates them with the evaluator that writes the first HTML, so a value the browser accepted passes on the server too. What only the server can check, such as an address that is already taken, goes back as `FieldErrors` through `reject`, and its message stays until the next submit.",
                ja: "`rules` はページの型 `S` について総称的なので、同じコードから式を 2 回作ります。ページでは、訪問者の入力に合わせてブラウザが式を評価します。ハンドラでは、`Form::validate` が最初の HTML を書く評価器で同じ式を評価するので、ブラウザが受け付けた値はサーバーでも通ります。登録済みのメールアドレスのようにサーバーにしか検査できないことは、`FieldErrors` にして `reject` で返します。そのメッセージは次の送信まで残ります。",
            },
        ),
        show(Snippet::Forms),
        para(
            site,
            L {
                en: "A patch of one field leaves the starting values alone, so a handler that has saved the values patches the whole form with `Form::new(saved)`. This site has no server, so the demo above sets a signal where this code calls `JOIN`.",
                ja: "1 つのフィールドへの差分は開始時の値を変えないので、値を保存したハンドラは `Form::new(saved)` でフォーム全体に差分を送ります。このサイトにはサーバーがないので、上のデモはこのコードが `JOIN` を呼ぶところでシグナルを書き換えています。",
            },
        ),
    ])
}
