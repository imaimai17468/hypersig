use hypersig::style::Theme;
use hypersig::style::value::Color;
use hypersig_ui::token;

// The radius is a length, and a color is given.
let theme = Theme::new().set(&token::radius(), Color::hex(0x0025_63eb));
# let _ = theme;
