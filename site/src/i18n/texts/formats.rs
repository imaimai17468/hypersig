//! The i18n page on plural forms and the formats of numbers and dates:
//! `Expr::plural`, `NumberFormat`, `DateFormat`, and `IntlLocale`, chosen
//! per language, and the check `Page::render` makes against `Page::lang`.

use crate::Site;
use crate::code::{self, Lang};
use crate::doc::{Doc, I18nPage};
use crate::lang::L;
use crate::learn::snippet::{Snippet, show};
use crate::prose::{Part, bullets, heading, lead, para, parts, table_of};

pub fn page(site: Site, head: Part) -> Part {
    parts(vec![
        head,
        lead(site, Doc::I18n(I18nPage::Formats).leads()),
        intl_locale(site),
        plurals(site),
        numbers(site),
        dates(site),
        lang_check(site),
        unchecked(site),
        example(site),
    ])
}

fn intl_locale(site: Site) -> Part {
    parts([
        heading(
            site,
            L {
                en: "IntlLocale",
                ja: "IntlLocale",
            },
        ),
        code::block(
            Lang::Rust,
            None,
            "#[non_exhaustive]\npub enum IntlLocale {\n    En,\n    Ja,\n}",
        ),
        para(
            site,
            L {
                en: "[`IntlLocale`](api:hypersig/enum.IntlLocale.html) names a language the browser's `Intl` writes plurals and dates in, and for which `hypersig` holds the same rules on the server, so the first HTML already has the text the browser would write. It has English and Japanese. It is a type of `hypersig`, separate from the application's `Lang`, so each language of `Lang` maps to one by a text of its own:",
                ja: "[`IntlLocale`](api:hypersig/enum.IntlLocale.html) は、ブラウザの `Intl` が複数形と日付を書く言語を表します。`hypersig` はサーバー側にも同じ規則を持っているので、最初の HTML にはブラウザが書くのと同じ文が入っています。対応している言語は英語と日本語です。`IntlLocale` は `hypersig` の型で、アプリケーションの `Lang` とは別の型なので、`Lang` の各言語にはそれ専用の文で `IntlLocale` を対応させます。",
            },
        ),
        code::block(
            Lang::Rust,
            None,
            "const INTL: ByLang<IntlLocale> = ByLang { en: IntlLocale::En, ja: IntlLocale::Ja };\nlet intl = lang.pick(INTL);",
        ),
        para(
            site,
            L {
                en: "Passing a `Lang` where an `IntlLocale` is expected is E0308:",
                ja: "`IntlLocale` を受け取る場所に `Lang` を渡すと E0308 になります。",
            },
        ),
        show(Snippet::I18nWrongLocale),
        para(
            site,
            L {
                en: "An `IntlLocale` converts into a `LanguageTag`, so `Page::lang(IntlLocale::Ja)` writes `<html lang=\"ja\">`.",
                ja: "`IntlLocale` は `LanguageTag` に変換できるので、`Page::lang(IntlLocale::Ja)` は `<html lang=\"ja\">` を書きます。",
            },
        ),
    ])
}

