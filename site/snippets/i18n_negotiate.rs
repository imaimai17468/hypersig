# use hypersig::Segment;
# use hypersig_i18n::Locale;
# use serde::{Deserialize, Serialize};
# #[derive(Segment, Serialize, Deserialize, Clone, Copy, PartialEq, Eq, Debug)]
# #[serde(rename_all = "lowercase")]
# enum Lang { En, Ja }
# struct ByLang<T> { en: T, ja: T }
# impl Locale for Lang {
#     const ALL: &'static [Self] = &[Self::En, Self::Ja];
#     type Text<T> = ByLang<T>;
#     fn pick<T>(self, text: ByLang<T>) -> T { match self { Self::En => text.en, Self::Ja => text.ja } }
# }
# use hypersig::{Page, Route, Signals};
# #[derive(Signals, Serialize, Deserialize)]
# struct Home { visits: i32 }
# async fn home((): (), _: Lang) -> Option<Page<Home>> { None }
use axum::Router;
use hypersig_i18n::{LocaleRouterExt, negotiate};

assert_eq!(negotiate::<Lang>("ja-JP,ja;q=0.9,en;q=0.8"), Lang::Ja);
assert_eq!(negotiate::<Lang>("fr, en;q=0.5"), Lang::En);
assert_eq!(negotiate::<Lang>("de"), Lang::En);

hypersig::routes! {
    fn pages(Router<()>);
    const HOME: Route<Lang> = "/{lang}" => home;
}

// `GET /` answers with a redirect to `/en` or `/ja`.
let _app: Router = pages(Router::new()).redirect_to_locale(HOME);
