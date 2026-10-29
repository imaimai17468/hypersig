//! The Components page on `Toaster` and `Toast` in the browser: short
//! notices that a statement adds to a list signal.

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
        lead(site, Doc::Components(Component::Toast).leads()),
        demo::show(site, Demo::Toast),
        accessibility(site),
        toaster(site),
        toasts(site),
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
                en: "The toaster is a `<section>` named \"Notifications\", which a screen reader lists as a region, holding an `aria-live=\"polite\"` list. When a toast is added, a screen reader reads its title and description once it finishes what it is reading. Each toast is read as a whole, and each has a button that removes it.",
                ja: "トーストを並べる領域（toaster）は「Notifications」という名前の `<section>` で、スクリーンリーダーはこれを領域（region）として一覧に挙げます。その中に `aria-live=\"polite\"` のリストがあります。トーストが追加されると、スクリーンリーダーは読み上げ中の内容を読み終えてから、トーストのタイトルと説明を読み上げます。トーストは 1 つずつ全体が読まれ、どのトーストにもそれを消すボタンがあります。",
            },
        ),
        para(
            site,
            L {
                en: "A toast stays until it is dismissed, unless `Toaster::dismiss_after` sets a time, because a visitor who reads slowly or uses a magnifier can miss a notice that leaves by itself (WCAG 2.2.1). The region is in the page rather than in the top layer, so while a modal dialog is open the region is inert, and it becomes usable again when the dialog closes.",
                ja: "トーストは `Toaster::dismiss_after` で時間を設定しない限り、消されるまで残ります。ゆっくり読む訪問者や拡大鏡を使う訪問者は、自動で消えるお知らせを見逃すことがあるからです（WCAG 2.2.1）。この領域は top layer ではなくページの中にあるので、モーダルダイアログが開いている間は inert になり、ダイアログが閉じると操作できるようになります。",
            },
        ),
        table_of(
            site,
            L {
                en: &["Key", "What it does"],
                ja: &["キー", "動作"],
            },
            L {
                en: &[&[
                    "Tab, then Enter or Space",
                    "Reaches a toast's Dismiss button and removes the toast",
                ]],
                ja: &[&[
                    "Tab のあと Enter か Space",
                    "トーストの Dismiss ボタンに移り、トーストを取り除きます",
                ]],
            },
        ),
    ])
}

