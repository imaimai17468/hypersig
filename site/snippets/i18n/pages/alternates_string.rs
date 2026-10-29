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
# use hypersig::{Page, Signals};
# #[derive(Signals, Serialize, Deserialize)]
# struct Post {}
use hypersig::Origin;
use hypersig_i18n::LocalePage;

const SITE: Origin = Origin::new("https://example.com");

// error[E0308]: mismatched types: expected `Url`, found `String`
let page = Page::new("Post", Post {})
    .alternates(SITE, |lang: Lang| format!("/{}/posts/3", lang.tag()));
# let _ = page;
