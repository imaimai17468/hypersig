//! The page on `TextField`, with `InputKind`, `InputMode`, `Autocomplete`,
//! and multiline fields.

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
        lead(site, Doc::Components(Component::TextField).leads()),
        demo::show(site, Demo::TextField),
        accessibility(site),
        builder(site),
        kinds(site),
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
        bullets(
            site,
            &[
                L {
                    en: "The `<label>` names the field through `for`, so a screen reader announces the label as focus enters the field, and a click on the label focuses it.",
                    ja: "`<label>` は `for` で入力欄の名前になります。そのため、フォーカスが入るとスクリーンリーダーはラベルを読み上げ、ラベルをクリックすると入力欄にフォーカスが移ります。",
                },
                L {
                    en: "`aria-describedby` points at the description and the error, which a screen reader reads after the label. `aria-invalid` is `true` while the error is not empty, and the field's border turns `destructive`.",
                    ja: "`aria-describedby` は説明とエラーを指し、スクリーンリーダーはラベルの後にそれらを読み上げます。`aria-invalid` はエラーが空でない間 `true` になり、入力欄の境界線は `destructive` の色になります。",
                },
                L {
                    en: "The error sits in an `aria-live=\"polite\"` region, so a screen reader reads a new message when it appears, after what it is reading.",
                    ja: "エラーは `aria-live=\"polite\"` の領域にあるので、新しいメッセージが現れると、スクリーンリーダーは読み上げ中の内容の後にそれを読み上げます。",
                },
                L {
                    en: "A required field shows an asterisk after the label that a screen reader skips, since it reads \"required\" from the field's `required` attribute.",
                    ja: "必須の入力欄はラベルの後にアスタリスクを表示しますが、スクリーンリーダーはこれを読み飛ばします。「必須」は入力欄の `required` 属性から読み上げるからです。",
                },
            ],
        ),
        para(
            site,
            L {
                en: "The field takes the keys of a native `<input>` or `<textarea>`, and Tab moves focus to it.",
                ja: "入力欄はネイティブの `<input>` や `<textarea>` のキー操作をそのまま受け付け、Tab キーでフォーカスが移ります。",
            },
        ),
    ])
}

