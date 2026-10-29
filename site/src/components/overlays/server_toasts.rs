//! The Components page on toasts a handler adds with a patch, and on the
//! server's message shown when a call fails.

use crate::Site;
use crate::code::{self, Lang};
use crate::doc::{Component, Doc};
use crate::lang::L;
use crate::learn::snippet::{Snippet, show};
use crate::prose::{Part, heading, lead, para, parts, table_of};

pub fn page(site: Site, head: Part) -> Part {
    parts(vec![
        head,
        lead(site, Doc::Components(Component::ServerToasts).leads()),
        accessibility(site),
        handler(site),
        http(site),
        failed_call(site),
        ids(site),
        compile_errors(site),
        unchecked(site),
        example(site),
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
                en: "A toast from the server goes into the same list as one the browser pushes, so the [Toaster](components/toast/) shows it in the same `aria-live=\"polite\"` region, and a screen reader reads it once it finishes what it is reading. The keys are those of the [Toast](components/toast/) page. A toast that is already in the list when the page loads is part of the first HTML, so a screen reader reads it with the page and does not announce it.",
                ja: "サーバーからのトーストは、ブラウザが追加するトーストと同じリストに入ります。そのため、[Toaster](components/toast/) が同じ `aria-live=\"polite\"` の領域に表示し、スクリーンリーダーは読み上げ中の内容を読み終えてから読み上げます。キー操作は [トースト](components/toast/) のページと同じです。ページを読み込んだ時点でリストにあるトーストは最初の HTML に含まれるので、スクリーンリーダーはページの一部として読み、お知らせとしては読み上げません。",
            },
        ),
        para(
            site,
            L {
                en: "This page has no live demo, because this site is static files and has no server to answer a call. The [Toast](components/toast/) page shows the toaster running.",
                ja: "このサイトは静的なファイルで、呼び出しに答えるサーバーがないので、このページには動くデモがありません。領域が動く様子は [トースト](components/toast/) のページで見られます。",
            },
        ),
    ])
}

fn handler(site: Site) -> Part {
    parts([
        heading(
            site,
            L {
                en: "Adding a toast from a handler",
                ja: "ハンドラからトーストを追加する",
            },
        ),
        para(
            site,
            L {
                en: "The toasts are a `Vec<Toast>` field of the page's signals, so a handler adds one with a list patch on that field. [`Sig::patch_append`](api:hypersig/struct.Sig.html#method.patch_append) adds items after the ones the browser holds, so the toasts the visitor already sees stay:",
                ja: "トーストはページのシグナルの `Vec<Toast>` のフィールドなので、ハンドラはそのフィールドへのリストの差分でトーストを追加します。[`Sig::patch_append`](api:hypersig/struct.Sig.html#method.patch_append) はブラウザが持つ項目の後ろに項目を追加するので、訪問者がすでに見ているトーストは残ります。",
            },
        ),
        code::block(
            Lang::Rust,
            None,
            "impl<S, T: Serialize> Sig<S, Vec<T>, Root> {
    pub fn patch_append(self, items: &[T]) -> ListPatch<S>;
}

impl<S: Signals> From<ListPatch<S>> for Patches<S>;",
        ),
        para(
            site,
            L {
                en: "A handler returns the `ListPatch<S>` itself, or a `Patches<S>` that joins it with other patches, such as the signal the handler saved. Build each toast with `Toast::new`, `description`, and `variant`, as on the [Toast](components/toast/) page. `push` and `push_expr` are statements the browser runs, so a handler does not use them. [List patches](learn/list-patches/) covers the other patches of a list.",
                ja: "ハンドラは `ListPatch<S>` をそのまま返すか、ハンドラが保存したシグナルなどほかの差分と合わせた `Patches<S>` を返します。各トーストは [トースト](components/toast/) のページと同じく、`Toast::new`、`description`、`variant` で作ります。`push` と `push_expr` はブラウザが実行する文なので、ハンドラでは使いません。リストのほかの差分は [リストの差分](learn/list-patches/) で説明しています。",
            },
        ),
        table_of(
            site,
            L {
                en: &["Patch on `toasts`", "What the visitor sees"],
                ja: &["`toasts` への差分", "訪問者に見えること"],
            },
            L {
                en: &[
                    &[
                        "`patch_append(&[toast])`",
                        "The new toast after the ones on screen",
                    ],
                    &[
                        "`patch_prepend(&[toast])`",
                        "The new toast before the ones on screen",
                    ],
                    &[
                        "`patch(vec![toast])`",
                        "Only the new toast: the ones on screen are replaced",
                    ],
                ],
                ja: &[
                    &[
                        "`patch_append(&[toast])`",
                        "表示中のトーストの後ろに新しいトースト",
                    ],
                    &[
                        "`patch_prepend(&[toast])`",
                        "表示中のトーストの前に新しいトースト",
                    ],
                    &[
                        "`patch(vec![toast])`",
                        "新しいトーストだけ。表示中のトーストは置き換えられます",
                    ],
                ],
            },
        ),
    ])
}

