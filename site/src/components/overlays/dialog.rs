//! The Components page on `Dialog`: a modal `<dialog>` opened and closed by
//! the browser's invoker commands.

use crate::Site;
use crate::code::{self, Lang};
use crate::demo::{self, Demo};
use crate::doc::{Component, Doc};
use crate::lang::L;
use crate::learn::snippet::{Snippet, show};
use crate::prose::{Part, heading, lead, para, parts, table_of};

pub fn page(site: Site, head: Part) -> Part {
    parts(vec![
        head,
        lead(site, Doc::Components(Component::Dialog).leads()),
        demo::show(site, Demo::Dialog),
        accessibility(site),
        building(site),
        html(site),
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
                en: "The dialog is a native `<dialog>` that the browser opens with `showModal()`. While it is open, the rest of the page is inert, so a screen reader reads only the dialog and Tab never leaves it. A screen reader announces it as a dialog named by its title, followed by its description, because `aria-labelledby` points at the title and `aria-describedby` at the description.",
                ja: "ダイアログはネイティブの `<dialog>` で、ブラウザが `showModal()` で開きます。開いている間はページのほかの部分が inert になるので、スクリーンリーダーはダイアログだけを読み、Tab でダイアログの外に出ることもありません。`aria-labelledby` がタイトルを、`aria-describedby` が説明文を指すので、スクリーンリーダーはタイトルを名前とするダイアログとして読み上げ、続けて説明文を読みます。",
            },
        ),
        para(
            site,
            L {
                en: "When it opens, focus moves to the first control in its content. When it closes, focus returns to the element that had it before, usually the button that opened it. The close button in the corner comes after the content, so it never takes the first focus.",
                ja: "ダイアログが開くと、フォーカスは中身の最初のコントロールに移ります。閉じると、開く前にフォーカスを持っていた要素に戻ります。多くの場合、ダイアログを開いたボタンです。角の閉じるボタンは中身のあとに置かれるので、最初のフォーカスを取ることはありません。",
            },
        ),
        table_of(
            site,
            L {
                en: &["Key", "What it does"],
                ja: &["キー", "動作"],
            },
            L {
                en: &[
                    &[
                        "Tab, Shift+Tab",
                        "Moves between the dialog's controls and never reaches the page behind",
                    ],
                    &[
                        "Escape",
                        "Closes the dialog and returns focus to the element that opened it",
                    ],
                ],
                ja: &[
                    &[
                        "Tab、Shift+Tab",
                        "ダイアログのコントロールの間を移り、後ろのページには移りません",
                    ],
                    &[
                        "Escape",
                        "ダイアログを閉じ、ダイアログを開いた要素にフォーカスを戻します",
                    ],
                ],
            },
        ),
        para(
            site,
            L {
                en: "A click on the backdrop also closes the dialog, in a browser that supports `closedby`. The buttons open and close it through invoker commands (`commandfor` and `command`), which Chrome 135, Firefox 144, and Safari 26.2 ship.",
                ja: "`closedby` に対応したブラウザでは、背景のクリックでもダイアログが閉じます。ボタンは invoker commands（`commandfor` と `command`）でダイアログを開閉します。invoker commands は Chrome 135、Firefox 144、Safari 26.2 から使えます。",
            },
        ),
    ])
}