fn toaster(site: Site) -> Part {
    parts([
        heading(
            site,
            L {
                en: "The toaster",
                ja: "トーストを並べる領域",
            },
        ),
        para(
            site,
            L {
                en: "The toasts live in a `Vec<Toast>` field of the page's signals. A [`Toaster<S>`](api:hypersig_ui/struct.Toaster.html) shows that list in the bottom corner of the page, newest last. Put it on the page once, anywhere, since it is fixed to the viewport:",
                ja: "トーストはページのシグナルの `Vec<Toast>` のフィールドに入ります。[`Toaster<S>`](api:hypersig_ui/struct.Toaster.html) はそのリストを、新しいものほど後ろにしてページの下の隅に表示します。ビューポートに固定されるので、ページのどこか 1 か所に置きます。",
            },
        ),
        code::block(
            Lang::Rust,
            None,
            "impl<S> Toaster<S> {
    pub fn new(toasts: Sig<S, Vec<Toast>>) -> Self;
    pub fn label(self, label: impl Into<String>) -> Self;
    pub fn dismiss_label(self, label: impl Into<String>) -> Self;
    pub fn dismiss_after(self, delay: Duration) -> Self;
    pub fn render(self) -> El<S>;
}

impl<S> From<Toaster<S>> for El<S>;
impl<S> From<Toaster<S>> for Node<S>;",
        ),
        table_of(
            site,
            L {
                en: &["Item", "What it does"],
                ja: &["項目", "動作"],
            },
            L {
                en: &[
                    &[
                        "[`Toaster::new`](api:hypersig_ui/struct.Toaster.html#method.new)",
                        "A region that shows `toasts`, newest last",
                    ],
                    &[
                        "[`label`](api:hypersig_ui/struct.Toaster.html#method.label)",
                        "Names the region, in place of \"Notifications\"",
                    ],
                    &[
                        "[`dismiss_label`](api:hypersig_ui/struct.Toaster.html#method.dismiss_label)",
                        "Names each toast's button, in place of \"Dismiss\"",
                    ],
                    &[
                        "[`dismiss_after`](api:hypersig_ui/struct.Toaster.html#method.dismiss_after)",
                        "Removes each toast `delay` after it appears",
                    ],
                    &[
                        "[`render`](api:hypersig_ui/struct.Toaster.html#method.render)",
                        "The region as an `El<S>`. `El::child` also takes the `Toaster` itself",
                    ],
                ],
                ja: &[
                    &[
                        "[`Toaster::new`](api:hypersig_ui/struct.Toaster.html#method.new)",
                        "`toasts` を新しいものほど後ろにして表示する領域",
                    ],
                    &[
                        "[`label`](api:hypersig_ui/struct.Toaster.html#method.label)",
                        "「Notifications」の代わりに、領域の名前を付けます",
                    ],
                    &[
                        "[`dismiss_label`](api:hypersig_ui/struct.Toaster.html#method.dismiss_label)",
                        "「Dismiss」の代わりに、各トーストのボタンの名前を付けます",
                    ],
                    &[
                        "[`dismiss_after`](api:hypersig_ui/struct.Toaster.html#method.dismiss_after)",
                        "各トーストを、表示してから `delay` が経ったら取り除きます",
                    ],
                    &[
                        "[`render`](api:hypersig_ui/struct.Toaster.html#method.render)",
                        "`El<S>` としての領域。`El::child` には `Toaster` をそのまま渡せます",
                    ],
                ],
            },
        ),
        para(
            site,
            L {
                en: "The labels are English until replaced, so a page in another language passes its own to `label` and `dismiss_label`. With `dismiss_after`, the time runs while the pointer or focus is on the toast, so give it long enough to read the longest toast twice.",
                ja: "名前は置き換えるまで英語なので、ほかの言語のページは `label` と `dismiss_label` に自分の言葉を渡します。`dismiss_after` の時間はポインタやフォーカスがトーストの上にある間も進むので、いちばん長いトーストを 2 回読める長さにします。",
            },
        ),
    ])
}

