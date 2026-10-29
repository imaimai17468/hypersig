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
use hypersig::html::{El, a, p};
use hypersig::{Page, SafeUrl, Signals};

#[derive(Signals, Serialize, Deserialize)]
struct Help {}

// A URL per language.
const GUIDE: ByLang<SafeUrl> = ByLang {
    en: SafeUrl::new("https://example.com/en/guide"),
    ja: SafeUrl::new("https://example.com/ja/guide"),
};

// A sentence with a link inside, whose position differs between the languages.
fn read_the_guide(lang: Lang) -> El<Help> {
    let link = || a().href(lang.pick(GUIDE));
    lang.pick(ByLang {
        en: p().text("Read ").child(link().text("the guide")).text(" first."),
        ja: p().text("まず").child(link().text("ガイド")).text("を読んでください。"),
    })
}

let html = Page::new("Help", Help {}).child(read_the_guide(Lang::Ja)).render();
# assert!(html.contains(r#"<p>まず<a href="https://example.com/ja/guide">ガイド</a>を読んでください。</p>"#), "{html}");
