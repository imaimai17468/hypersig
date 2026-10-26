# use axum::http::StatusCode;
# use hypersig::html::{button, div, p};
# use hypersig::{Action, Patches, Signals};
# use serde::{Deserialize, Serialize};
#
#[derive(Signals, Serialize, Deserialize)]
struct Booking {
    seat: String,
    failure: i32, //+
}

# #[derive(Clone)]
# struct AppState;
#
# impl AppState {
#     fn hold(&self, _seat: &str) -> bool {
#         false
#     }
# }
#
# hypersig::routes! {
#     fn pages(Router<AppState>);
#     action HOLD: Action<Booking, String> = post "/seats/hold" => hold;
# }
#
let taken = i32::from(StatusCode::CONFLICT.as_u16()); //+
let _ = div()
//-    .child(button().on_click(HOLD.call_with(Booking::seat())).text("Hold this seat"))
    .child(button().on_click(HOLD.call_with(Booking::seat()).status(Booking::failure())).text("Hold this seat")) //+
    .child(p().show(Booking::failure().equals(taken)).text("Someone took this seat a moment ago. Pick another.")); //+

async fn hold(state: AppState, seat: String) -> Result<Patches<Booking>, StatusCode> {
    if !state.hold(&seat) {
        return Err(StatusCode::CONFLICT);
    }
    Ok(Patches::new())
}
# let _ = pages;
