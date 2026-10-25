#![allow(unused)]
use hypersig::html::{El, button, div, part};
use hypersig::{Action, Part, Patches, Signals};
use serde::{Deserialize, Serialize};

#[derive(Signals, Serialize, Deserialize)]
struct Account {
    signing_out: bool,
}

hypersig::routes! {
    fn actions(Router<()>);
    action SIGN_OUT: Action<Account> = post "/sign-out" => |(), ()| async { Patches::<Account>::new() };
}

impl Part for Account {
    fn render() -> El<Self> {
        button().on_click(SIGN_OUT.call())
    }
}

#[derive(Signals, Serialize, Deserialize)]
struct Inbox {
    account: Account,
}

fn main() {
    // A part's action is called inside the part, which tells the server the
    // field its patches land under.
    let _ = div::<Inbox>()
        .child(part(Inbox::account()))
        .child(button().on_click(SIGN_OUT.call()));
}
