//! The i18n section: typed translations from `hypersig_i18n`.

mod start;
mod texts;

use crate::Site;
use crate::doc::{Doc, I18nPage};
use crate::lang::L;
use crate::learn::snippet::{Snippet, show};
use crate::prose::{Part, heading, lead, note, para, parts, title};

/// The article of `page`.
pub fn content(site: Site, page: I18nPage) -> Part {
    let head = title(Doc::I18n(page).title(site));
    match page {
        // Get started
        I18nPage::Overview => start::overview::page(site, head),
        // Texts
        I18nPage::Locale => texts::locale::page(site, head),
        I18nPage::Texts => texts::writing::page(site, head),
        I18nPage::Messages => texts::messages::page(site, head),
        I18nPage::Formats => texts::formats::page(site, head),
        // Pages in each language
        I18nPage::Pages
        | I18nPage::LanguageSwitcher
        | I18nPage::PageLanguage
        | I18nPage::Alternates => pages(site, head),
        // Choosing the language
        I18nPage::Negotiation | I18nPage::Redirect => negotiation(site, head),
    }
}

fn pages(site: Site, head: Part) -> Part {
    parts([
        head,
        lead(site, Doc::I18n(I18nPage::Pages).leads()),
        heading(
            site,
            L {
                en: "The language in the route",
                ja: "ルートに言語を入れる",
            },
        ),
        para(
            site,
            L {
                en: "A `Locale` is a [`Segment`](api:hypersig/trait.Segment.html), so it can be the first parameter of a route, as in `Route<(Lang, i32)>` for `\"/{lang}/members/{id}\"`. The handler receives the language with the other parameters, and a request for a language the enum does not have gets a 404 before the handler runs.",
                ja: "`Locale` は [`Segment`](api:hypersig/trait.Segment.html) なので、`\"/{lang}/members/{id}\"` に対する `Route<(Lang, i32)>` のように、ルートの最初のパラメータにできます。ハンドラはほかのパラメータと一緒に言語を受け取ります。enum にない言語へのリクエストには、ハンドラを呼ぶ前に 404 を返します。",
            },
        ),
        show(Snippet::I18nPages),
        heading(
            site,
            L {
                en: "Links to the other languages",
                ja: "ほかの言語へのリンク",
            },
        ),
        para(
            site,
            L {
                en: "To link to the same page in another language, pass `link_with` the other language and the same remaining parameters. The route then checks that link as it checks any other. A language switcher lists `locales::<Lang>()`, every variant of the enum, and links, in each language, to the page the reader is on.",
                ja: "同じページの別の言語へのリンクは、`link_with` に別の言語とそのほかの同じパラメータを渡して作ります。そのため、ほかのリンクと同じくルートがこのリンクを検査します。言語を切り替える UI は列挙型のすべてのバリアントを返す `locales::<Lang>()` を並べ、言語ごとに読者が今いるページへリンクします。",
            },
        ),
        heading(
            site,
            L {
                en: "The language of the page",
                ja: "ページの言語",
            },
        ),
        para(
            site,
            L {
                en: "[`Page::lang`](api:hypersig/struct.Page.html#method.lang) writes the tag into `<html lang>`, and `Page::alternate` writes it into `hreflang`. Both take a [`LanguageTag`](api:hypersig/struct.LanguageTag.html), which `tag` returns, so a segment such as `en_US` that is not shaped as a language tag panics when `tag` reads it. The browser uses it to decide hyphenation, the voice a screen reader uses, and the fonts, so Japanese text set with `lang=\"ja\"` gets Japanese glyphs. In a debug build, `Page::render` panics where `lang` names the language of an `IntlLocale` and the page formats its plurals and dates only with other ones, such as `lang=\"ja\"` with only `IntlLocale::En`.",
                ja: "[`Page::lang`](api:hypersig/struct.Page.html#method.lang) はタグを `<html lang>` に書き、`Page::alternate` は `hreflang` に書きます。どちらも `tag` が返す [`LanguageTag`](api:hypersig/struct.LanguageTag.html) を受け取ります。そのため、`en_US` のように言語タグの形をしていないセグメントは、`tag` が読んだときにパニックします。ブラウザはこの属性を見て、ハイフネーション、スクリーンリーダーの読み上げの声、使うフォントを決めます。そのため、`lang=\"ja\"` の付いた日本語の文は日本語の字形で表示されます。デバッグビルドでは、`lang` が `IntlLocale` のある言語を指しており、ページが複数形と日付をほかの `IntlLocale` だけで書くと、`Page::render` がパニックします。`lang=\"ja\"` のページで `IntlLocale::En` だけを使う場合がこれに当たります。",
            },
        ),
        para(
            site,
            L {
                en: "[`LocalePage::alternates`](api:hypersig_i18n/trait.LocalePage.html#tymethod.alternates) takes an `Origin` and a function from a language to the page's `Url`, as in `|lang| POST.url((lang, id))`. It writes one `<link rel=\"alternate\" hreflang>` per variant of the language enum, and an `x-default` at the first variant, so a search engine shows each reader the page in their language.",
                ja: "[`LocalePage::alternates`](api:hypersig_i18n/trait.LocalePage.html#tymethod.alternates) は `Origin` と、`|lang| POST.url((lang, id))` のように言語からページの `Url` を返す関数を受け取ります。言語の列挙型のバリアントごとに `<link rel=\"alternate\" hreflang>` を 1 つ書き、最初のバリアントには `x-default` も書きます。検索エンジンはこれを見て、読者の言語のページを結果に出します。",
            },
        ),
    ])
}

