//! The page on signal structs inside signal structs.

use crate::Site;
use crate::doc::{Doc, Learn};
use crate::lang::L;
use crate::learn::snippet::{Snippet, show};
use crate::prose::{Part, bullets, heading, lead, para, parts, title};

pub fn page(site: Site) -> Part {
    parts(vec![
        title(Doc::Learn(Learn::NestedSignals).title(site)),
        lead(site, Doc::Learn(Learn::NestedSignals).leads()),
        heading(
            site,
            L {
                en: "Fields inside fields",
                ja: "フィールドの中のフィールド",
            },
        ),
        para(
            site,
            L {
                en: "A field may itself be a `Signals` struct. [`Sig::at`](api:hypersig/struct.Sig.html#method.at), `fn at<U>(self, field: Sig<T, U>) -> Sig<S, U, R>`, reaches into it, and the result is a `Sig` like any other. Its `path()` joins the field names with dots.",
                ja: "フィールド自体を `Signals` の構造体にもできます。その中へは [`Sig::at`](api:hypersig/struct.Sig.html#method.at)（`fn at<U>(self, field: Sig<T, U>) -> Sig<S, U, R>`）でたどり、結果はほかと同じ `Sig` になります。その `path()` はフィールド名をドットでつないだものです。",
            },
        ),
        show(Snippet::Nested),
        para(
            site,
            L {
                en: "`at` takes a `Sig<T, U>`, where `T` is the type the outer signal holds, so a field of another struct is a compile error:",
                ja: "`at` が受け取るのは `Sig<T, U>` で、`T` は外側のシグナルが持つ型です。そのため、別の構造体のフィールドを渡すとコンパイルエラーになります。",
            },
        ),
        show(Snippet::StateNestedWrongStruct),
        para(
            site,
            L {
                en: "A path is at most [`MAX_DEPTH`](api:hypersig/constant.MAX_DEPTH.html), 8, fields deep. The types do not count the depth, so the `at` call that would make a ninth field panics with `a signal path is at most 8 fields deep`.",
                ja: "パスの深さは [`MAX_DEPTH`](api:hypersig/constant.MAX_DEPTH.html)、つまりフィールド 8 つまでです。深さは型では数えないので、9 つ目のフィールドを足す `at` の呼び出しが `a signal path is at most 8 fields deep` で panic します。",
            },
        ),
        heading(
            site,
            L {
                en: "A whole struct as one value",
                ja: "構造体を 1 つの値として扱う",
            },
        ),
        para(
            site,
            L {
                en: "A signal that holds a struct is a value of its own. `set` copies another struct of the same type into it, and `with(field, value)` makes a copy with one field replaced. On an expression that evaluates to a struct, `at(field)` reads one field, as `Sig::at` does on a signal.",
                ja: "構造体を持つシグナルは、それ自体が 1 つの値です。`set` は同じ型の別の構造体をそこに複製し、`with(field, value)` は 1 つのフィールドを置き換えた複製を作ります。構造体に評価される式ではシグナルの `Sig::at` と同じように、`at(field)` で 1 つのフィールドを読めます。",
            },
        ),
        show(Snippet::StateStructValues),
        heading(
            site,
            L {
                en: "Optional fields",
                ja: "省略できるフィールド",
            },
        ),
        para(
            site,
            L {
                en: "The signal of an `Option` field has `is_some` and `is_none`, which give a `bool`, and `unwrap_or(default)`, which gives the held value or `default`. `some()` wraps any value in `Some`, and `&None` as a literal sets the field back to `None`.",
                ja: "`Option` のフィールドのシグナルには `bool` を返す `is_some` と `is_none`、値か `default` を返す `unwrap_or(default)` があります。`some()` は任意の値を `Some` で包み、リテラルの `&None` はフィールドを `None` に戻します。",
            },
        ),
        show(Snippet::StateOptions),
        heading(
            site,
            L {
                en: "The state of components",
                ja: "部品の状態",
            },
        ),
        para(
            site,
            L {
                en: "A component of `hypersig_ui` that keeps state of its own, such as `Combobox` or `DataTable`, takes it as a `Sig` of its state struct, so that state is a field of the page's struct too. Gather those fields into one struct that derives `Default`, and the page's struct gains one field, set with `Default::default()`. Adding a component then adds a field to that struct and leaves the page's initializer as it is. `ComboboxState` and `TableState`, the state structs of `hypersig_ui`, implement `Default`, which gives an empty, closed combobox and an unfiltered, unsorted table on its first page.",
                ja: "`Combobox` や `DataTable` のように自分の状態を持つ `hypersig_ui` の部品は、その状態を状態用の構造体の `Sig` で受け取るので、その状態もページの構造体のフィールドになります。こうしたフィールドを `Default` を実装する 1 つの構造体にまとめると、ページの構造体に増えるフィールドは 1 つで、初期化は `Default::default()` で済みます。部品を追加するときはその構造体にフィールドを追加するだけで、ページの初期化は変わりません。`hypersig_ui` の状態用の構造体である `ComboboxState` と `TableState` は `Default` を実装しています。その値は空で閉じたコンボボックスと、絞り込みも並べ替えもない 1 ページ目の表です。",
            },
        ),
        show(Snippet::ComponentState),
        heading(
            site,
            L {
                en: "Other places a nested struct appears",
                ja: "入れ子の構造体が現れるほかの場所",
            },
        ),
        bullets(
            site,
            &[
                L {
                    en: "A `Vec` of values or of structs is a list, which [Lists](learn/lists/) covers.",
                    ja: "値や構造体の `Vec` はリストで、[リスト](learn/lists/) で説明します。",
                },
                L {
                    en: "A field marked `#[signals(computed)]` inside a nested struct is reached with `at_computed` instead of `at`, as [Computed signals](learn/computed-signals/) shows.",
                    ja: "入れ子の構造体の中にある `#[signals(computed)]` 付きのフィールドには、`at` の代わりに `at_computed` でたどり着きます。[計算シグナル](learn/computed-signals/) で示します。",
                },
                L {
                    en: "A nested struct that implements `Part` renders its own piece of page with its paths under the field, which [Components](learn/components/) covers.",
                    ja: "`Part` を実装した入れ子の構造体は、パスをそのフィールドの下に置いて自分の部分を描画します。[コンポーネント](learn/components/) で説明します。",
                },
            ],
        ),
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
                en: "[Expressions](learn/expressions/) computes values from these fields, and [Statements](learn/statements/) changes them.",
                ja: "[式](learn/expressions/) ではこれらのフィールドから値を計算し、[文](learn/statements/) ではフィールドを変更します。",
            },
        ),
    ])
}
