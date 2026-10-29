# use hypersig::style::Style;
# use hypersig::style::value::Color;
#
//-let body = Style::new().line_height(2);
let body = Style::new().line_height(2.0); //+
//-let overlay = Style::new().background_color(Color::hex(0x2563_eb80));
let overlay = Style::new().background_color(Color::hex_alpha(0x2563_eb80)); //+
# let _ = (body, overlay);
