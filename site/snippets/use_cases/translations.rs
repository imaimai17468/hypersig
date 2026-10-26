# use hypersig::html::p;
# use hypersig::{Page, Route, Segment, Signals};
# use hypersig_i18n::Locale;
# use serde::{Deserialize, Serialize};
#
#[derive(Segment, Serialize, Deserialize, Clone, Copy, PartialEq, Eq, Debug)] //+
#[serde(rename_all = "lowercase")] //+
enum Lang { //+
    En, //+
    Ja, //+
} //+

# struct ByLang<T> {
#     en: T,
#     ja: T,
# }
#
# impl Locale for Lang {
#     type Text<T> = ByLang<T>;
#
#     fn pick<T>(self, text: ByLang<T>) -> T {
#         match self {
#             Self::En => text.en,
#             Self::Ja => text.ja,
#         }
#     }
# }
#
# #[derive(Signals, Serialize, Deserialize)]
# struct Cart {}
#
hypersig::routes! {
    fn pages(Router<()>);
//-    const CART: Route = "/cart" => cart;
    const CART: Route<Lang> = "/{lang}/cart" => cart; //+
}

//-async fn cart((): (), (): ()) -> Page<Cart> {
//-    Page::new("ご注文内容", Cart {}).child(p().text("送料は無料です。"))
async fn cart((): (), lang: Lang) -> Page<Cart> { //+
    let title = lang.pick(ByLang { en: "Your order", ja: "ご注文内容" }); //+
    let shipping = lang.pick(ByLang { en: "Shipping is free.", ja: "送料は無料です。" }); //+
    Page::new(title, Cart {}).child(p().text(shipping)) //+
}
# let _ = pages;
