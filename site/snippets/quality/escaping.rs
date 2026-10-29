use hypersig::html::{attr, div, p};

// A name a visitor stored, written into an attribute and into the text.
let name = r#"<img src=x onerror="alert(1)">"#;
let card = div::<()>().attr(attr::TITLE, name).child(p().text(name));

assert_eq!(
    card.render(),
    r#"<div title="&lt;img src=x onerror=&quot;alert(1)&quot;&gt;"><p>&lt;img src=x onerror="alert(1)"&gt;</p></div>"#,
);