fn plurals(site: Site) -> Part {
    parts([
        heading(
            site,
            L {
                en: "Plural forms",
                ja: "複数形",
            },
        ),
        code::block(
            Lang::Rust,
            None,
            "impl<S, T: Number> Expr<S, T> {\n    pub fn plural<U, L: IntoExpr<S, U>>(\n        &self,\n        locale: IntlLocale,\n        form: impl Fn(PluralCategory) -> L,\n    ) -> Expr<S, U>;\n}\n\npub enum PluralCategory { Zero, One, Two, Few, Many, Other }",
        ),
        para(
            site,
            L {
                en: "[`plural`](api:hypersig/struct.Expr.html#method.plural) is a method of a number expression or a number signal. It returns the form that `form` gives for the [`PluralCategory`](api:hypersig/enum.PluralCategory.html) of the number in `locale`, as `Intl.PluralRules` picks it. English uses `One` for 1 and -1 and `Other` for every other number. Japanese uses `Other` for every number, so a Japanese text needs no `plural` at all. `form` is called once per category, six times, so a `match` on the category names a form for each one, and the browser shows the form of the current number as the signal changes:",
                ja: "[`plural`](api:hypersig/struct.Expr.html#method.plural) は数値の式と数値のシグナルのメソッドです。`Intl.PluralRules` が選ぶのと同じく、`locale` でのその数の [`PluralCategory`](api:hypersig/enum.PluralCategory.html) に対して `form` が返す形を返します。英語は 1 と -1 に `One` を、それ以外の数に `Other` を使います。日本語はどの数にも `Other` を使うので、日本語の文には `plural` は要りません。`form` はカテゴリごとに 1 回、計 6 回呼ばれるので、カテゴリに対する `match` で各カテゴリの形を書きます。シグナルが変わると、ブラウザはその時点の数に合う形を表示します。",
            },
        ),
        show(Snippet::I18nPlural),
        para(
            site,
            L {
                en: "For a `count` of 1, the server writes `1 item` into the `<p>` of the first HTML, and the browser changes it to `2 items` when `count` becomes 2.",
                ja: "`count` が 1 のとき、サーバーは最初の HTML の `<p>` に `1 item` と書き、`count` が 2 になるとブラウザがそれを `2 items` に変えます。",
            },
        ),
    ])
}

fn numbers(site: Site) -> Part {
    parts([
        heading(
            site,
            L {
                en: "Numbers",
                ja: "数値",
            },
        ),
        code::block(
            Lang::Rust,
            None,
            "pub const fn new() -> NumberFormat;\npub const fn digits(self, digits: u8) -> Self;\npub const fn group(self, separator: &'static str) -> Self;\npub const fn point(self, point: &'static str) -> Self;\npub const fn prefix(self, prefix: &'static str) -> Self;\npub const fn suffix(self, suffix: &'static str) -> Self;",
        ),
        para(
            site,
            L {
                en: "[`NumberFormat`](api:hypersig/struct.NumberFormat.html) takes no `IntlLocale`. It writes a number with a fixed count of decimals, a group separator, a decimal point, and affixes, and the browser and the server write the same text whatever the visitor's locale. A format per language is a `ByLang<NumberFormat>`, and [`format`](api:hypersig/struct.Expr.html#method.format) takes the chosen one:",
                ja: "[`NumberFormat`](api:hypersig/struct.NumberFormat.html) は `IntlLocale` を受け取りません。小数点以下の桁数、桁区切り、小数点、前後に付ける文字を決めて数を書き、ブラウザとサーバーは訪問者のロケールにかかわらず同じ文を書きます。言語ごとの書式は `ByLang<NumberFormat>` にし、選んだものを [`format`](api:hypersig/struct.Expr.html#method.format) に渡します。",
            },
        ),
        code::block(
            Lang::Rust,
            None,
            "const PRICE: ByLang<NumberFormat> = ByLang {\n    en: NumberFormat::new().digits(2).prefix(\"€\"),\n    ja: NumberFormat::new().digits(2).suffix(\" ユーロ\"),\n};\n// 1536.5 is \"€1,536.50\" in English and \"1,536.50 ユーロ\" in Japanese.",
        ),
        para(
            site,
            L {
                en: "A language that writes `1.234,50 €`, such as German, sets `.group(\".\").point(\",\").suffix(\" €\")`. [`NumberFormat::write`](api:hypersig/struct.NumberFormat.html#method.write) returns the same text on the server, so a test can check it. [Expressions](learn/expressions/) covers the rounding.",
                ja: "ドイツ語のように `1.234,50 €` と書く言語では `.group(\".\").point(\",\").suffix(\" €\")` を設定します。[`NumberFormat::write`](api:hypersig/struct.NumberFormat.html#method.write) はサーバーで同じ文を返すので、テストで確かめられます。丸め方は [式](learn/expressions/) で説明しています。",
            },
        ),
    ])
}

