//! The page on running a JavaScript library, such as a map or an editor,
//! inside a page.

use super::snippet::{Snippet, show};
use crate::Site;
use crate::code::{self, Lang};
use crate::demo::{self, Demo};
use crate::doc::{Doc, Learn};
use crate::lang::L;
use crate::prose::{Part, heading, lead, para, parts, title};

pub fn javascript(site: Site) -> Part {
    parts([
        title(Doc::Learn(Learn::JavaScript).title(site)),
        lead(site, Doc::Learn(Learn::JavaScript).leads()),
        heading(
            site,
            L {
                en: "Wrap the library in a custom element",
                ja: "ライブラリをカスタム要素で包む",
            },
        ),
        para(
            site,
            L {
                en: "Three methods connect a page to a custom element. `Page::script` loads the script that defines the element, `bind_attr` keeps an attribute of the element equal to a signal, and `El::on_custom` runs a statement on an event the element dispatches, with the event's `detail` as a typed expression. Declare the event once as a [`CustomEvent`](api:hypersig/html/struct.CustomEvent.html) whose type parameter is the type of its `detail`.",
                ja: "ページとカスタム要素は 3 つのメソッドでつながります。`Page::script` は要素を定義するスクリプトを読み込み、`bind_attr` は要素の属性をシグナルの値に合わせ、`El::on_custom` は要素が投げたイベントで文を実行します。イベントの `detail` は型付きの式として文に渡ります。イベントは、型引数を `detail` の型にした [`CustomEvent`](api:hypersig/html/struct.CustomEvent.html) として 1 回宣言します。",
            },
        ),
        para(
            site,
            L {
                en: "The rating below is a custom element written in 34 lines of JavaScript. A click on a star dispatches `rate`, the statement puts its `detail` into the page's signal, and the signal goes back into the element's `value` attribute. The clear button changes only the signal, and the stars follow.",
                ja: "下の評価は、34 行の JavaScript で書いたカスタム要素です。星をクリックすると要素が `rate` を投げ、文がその `detail` をページのシグナルに入れ、シグナルが要素の `value` 属性に戻ります。クリアのボタンはシグナルだけを変え、星の表示がそれに合わせて変わります。",
            },
        ),
        demo::show(site, Demo::Rating),
        code::block(
            Lang::JavaScript,
            Some("star-rating.js"),
            demo::STAR_RATING_JS,
        ),
        heading(
            site,
            L {
                en: "What the compiler checks",
                ja: "コンパイラが検査する範囲",
            },
        ),
        para(
            site,
            L {
                en: "The event's name and the type of its `detail` describe the script, so nothing checks them against it. An event the script never dispatches runs nothing, and a field the `detail` lacks reads as `null`. What the page does with the `detail` is checked: putting it into a signal of another type is a compile error.",
                ja: "イベント名と `detail` の型はスクリプトの側の取り決めなので、スクリプトと突き合わせる検査はありません。スクリプトが投げないイベントでは何も実行されず、`detail` にないフィールドを読むと `null` になります。検査するのは、ページが `detail` をどう使うかです。型の違うシグナルに入れると、コンパイルエラーになります。",
            },
        ),
        heading(
            site,
            L {
                en: "A library that draws into an element",
                ja: "要素の中に描くライブラリ",
            },
        ),
        para(
            site,
            L {
                en: "A library that is not a custom element, such as Leaflet, draws into the element it is given and adds children, classes, and inline styles to it. A server patch that reaches the element would set it back to what the server rendered. Mark the element with `El::foreign`, and a patch keeps it as it is while its bindings keep running. Among its siblings a patch finds it by its node ref's name, so an element added before it does not make the map start over.",
                ja: "Leaflet のようにカスタム要素ではないライブラリは、渡された要素の中に描き、子要素やクラス、インラインスタイルを追加します。サーバーのパッチがその要素に届くと、サーバーが描いた形に戻してしまいます。`El::foreign` を付けると、パッチはその要素をそのまま残し、要素の束縛は動き続けます。兄弟要素の中では node ref の名前でその要素を見つけるので、前に要素が増えても地図は作り直されません。",
            },
        ),
        show(Snippet::Foreign),
    ])
}
