//! The Components page on `skeleton`: a pulsing block that stands in for
//! content still loading.

use crate::Site;
use crate::code::{self, Lang};
use crate::demo::{self, Demo};
use crate::doc::{Component, Doc};
use crate::lang::L;
use crate::learn::snippet::{Snippet, show};
use crate::prose::{Part, heading, lead, para, parts};

pub fn page(site: Site, head: Part) -> Part {
    parts(vec![
        head,
        lead(site, Doc::Components(Component::Skeleton).leads()),
        demo::show(site, Demo::Skeleton),
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
                en: "A skeleton has `aria-hidden=\"true\"`, so a screen reader does not read it. Tell a screen reader that the content is loading somewhere else, such as with `aria-busy=\"true\"` on the region the skeleton fills, as the demo does, or with a [Spinner](components/spinner/) named \"Loading\".",
                ja: "スケルトンは `aria-hidden=\"true\"` を持つので、スクリーンリーダーは読み上げません。内容を読み込み中であることは、別の場所でスクリーンリーダーに伝えてください。デモのようにスケルトンを置いた領域に `aria-busy=\"true\"` を付けるか、「Loading」という名前の [スピナー](components/spinner/) を置きます。",
            },
        ),
        para(
            site,
            L {
                en: "The block pulses every two seconds. For a visitor who asks for reduced motion, it stays still. The skeleton takes no keys.",
                ja: "ブロックは 2 秒ごとに明滅します。動きを減らす設定の訪問者には、止まったまま表示されます。スケルトンはキー操作を受けません。",
            },
        ),
    ])
}

fn building(site: Site) -> Part {
    parts([
        heading(
            site,
            L {
                en: "Building a skeleton",
                ja: "スケルトンを作る",
            },
        ),
        para(
            site,
            L {
                en: "[`skeleton`](api:hypersig_ui/fn.skeleton.html) returns an empty block in `token::accent` with rounded corners. It has no size of its own, so give it one with `El::css`, close to the size of the content it stands in for:",
                ja: "[`skeleton`](api:hypersig_ui/fn.skeleton.html) は、`token::accent` の色の、角の丸い空のブロックを返します。ブロック自体には大きさがないので、`El::css` で、代わりに置く内容に近い大きさを指定します。",
            },
        ),
        code::block(Lang::Rust, None, "pub fn skeleton<S>() -> El<S>;"),
        para(
            site,
            L {
                en: "Show the skeletons and the content with `El::show` on one signal, so that a patch on that signal swaps them.",
                ja: "スケルトンと内容は、1 つのシグナルに対する `El::show` で出し分けます。そうすると、そのシグナルへの差分で入れ替わります。",
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
                en: "While `loading` is true, the example at the end of this page renders this HTML, with the classes left out. The paragraph that says the report is ready is in the HTML too, hidden until `loading` turns false, and is left out here:",
                ja: "`loading` が true の間、このページの最後の例は次の HTML を出力します（クラスは省いています）。レポートの準備ができたことを伝える段落も HTML にありますが、`loading` が false になるまで隠れているので、ここでは省いています。",
            },
        ),
        code::block(
            Lang::Html,
            None,
            r#"<div aria-busy="true">
  <div>
    <div aria-hidden="true"></div>
    <div aria-hidden="true"></div>
  </div>
</div>"#,
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
                en: "`skeleton` takes no arguments, and its types stop no mistake. Its size goes through `El::css` like any element's, where a length without a unit is a compile error, as [Lengths and units](style/lengths/) shows.",
                ja: "`skeleton` は引数を取らず、型が防ぐ誤りはありません。大きさはほかの要素と同じく `El::css` で指定し、単位のない長さはそこでコンパイルエラーになります。例は [長さと単位](style/lengths/) にあります。",
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
                en: "Nothing checks that a skeleton has a size. Without a height it is 0 pixels tall and does not show. Nothing checks either that something tells a screen reader the content is loading, so a page whose only sign of loading is skeletons says nothing to a visitor who cannot see them.",
                ja: "スケルトンに大きさがあるかどうかは検査されません。高さを指定しないと高さが 0 になり、表示されません。内容を読み込み中であることをスクリーンリーダーに伝えているかどうかも検査されないので、読み込み中の印がスケルトンだけのページは、スケルトンが見えない訪問者には何も伝えません。",
            },
        ),
    ])
}

fn example(site: Site) -> Part {
    parts([
        heading(
            site,
            L {
                en: "Example: a report that is loading",
                ja: "例: 読み込み中のレポート",
            },
        ),
        para(
            site,
            L {
                en: "Two lines of skeleton show while `loading` is true, and the region's `aria-busy` follows the same signal:",
                ja: "`loading` が true の間は 2 行のスケルトンを表示し、領域の `aria-busy` も同じシグナルに従います。",
            },
        ),
        show(Snippet::UiSkeleton),
        para(
            site,
            L {
                en: "Work whose progress is known shows a [Progress](components/progress/) bar, and work whose length is not known a [Spinner](components/spinner/).",
                ja: "進み具合がわかる処理には [プログレス](components/progress/) のバーを、どれだけかかるかわからない処理には [スピナー](components/spinner/) を表示します。",
            },
        ),
    ])
}
