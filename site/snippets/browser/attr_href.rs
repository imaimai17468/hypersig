use hypersig::html::a;

// `El::href` sets `href`, so `attr!` refuses the name.
let link = a::<()>().attr(hypersig::attr!("href"), "javascript:alert(1)");
# let _ = link;
