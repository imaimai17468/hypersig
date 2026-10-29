# use hypersig::html::{El, button, div, part};
# use hypersig::{Action, Part, Patches, Signals};
# use serde::{Deserialize, Serialize};
#[derive(Signals, Serialize, Deserialize)]
struct Account {
    signing_out: bool,
}

hypersig::routes! {
    fn pages(Router<()>);
    action SIGN_OUT: Action<Account> = post "/sign-out" => sign_out;
}
# async fn sign_out((): (), (): ()) -> Patches<Account> { Patches::new() }

impl Part for Account {
    fn render() -> El<Self> {
        button().on_click(SIGN_OUT.call()).text("Sign out")
    }
}

#[derive(Signals, Serialize, Deserialize)]
struct Inbox {
    account: Account,
}

// error[E0277]: `El<Account>` is not a child of an element on the page whose
// signals are `Inbox`
let shell = div::<Inbox>()
    .child(part(Inbox::account()))
    .child(button().on_click(SIGN_OUT.call()).text("Sign out"));
