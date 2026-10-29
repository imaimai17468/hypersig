# use hypersig::html::{El, each, li, part, span, ul};
# use hypersig::{Part, Signals};
# use serde::{Deserialize, Serialize};
#[derive(Signals, Serialize, Deserialize)]
struct Badge {
    count: i32,
}

impl Part for Badge {
    fn render() -> El<Self> {
        span().bind_text(Badge::count().to_text())
    }
}

#[derive(Signals, Serialize, Deserialize)]
struct Shelf {
    badges: Vec<Badge>,
}

// error[E0308]: mismatched types: expected `Sig<_, _>`, found
// `Sig<Shelf, Badge, Local>`
let list = ul::<Shelf>().child(each(Shelf::badges(), |badge| li().child(part(badge.value()))));
