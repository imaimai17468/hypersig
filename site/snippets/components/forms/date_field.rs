use hypersig::html::div;
use hypersig::{CalendarDate, Page, Signals};
use hypersig_ui::DateField;
use serde::{Deserialize, Serialize};

#[derive(Signals, Serialize, Deserialize)]
struct Booking {
    // Empty until the visitor picks a date.
    arrival: Option<CalendarDate>,
    // Always holds a date; emptying the field keeps it.
    departure: CalendarDate,
}

let initial = Booking { arrival: None, departure: CalendarDate::new(2026, 4, 3) };
let page = Page::new("Booking", initial).child(
    div()
        .child(
            DateField::new("Arrival", Booking::arrival())
                .min(CalendarDate::new(2026, 1, 1))
                .max(CalendarDate::new(2026, 12, 31))
                .description("Any day in 2026.")
                .required(),
        )
        .child(DateField::new("Departure", Booking::departure()).min(CalendarDate::new(2026, 1, 2))),
);
# let html = page.render();
# assert!(html.contains(r#"type="date" min="2026-01-01" max="2026-12-31""#));
# assert!(html.contains(r#"value="2026-04-03""#));
