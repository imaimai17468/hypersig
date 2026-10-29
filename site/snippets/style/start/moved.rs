use hypersig::Signals;
use hypersig::html::{button, div};
use hypersig::style::Style;
use hypersig::style::value::rem;
use serde::{Deserialize, Serialize};

#[derive(Signals, Serialize, Deserialize)]
struct Form {}

let pill = Style::new().padding_inline(rem(1)).border_radius(rem(1));
// error[E0382]: use of moved value: `pill`
let actions = div::<Form>()
    .child(button().css(pill).text("Save"))
    .child(button().css(pill).text("Cancel"));