fn builder(site: Site) -> Part {
    parts([
        heading(
            site,
            L {
                en: "`TextField`",
                ja: "`TextField`",
            },
        ),
        para(
            site,
            L {
                en: "[`TextField`](api:hypersig_ui/struct.TextField.html) is a builder. `new` takes the label and the `String` signal the field is bound to, each method adds one thing, and `render` returns the `<div>` that holds the label, the field, the description, and the error:",
                ja: "[`TextField`](api:hypersig_ui/struct.TextField.html) はビルダーです。`new` はラベルと、入力欄が結び付く `String` のシグナルを受け取り、各メソッドが 1 つずつ設定を加え、`render` がラベル、入力欄、説明、エラーを入れた `<div>` を返します。",
            },
        ),
        code::block(
            Lang::Rust,
            None,
            "impl<S> TextField<S> {\n    \
             pub fn new(label: impl Into<String>, value: Sig<S, String>) -> Self\n    \
             pub fn kind(self, kind: InputKind) -> Self\n    \
             pub fn multiline(self) -> Self\n    \
             pub fn autocomplete(self, autocomplete: Autocomplete) -> Self\n    \
             pub fn input_mode(self, mode: InputMode) -> Self\n    \
             pub fn max_length(self, count: u32) -> Self\n    \
             pub fn min_length(self, count: u32) -> Self\n    \
             pub fn placeholder(self, text: impl Into<String>) -> Self\n    \
             pub fn description(self, text: impl Into<String>) -> Self\n    \
             pub fn error(self, message: impl IntoExpr<S, String>) -> Self\n    \
             pub fn required(self) -> Self\n    \
             pub fn disabled(self, when: impl IntoExpr<S, bool>) -> Self\n    \
             pub fn render(self) -> El<S>\n}",
        ),
        para(
            site,
            L {
                en: "`error` takes an expression, so the message can be computed in the browser from the value, or be a signal that an action call fills with `Call::error`. The field is invalid while the message is not empty. `TextField::in_form`, on [Fields bound to a form](components/form-fields/), takes the message from a form's rules instead.",
                ja: "`error` は式を受け取るので、メッセージはブラウザで値から計算することも、アクションの呼び出しが `Call::error` で埋めるシグナルにすることもできます。メッセージが空でない間、入力欄は不正な状態です。[フォームに結び付けた入力欄](components/form-fields/)の `TextField::in_form` は、代わりにフォームのルールからメッセージを取ります。",
            },
        ),
        para(
            site,
            L {
                en: "An email field with a description and an error, while the error is shown, renders as follows. The ids, which come from the signal's path, and the `class` are left out:",
                ja: "説明とエラーを持つメールアドレスの入力欄は、エラーの表示中は次のように描画されます。シグナルのパスから作られる id と `class` は省いています。",
            },
        ),
        code::block(
            Lang::Html,
            None,
            "<div>\n  \
             <label for=\"…\">Email<span aria-hidden=\"true\"> *</span></label>\n  \
             <input id=\"…\" type=\"email\" autocomplete=\"email\" aria-describedby=\"… …\" required=\"\" aria-invalid=\"true\" value=\"\">\n  \
             <p id=\"…\">We send the receipt here.</p>\n  \
             <p id=\"…\" aria-live=\"polite\">Enter an email address.</p>\n\
             </div>",
        ),
        para(
            site,
            L {
                en: "`multiline` replaces the `<input>` with a `<textarea>` that the visitor can resize vertically, with the same label, description, and error.",
                ja: "`multiline` は `<input>` を、縦方向にサイズを変えられる `<textarea>` に置き換えます。ラベル、説明、エラーは同じです。",
            },
        ),
        para(
            site,
            L {
                en: "The field writes the visitor's text to its signal, so `new` takes a `Sig<S, String>`, and a signal of another type fails to compile with E0308. A number goes in a [number field](components/number-field/), and a date in a [date field](components/date-field/):",
                ja: "入力欄は訪問者の入力した文字をシグナルに書き込むので、`new` は `Sig<S, String>` を受け取り、ほかの型のシグナルは E0308 でコンパイルできません。数値は[数値フィールド](components/number-field/)に、日付は[日付フィールド](components/date-field/)に入れます。",
            },
        ),
        show(Snippet::UiTextFieldNumber),
    ])
}

