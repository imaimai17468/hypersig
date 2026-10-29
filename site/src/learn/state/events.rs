//! The page on running statements on DOM events.

use crate::Site;
use crate::doc::{Doc, Learn};
use crate::lang::L;
use crate::learn::snippet::{Snippet, show};
use crate::prose::{Part, bullets, heading, lead, para, parts, subheading, table_of, title};

pub fn page(site: Site) -> Part {
    parts(vec![
        title(Doc::Learn(Learn::Events).title(site)),
        lead(site, Doc::Learn(Learn::Events).leads()),
        heading(
            site,
            L {
                en: "Running a statement on an event",
                ja: "イベントで文を実行する",
            },
        ),
        para(
            site,
            L {
                en: "[`El::on`](api:hypersig/html/struct.El.html#method.on), `fn on(self, event: Event, stmt: impl IntoStmt<S>) -> Self`, runs `stmt` each time `event` reaches the element. Each [`Event`](api:hypersig/html/enum.Event.html) stands for a DOM event, and the common ones have a method of their own:",
                ja: "[`El::on`](api:hypersig/html/struct.El.html#method.on)（`fn on(self, event: Event, stmt: impl IntoStmt<S>) -> Self`）は、`event` が要素に届くたびに `stmt` を実行します。[`Event`](api:hypersig/html/enum.Event.html) のバリアントはそれぞれ DOM のイベントを表し、よく使うものには専用のメソッドがあります。",
            },
        ),
        table_of(
            site,
            L {
                en: &["`Event`", "DOM event", "Method"],
                ja: &["`Event`", "DOM のイベント", "メソッド"],
            },
            L {
                en: &[
                    &["`Click`", "`click`", "`on_click`"],
                    &["`DoubleClick`", "`dblclick`", "`on_double_click`"],
                    &["`Input`", "`input`", "`on_input`"],
                    &["`Change`", "`change`", "`on_change`"],
                    &["`Submit`", "`submit`", "`on_submit`"],
                    &["`KeyDown`", "`keydown`, any key", "`on_key` for one key"],
                    &["`Focus`", "`focusin`", "none"],
                    &["`Blur`", "`focusout`", "`on_blur`"],
                    &["`MouseEnter`", "`mouseenter`", "none"],
                    &["`MouseLeave`", "`mouseleave`", "none"],
                    &["`Load`", "the element enters the document", "`on_load`"],
                    &[
                        "`Visible`",
                        "the element enters the viewport",
                        "`on_visible`",
                    ],
                ],
                ja: &[
                    &["`Click`", "`click`", "`on_click`"],
                    &["`DoubleClick`", "`dblclick`", "`on_double_click`"],
                    &["`Input`", "`input`", "`on_input`"],
                    &["`Change`", "`change`", "`on_change`"],
                    &["`Submit`", "`submit`", "`on_submit`"],
                    &[
                        "`KeyDown`",
                        "`keydown`（すべてのキー）",
                        "1 つのキーには `on_key`",
                    ],
                    &["`Focus`", "`focusin`", "なし"],
                    &["`Blur`", "`focusout`", "`on_blur`"],
                    &["`MouseEnter`", "`mouseenter`", "なし"],
                    &["`MouseLeave`", "`mouseleave`", "なし"],
                    &["`Load`", "要素が文書に入る", "`on_load`"],
                    &["`Visible`", "要素がビューポートに入る", "`on_visible`"],
                ],
            },
        ),
        para(
            site,
            L {
                en: "Each method below is `on` with its event, so `button().on_click(stmt)` and `button().on(Event::Click, stmt)` do the same.",
                ja: "以下のメソッドはどれも、それぞれのイベントを指定した `on` です。そのため `button().on_click(stmt)` と `button().on(Event::Click, stmt)` は同じ動作になります。",
            },
        ),
        heading(
            site,
            L {
                en: "Clicks",
                ja: "クリック",
            },
        ),
        para(
            site,
            L {
                en: "[`on_click`](api:hypersig/html/struct.El.html#method.on_click) and [`on_double_click`](api:hypersig/html/struct.El.html#method.on_double_click) take the statement as `impl IntoStmt<S>` and return the element:",
                ja: "[`on_click`](api:hypersig/html/struct.El.html#method.on_click) と [`on_double_click`](api:hypersig/html/struct.El.html#method.on_double_click) は文を `impl IntoStmt<S>` として受け取り、要素を返します。",
            },
        ),
        show(Snippet::StateEventsClick),
        heading(
            site,
            L {
                en: "Typing",
                ja: "入力",
            },
        ),
        para(
            site,
            L {
                en: "[`on_input`](api:hypersig/html/struct.El.html#method.on_input) runs on every edit of a text field. [`on_change`](api:hypersig/html/struct.El.html#method.on_change) runs on the DOM's `change`, which a text field fires when focus leaves it after its text changed, and a checkbox or a `<select>` fires on each choice. A field bound with `bind` already writes its signal, so these are for work beyond keeping the signal equal to the field:",
                ja: "[`on_input`](api:hypersig/html/struct.El.html#method.on_input) はテキスト欄が編集されるたびに実行されます。[`on_change`](api:hypersig/html/struct.El.html#method.on_change) は DOM の `change` で実行されます。テキスト欄は文字が変わったあとフォーカスが離れたときに、チェックボックスや `<select>` は選ぶたびに `change` を発火します。`bind` で束縛した欄はすでにシグナルに書き込むので、これらはシグナルを欄に合わせる以外の処理に使います。",
            },
        ),
        show(Snippet::StateEventsText),
        heading(
            site,
            L {
                en: "Submitting a form",
                ja: "フォームの送信",
            },
        ),
        para(
            site,
            L {
                en: "[`on_submit`](api:hypersig/html/struct.El.html#method.on_submit) runs on a `<form>`'s `submit`. The runtime calls `preventDefault` first, so the browser does not load another page, and the statement decides what happens:",
                ja: "[`on_submit`](api:hypersig/html/struct.El.html#method.on_submit) は `<form>` の `submit` で実行されます。ランタイムが先に `preventDefault` を呼ぶのでブラウザは別のページを読み込まず、何をするかは文が決めます。",
            },
        ),
        show(Snippet::StateEventsSubmit),
        para(
            site,
            L {
                en: "A form that the server validates and answers is a [`Form`](api:hypersig/struct.Form.html), which [Forms](learn/forms/) covers.",
                ja: "サーバーが検証して応答するフォームには [`Form`](api:hypersig/struct.Form.html) を使います。[フォーム](learn/forms/) で説明します。",
            },
        ),
        heading(
            site,
            L {
                en: "Focus",
                ja: "フォーカス",
            },
        ),
        para(
            site,
            L {
                en: "`Event::Focus` listens for `focusin`, which runs when focus enters the element or one inside it from outside. [`on_blur`](api:hypersig/html/struct.El.html#method.on_blur) listens for `focusout`, and runs when focus leaves the element and everything inside it. Focus moving between two elements inside runs neither, so a group of fields can hold one state while the visitor is anywhere in it:",
                ja: "`Event::Focus` は `focusin` を待ち受け、外から要素かその内側の要素にフォーカスが入ったときに実行されます。[`on_blur`](api:hypersig/html/struct.El.html#method.on_blur) は `focusout` を待ち受け、要素とその内側すべてからフォーカスが離れたときに実行されます。内側の 2 つの要素の間でフォーカスが移っても、どちらも実行されません。そのため、入力欄のまとまりは、訪問者がその中のどこにいても 1 つの状態を保てます。",
            },
        ),
        show(Snippet::StateEventsFocus),
        heading(
            site,
            L {
                en: "The pointer",
                ja: "ポインター",
            },
        ),
        para(
            site,
            L {
                en: "`Event::MouseEnter` and `Event::MouseLeave` run when the pointer enters and leaves the element. They have no method of their own, so they go through `on`:",
                ja: "`Event::MouseEnter` と `Event::MouseLeave` は、ポインターが要素に入ったときと出たときに実行されます。専用のメソッドはないので、`on` を使います。",
            },
        ),
        show(Snippet::StateEventsPointer),
        heading(
            site,
            L {
                en: "Entering the document and the viewport",
                ja: "文書とビューポートに入ったとき",
            },
        ),
        para(
            site,
            L {
                en: "[`on_load`](api:hypersig/html/struct.El.html#method.on_load) runs once per element: for an element of the first HTML when the runtime starts, and for one added later when it is added, as the element of a new list item is or one that [`when`](api:hypersig/html/fn.when.html) puts back. A stream that sends changes made elsewhere opens from here, as [Streams](learn/streams/) shows.",
                ja: "[`on_load`](api:hypersig/html/struct.El.html#method.on_load) は要素ごとに 1 回実行されます。最初の HTML にある要素ではランタイムが起動したとき、新しいリスト項目の要素や [`when`](api:hypersig/html/fn.when.html) が戻した要素のように後から追加される要素では、追加されたときです。ほかの場所で起きた変更を送るストリームはここから開きます。[ストリーム](learn/streams/) で示します。",
            },
        ),
        para(
            site,
            L {
                en: "[`on_visible`](api:hypersig/html/struct.El.html#method.on_visible) runs when the element is added in view, and again each time it enters the viewport after leaving it. An element hidden by `show` is out of view, so showing it again in view runs the statement. The browser decides with an `IntersectionObserver`, and a browser without one never runs it. [List patches](learn/list-patches/) uses it to load the next page of a list.",
                ja: "[`on_visible`](api:hypersig/html/struct.El.html#method.on_visible) は、要素が見える位置に追加されたときと、いったん外れてから再びビューポートに入るたびに実行されます。`show` で隠した要素は見えていない扱いなので、見える位置で再び表示すると文が実行されます。判定はブラウザの `IntersectionObserver` が行い、これがないブラウザでは実行されません。[リストの差分](learn/list-patches/) ではこれを使ってリストの続きを読み込みます。",
            },
        ),
        show(Snippet::StateEventsLoad),
        heading(
            site,
            L {
                en: "Waiting for a pause",
                ja: "手が止まるまで待つ",
            },
        ),
        para(
            site,
            L {
                en: "[`on_debounced`](api:hypersig/html/struct.El.html#method.on_debounced), `fn on_debounced(self, event: Event, quiet: Duration, stmt: impl IntoStmt<S>) -> Self`, runs `stmt` once `event` has stopped firing for `quiet`. Each new event restarts the wait, so an action behind it runs once per pause rather than once per keystroke:",
                ja: "[`on_debounced`](api:hypersig/html/struct.El.html#method.on_debounced)（`fn on_debounced(self, event: Event, quiet: Duration, stmt: impl IntoStmt<S>) -> Self`）は、`event` が `quiet` の間起きなくなったときに `stmt` を実行します。イベントが起きるたびに待ち時間はやり直しになるので、その先のアクションはキーを押すたびではなく、手が止まるたびに 1 回実行されます。",
            },
        ),
        show(Snippet::StateEventsDebounce),
        para(
            site,
            L {
                en: "The browser's timer waits at most 2^31 - 1 milliseconds, about 24.8 days, and the types do not limit the `Duration`, so a longer `quiet` panics when the element is built.",
                ja: "ブラウザのタイマーが待てるのは最大 2^31 - 1 ミリ秒（約 24.8 日）で、`Duration` の長さは型では制限しません。そのため、これより長い `quiet` を渡すと要素を組み立てるときに panic します。",
            },
        ),
        heading(
            site,
            L {
                en: "Two statements on one event",
                ja: "1 つのイベントに 2 つの文",
            },
        ),
        para(
            site,
            L {
                en: "Calling `on`, or one of its methods, twice for the same event joins the two statements with `then`, so the second runs after the first. `on_debounced` joins the same way, so a field can do something on every keystroke and something else after a pause:",
                ja: "同じイベントに `on` やそのメソッドを 2 回呼ぶと、2 つの文は `then` でつながり、2 つ目は 1 つ目のあとに実行されます。`on_debounced` も同じようにつながるので、1 つの欄でキーを押すたびの処理と手が止まったあとの処理を両方行えます。",
            },
        ),
        show(Snippet::StateEventsTwice),
        heading(
            site,
            L {
                en: "Keys",
                ja: "キー",
            },
        ),
        para(
            site,
            L {
                en: "[`on_key`](api:hypersig/html/struct.El.html#method.on_key), `fn on_key(self, shortcut: impl Into<Shortcut>, stmt: impl IntoStmt<S>) -> Self`, runs `stmt` when the key is pressed while focus is in the element. The key is a [`Key`](api:hypersig/html/enum.Key.html), such as `Key::Enter` or `Key::Char('k')`, or a [`Shortcut`](api:hypersig/html/struct.Shortcut.html) that a `Key` method makes with a modifier, such as `Key::Char('k').primary()`. The runtime calls `preventDefault` for a key it matches.",
                ja: "[`on_key`](api:hypersig/html/struct.El.html#method.on_key)（`fn on_key(self, shortcut: impl Into<Shortcut>, stmt: impl IntoStmt<S>) -> Self`）は、フォーカスが要素の中にあるときにキーが押されると `stmt` を実行します。キーは `Key::Enter` や `Key::Char('k')` のような [`Key`](api:hypersig/html/enum.Key.html) か、`Key::Char('k').primary()` のように `Key` のメソッドが修飾キー付きで作る [`Shortcut`](api:hypersig/html/struct.Shortcut.html) です。キーが一致すると、ランタイムは `preventDefault` を呼びます。",
            },
        ),
        show(Snippet::StateEventsKeys),
        subheading(
            site,
            L {
                en: "How the modifiers match",
                ja: "修飾キーの一致のしかた",
            },
        ),
        bullets(
            site,
            &[
                L {
                    en: "A `Key` alone matches only while Ctrl, Alt, and Meta are all released. `ctrl`, `alt`, `meta`, and `shift` require that modifier, and `primary` requires Command on Apple platforms and Ctrl elsewhere.",
                    ja: "`Key` だけのときは、Ctrl、Alt、Meta がすべて離れているときだけ一致します。`ctrl`、`alt`、`meta`、`shift` はその修飾キーを求めます。`primary` は Apple のプラットフォームでは Command、それ以外では Ctrl を求めます。",
                },
                L {
                    en: "Ctrl, Alt, and Meta match exactly: `Key::Char('k').ctrl()` runs on Ctrl+K and not on Ctrl+Alt+K. Shift matches exactly for a named key, so `Key::Enter` leaves Shift+Enter to a `<textarea>`.",
                    ja: "Ctrl、Alt、Meta は完全に一致する必要があります。`Key::Char('k').ctrl()` は Ctrl+K で実行され、Ctrl+Alt+K では実行されません。名前の付いたキーでは Shift も完全に一致する必要があるので、`Key::Enter` は Shift+Enter を `<textarea>` に任せます。",
                },
                L {
                    en: "A `Key::Char` already says whether Shift was held, as `'K'` and `'?'` do on a US layout, so Shift is compared only where `shift` asks for it, and the character is then compared without regard to case.",
                    ja: "`Key::Char` の文字はUS 配列の `'K'` や `'?'` のように Shift を押したかどうかをすでに表しています。そのため Shift は `shift` で求めたときだけ比べ、そのとき文字は大文字と小文字を区別せずに比べます。",
                },
                L {
                    en: "On macOS, Option changes the character a key produces, so a `Key::Char` with `alt` does not match there.",
                    ja: "macOS では Option でキーが生む文字が変わるので、`alt` を付けた `Key::Char` は macOS では一致しません。",
                },
            ],
        ),
        subheading(
            site,
            L {
                en: "Keys a text field keeps",
                ja: "テキスト欄に任せるキー",
            },
        ),
        para(
            site,
            L {
                en: "A text field inside the element keeps the keys it edits text with, unless Ctrl, Alt, Meta, or `primary` is part of the shortcut. Those keys are a character, Space, Backspace, Delete, the left and right arrows, Home, and End, and in a `<textarea>` or an editable element also Enter and the up and down arrows. A handler on the field itself still takes them, as the Escape on the title field above does. Where several elements around the focus have handlers, the nearest one with a shortcut that matches runs, and the ones further out do not.",
                ja: "要素の中のテキスト欄は文字を編集するためのキーを自分で受け取ります。ただし、ショートカットに Ctrl、Alt、Meta、`primary` のどれかが含まれる場合は除きます。そのキーとは文字、Space、Backspace、Delete、左右の矢印、Home、End で、`<textarea>` や編集可能な要素ではさらに Enter と上下の矢印です。上のタイトル欄の Escape のように、欄そのものに付けたハンドラはこれらのキーも受け取ります。フォーカスを囲む複数の要素にハンドラがある場合は、一致するショートカットを持つ最も近い要素のものが実行され、その外側のものは実行されません。",
            },
        ),
        heading(
            site,
            L {
                en: "Keys anywhere on the page",
                ja: "ページ全体のキー",
            },
        ),
        para(
            site,
            L {
                en: "[`Page::on_key`](api:hypersig/struct.Page.html#method.on_key) takes the same arguments and applies to the whole page. It is the outermost handler, so it runs wherever focus is and while nothing has focus, unless a handler nearer the focus matched the key first:",
                ja: "[`Page::on_key`](api:hypersig/struct.Page.html#method.on_key) は同じ引数を受け取り、ページ全体に効きます。最も外側のハンドラなので、フォーカスがどこにあっても、どこにもフォーカスがないときにも実行されます。ただし、フォーカスに近いハンドラが先にキーに一致した場合は実行されません。",
            },
        ),
        show(Snippet::StateEventsPageKey),
        para(
            site,
            L {
                en: "Arrow-key navigation inside a list and moving focus with a key are built from these handlers and node refs, which [Keyboard patterns](learn/keyboard/) covers.",
                ja: "リストの中の矢印キーによる移動や、キーでフォーカスを動かす操作は、これらのハンドラとノード参照で組み立てます。[キーボード操作](learn/keyboard/) で説明します。",
            },
        ),
        heading(
            site,
            L {
                en: "A statement from another page",
                ja: "別のページの文",
            },
        ),
        para(
            site,
            L {
                en: "Every event method takes an `IntoStmt<S>` with the element's own `S`, so a statement over another page's signals is a compile error:",
                ja: "どのイベントのメソッドも要素と同じ `S` の `IntoStmt<S>` を受け取ります。そのため、別のページのシグナルを使う文はコンパイルエラーになります。",
            },
        ),
        show(Snippet::StateEventsWrongPage),
        heading(
            site,
            L {
                en: "What the types leave unchecked",
                ja: "型では検査しないこと",
            },
        ),
        para(
            site,
            L {
                en: "Nothing checks that the event can reach the element. A `<div>` with `on_submit` and no form inside it never runs the statement, and nothing reports it.",
                ja: "イベントが要素に届くかどうかは検査しません。中にフォームがない `<div>` に `on_submit` を付けても文は実行されず、そのことは報告もされません。",
            },
        ),
        heading(
            site,
            L {
                en: "Events from timers and scripts",
                ja: "タイマーとスクリプトのイベント",
            },
        ),
        para(
            site,
            L {
                en: "A statement can also run on a timer, after a signal changes, or on an event a script dispatches. [Timers and reactions](learn/timers/) covers `on_interval`, `on_timeout`, `effect`, and `on_signal_change`, and [JavaScript libraries](learn/javascript/) covers `on_custom` with a `CustomEvent`.",
                ja: "文は、タイマー、シグナルの変化、スクリプトが発行するイベントでも実行できます。`on_interval`、`on_timeout`、`effect`、`on_signal_change` は [タイマーと変化への反応](learn/timers/) で、`CustomEvent` を使う `on_custom` は [JavaScript のライブラリ](learn/javascript/) で説明します。",
            },
        ),
        heading(
            site,
            L {
                en: "A chat composer",
                ja: "チャットの入力欄",
            },
        ),
        para(
            site,
            L {
                en: "The composer sends on submit, clears on Escape, and shows that the visitor is typing from the moment the field gets focus until half a second after the last keystroke:",
                ja: "この入力欄は送信で送り、Escape で空にし、欄にフォーカスが入ってから最後のキー入力の 0.5 秒後まで入力中であることを示します。",
            },
        ),
        show(Snippet::Events),
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
                en: "[Binding form controls](learn/inputs/) keeps a form control and a signal equal in both directions.",
                ja: "[入力欄の束縛](learn/inputs/) ではフォームの入力欄とシグナルを双方向に一致させます。",
            },
        ),
    ])
}