fn dates(site: Site) -> Part {
    parts([
        heading(
            site,
            L {
                en: "Dates and times",
                ja: "日付と時刻",
            },
        ),
        code::block(
            Lang::Rust,
            None,
            "pub const fn date(locale: IntlLocale, style: DateStyle) -> DateFormat;\npub const fn time(locale: IntlLocale, style: TimeStyle) -> DateFormat;\npub const fn date_time(locale: IntlLocale, date: DateStyle, time: TimeStyle) -> DateFormat;\npub const fn zone(self, zone: TimeZone) -> Self;",
        ),
        para(
            site,
            L {
                en: "[`DateFormat`](api:hypersig/struct.DateFormat.html) takes an `IntlLocale`, so a format per language is a `ByLang<DateFormat>`, and [`format_date`](api:hypersig/struct.Expr.html#method.format_date) takes the chosen one. The browser writes the date with `Intl.DateTimeFormat`. For 5 January 2026:",
                ja: "[`DateFormat`](api:hypersig/struct.DateFormat.html) は `IntlLocale` を受け取るので、言語ごとの書式は `ByLang<DateFormat>` にし、選んだものを [`format_date`](api:hypersig/struct.Expr.html#method.format_date) に渡します。ブラウザは `Intl.DateTimeFormat` で日付を書きます。2026 年 1 月 5 日の場合は次のとおりです。",
            },
        ),
        table_of(
            site,
            L {
                en: &["DateStyle", "IntlLocale::En", "IntlLocale::Ja"],
                ja: &["DateStyle", "IntlLocale::En", "IntlLocale::Ja"],
            },
            L {
                en: DATE_STYLES,
                ja: DATE_STYLES,
            },
        ),
        para(
            site,
            L {
                en: "The time zone is UTC until `zone` sets another, such as `TimeZone::Offset(540)` for Japan. `TimeZone::Browser` uses the zone of the visitor's browser, which the server does not know, so the first HTML has no text there and the browser writes it when the page starts. [`relative_to`](api:hypersig/struct.Expr.html#method.relative_to), which writes `3 minutes ago`, takes an `IntlLocale` the same way.",
                ja: "タイムゾーンは `zone` でほかを設定するまで UTC です。日本なら `TimeZone::Offset(540)` を使います。`TimeZone::Browser` は訪問者のブラウザのタイムゾーンを使いますが、サーバーはそれを知らないので、最初の HTML のその位置には文がなく、ページの起動時にブラウザが書きます。`3 minutes ago` のような文を書く [`relative_to`](api:hypersig/struct.Expr.html#method.relative_to) も、同じように `IntlLocale` を受け取ります。",
            },
        ),
    ])
}

/// The rows of the `DateStyle` table, copied from the doc comment of
/// `hypersig::DateStyle`.
const DATE_STYLES: &[&[&str]] = &[
    &["`Full`", "Monday, January 5, 2026", "2026年1月5日月曜日"],
    &["`Long`", "January 5, 2026", "2026年1月5日"],
    &["`Medium`", "Jan 5, 2026", "2026/01/05"],
    &["`Short`", "1/5/26", "2026/01/05"],
];

