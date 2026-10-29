use hypersig::html::{li, small, ul};

let names = ["Ada", "Grace", "Alan"];
let note: Option<&str> = None;

let list = ul::<()>()
    .children(names.iter().map(|name| li().text(*name)))
    // An `Option` adds zero or one child.
    .children(note.map(|text| li().child(small().text(text))));
assert_eq!(list.render(), "<ul><li>Ada</li><li>Grace</li><li>Alan</li></ul>");
