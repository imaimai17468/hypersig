use hypersig::Signals;
use hypersig::html::a;
use hypersig_ui::{ButtonSize, ButtonVariant, button_style};
use serde::{Deserialize, Serialize};

#[derive(Signals, Serialize, Deserialize)]
struct App {}

// The address written as a string rather than taken from a route.
let signup = a::<App>()
    .href("/signup")
    .css(button_style(ButtonVariant::Default, ButtonSize::Lg))
    .text("Sign up");
# let _ = signup;
