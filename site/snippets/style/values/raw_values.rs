use hypersig::style::Style;
use hypersig::style::value::raw;

let overlay = Style::new()
    // A `calc()` of numbers, or of a length and a percentage, has no typed form.
    .z_index(raw("calc(var(--layer) + 1)"))
    .padding_top(raw("calc(100% - 2rem)"))
    // `Color` converts from `Raw` too, for a color function it has no constructor for.
    .color(raw("color(display-p3 1 0.5 0)"));
# let css = hypersig::style::Global::new().rule(hypersig::style::Element::Body, overlay).to_css();
# assert!(css.contains("z-index:calc(var(--layer) + 1)"));
# assert!(css.contains("padding-top:calc(100% - 2rem)"));
# assert!(css.contains("color:color(display-p3 1 0.5 0)"));