fn toasts(site: Site) -> Part {
    parts([
        heading(
            site,
            L {
                en: "Adding a toast",
                ja: "トーストを追加する",
            },
        ),
        para(
            site,
            L {
                en: "A [`Toast`](api:hypersig_ui/struct.Toast.html) is a title, an optional description, a [`ToastVariant`](api:hypersig_ui/enum.ToastVariant.html), and an id. It derives `Signals`, so its fields are signals of their own inside an expression:",
                ja: "[`Toast`](api:hypersig_ui/struct.Toast.html) は、タイトル、省略できる説明、[`ToastVariant`](api:hypersig_ui/enum.ToastVariant.html)、id を持ちます。`Signals` を導出しているので、式の中ではフィールドもそれぞれシグナルになります。",
            },
        ),
        code::block(
            Lang::Rust,
            None,
            "#[non_exhaustive]
pub struct Toast {
    pub title: String,
    pub body: String,
    pub tone: ToastVariant,
    // and an id that only `Toast` sets
}

impl Toast {
    pub fn new(title: impl Into<String>) -> Self;
    pub fn description(self, text: impl Into<String>) -> Self;
    pub fn variant(self, variant: ToastVariant) -> Self;
    pub fn push<S>(self, toasts: Sig<S, Vec<Self>>) -> Stmt<S>;
    pub fn push_expr<S>(toasts: Sig<S, Vec<Self>>, toast: impl IntoExpr<S, Self>) -> Stmt<S>;
}

#[non_exhaustive]
pub enum ToastVariant {
    Default,
    Destructive,
}",
        ),
        table_of(
            site,
            L {
                en: &["Item", "What it does"],
                ja: &["項目", "動作"],
            },
            L {
                en: &[
                    &[
                        "[`Toast::new`](api:hypersig_ui/struct.Toast.html#method.new)",
                        "A toast titled `title`, with an empty description and `ToastVariant::Default`",
                    ],
                    &[
                        "[`description`](api:hypersig_ui/struct.Toast.html#method.description)",
                        "Sets the line under the title, which is hidden while it is empty",
                    ],
                    &[
                        "[`variant`](api:hypersig_ui/struct.Toast.html#method.variant)",
                        "Sets the look. `ToastVariant::Destructive` draws the title in `token::destructive`, for a failure",
                    ],
                    &[
                        "[`push`](api:hypersig_ui/struct.Toast.html#method.push)",
                        "A statement that adds this toast to the end of `toasts` in the browser",
                    ],
                    &[
                        "[`push_expr`](api:hypersig_ui/struct.Toast.html#method.push_expr)",
                        "A statement that adds a toast the browser evaluates, so its text can come from a signal",
                    ],
                ],
                ja: &[
                    &[
                        "[`Toast::new`](api:hypersig_ui/struct.Toast.html#method.new)",
                        "`title` をタイトルに持ち、説明が空で `ToastVariant::Default` のトースト",
                    ],
                    &[
                        "[`description`](api:hypersig_ui/struct.Toast.html#method.description)",
                        "タイトルの下の行を設定します。空の間は表示されません",
                    ],
                    &[
                        "[`variant`](api:hypersig_ui/struct.Toast.html#method.variant)",
                        "見た目を設定します。`ToastVariant::Destructive` は失敗を伝えるときに使い、タイトルを `token::destructive` の色で描きます",
                    ],
                    &[
                        "[`push`](api:hypersig_ui/struct.Toast.html#method.push)",
                        "ブラウザで、このトーストを `toasts` の末尾に追加する文",
                    ],
                    &[
                        "[`push_expr`](api:hypersig_ui/struct.Toast.html#method.push_expr)",
                        "ブラウザが評価するトーストを追加する文。テキストをシグナルから取れます",
                    ],
                ],
            },
        ),
        para(
            site,
            L {
                en: "`push` sends the toast as it is when the page renders. For text from a signal, `push_expr` takes an expression: start from `Expr::lit(&toast)` and replace a field with `Expr::with(Toast::body(), …)`, as the example does with the name the visitor typed. Either way the browser gives the new toast an id above every id in the list.",
                ja: "`push` は、ページを描画したときのトーストをそのまま送ります。テキストをシグナルから取るには、`push_expr` に式を渡します。`Expr::lit(&toast)` から始め、`Expr::with(Toast::body(), …)` でフィールドを置き換えます。例では、訪問者が入力した名前をこうして説明に入れています。どちらの場合も、ブラウザは新しいトーストに、リストのどの id よりも大きい id を付けます。",
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
                en: "With one toast in the list, the toaster is this HTML, with the classes left out. A toast that the browser adds later gets the same `<li>`:",
                ja: "リストにトーストが 1 つあるとき、領域は次の HTML になります（クラスは省いています）。ブラウザがあとから追加するトーストも、同じ `<li>` になります。",
            },
        ),
        code::block(
            Lang::Html,
            None,
            r#"<section aria-label="Notifications">
  <ol aria-live="polite">
    <li aria-atomic="true">
      <div>
        <p>Saved</p>
        <p>Your changes are live.</p>
      </div>
      <button type="button" aria-label="Dismiss">
        <svg aria-hidden="true" …></svg>
      </button>
    </li>
  </ol>
</section>"#,
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
                en: "`push`, `push_expr`, and `Toaster::new` take a `Sig<S, Vec<Toast>>`, so a list of another type is E0308, mismatched types:",
                ja: "`push`、`push_expr`、`Toaster::new` は `Sig<S, Vec<Toast>>` を受け取るので、ほかの型のリストを渡すと E0308（型の不一致）になります。",
            },
        ),
        show(Snippet::UiToastList),
        para(
            site,
            L {
                en: "`dismiss_after` takes a `Duration`, so a bare number, which could mean seconds or milliseconds, is E0308:",
                ja: "`dismiss_after` は `Duration` を受け取るので、秒ともミリ秒とも読める数をそのまま渡すと E0308 になります。",
            },
        ),
        show(Snippet::UiToastDelay),
        para(
            site,
            L {
                en: "`push_expr` takes `impl IntoExpr<S, Toast>`, so an expression of another type, such as the `String` from `on_error`, is E0277 until it goes into a field with `Expr::with`. `variant` takes a `ToastVariant`, so a string is E0308. A toast with a hand-written id does not compile either, as [Toasts from the server](components/server-toasts/) shows.",
                ja: "`push_expr` は `impl IntoExpr<S, Toast>` を受け取るので、`on_error` が渡す `String` のようなほかの型の式は、`Expr::with` でフィールドに入れるまで E0277 になります。`variant` は `ToastVariant` を受け取るので、文字列を渡すと E0308 になります。id を自分で書いたトーストもコンパイルできません。例は [サーバーからのトースト](components/server-toasts/) にあります。",
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
                en: "The id tells toasts apart while the list changes. A clone of a `Toast` keeps its id, so a list that holds a toast and its clone repeats an id. The browser then tells the two apart by their position and writes a warning to the console. `push` and `push_expr` always give a new id, so this happens only when the list itself holds a clone, such as one the server sent twice.",
                ja: "id は、リストが変わる間にトーストを見分けるためのものです。`Toast` を複製しても id は同じなので、トーストとその複製を持つリストでは id が重なります。このとき、ブラウザは 2 つを位置で見分け、コンソールに警告を書きます。`push` と `push_expr` は必ず新しい id を付けるので、これが起きるのは、サーバーが同じトーストを 2 回送ったときのように、リスト自体が複製を持つ場合だけです。",
            },
        ),
        para(
            site,
            L {
                en: "Nothing checks the length of `dismiss_after`. A short delay removes a toast before a slow reader finishes it, and `Duration::ZERO` removes each one as it appears. A page with no `Toaster` renders, and `push` then grows a list that nothing shows.",
                ja: "`dismiss_after` の長さは検査されません。短くすると、ゆっくり読む訪問者が読み終える前にトーストが消え、`Duration::ZERO` では表示したとたんに消えます。`Toaster` のないページも描画されますが、その場合 `push` は何も表示しないリストを伸ばすだけです。",
            },
        ),
    ])
}

fn example(site: Site) -> Part {
    parts([
        heading(
            site,
            L {
                en: "Example: notices from three buttons",
                ja: "例: 3 つのボタンから出すお知らせ",
            },
        ),
        para(
            site,
            L {
                en: "Save pushes a fixed toast, Upload a destructive one, and Rename one whose description holds the name in the text field. Each toast leaves after ten seconds:",
                ja: "Save は決まった内容のトーストを、Upload は destructive のトーストを、Rename はテキスト欄の名前を説明に入れたトーストを追加します。どのトーストも 10 秒で消えます。",
            },
        ),
        show(Snippet::UiToast),
        para(
            site,
            L {
                en: "A handler adds toasts with a patch, and a failed call can show the server's message: [Toasts from the server](components/server-toasts/) covers both. A message that stays in the page until the problem is fixed is an [Alert](components/alert/).",
                ja: "ハンドラから差分でトーストを追加する方法と、失敗した呼び出しでサーバーのメッセージを表示する方法は、[サーバーからのトースト](components/server-toasts/) で説明します。問題が解決するまでページに残すメッセージには [アラート](components/alert/) を使います。",
            },
        ),
    ])
}
