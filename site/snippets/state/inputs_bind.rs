use hypersig::html::{attr, div, input, textarea};
use hypersig::{CalendarDate, Page, Signals};
use serde::{Deserialize, Serialize};

#[derive(Signals, Serialize, Deserialize)]
struct Booking {
    name: String,
    guests: i32,
    budget: Option<f64>,
    arrival: CalendarDate,
    newsletter: bool,
    notes: String,
}

let form = div::<Booking>()
    .child(input().bind(Booking::name()))
    .child(input().attr(attr::TYPE, "number").bind(Booking::guests()))
    .child(input().attr(attr::TYPE, "number").bind(Booking::budget()))
    .child(input().attr(attr::TYPE, "date").bind(Booking::arrival()))
    .child(input().attr(attr::TYPE, "checkbox").bind(Booking::newsletter()))
    .child(textarea().bind(Booking::notes()));
# let booking = Booking {
#     name: "Ada".into(),
#     guests: 2,
#     budget: None,
#     arrival: CalendarDate::new(2026, 5, 1),
#     newsletter: true,
#     notes: String::new(),
# };
# let html = Page::new("Booking", booking).child(form).render();
# for shown in [r#"value="Ada""#, r#"value="2""#, r#"value="2026-05-01""#, r#"type="checkbox" checked"#] {
#     assert!(html.contains(shown), "{html}");
# }