fn kinds(site: Site) -> Part {
    parts([
        heading(
            site,
            L {
                en: "Kind, keyboard, and autocomplete",
                ja: "種類、キーボード、自動入力",
            },
        ),
        para(
            site,
            L {
                en: "Three enums set three attributes of the `<input>`, each with one variant per value the attribute takes:",
                ja: "3 つの列挙型が `<input>` の 3 つの属性を設定します。どれも、属性が取る値ごとに 1 つのバリアントを持ちます。",
            },
        ),
        table_of(
            site,
            L {
                en: &["Enum", "Attribute", "Variants", "What it changes"],
                ja: &["列挙型", "属性", "バリアント", "変わること"],
            },
            L {
                en: &[
                    &[
                        "[`InputKind`](api:hypersig_ui/enum.InputKind.html)",
                        "`type`",
                        "`Text`, `Email`, `Password`, `Search`, `Tel`, `Url`",
                        "The on-screen keyboard and what the browser checks before submitting, such as an `@` in an email",
                    ],
                    &[
                        "[`InputMode`](api:hypersig_ui/enum.InputMode.html)",
                        "`inputmode`",
                        "`None`, `Text`, `Decimal`, `Numeric`, `Tel`, `Search`, `Email`, `Url`",
                        "Only the on-screen keyboard, such as digits for a one-time code",
                    ],
                    &[
                        "[`Autocomplete`](api:hypersig_ui/enum.Autocomplete.html)",
                        "`autocomplete`",
                        "`Off`, `Name`, `Email`, `NewPassword`, `OneTimeCode`, `PostalCode`, `CcNumber`, and the other autofill names of a sign-up form and a checkout",
                        "What the browser offers to fill in, which WCAG 1.3.5 asks for",
                    ],
                ],
                ja: &[
                    &[
                        "[`InputKind`](api:hypersig_ui/enum.InputKind.html)",
                        "`type`",
                        "`Text`、`Email`、`Password`、`Search`、`Tel`、`Url`",
                        "画面のキーボードと、送信前にブラウザが行う検査（メールアドレスの `@` など）",
                    ],
                    &[
                        "[`InputMode`](api:hypersig_ui/enum.InputMode.html)",
                        "`inputmode`",
                        "`None`、`Text`、`Decimal`、`Numeric`、`Tel`、`Search`、`Email`、`Url`",
                        "画面のキーボードだけ（ワンタイムコードの数字のキーボードなど）",
                    ],
                    &[
                        "[`Autocomplete`](api:hypersig_ui/enum.Autocomplete.html)",
                        "`autocomplete`",
                        "`Off`、`Name`、`Email`、`NewPassword`、`OneTimeCode`、`PostalCode`、`CcNumber` など、登録フォームと購入手続きで使う自動入力の名前",
                        "ブラウザが自動入力を提案する内容。WCAG 1.3.5 が求めるもの",
                    ],
                ],
            },
        ),
        para(
            site,
            L {
                en: "A field without `kind` is `type=\"text\"`, and a field without `input_mode` or `autocomplete` has no such attribute. `Autocomplete::Country` writes `country-name` and `Autocomplete::CountryCode` writes `country`, as the WHATWG names them. All three enums are `#[non_exhaustive]`.",
                ja: "`kind` を指定しない入力欄は `type=\"text\"` で、`input_mode` や `autocomplete` を指定しない入力欄にはその属性がありません。`Autocomplete::Country` は `country-name` を、`Autocomplete::CountryCode` は `country` を書き出します。これは WHATWG の名前のとおりです。3 つの列挙型はどれも `#[non_exhaustive]` です。",
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
                    en: "Two fields bound to one signal on a page get the same ids, so the second field's label and messages point at the first field. Bind each field to its own signal.",
                    ja: "1 つのページで同じシグナルに結び付けた 2 つの入力欄は同じ id になり、2 つ目の入力欄のラベルとメッセージは 1 つ目の入力欄を指します。入力欄ごとに別のシグナルに結び付けます。",
                },
                L {
                    en: "`kind` does nothing on a `multiline` field, since a `<textarea>` has no `type`.",
                    ja: "`<textarea>` には `type` がないので、`multiline` の入力欄では `kind` は何もしません。",
                },
                L {
                    en: "`min_length` lets an empty field pass unless it is `required`, and `max_length` stops typing but not a longer value the signal already holds. The browser checks `minlength` only when a plain `<form>` is submitted, and not in a form from `Sig::form`, which has `novalidate`. Check the value again on the server, since a request need not come from this page.",
                    ja: "`min_length` は、`required` でなければ空の入力欄を通します。`max_length` は入力を止めますが、シグナルがすでに持っている長い値は止めません。ブラウザが `minlength` を検査するのは通常の `<form>` の送信時だけで、`novalidate` を持つ `Sig::form` のフォームでは検査しません。リクエストはこのページから来るとは限らないので、サーバーでも値を検査します。",
                },
                L {
                    en: "A placeholder disappears as the visitor types, so put what the visitor needs to keep in the label or the description.",
                    ja: "プレースホルダーは入力を始めると消えるので、訪問者が見続ける必要のある内容はラベルか説明に書きます。",
                },
                L {
                    en: "`Autocomplete` has no section or address-type tokens, so a form with a shipping and a billing address on one page cannot mark which is which.",
                    ja: "`Autocomplete` にはセクションや住所の種類を表すトークンがないので、配送先と請求先の住所を 1 ページに持つフォームでは、どちらがどちらかを示せません。",
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
                en: "Example: an email, a code, and a bio",
                ja: "例: メールアドレス、コード、自己紹介",
            },
        ),
        show(Snippet::UiTextField),
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
                en: "[Number field](components/number-field/) and [Date field](components/date-field/) share this label, description, and error. [Fields bound to a form](components/form-fields/) checks the value with a form's rules. [Binding form controls](learn/inputs/) covers `El::bind`, which these fields are built on.",
                ja: "[数値フィールド](components/number-field/)と[日付フィールド](components/date-field/)も、同じラベル、説明、エラーを持ちます。フォームのルールで値を検査するには[フォームに結び付けた入力欄](components/form-fields/)を使います。これらの入力欄が使っている `El::bind` は[入力欄の束縛](learn/inputs/)で説明しています。",
            },
        ),
    ])
}