fn building(site: Site) -> Part {
    parts([
        heading(
            site,
            L {
                en: "Building a dialog",
                ja: "ダイアログを作る",
            },
        ),
        para(
            site,
            L {
                en: "A [`Dialog<S>`](api:hypersig_ui/struct.Dialog.html) is a `Copy` handle on a page whose signals are `S`. [`Dialog::new`](api:hypersig_ui/struct.Dialog.html#method.new) takes a `DialogRef<S>` from `dialog_ref!`, and the handle is declared as a `const` so that the dialog and its buttons share one value. [Handles](components/handles/) covers the declaration. The methods below each take the handle by value:",
                ja: "[`Dialog<S>`](api:hypersig_ui/struct.Dialog.html) は、シグナルが `S` のページで使う `Copy` のハンドルです。[`Dialog::new`](api:hypersig_ui/struct.Dialog.html#method.new) は `dialog_ref!` で宣言した `DialogRef<S>` を受け取ります。ダイアログとそのボタンが 1 つの値を共有するように、ハンドルは `const` として宣言します。宣言のしかたは [ハンドル](components/handles/) で説明しています。次のメソッドはどれもハンドルを値で受け取ります。",
            },
        ),
        code::block(
            Lang::Rust,
            None,
            "impl<S> Dialog<S> {
    pub const fn new(dialog: DialogRef<S>) -> Self;
    pub fn trigger(self, variant: ButtonVariant, size: ButtonSize) -> El<S>;
    pub fn close(self, variant: ButtonVariant, size: ButtonSize) -> El<S>;
    pub fn open(self) -> Stmt<S>;
    pub fn dismiss(self) -> Stmt<S>;
    pub fn dialog(
        self,
        title: impl Into<String>,
        description: impl Into<String>,
        content: impl Into<Node<S>>,
        text: &DialogText,
    ) -> El<S>;
}

pub struct DialogText {
    pub close: String,
}

pub fn dialog_footer<S>() -> El<S>;",
        ),
        table_of(
            site,
            L {
                en: &["Item", "What it gives"],
                ja: &["項目", "返すもの"],
            },
            L {
                en: &[
                    &[
                        "[`trigger`](api:hypersig_ui/struct.Dialog.html#method.trigger)",
                        "A button that opens the dialog. Its text is added with `El::text`",
                    ],
                    &[
                        "[`close`](api:hypersig_ui/struct.Dialog.html#method.close)",
                        "A button that closes the dialog, for its footer. It takes `on_click` like any other button",
                    ],
                    &[
                        "[`open`](api:hypersig_ui/struct.Dialog.html#method.open)",
                        "A statement that opens the dialog, for a place with no button, such as a shortcut from `Page::on_key`",
                    ],
                    &[
                        "[`dismiss`](api:hypersig_ui/struct.Dialog.html#method.dismiss)",
                        "A statement that closes the dialog, such as after a call succeeds",
                    ],
                    &[
                        "[`dialog`](api:hypersig_ui/struct.Dialog.html#method.dialog)",
                        "The `<dialog>` with `title`, `description`, `content`, and a close button in its corner",
                    ],
                    &[
                        "[`DialogText`](api:hypersig_ui/struct.DialogText.html)",
                        "The name of the corner button. `DialogText::default()` names it \"Close\"",
                    ],
                    &[
                        "[`dialog_footer`](api:hypersig_ui/fn.dialog_footer.html)",
                        "A row for the dialog's buttons, aligned to the end and wrapping on a narrow screen",
                    ],
                ],
                ja: &[
                    &[
                        "[`trigger`](api:hypersig_ui/struct.Dialog.html#method.trigger)",
                        "ダイアログを開くボタン。テキストは `El::text` で加えます",
                    ],
                    &[
                        "[`close`](api:hypersig_ui/struct.Dialog.html#method.close)",
                        "ダイアログを閉じるボタン。フッターに置きます。ほかのボタンと同じく `on_click` を受け取ります",
                    ],
                    &[
                        "[`open`](api:hypersig_ui/struct.Dialog.html#method.open)",
                        "ダイアログを開く文。`Page::on_key` のショートカットのように、ボタンのない場所で使います",
                    ],
                    &[
                        "[`dismiss`](api:hypersig_ui/struct.Dialog.html#method.dismiss)",
                        "ダイアログを閉じる文。呼び出しが成功したあとなどに使います",
                    ],
                    &[
                        "[`dialog`](api:hypersig_ui/struct.Dialog.html#method.dialog)",
                        "`title`、`description`、`content` と角の閉じるボタンを持つ `<dialog>`",
                    ],
                    &[
                        "[`DialogText`](api:hypersig_ui/struct.DialogText.html)",
                        "角のボタンの名前。`DialogText::default()` では「Close」です",
                    ],
                    &[
                        "[`dialog_footer`](api:hypersig_ui/fn.dialog_footer.html)",
                        "ダイアログのボタンを並べる行。末尾に寄せて並べ、狭い画面では折り返します",
                    ],
                ],
            },
        ),
        para(
            site,
            L {
                en: "`DialogText` is English until replaced, so a page in another language passes its own, such as `DialogText { close: \"閉じる\".into() }`. [Text in other languages](components/component-text/) covers the words of every component.",
                ja: "`DialogText` は置き換えるまで英語なので、ほかの言語のページは `DialogText { close: \"閉じる\".into() }` のように自分の言葉を渡します。各コンポーネントの文言は [ほかの言語の文言](components/component-text/) で説明しています。",
            },
        ),
    ])
}

