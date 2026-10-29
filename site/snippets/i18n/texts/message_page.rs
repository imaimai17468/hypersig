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
use hypersig::html::{h1, p};
use hypersig::{Page, Route, Signals};

#[derive(Signals, Serialize, Deserialize)]
struct Archive {}

const TITLE: ByLang<&str> = ByLang { en: "Archive", ja: "アーカイブ" };

const PAGE_OF: ByLang<fn(u32, u32) -> String> = ByLang {
    en: |page, total| format!("Page {page} of {total}"),
    ja: |page, total| format!("{total} ページ中 {page} ページ目"),
};

hypersig::routes! {
    fn pages(Router<()>);
    const ARCHIVE: Route<(Lang, u32)> = "/{lang}/archive/{page}" => archive;
}

const TOTAL: u32 = 12;

async fn archive((): (), (lang, page): (Lang, u32)) -> Option<Page<Archive>> {
    (1..=TOTAL).contains(&page).then(|| {
        Page::new(lang.pick(TITLE), Archive {})
            .lang(lang.tag())
            .child(h1().text(lang.pick(TITLE)))
            .child(p().text(lang.pick(PAGE_OF)(page, TOTAL)))
    })
}
# let page = tokio::runtime::Runtime::new().unwrap().block_on(archive((), (Lang::Ja, 3)));
# let html = page.expect("page 3 exists").render();
# assert!(html.contains("<p>12 ページ中 3 ページ目</p>"), "{html}");
# assert_eq!(ARCHIVE.url((Lang::Ja, 3)).as_str(), "/ja/archive/3");
# let _ = pages;
