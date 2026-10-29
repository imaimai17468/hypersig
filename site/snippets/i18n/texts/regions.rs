use hypersig::Segment;
use hypersig_i18n::Locale;
use serde::{Deserialize, Serialize};

#[derive(Segment, Serialize, Clone, Copy, PartialEq, Eq, Debug)]
#[serde(rename_all = "kebab-case")]
enum Lang {
    // `en`, the default.
    En,
    // `pt-br`.
    PtBr,
    // `zh-Hant`, renamed by hand.
    #[serde(rename = "zh-Hant")]
    ZhHant,
}

struct ByLang<T> {
    en: T,
    pt_br: T,
    zh_hant: T,
}

impl Locale for Lang {
    type Text<T> = ByLang<T>;

    fn pick<T>(self, text: ByLang<T>) -> T {
        match self {
            Self::En => text.en,
            Self::PtBr => text.pt_br,
            Self::ZhHant => text.zh_hant,
        }
    }
}

assert_eq!(Lang::PtBr.tag().as_str(), "pt-br");
assert_eq!(Lang::ZhHant.tag().as_str(), "zh-Hant");
# use hypersig::{Page, Route, Signals};
# #[derive(Signals, Serialize, Deserialize)]
# struct Home {}
# async fn home((): (), _: Lang) -> Option<Page<Home>> { None }
# hypersig::routes! {
#     fn pages(Router<()>);
#     const HOME: Route<Lang> = "/{lang}" => home;
# }
# assert_eq!(HOME.url(Lang::ZhHant).as_str(), "/zh-Hant");
# let _ = pages;
