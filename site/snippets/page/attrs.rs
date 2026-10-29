use hypersig::html::{attr, el, input};

let name = input::<()>()
    .attr(attr::TYPE, "text")
    .attr(attr::PLACEHOLDER, "Name");
assert_eq!(name.render(), r#"<input type="text" placeholder="Name">"#);

// A tag and attributes with no constant, each checked at compile time.
let map = el::<()>(hypersig::tag!("geo-map"))
    .attr(hypersig::attr!("data-lat"), "35.68")
    .attr(hypersig::attr!("data-lng"), "139.76");
assert_eq!(
    map.render(),
    r#"<geo-map data-lat="35.68" data-lng="139.76"></geo-map>"#,
);
