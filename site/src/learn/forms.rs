//! The page on forms: rules written once and checked on both sides, and
//! what the form tracks about the visitor.

use super::snippet::{Snippet, show};
use crate::Site;
use crate::demo::{self, Demo};
use crate::doc::{Doc, Learn};
use crate::lang::L;
use crate::prose::{Part, heading, lead, note, para, parts, title};

pub fn forms(site: Site) -> Part {
    parts([
        title(Doc::Learn(Learn::Forms).title(site)),
        lead(
            site,
            L {
                en: "A form's rules are written once in Rust. The browser checks them as the visitor types, and the handler checks the same rules before it trusts the values.",
                ja: "フォームのルールは Rust で 1 回だけ書きます。ブラウザは入力に合わせてそのルールを検査し、ハンドラは値を使う前に同じルールで検査します。",
            },
        ),
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
                ja: "型が [`Form<T>`](api:hypersig/struct.Form.html) のフィールドは、フォームの値、開始時の値、訪問者が離れたフィールド、サーバーが送ったメッセージを持ちます。`T` は値を持つシグナルの構造体で、[`Validate`](api:hypersig/trait.Validate.html) を実装します。`Validate` の `rules` は、フィールドごとに検査を足します。検査は、真であるべき式と、偽のあいだ表示するメッセージの組です。",
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
                en: "`error(field)` shows a rule's message once the visitor has left the field, which `touch` records, or has submitted the form. `TextField::in_form` binds a text field to the form's value and wires both.",
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
                en: "`field`, `touch`, `error`, and `Rules::check` take a field of any type, so the plan in the demo is an enum and the terms are a `bool`. A control that takes a `Sig`, from `hypersig_ui` or one you write, takes `form.field(f)`. [`Event::Blur`](api:hypersig/html/enum.Event.html#variant.Blur) is `focusout`, which bubbles, so `on_blur(form.touch(f))` on the element a component returns runs when the control inside it loses focus. The message goes on an element of its own with `bind_text(form.error(f))`.",
                ja: "`field`、`touch`、`error`、`Rules::check` はどの型のフィールドも受け取るので、デモのプランは列挙型、規約への同意は `bool` です。`Sig` を受け取るコントロールには、`hypersig_ui` のものでも自作のものでも、`form.field(f)` を渡せます。[`Event::Blur`](api:hypersig/html/enum.Event.html#variant.Blur) は伝播する `focusout` なので、コンポーネントが返す要素に `on_blur(form.touch(f))` を付けると、中のコントロールからフォーカスが外れたときに実行されます。メッセージは、`bind_text(form.error(f))` を付けた別の要素に出します。",
            },
        ),
        note(
            site,
            L {
                en: "Only `TextField::in_form` sets `aria-invalid` and points `aria-describedby` at the message. With another control, a screen reader announces the message line when it has `aria-live`, but does not read it as the control's description.",
                ja: "`aria-invalid` を付け、`aria-describedby` でメッセージを指すのは `TextField::in_form` だけです。ほかのコントロールでは、メッセージの行に `aria-live` があればスクリーンリーダーが読み上げますが、コントロールの説明としては読まれません。",
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
