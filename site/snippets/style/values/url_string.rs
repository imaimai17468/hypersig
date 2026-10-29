use hypersig::style::Style;
use hypersig::style::value::url;

let avatar: String = format!("/avatars/{}.png", 42);
// error[E0308]: mismatched types
//     expected `&str`, found `String`
let style = Style::new().background_image(url(avatar));
