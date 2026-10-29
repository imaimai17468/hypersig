//! The page on computed signals.

use crate::Site;
use crate::demo::{self, Demo};
use crate::doc::{Doc, Learn};
use crate::lang::L;
use crate::learn::snippet::{Snippet, show};
use crate::prose::{Part, bullets, heading, lead, para, parts, title};

pub fn page(site: Site) -> Part {
    parts(vec![
        title(Doc::Learn(Learn::ComputedSignals).title(site)),
        lead(site, Doc::Learn(Learn::ComputedSignals).leads()),
        heading(
            site,
            L {
                en: "A field kept equal to an expression",
                ja: "式の値に保たれるフィールド",
            },
        ),
        para(
            site,
            L {
                en: "[`Page::computed`](api:hypersig/struct.Page.html#method.computed), `fn computed<T, M: IsComputed>(self, signal: Sig<S, T, M>, value: impl IntoExpr<S, T>) -> Self`, keeps a field equal to an expression over other fields. The browser recomputes it when a field the expression reads changes. Bindings read it like any field and run again only when its value changes, so no binding evaluates the expression itself. The field is marked `#[signals(computed)]` in the struct:",
                ja: "[`Page::computed`](api:hypersig/struct.Page.html#method.computed)（`fn computed<T, M: IsComputed>(self, signal: Sig<S, T, M>, value: impl IntoExpr<S, T>) -> Self`）は、フィールドをほかのフィールドを使った式の値に保ち続けます。式が読むフィールドが変わると、ブラウザが計算し直します。束縛はそれぞれ式を評価する代わりに、ほかのフィールドと同じようにこの値を読み、値が変わったときだけ評価し直します。構造体ではそのフィールドに `#[signals(computed)]` を付けます。",
            },
        ),
        show(Snippet::StateComputed),
        para(
            site,
            L {
                en: "The server computes the value once for the first HTML, so the page shows it before the runtime loads. The value the struct holds for the field is used only where the server cannot evaluate the expression, as with one that reads `Expr::now()`, and the browser replaces it once the runtime has started.",
                ja: "サーバーは最初の HTML のために値を一度計算するので、ページはランタイムを読み込む前からその値を表示します。構造体がそのフィールドに持たせた値が使われるのは、`Expr::now()` を読む式のようにサーバーが評価できない場合だけで、その値もランタイムの起動後にブラウザが置き換えます。",
            },
        ),
        para(
            site,
            L {
                en: "An action call can pass a computed signal as an argument like any other, and [`Page::value_of`](api:hypersig/struct.Page.html#method.value_of) returns its computed value in a test.",
                ja: "アクションの呼び出しはほかのシグナルと同じように計算シグナルを引数として渡せます。テストでは、[`Page::value_of`](api:hypersig/struct.Page.html#method.value_of) が計算された値を返します。",
            },
        ),
        heading(
            site,
            L {
                en: "The order of computed signals",
                ja: "計算シグナルの順序",
            },
        ),
        para(
            site,
            L {
                en: "Computed signals are evaluated in the order they are declared, so one may read an earlier one and no other:",
                ja: "計算シグナルは宣言した順に評価されるので、先に宣言したものだけを読めます。",
            },
        ),
        show(Snippet::StateComputedOrder),
        para(
            site,
            L {
                en: "The types do not see the order, so `Page::render`, `Page::value_of`, and `Page::simulate` panic in these cases:",
                ja: "この順序は型には見えないので、次の場合には `Page::render`、`Page::value_of`、`Page::simulate` が panic します。",
            },
        ),
        bullets(
            site,
            &[
                L {
                    en: "A computed signal reads itself or one declared after it.",
                    ja: "計算シグナルが自分自身か、後に宣言したものを読むとき。",
                },
                L {
                    en: "Two `Page::computed` calls name one field.",
                    ja: "2 つの `Page::computed` の呼び出しが同じフィールドを指すとき。",
                },
                L {
                    en: "A field marked `#[signals(computed)]` is named by no `Page::computed` call, since nothing would then keep it up to date.",
                    ja: "`#[signals(computed)]` を付けたフィールドを、どの `Page::computed` の呼び出しも指していないとき。値を最新に保つものがなくなるからです。",
                },
                L {
                    en: "The expression reads a list item, which has no fixed path. A total over a list is an aggregate such as `sum_of` instead.",
                    ja: "式がリストの項目を読むとき。項目には固定したパスがないからです。リスト全体の合計には代わりに `sum_of` のような集計を使います。",
                },
            ],
        ),
        heading(
            site,
            L {
                en: "Writing a computed signal is a compile error",
                ja: "計算シグナルへの書き込みはコンパイルエラーになる",
            },
        ),
        para(
            site,
            L {
                en: "The accessor of a marked field returns a `Sig<S, T, Computed>`. [`Computed`](api:hypersig/enum.Computed.html) lacks the [`Writable`](api:hypersig/trait.Writable.html) trait, so the type has no `set`, `toggle`, list statement, or `patch`, and `bind` refuses it. The browser would overwrite a written value the next time a field the expression reads changes, so the write fails to compile:",
                ja: "印を付けたフィールドのアクセサーは `Sig<S, T, Computed>` を返します。[`Computed`](api:hypersig/enum.Computed.html) は [`Writable`](api:hypersig/trait.Writable.html) トレイトを実装していないので、この型には `set`、`toggle`、リストの文、`patch` がなく、`bind` にも渡せません。書き込んだ値は、式が読むフィールドが次に変わったときにブラウザが上書きするので、書き込みはコンパイルが通りません。",
            },
        ),
        show(Snippet::StateComputedSet),
        para(
            site,
            L {
                en: "The error notes say to write one of the signals the expression reads instead. In the other direction, `Page::computed` takes only a field with the mark, through the [`IsComputed`](api:hypersig/trait.IsComputed.html) trait, and its error names the missing attribute:",
                ja: "エラーの注記には、代わりに式が読むシグナルのどれかに書き込むよう示されます。逆に、`Page::computed` は [`IsComputed`](api:hypersig/trait.IsComputed.html) トレイトを通して印の付いたフィールドだけを受け取り、そのエラーは足りない属性を示します。",
            },
        ),
        show(Snippet::StateComputedNotMarked),
        heading(
            site,
            L {
                en: "Computed fields of a nested struct",
                ja: "入れ子の構造体にある計算フィールド",
            },
        ),
        para(
            site,
            L {
                en: "[`Sig::at`](api:hypersig/struct.Sig.html#method.at) takes only fields the browser may write, so a marked field of a nested struct is reached with [`Sig::at_computed`](api:hypersig/struct.Sig.html#method.at_computed), and only from the page root, where `Page::computed` can name it. A field of a list item cannot be computed: `Item::at` refuses a marked field, because `Page::computed` names one path from the page's struct.",
                ja: "[`Sig::at`](api:hypersig/struct.Sig.html#method.at) はブラウザが書き込めるフィールドしか受け取らないので、入れ子の構造体にある印の付いたフィールドには [`Sig::at_computed`](api:hypersig/struct.Sig.html#method.at_computed) でたどり着きます。これはページのルートからだけ使え、そこでは `Page::computed` がそのフィールドを指せます。リスト項目のフィールドは計算シグナルにできません。`Page::computed` が指すのはページの構造体からの 1 つのパスなので、`Item::at` は印の付いたフィールドを拒否します。",
            },
        ),
        show(Snippet::StateComputedNested),
        para(
            site,
            L {
                en: "Rendering the page panics when the page reads a field reached with `at_computed` and no `Page::computed` call names it.",
                ja: "`at_computed` でたどったフィールドをページが読むのに、どの `Page::computed` の呼び出しもそれを指していなければ、ページの描画が panic します。",
            },
        ),
        heading(
            site,
            L {
                en: "Functions that only read a signal",
                ja: "シグナルを読むだけの関数",
            },
        ),
        para(
            site,
            L {
                en: "A function that only reads a signal can take a plain field and a computed one alike, as `hypersig_ui::Chart` does with its rows. It takes a `Sig<S, T, R>` with `R:` [`FromPage`](api:hypersig/trait.FromPage.html) and calls [`read()`](api:hypersig/struct.Sig.html#method.read) on it, which gives a `Sig<S, T, Read>` with the same path. That type reads like the signal it came from, has no statement that writes it, and is refused by `Page::computed` and `Page::persist`:",
                ja: "シグナルを読むだけの関数は印のないフィールドと計算フィールドのどちらも受け取れます。`hypersig_ui::Chart` は行をこの方法で受け取ります。`R:` [`FromPage`](api:hypersig/trait.FromPage.html) の `Sig<S, T, R>` を受け取って [`read()`](api:hypersig/struct.Sig.html#method.read) を呼ぶと、同じパスを持つ `Sig<S, T, Read>` が得られます。この型でも値は元のシグナルと同じように読めますが、書き込む文はなく、`Page::computed` と `Page::persist` にも渡せません。",
            },
        ),
        show(Snippet::StateComputedRead),
        para(
            site,
            L {
                en: "A view of a computed field is checked as the field is, so rendering the page still panics when the page reads it and no `Page::computed` call names the field.",
                ja: "計算フィールドから作ったビューも元のフィールドと同じように検査されるので、ページがそれを読むのに、どの `Page::computed` の呼び出しもそのフィールドを指していなければ、ページの描画が panic します。",
            },
        ),
        heading(
            site,
            L {
                en: "A cart line",
                ja: "カートの 1 行",
            },
        ),
        para(
            site,
            L {
                en: "The total below is a computed signal over a field of a nested struct and the quantity, and the buttons change only the quantity:",
                ja: "下の合計は入れ子の構造体のフィールドと数量から作る計算シグナルで、ボタンは数量だけを変えます。",
            },
        ),
        demo::show(site, Demo::Cart),
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
                en: "[Signals kept across page loads](learn/persisted-signals/) keeps a field in the browser's storage for the next page.",
                ja: "[ページをまたいで保つシグナル](learn/persisted-signals/) ではフィールドをブラウザのストレージに保存して次のページに引き継ぎます。",
            },
        ),
    ])
}
