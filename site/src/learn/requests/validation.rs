//! The page on a form's rules: `Validate`, `Rules::check`, and checking
//! the same rules in the handler with `Form::validate` and `FieldErrors`.

use crate::Site;
use crate::code::{self, Lang};
use crate::doc::{Doc, Learn};
use crate::lang::L;
use crate::learn::snippet::{Snippet, show};
use crate::prose::{Part, heading, lead, para, parts, subheading, table_of, title};

/// The article.
pub fn page(site: Site) -> Part {
    parts([
        title(Doc::Learn(Learn::FormValidation).title(site)),
        lead(site, Doc::Learn(Learn::FormValidation).leads()),
        the_trait(site),
        checks(site),
        on_the_server(site),
        field_errors(site),
        unchecked(site),
        next(site),
    ])
}

/// `Validate` and its one method.
fn the_trait(site: Site) -> Part {
    parts([
        heading(
            site,
            L {
                en: "The Validate trait",
                ja: "Validate トレイト",
            },
        ),
        para(
            site,
            L {
                en: "The values `T` of a [`Form<T>`](api:hypersig/struct.Form.html) implement [`Validate`](api:hypersig/trait.Validate.html), which a `Signals` struct can implement:",
                ja: "[`Form<T>`](api:hypersig/struct.Form.html) の値 `T` は [`Validate`](api:hypersig/trait.Validate.html) を実装します。`Validate` は `Signals` の構造体が実装できるトレイトです。",
            },
        ),
        code::block(
            Lang::Rust,
            None,
            "pub trait Validate: Signals {\n    fn rules<S>(form: Sig<S, Self>, rules: &mut Rules<S, Self>) {}\n}",
        ),
        para(
            site,
            L {
                en: "`rules` builds expressions from `form`, the values on whatever page `S` the form sits on, and adds them to `rules`. The default adds none, for a form that only needs its dirty state and `reset`: `impl Validate for Note {}`.",
                ja: "`rules` は、フォームが置かれたページ `S` での値 `form` から式を作り、`rules` に追加します。既定の実装は何も追加しません。変更の検出と `reset` だけが要るフォームには `impl Validate for Note {}` と書きます。",
            },
        ),
    ])
}

/// `Rules::check`, the order of the checks, and the compile error for a check that is not a `bool`.
fn checks(site: Site) -> Part {
    parts([
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
                en: "[`Rules::check`](api:hypersig/struct.Rules.html#method.check) adds one check: the field it belongs to, an expression that must be true, and the message shown while it is false.",
                ja: "[`Rules::check`](api:hypersig/struct.Rules.html#method.check) は検査を 1 つ追加します。検査は、属するフィールド、真であるべき式、偽のあいだ表示するメッセージの 3 つです。",
            },
        ),
        code::block(
            Lang::Rust,
            None,
            "pub fn check<U>(\n    &mut self,\n    field: Sig<T, U>,\n    ok: impl IntoExpr<S, bool>,\n    message: impl IntoExpr<S, String>,\n) -> &mut Self",
        ),
        para(
            site,
            L {
                en: "A field with several checks shows the message of the first one that fails, in the order they were added. A check may read every field of `form`, so one field can be compared with another. The message is an expression too, so it can hold a value the visitor typed.",
                ja: "検査が複数あるフィールドには、追加した順で最初に失敗した検査のメッセージが出ます。検査は `form` のどのフィールドも読めるので、あるフィールドをほかのフィールドと比べられます。メッセージも式なので、訪問者が入力した値を含められます。",
            },
        ),
        show(Snippet::FormRules),
        para(
            site,
            L {
                en: "`ok` must be an expression of type `bool` on the same page, so a number where a condition belongs does not compile:",
                ja: "`ok` は同じページの `bool` 型の式でなければならないので、条件を書くところに数を書くとコンパイルできません。",
            },
        ),
        show(Snippet::FormRuleType),
    ])
}

/// `Form::validate` and why the handler calls it.
fn on_the_server(site: Site) -> Part {
    parts([
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
                en: "`rules` is generic over the page `S`, so the same code builds the expressions twice. On the page, the browser evaluates them as the visitor types. In the handler, [`Form::validate`](api:hypersig/struct.Form.html#method.validate) evaluates them with the evaluator that writes the first HTML, so a value the browser accepted passes on the server too.",
                ja: "`rules` はページの型 `S` について総称的なので、同じコードから式を 2 回作ります。ページでは、訪問者の入力に合わせてブラウザが式を評価します。ハンドラでは、[`Form::validate`](api:hypersig/struct.Form.html#method.validate) が最初の HTML を書く評価器で同じ式を評価するので、ブラウザが受け付けた値はサーバーでも通ります。",
            },
        ),
        code::block(
            Lang::Rust,
            None,
            "pub fn validate(value: &T) -> Result<(), FieldErrors<T>>",
        ),
        para(
            site,
            L {
                en: "It returns the message of the first failing rule of each field. Call it in the action handler: anyone can send the action's URL a request without loading the page, so the rules in the browser do not protect the handler.",
                ja: "フィールドごとに、最初に失敗したルールのメッセージを返します。アクションのハンドラで呼んでください。ページを読み込まずにアクションの URL へリクエストを送ることは誰にでもできるので、ブラウザのルールはハンドラを守りません。",
            },
        ),
        show(Snippet::FormHandler),
        para(
            site,
            L {
                en: "A patch of one field leaves the starting values alone, so a handler that has saved the values patches the whole form with `Form::new(saved)`, as the last line does. The saved values become the new starting values, and the form forgets the fields the visitor has left, the submit, and the messages from the server.",
                ja: "1 つのフィールドへの差分は開始時の値を変えないので、値を保存したハンドラは最後の行のように `Form::new(saved)` でフォーム全体に差分を送ります。保存した値が新しい開始時の値になり、訪問者が離れたフィールド、送信したこと、サーバーからのメッセージは消えます。",
            },
        ),
    ])
}