fn http(site: Site) -> Part {
    parts([
        heading(
            site,
            L {
                en: "The HTTP",
                ja: "HTTP のやりとり",
            },
        ),
        para(
            site,
            L {
                en: "A click on Save in the example below sends the call, and the handler's answer carries the patch. The answer is `200` with `content-type: text/event-stream`, and the browser applies the patch as it arrives:",
                ja: "下の例で Save をクリックすると呼び出しが送られ、ハンドラの応答が差分を運びます。応答は `200` で `content-type: text/event-stream` です。ブラウザは差分が届いたところで適用します。",
            },
        ),
        code::block(
            Lang::Html,
            None,
            "POST /save
accept: text/event-stream
hypersig-call: 1
content-type: application/json

null

HTTP/1.1 200 OK
content-type: text/event-stream",
        ),
        para(
            site,
            L {
                en: "[Actions](learn/actions/) covers the request and the answer in full.",
                ja: "リクエストと応答の詳細は [アクション](learn/actions/) で説明しています。",
            },
        ),
    ])
}

fn failed_call(site: Site) -> Part {
    parts([
        heading(
            site,
            L {
                en: "Showing the server's message when a call fails",
                ja: "呼び出しが失敗したときにサーバーのメッセージを表示する",
            },
        ),
        para(
            site,
            L {
                en: "When a call fails, `on_error` passes the server's message to the browser as an `Expr<S, String>`. `Toast::push_expr` adds a toast built from it in the browser: start from a literal toast with `Expr::lit` and put the message in its description with `Expr::with(Toast::body(), message)`:",
                ja: "呼び出しが失敗すると、`on_error` はサーバーのメッセージを `Expr<S, String>` としてブラウザ側に渡します。`Toast::push_expr` で、そのメッセージからトーストを作ってブラウザで追加します。`Expr::lit` でトーストのリテラルから始め、`Expr::with(Toast::body(), message)` でメッセージを説明に入れます。",
            },
        ),
        code::block(
            Lang::Rust,
            None,
            "let failed = Toast::new(\"Save failed\").variant(ToastVariant::Destructive);
let call = SAVE.call().on_error(|message| {
    Toast::push_expr(App::toasts(), Expr::lit(&failed).with(Toast::body(), message))
});",
        ),
        para(
            site,
            L {
                en: "[Busy and failed calls](learn/call-status/) covers `on_error` and the message it receives.",
                ja: "`on_error` と、それが受け取るメッセージは [呼び出し中と失敗](learn/call-status/) で説明しています。",
            },
        ),
    ])
}

fn ids(site: Site) -> Part {
    parts([
        heading(
            site,
            L {
                en: "Ids from the server and from the browser",
                ja: "サーバーとブラウザの id",
            },
        ),
        para(
            site,
            L {
                en: "Each toast has an id that tells it apart while the list changes, and nothing outside `Toast` sets it. `Toast::new` on the server counts down from -1, so every toast made in one process has its own id below 0. `push` and `push_expr` in the browser give one more than the highest id in the list, and 1 where no id is above 0. A toast from the server and one from the browser therefore never share an id, whichever order they arrive in.",
                ja: "各トーストは、リストが変わる間にトーストを見分けるための id を持ちます。id は `Toast` の外からは設定できません。サーバーの `Toast::new` は -1 から順に減らした値を付けるので、1 つのプロセスで作ったトーストはどれも 0 未満の異なる id を持ちます。ブラウザの `push` と `push_expr` はリストの最大の id に 1 を足した値を付け、0 より大きい id がなければ 1 を付けます。そのため、サーバーのトーストとブラウザのトーストは、どんな順で届いても id が重なりません。",
            },
        ),
    ])
}