fn lang_check(site: Site) -> Part {
    parts([
        heading(
            site,
            L {
                en: "The check against the page's language",
                ja: "ページの言語との照合",
            },
        ),
        para(
            site,
            L {
                en: "In a debug build, [`Page::render`](api:hypersig/struct.Page.html#method.render) panics where [`Page::lang`](api:hypersig/struct.Page.html#method.lang) names the language of an `IntlLocale` and every plural, date, and relative time on the page uses another `IntlLocale`. A Japanese page that writes its plurals in English is read by a screen reader in the wrong language, as happens when an `IntlLocale::En` sits in code both languages share:",
                ja: "デバッグビルドでは、[`Page::lang`](api:hypersig/struct.Page.html#method.lang) が `IntlLocale` のある言語を指し、ページのすべての複数形、日付、相対時間がそれとは別の `IntlLocale` を使っているとき、[`Page::render`](api:hypersig/struct.Page.html#method.render) がパニックします。複数形を英語で書いた日本語のページは、スクリーンリーダーに違う言語で読み上げられます。両方の言語で使うコードに `IntlLocale::En` が残っていると、こうなります。",
            },
        ),
        show(Snippet::I18nLangMismatch),
        bullets(
            site,
            &[
                L {
                    en: "Only the first subtag of the tag is compared, so `lang` set to `en-GB` matches `IntlLocale::En`.",
                    ja: "比べるのはタグの最初のサブタグだけなので、`en-GB` を設定した `lang` は `IntlLocale::En` と一致します。",
                },
                L {
                    en: "A page that also formats in its own language passes, so a Japanese page may show an English date beside its Japanese ones.",
                    ja: "自分の言語でも書式を使っているページは通ります。そのため、日本語のページに日本語の日付と並べて英語の日付を表示できます。",
                },
                L {
                    en: "A page in a language no `IntlLocale` writes, such as `fr`, passes, because it has to format in some other one.",
                    ja: "`fr` のように `IntlLocale` にない言語のページは通ります。ほかのどれかで書式を書くしかないからです。",
                },
                L {
                    en: "A page without `lang`, or without any plural, date, or relative time, is not checked. `NumberFormat` takes no locale and is never counted.",
                    ja: "`lang` のないページや、複数形、日付、相対時間を 1 つも使わないページは照合しません。`NumberFormat` はロケールを受け取らないので、照合の対象になりません。",
                },
                L {
                    en: "A release build skips the check and renders the page as written.",
                    ja: "リリースビルドは照合を省き、書いたとおりにページを描画します。",
                },
            ],
        ),
    ])
}

fn unchecked(site: Site) -> Part {
    parts([
        heading(
            site,
            L {
                en: "What stays unchecked",
                ja: "検査しないこと",
            },
        ),
        bullets(
            site,
            &[
                L {
                    en: "The words of each plural form. `PluralCategory::One => \" items\"` compiles and shows `1 items`.",
                    ja: "複数形の各形の言葉。`PluralCategory::One => \" items\"` と書いてもコンパイルは通り、`1 items` と表示されます。",
                },
                L {
                    en: "Which `IntlLocale` a `Lang` maps to, beyond the debug check above. `ja: IntlLocale::En` in the `ByLang<IntlLocale>` compiles, and in a release build the Japanese page shows English dates.",
                    ja: "`Lang` をどの `IntlLocale` に対応させるか。上のデバッグビルドでの照合を除けば検査しません。`ByLang<IntlLocale>` に `ja: IntlLocale::En` と書いてもコンパイルは通り、リリースビルドでは日本語のページに英語の日付が表示されます。",
                },
                L {
                    en: "The browser's copy of the Unicode CLDR data. A browser whose version differs from the server's rules in a space or a comma replaces the server's date text when the page starts.",
                    ja: "ブラウザが持つ Unicode CLDR のデータ。そのバージョンがサーバーの規則と空白やカンマ 1 つでも違うと、ページの起動時にブラウザがサーバーの書いた日付の文を書き換えます。",
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
                en: "Example",
                ja: "例",
            },
        ),
        para(
            site,
            L {
                en: "An inbox that shows the unread count, the credit left, and the date of the last update in the page's language:",
                ja: "未読の数、残りのクレジット、最終更新日をページの言語で表示する受信箱です。",
            },
        ),
        show(Snippet::I18nFormats),
        para(
            site,
            L {
                en: "The server writes this text into the three `<p>` elements of the first HTML, in English and in Japanese:",
                ja: "サーバーは最初の HTML の 3 つの `<p>` 要素に、英語と日本語でそれぞれ次の文を書きます。",
            },
        ),
        code::block(
            Lang::Html,
            None,
            "1 unread message\n€1,536.50\nJanuary 5, 2026\n\n3 件の未読メッセージ\n1,536.50 ユーロ\n2026年1月5日",
        ),
        para(
            site,
            L {
                en: "Next, [Pages in each language](i18n/pages/) covers putting the language in each page's route.",
                ja: "次の [言語ごとのページ](i18n/pages/) で各ページのルートに言語を入れる方法を説明します。",
            },
        ),
    ])
}
