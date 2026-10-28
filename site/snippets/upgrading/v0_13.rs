# use hypersig::style::Style;
# use hypersig::style::value::Color;
#
//-let style = Style::new().scrollbar_color(Color::GRAY);
let style = Style::new().scrollbar_color((Color::GRAY, Color::TRANSPARENT)); //+
# let _ = style;