fn compile_errors(site: Site) -> Part {
    parts([
        heading(
            site,
            L {
                en: "What fails to compile",
                ja: "コンパイルエラーになるもの",
            },
        ),
        para(
            site,
            L {
                en: "`patch_append` on `toasts` takes `&[Toast]`, so a bare message is E0308, mismatched types:",
                ja: "`toasts` の `patch_append` は `&[Toast]` を受け取るので、メッセージの文字列だけを渡すと E0308（型の不一致）になります。",
            },
        ),
        show(Snippet::UiServerToastText),
        para(
            site,
            L {
                en: "`Toast` is `#[non_exhaustive]` and its id is private, so a toast written as a struct literal, with an id chosen by hand, is E0639:",
                ja: "`Toast` は `#[non_exhaustive]` で id は非公開なので、構造体リテラルで書いたトースト（id を自分で決めたトースト）は E0639 になります。",
            },
        ),
        show(Snippet::UiToastId),
        para(
            site,
            L {
                en: "A handler returns `Patches<S>` of the page the action belongs to, so a patch on another page's `toasts` is E0308 too.",
                ja: "ハンドラはアクションが属するページの `Patches<S>` を返すので、ほかのページの `toasts` への差分も E0308 になります。",
            },
        ),
    ])
}

fn unchecked(site: Site) -> Part {
    parts([
        heading(
            site,
            L {
                en: "What is not checked",
                ja: "検査されないこと",
            },
        ),
        para(
            site,
            L {
                en: "Nothing checks which patch a handler uses. `patch` in place of `patch_append` replaces the toasts on screen, so a toast the visitor has not read yet disappears. Sending one `Toast` value twice, or a clone of it, puts two toasts with one id in the list; the browser tells them apart by position and writes a warning to the console. After 2³¹ toasts in one process the server's ids start again from -1.",
                ja: "ハンドラがどの差分を使うかは検査されません。`patch_append` の代わりに `patch` を使うと表示中のトーストが置き換えられ、訪問者がまだ読んでいないトーストも消えます。1 つの `Toast` の値やその複製を 2 回送ると、同じ id のトーストが 2 つリストに入ります。このとき、ブラウザは 2 つを位置で見分け、コンソールに警告を書きます。1 つのプロセスで 2³¹ 個のトーストを作ると、サーバーの id はまた -1 から始まります。",
            },
        ),
    ])
}

fn example(site: Site) -> Part {
    parts([
        heading(
            site,
            L {
                en: "Example: saving with a toast",
                ja: "例: 保存してトーストを出す",
            },
        ),
        para(
            site,
            L {
                en: "The page starts with a welcome toast in its signals, which is in the first HTML. Save calls the action, whose handler appends a toast, and a failed call shows the server's message:",
                ja: "ページのシグナルは最初から歓迎のトーストを持っており、このトーストは最初の HTML に含まれます。Save はアクションを呼び出し、そのハンドラがトーストを追加します。呼び出しが失敗したときは、サーバーのメッセージを表示します。",
            },
        ),
        show(Snippet::UiServerToasts),
        para(
            site,
            L {
                en: "[Toast](components/toast/) covers the toaster and toasts pushed from the browser. [Patches](learn/patches/) and [List patches](learn/list-patches/) cover everything else a handler can send.",
                ja: "領域とブラウザから追加するトーストは [トースト](components/toast/) で説明しています。ハンドラが送れるそのほかの差分は [差分](learn/patches/) と [リストの差分](learn/list-patches/) で説明しています。",
            },
        ),
    ])
}
