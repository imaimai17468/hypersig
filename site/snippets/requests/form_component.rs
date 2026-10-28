# use hypersig::html::{El, Id, div, input, p};
# use hypersig::{Form, Rules, Sig, Signals, Validate};
# use serde::{Deserialize, Serialize};
# #[derive(Signals, Serialize, Deserialize, Clone, Default)]
# struct Shipping {
#     street: String,
#     city: String,
# }
# impl Validate for Shipping {
#     fn rules<S>(form: Sig<S, Self>, rules: &mut Rules<S, Self>) {
#         let city = form.at(Shipping::city());
#         rules.check(Shipping::city(), !city.is_empty(), "Enter a city.");
#     }
# }
# #[derive(Signals, Serialize, Deserialize)]
# struct Checkout {
#     shipping: Form<Shipping>,
# }
const CITY_ERROR: Id = Id::new("city-error");

// Two inputs for one field: the street and the city are checked as the city.
fn address(form: Sig<Checkout, Form<Shipping>>) -> El<Checkout> {
    let city = Shipping::city();
    div()
        // Focus that moves from one input to the other does not leave the `div`.
        .on_blur(form.touch(city))
        .child(input().bind(form.field(Shipping::street())))
        .child(input().bind(form.field(city)).aria_describedby(&[&CITY_ERROR]))
        .child(p().id(CITY_ERROR).bind_text(form.error(city)))
}
# let _ = address(Checkout::shipping());