fn html(site: Site) -> Part {
    parts([
        heading(
            site,
            L {
                en: "The HTML",
                ja: "出力される HTML",
            },
        ),
        para(
            site,
            L {
                en: "For the trigger and the dialog, the example at the end of this page renders this HTML, with the classes left out. Each `…` is an id made from the dialog ref's name, so the buttons, the dialog, its title, and its description point at each other:",
                ja: "このページの最後の例はトリガーとダイアログについて次の HTML を出力します（クラスは省いています）。`…` はどれもダイアログ参照の名前から作った id で、ボタン、ダイアログ、タイトル、説明文はこの id で互いを指します。",
            },
        ),
        code::block(
            Lang::Html,
            None,
            r#"<button type="button" aria-haspopup="dialog" commandfor="…" command="show-modal">Rename</button>

<dialog id="…" aria-labelledby="…" aria-describedby="…" closedby="any">
  <div>
    <div>
      <h2 id="…">Rename project</h2>
      <p id="…">The new name shows everywhere the project appears.</p>
    </div>
    <div>
      <!-- the text field -->
      <div>
        <button type="button" commandfor="…" command="close">Cancel</button>
        <button type="button" commandfor="…" command="close">Save</button>
      </div>
    </div>
  </div>
  <button type="button" aria-label="Close" commandfor="…" command="close">
    <svg aria-hidden="true" …></svg>
  </button>
</dialog>"#,
        ),
        para(
            site,
            L {
                en: "The browser runs `show-modal` and `close` itself, so opening and closing the dialog works before the runtime has loaded. `open` and `dismiss` are statements the runtime runs, and they call the same `showModal()` and `close()` on the `<dialog>`.",
                ja: "`show-modal` と `close` はブラウザ自身が実行するので、ランタイムの読み込みが終わる前でもダイアログは開閉できます。`open` と `dismiss` はランタイムが実行する文で、同じ `<dialog>` の `showModal()` と `close()` を呼びます。",
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
                en: "`Dialog::new` takes a `DialogRef`, which can only be attached to a `<dialog>`. A `NodeRef` can name any element, so passing one is E0308, mismatched types:",
                ja: "`Dialog::new` が受け取るのは `DialogRef` で、これは `<dialog>` にしか付けられません。`NodeRef` はどの要素にも付けられるので、`NodeRef` を渡すと E0308（型の不一致）になります。",
            },
        ),
        show(Snippet::UiDialogNodeRef),
        para(
            site,
            L {
                en: "`Dialog::dialog` takes the `DialogText` as an argument, so the button in the corner always has a name. Leaving it out is E0061, wrong number of arguments:",
                ja: "`Dialog::dialog` は `DialogText` を引数に取るので、角のボタンには必ず名前が付きます。省くと E0061（引数の数の誤り）になります。",
            },
        ),
        show(Snippet::UiDialogText),
        para(
            site,
            L {
                en: "`trigger`, `close`, `open`, and `dismiss` return an `El<S>` or a `Stmt<S>` of the handle's page, so using one on a page whose signals are another type is E0308, as [Handles](components/handles/) shows. `dialog_footer` takes only children, and its types stop no mistake.",
                ja: "`trigger`、`close`、`open`、`dismiss` はハンドルのページの `El<S>` か `Stmt<S>` を返すので、シグナルの型が違うページで使うと E0308 になります。例は [ハンドル](components/handles/) にあります。`dialog_footer` は子要素を受け取るだけで、型が防ぐ誤りはありません。",
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
                en: "The compiler does not see how many times the dialog is placed. One handle's `dialog` placed twice on a page gives two elements one id, and `Page::render` panics with `two elements carry the … where the runtime can reach both`. A trigger with no dialog on the page renders, and pressing it does nothing.",
                ja: "ダイアログを何回置いたかはコンパイラには分かりません。1 つのハンドルの `dialog` をページに 2 回置くと 2 つの要素が同じ id を持ち、`Page::render` が `two elements carry the … where the runtime can reach both` というメッセージで panic します。ページにダイアログがないトリガーは描画されますが、押しても何も起きません。",
            },
        ),
        para(
            site,
            L {
                en: "A `close` button placed outside the dialog renders and does nothing while the dialog is closed. In a browser without invoker commands, the trigger and the close buttons do nothing, while `open` and `dismiss` still work. Without `closedby`, a click on the backdrop leaves the dialog open, and Escape still closes it.",
                ja: "ダイアログの外に置いた `close` のボタンは描画されますが、ダイアログが閉じている間は何もしません。invoker commands に対応していないブラウザでは、トリガーと閉じるボタンは何もしません。`open` と `dismiss` はそのブラウザでも動きます。`closedby` に対応していないブラウザでは、背景をクリックしてもダイアログは開いたままで、Escape では閉じます。",
            },
        ),
    ])
}

fn example(site: Site) -> Part {
    parts([
        heading(
            site,
            L {
                en: "Example: renaming a project",
                ja: "例: プロジェクトの名前を変更する",
            },
        ),
        para(
            site,
            L {
                en: "A button and a shortcut open the dialog. Cancel only closes it, and Save sets a status before closing it:",
                ja: "ボタンとショートカットでダイアログを開きます。Cancel はダイアログを閉じるだけで、Save は状態のテキストを設定してから閉じます。",
            },
        ),
        show(Snippet::UiDialog),
        para(
            site,
            L {
                en: "A dialog that asks the visitor to confirm before going on is an [Alert dialog](components/alert-dialog/), and one along an edge of the viewport is a [Sheet](components/sheet/). A panel that stays beside its button without blocking the page is a [Popover](components/popover/).",
                ja: "先に進む前に訪問者に確認を求めるダイアログは [アラートダイアログ](components/alert-dialog/) で、ビューポートの端に沿って出るダイアログは [シート](components/sheet/) です。ページを操作できるままボタンの横に開くパネルには [ポップオーバー](components/popover/) を使います。",
            },
        ),
    ])
}