/// `FieldErrors` and `reject`.
fn field_errors(site: Site) -> Part {
    parts([
        subheading(
            site,
            L {
                en: "Messages from the server",
                ja: "サーバーからのメッセージ",
            },
        ),
        para(
            site,
            L {
                en: "[`FieldErrors<T>`](api:hypersig/struct.FieldErrors.html) holds a message per field of `T`. `Form::validate` returns one, and a handler builds one for what only the server can check, such as an address that is already taken.",
                ja: "[`FieldErrors<T>`](api:hypersig/struct.FieldErrors.html) は `T` のフィールドごとにメッセージを持ちます。`Form::validate` が返すほか、登録済みのメールアドレスのようにサーバーにしか検査できないことのために、ハンドラが作ることもできます。",
            },
        ),
        table_of(
            site,
            L {
                en: &["Item", "What it does"],
                ja: &["項目", "内容"],
            },
            L {
                en: &[
                    &["`FieldErrors::new()`", "No messages"],
                    &[
                        "`add(field, message)`",
                        "These messages, with `message` for `field` in place of any it had",
                    ],
                    &[
                        "`message(field)`",
                        "The message for `field`, as `Option<&str>`",
                    ],
                    &["`is_empty()`", "Whether no field has a message"],
                    &[
                        "`Display`",
                        "`email: That address is taken.`, with `; ` between fields, for a log",
                    ],
                ],
                ja: &[
                    &["`FieldErrors::new()`", "メッセージなし"],
                    &[
                        "`add(field, message)`",
                        "`field` のメッセージを `message` に置き換えたもの",
                    ],
                    &[
                        "`message(field)`",
                        "`field` のメッセージ。`Option<&str>` です",
                    ],
                    &["`is_empty()`", "どのフィールドにもメッセージがないか"],
                    &[
                        "`Display`",
                        "`email: That address is taken.` の形で、フィールドのあいだは `; ` です。ログに使います",
                    ],
                ],
            },
        ),
        para(
            site,
            L {
                en: "[`reject`](api:hypersig/struct.Sig.html#method.reject) on the form's `Sig` turns them into a patch of the form's messages from the server, which a handler returns as `signup.reject(&errors).into()`. `error(field)` shows such a message where no rule of the field fails, and the next submit empties it.",
                ja: "フォームの `Sig` の [`reject`](api:hypersig/struct.Sig.html#method.reject) は、これらのメッセージをフォームが持つサーバーからのメッセージへの差分にします。ハンドラはそれを `signup.reject(&errors).into()` として返します。`error(field)` は、そのフィールドのルールがどれも失敗していないときにこのメッセージを返し、次の送信で空になります。",
            },
        ),
        para(
            site,
            L {
                en: "`is_valid()` on the form's `Sig` is an `Expr<S, bool>` that is true while every rule passes, for a page that shows the state of the whole form, such as a hint beside the submit button.",
                ja: "フォームの `Sig` の `is_valid()` は、すべてのルールが通るあいだ真になる `Expr<S, bool>` です。送信ボタンの横の案内のように、フォーム全体の状態を表示するページで使います。",
            },
        ),
    ])
}

/// What the types leave unchecked, and what happens then.
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
                en: "A rule that reads [`Expr::now`](api:hypersig/struct.Expr.html#method.now) or `Expr::random_below` compiles, but only the browser has those values, so `Form::validate` could not check it. `check` panics with a message that names the field and says the rule reads a value only the browser has. The rules are built while the page is, so the page panics before it is served.",
                ja: "[`Expr::now`](api:hypersig/struct.Expr.html#method.now) や `Expr::random_below` を読むルールはコンパイルできますが、その値はブラウザにしかないので、`Form::validate` では検査できません。`check` は、フィールドの名前を挙げ、ルールがブラウザにしかない値を読むと伝えるメッセージでパニックします。ルールはページと一緒に作られるので、ページは配信される前にパニックします。",
            },
        ),
        para(
            site,
            L {
                en: "Some values pass in the browser and fail on the server. A rule whose arithmetic leaves the finite range, such as a division by zero, or that reads a date outside the years 1000 to 9999, fails in `Form::validate`, with an empty message where the message cannot be evaluated either. A value that holds an infinite or NaN number, which an HTML form's `inf` or `NaN` parses to, fails every rule.",
                ja: "ブラウザでは通り、サーバーでは失敗する値があります。0 での割り算のように演算の結果が有限の範囲を出るルールや、1000 年から 9999 年の外の日付を読むルールは、`Form::validate` で失敗します。メッセージも評価できないときは、メッセージは空になります。HTML のフォームの `inf` や `NaN` を解釈した値のように、無限大か NaN の数を持つ値は、どのルールにも失敗します。",
            },
        ),
        para(
            site,
            L {
                en: "A handler that never calls `Form::validate` compiles, and then accepts whatever the request carries.",
                ja: "`Form::validate` を呼ばないハンドラもコンパイルでき、その場合はリクエストの中身をそのまま受け付けます。",
            },
        ),
    ])
}

/// The links to the pages that come next.
fn next(site: Site) -> Part {
    para(
        site,
        L {
            en: "Next, [Extractors and headers](learn/requests/) reads more of the request than the arguments, and [Errors and logging](learn/errors/) covers a handler that fails.",
            ja: "次の [エクストラクタとヘッダー](learn/requests/) では引数のほかにリクエストから読めるものを、[エラーとログ](learn/errors/) では失敗するハンドラを説明します。",
        },
    )
}
