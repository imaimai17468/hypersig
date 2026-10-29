//! The page on `DateField`, the `Dated` trait, and `CalendarDate`.

use crate::Site;
use crate::code::{self, Lang};
use crate::demo::{self, Demo};
use crate::doc::{Component, Doc};
use crate::lang::L;
use crate::learn::snippet::{Snippet, show};
use crate::prose::{Part, bullets, heading, lead, para, parts};

pub fn page(site: Site, head: Part) -> Part {
    parts(vec![
        head,
        lead(site, Doc::Components(Component::DateField).leads()),
        demo::show(site, Demo::TextField),
        accessibility(site),
        builder(site),
        dated(site),
        unchecked(site),
        example(site),
        next(site),
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
                en: "It is an `<input type=\"date\">` with the label, `aria-describedby`, `aria-invalid`, and live error of a [text field](components/text-field/). The browser draws the field and its date picker in the visitor's own date format and language, and gives them their keys, which differ between browsers: in most, Tab or the arrow keys move between the day, month, and year, Up and Down change the part with focus, and a button or Alt+Down opens the calendar. The Arrival field in the demo above is one.",
                ja: "[テキストフィールド](components/text-field/)と同じラベル、`aria-describedby`、`aria-invalid`、ライブ領域のエラーを持つ `<input type=\"date\">` です。入力欄と日付ピッカーは、訪問者の日付の書式と言語でブラウザが描き、キー操作もブラウザが与えます。キー操作はブラウザによって異なり、多くのブラウザでは Tab や矢印キーで日、月、年の間を移り、↑ と ↓ でフォーカスのある部分を変え、ボタンや Alt+↓ でカレンダーを開きます。上のデモの「到着日」の入力欄がこれです。",
            },
        ),
    ])
}

fn builder(site: Site) -> Part {
    parts([
        heading(
            site,
            L {
                en: "`DateField`",
                ja: "`DateField`",
            },
        ),
        para(
            site,
            L {
                en: "[`DateField`](api:hypersig_ui/struct.DateField.html) is a builder like `TextField`. Its signal holds a [`CalendarDate`](api:hypersig/struct.CalendarDate.html), a day with no time or time zone, and `min` and `max` take a `CalendarDate`, so a bound cannot be text:",
                ja: "[`DateField`](api:hypersig_ui/struct.DateField.html) は `TextField` と同じくビルダーです。シグナルは時刻もタイムゾーンも持たない日である [`CalendarDate`](api:hypersig/struct.CalendarDate.html) を持ち、`min` と `max` も `CalendarDate` を受け取るので、範囲を文字列で書くことはできません。",
            },
        ),
        code::block(
            Lang::Rust,
            None,
            "impl<S, T: Dated> DateField<S, T> {\n    \
             pub fn new(label: impl Into<String>, value: Sig<S, T>) -> Self\n    \
             pub fn min(self, min: CalendarDate) -> Self\n    \
             pub fn max(self, max: CalendarDate) -> Self\n    \
             pub fn description(self, text: impl Into<String>) -> Self\n    \
             pub fn error(self, message: impl IntoExpr<S, String>) -> Self\n    \
             pub fn required(self) -> Self\n    \
             pub fn disabled(self, when: impl IntoExpr<S, bool>) -> Self\n    \
             pub fn render(self) -> El<S>\n}",
        ),
        para(
            site,
            L {
                en: "The browser reads and writes the value as `YYYY-MM-DD`, which is how `CalendarDate` displays. With `min(CalendarDate::new(2026, 1, 1))` and a signal holding April 3, 2026, the field renders as follows, without the ids and the `class`:",
                ja: "ブラウザは値を `YYYY-MM-DD` として読み書きします。これは `CalendarDate` の表示と同じ形です。`min(CalendarDate::new(2026, 1, 1))` を指定し、シグナルが 2026 年 4 月 3 日を持つとき、入力欄は id と `class` を除くと次のように描画されます。",
            },
        ),
        code::block(
            Lang::Html,
            None,
            "<label for=\"…\">Departure</label>\n\
             <input id=\"…\" type=\"date\" min=\"2026-01-01\" value=\"2026-04-03\">",
        ),
    ])
}

