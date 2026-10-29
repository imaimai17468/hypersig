//! The page on signals kept in `localStorage` across page loads.

use crate::Site;
use crate::doc::{Doc, Learn};
use crate::lang::L;
use crate::learn::snippet::{Snippet, show};
use crate::prose::{Part, bullets, heading, lead, para, parts, title};

pub fn page(site: Site) -> Part {
    parts(vec![
        title(Doc::Learn(Learn::PersistedSignals).title(site)),
        lead(site, Doc::Learn(Learn::PersistedSignals).leads()),
        heading(
            site,
            L {
                en: "Keeping a choice for the next page",
                ja: "選んだものを次のページに残す",
            },
        ),
        para(
            site,
            L {
                en: "A page loads with the values `Page::new` was given, so a choice the visitor made in the browser, such as a color theme or a list's density, is gone on the next page. [`Page::persist`](api:hypersig/struct.Page.html#method.persist), `fn persist<T>(self, signal: Sig<S, T>) -> Self`, keeps one signal in the browser's `localStorage`. The browser stores the signal's value after each write, and the next page that persists the same path starts from the stored value in place of the one `Page::new` was given.",
                ja: "ページは `Page::new` に渡した値で読み込まれるので、配色やリストの表示密度のように訪問者がブラウザで選んだものは、次のページでは元に戻ります。[`Page::persist`](api:hypersig/struct.Page.html#method.persist)（`fn persist<T>(self, signal: Sig<S, T>) -> Self`）は、1 つのシグナルをブラウザの `localStorage` に保存します。ブラウザはシグナルに書き込むたびにその値を保存し、同じパスを保存する次のページは、`Page::new` に渡した値の代わりに保存された値から始めます。",
            },
        ),
        show(Snippet::Persist),
        heading(
            site,
            L {
                en: "Before the stored value is drawn",
                ja: "保存された値を描画するまで",
            },
        ),
        para(
            site,
            L {
                en: "The server renders the first HTML from the initial value, because it never sees the stored one. Where the stored value differs from the initial one, the page stays hidden until the runtime has drawn the stored value, so the visitor does not see the initial value drawn first. If the runtime fails to start, the page shows after a short wait.",
                ja: "サーバーは保存された値を受け取らないので、最初の HTML は初期値で描画します。保存された値が初期値と異なる場合は、ランタイムが保存された値を描画し終えるまでページが隠れたままになります。そのため、訪問者が初期値で描画された画面を先に目にすることはありません。ランタイムが起動しない場合は、少し待つとページが表示されます。",
            },
        ),
        heading(
            site,
            L {
                en: "Pages that share a stored value",
                ja: "保存された値を共有するページ",
            },
        ),
        para(
            site,
            L {
                en: "Every page of one application under one base that persists the same path shares one stored value. Two pages that persist the same path with different types share it too, and nothing checks that. Give a persisted field a name that no other page uses for another type.",
                ja: "同じベースの下にある 1 つのアプリケーションのページは、同じパスを保存するかぎり 1 つの保存された値を共有します。同じパスを別の型で保存するページ同士も同じ値を共有し、そのことは検査されません。保存するフィールドには、ほかのページが別の型に使っていない名前を付けてください。",
            },
        ),
        heading(
            site,
            L {
                en: "Signals that cannot be persisted",
                ja: "保存できないシグナル",
            },
        ),
        para(
            site,
            L {
                en: "`persist` takes a `Sig<S, T>`, a field reached from the page root. A computed signal would be recomputed on load, a read-only view names a field the page does not write, and a field of a list item has no fixed path, so each of them is a compile error:",
                ja: "`persist` が受け取るのは、ページのルートからたどったフィールドである `Sig<S, T>` です。計算シグナルは読み込み時に計算し直され、読み取り専用のビューはページが書き込まないフィールドを指し、リスト項目のフィールドには固定したパスがありません。そのため、どれを渡してもコンパイルエラーになります。",
            },
        ),
        show(Snippet::StatePersistComputed),
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
                    en: "The runtime checks only that a stored value is the same kind of value as the initial one: text, a number, a boolean, a list, a struct, or `None`. A value of another kind is skipped. An enum variant that an older build wrote and that was since removed is still a string, so it reaches the page as stored.",
                    ja: "ランタイムが確かめるのは、保存された値が初期値と同じ種類の値（テキスト、数値、真偽値、リスト、構造体、`None`）かどうかだけです。種類の違う値は使いません。以前のビルドが書き込み、その後削除された列挙型のバリアントは文字列のままなので、保存されたとおりにページに入ります。",
                },
                L {
                    en: "Every script on the page's origin can read and write `localStorage`, and the value stays after the visitor signs out, so persist a preference and leave a token or personal data out.",
                    ja: "`localStorage` は、ページと同じオリジンのスクリプトならどれでも読み書きでき、訪問者がサインアウトしても値が保持されます。保存するのは表示の設定のようなものにして、トークンや個人の情報は保存しないでください。",
                },
                L {
                    en: "An action sees a persisted value only when its call passes the signal as an argument, and then gets the value the page holds, which is the stored one once the runtime has started.",
                    ja: "アクションが保存された値を受け取るのは、呼び出しがそのシグナルを引数として渡したときだけです。そのとき受け取るのはページが持つ値で、ランタイムの起動後は保存された値です。",
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
                en: "[Lists](learn/lists/) renders a `Vec` field and changes it in the browser. The theme picker in [Theming](components/theming/#light-and-dark) persists the visitor's color scheme.",
                ja: "[リスト](learn/lists/) では `Vec` のフィールドを描画し、ブラウザで変更します。[テーマ](components/theming/#light-and-dark) の配色の切り替えは、訪問者が選んだ配色を保存します。",
            },
        ),
    ])
}
