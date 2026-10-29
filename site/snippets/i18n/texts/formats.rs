# use hypersig::Segment;
# use hypersig_i18n::Locale;
# use serde::{Deserialize, Serialize};
# #[derive(Segment, Serialize, Deserialize, Clone, Copy, PartialEq, Eq, Debug)]
# #[serde(rename_all = "lowercase")]
# enum Lang { En, Ja }
# struct ByLang<T> { en: T, ja: T }
# impl Locale for Lang {
#     type Text<T> = ByLang<T>;
#     fn pick<T>(self, text: ByLang<T>) -> T { match self { Self::En => text.en, Self::Ja => text.ja } }
# }
// `Lang` and `ByLang` are the ones the Overview declares.
use hypersig::html::p;
use hypersig::{
    DateFormat, DateStyle, Expr, IntlLocale, NumberFormat, Page, PluralCategory, Signals,
    Timestamp,
};

#[derive(Signals, Serialize, Deserialize)]
struct Inbox {
    unread: i32,
    credit: f64,
    updated: Timestamp,
}

const PRICE: ByLang<NumberFormat> = ByLang {
    en: NumberFormat::new().digits(2).prefix("€"),
    ja: NumberFormat::new().digits(2).suffix(" ユーロ"),
};

const UPDATED: ByLang<DateFormat> = ByLang {
    en: DateFormat::date(IntlLocale::En, DateStyle::Long),
    ja: DateFormat::date(IntlLocale::Ja, DateStyle::Long),
};

fn unread(lang: Lang) -> Expr<Inbox, String> {
    let count = Inbox::unread().to_text();
    lang.pick(ByLang {
        // English has one form for 1 and another for every other number.
        en: count.concat(Inbox::unread().plural(IntlLocale::En, |category| match category {
            PluralCategory::One => " unread message",
            _ => " unread messages",
        })),
        // Japanese has one form for every number.
        ja: count.concat(" 件の未読メッセージ"),
    })
}

fn inbox(lang: Lang, inbox: Inbox) -> Page<Inbox> {
    Page::new(lang.pick(ByLang { en: "Inbox", ja: "受信箱" }), inbox)
        .lang(lang.tag())
        .child(p().bind_text(unread(lang)))
        .child(p().bind_text(Inbox::credit().format(&lang.pick(PRICE))))
        .child(p().bind_text(Inbox::updated().format_date(&lang.pick(UPDATED))))
}

// 5 January 2026, 00:00 UTC.
let updated = Timestamp::from_millis(1_767_571_200_000);
let en = inbox(Lang::En, Inbox { unread: 1, credit: 1536.5, updated }).render();
let ja = inbox(Lang::Ja, Inbox { unread: 3, credit: 1536.5, updated }).render();
# assert!(en.contains(">1 unread message</p>"), "{en}");
# assert!(en.contains(">€1,536.50</p>"), "{en}");
# assert!(en.contains(">January 5, 2026</p>"), "{en}");
# assert!(ja.contains(">3 件の未読メッセージ</p>"), "{ja}");
# assert!(ja.contains(">1,536.50 ユーロ</p>"), "{ja}");
# assert!(ja.contains(">2026年1月5日</p>"), "{ja}");
# // The `DateStyle` table of the page.
# for (style, en_text, ja_text) in [
#     (DateStyle::Full, "Monday, January 5, 2026", "2026年1月5日月曜日"),
#     (DateStyle::Long, "January 5, 2026", "2026年1月5日"),
#     (DateStyle::Medium, "Jan 5, 2026", "2026/01/05"),
#     (DateStyle::Short, "1/5/26", "2026/01/05"),
# ] {
#     assert_eq!(DateFormat::date(IntlLocale::En, style).write(updated).as_deref(), Some(en_text));
#     assert_eq!(DateFormat::date(IntlLocale::Ja, style).write(updated).as_deref(), Some(ja_text));
# }