fn dated(site: Site) -> Part {
    parts([
        heading(
            site,
            L {
                en: "`Dated`",
                ja: "`Dated`",
            },
        ),
        para(
            site,
            L {
                en: "[`Dated`](api:hypersig_ui/trait.Dated.html) is the bound on `T`, sealed and implemented for `CalendarDate` and `Option<CalendarDate>`. An `Option` field starts empty for `None`, and emptying it sets `None`. A plain `CalendarDate` always holds a date, and emptying the field keeps the date it had:",
                ja: "[`Dated`](api:hypersig_ui/trait.Dated.html) は `T` の境界で、封印されたトレイトとして `CalendarDate` と `Option<CalendarDate>` に実装されています。`Option` の入力欄は `None` なら空で始まり、空にすると `None` になります。`CalendarDate` は常に日付を持ち、入力欄を空にしても元の日付のままです。",
            },
        ),
        code::block(
            Lang::Rust,
            None,
            "pub trait Dated: Bindable + Sealed {}\n\n\
             impl Dated for CalendarDate {}\nimpl Dated for Option<CalendarDate> {}",
        ),
        para(
            site,
            L {
                en: "A date kept as text, such as a `String` signal, fails to compile with E0277 and the message `a date field cannot be bound to a String signal`:",
                ja: "`String` のシグナルのように日付を文字列で持つと、E0277 でコンパイルできず、`a date field cannot be bound to a String signal` というメッセージが出ます。",
            },
        ),
        show(Snippet::UiDateFieldText),
    ])
}

fn unchecked(site: Site) -> Part {
    parts([
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
                    en: "`min` panics when it is after a maximum set before, and `max` when it is before a minimum set before. The panic comes where the field is built.",
                    ja: "`min` は先に設定した最大値より後のとき、`max` は先に設定した最小値より前のときに panic します。panic は入力欄を作ったところで起きます。",
                },
                L {
                    en: "The browser's picker offers no day outside `min` and `max`, but a day typed outside them still goes to the signal. A plain `<form>` refuses to submit it, and a form from `Sig::form` does not, since it has `novalidate`. Check the range again in an error message or a form rule, and on the server.",
                    ja: "ブラウザのピッカーでは `min` と `max` の範囲外の日を選べませんが、範囲外の日を入力すると、その日はシグナルに入ります。通常の `<form>` はそれを送信しませんが、`Sig::form` のフォームは `novalidate` を持つので送信します。範囲はエラーメッセージかフォームのルールと、サーバーでも検査します。",
                },
                L {
                    en: "The field writes a day to the signal only when it is a real date with a year from 1 to 9999. While the visitor has filled only part of it, the signal keeps its last value.",
                    ja: "入力欄がシグナルに日を書き込むのは、年が 1 から 9999 の実在する日付のときだけです。訪問者が一部しか入力していない間、シグナルは最後の値のままです。",
                },
            ],
        ),
    ])
}

fn example(site: Site) -> Part {
    parts([
        heading(
            site,
            L {
                en: "Example: arrival and departure",
                ja: "例: 到着日と出発日",
            },
        ),
        show(Snippet::UiDateField),
    ])
}

fn next(site: Site) -> Part {
    parts([
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
                en: "[Fields bound to a form](components/form-fields/) has `DateField::in_form`, which takes its message from a form's rules. [Number field](components/number-field/) takes a number in the same frame.",
                ja: "フォームのルールからメッセージを取る `DateField::in_form` は、[フォームに結び付けた入力欄](components/form-fields/)で説明しています。同じ枠組みで数値を受け取るのは[数値フィールド](components/number-field/)です。",
            },
        ),
    ])
}