fn negotiation(site: Site, head: Part) -> Part {
    parts([
        head,
        lead(site, Doc::I18n(I18nPage::Negotiation).leads()),
        heading(
            site,
            L {
                en: "negotiate",
                ja: "negotiate",
            },
        ),
        para(
            site,
            L {
                en: "[`negotiate`](api:hypersig_i18n/fn.negotiate.html) reads an `Accept-Language` value and returns the variant of the language enum that matches it best. It tries the ranges from the highest `q` to the lowest, and in the order written where two have the same `q`. A range first matches a tag equal to it, ignoring case. Next it matches a tag equal to the range with its last subtags dropped, so `ja-JP` matches `ja`. Last it matches a tag that starts with the range, so `en` matches `en-US`. `*`, an empty header, and a header with no match give the first variant, the default.",
                ja: "[`negotiate`](api:hypersig_i18n/fn.negotiate.html) は `Accept-Language` の値を読み、言語の列挙型のバリアントから最も合うものを返します。範囲は `q` の大きい順に試し、`q` が同じものは書かれた順に試します。範囲はまず、大文字と小文字を区別せずに範囲と等しいタグに一致します。次に、範囲の後ろのサブタグを削った形と等しいタグに一致するので、`ja-JP` は `ja` に一致します。最後に、範囲で始まるタグに一致するので、`en` は `en-US` に一致します。`*`、空のヘッダー、一致する言語のないヘッダーには、既定の言語である最初のバリアントを返します。",
            },
        ),
        heading(
            site,
            L {
                en: "The redirect from /",
                ja: "/ からのリダイレクト",
            },
        ),
        para(
            site,
            L {
                en: "With the `axum` feature, [`redirect_to_locale`](api:hypersig_i18n/trait.LocaleRouterExt.html#tymethod.redirect_to_locale) responds to `GET /` with a `307 Temporary Redirect` to a route whose only parameter is the language. The response includes `Vary: Accept-Language`, because the language redirected to can differ between browsers.",
                ja: "`axum` の feature を有効にすると、[`redirect_to_locale`](api:hypersig_i18n/trait.LocaleRouterExt.html#tymethod.redirect_to_locale) は `GET /` に対して、言語だけをパラメータに持つルートへの `307 Temporary Redirect` を返します。ブラウザによってリダイレクト先の言語が変わることがあるので、レスポンスには `Vary: Accept-Language` を付けます。",
            },
        ),
        show(Snippet::I18nNegotiate),
        note(
            site,
            L {
                en: "A site served as static files, such as this one, has no server to read `Accept-Language`. Such a site puts its default language at the root and links to the others from the header.",
                ja: "このサイトのように静的ファイルとして配信するサイトには、`Accept-Language` を読むサーバーがありません。こうしたサイトでは既定の言語をルートに置き、ほかの言語へはヘッダーからリンクします。",
            },
        ),
    ])
}
