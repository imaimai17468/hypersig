use hypersig::html::button;
use hypersig::style::value::{Color, Cursor, Opacity, OutlineStyle, px, rem};
use hypersig::style::{PseudoClass, Style};
use hypersig::{Page, Signals};
use serde::{Deserialize, Serialize};

#[derive(Signals, Serialize, Deserialize)]
struct Form {
    saving: bool,
}

let save = button::<Form>()
    .css(
        Style::new()
            .padding(rem(0.5))
            .background_color(Color::hex(0x0025_63eb))
            .color(Color::WHITE)
            .cursor(Cursor::Pointer)
            .on(PseudoClass::Hover, Style::new().background_color(Color::hex(0x001d_4ed8)))
            .on(PseudoClass::Active, Style::new().background_color(Color::hex(0x001e_3a8a)))
            .on(
                PseudoClass::FocusVisible,
                Style::new().outline_width(px(2)).outline_style(OutlineStyle::Solid),
            )
            .on(
                PseudoClass::Disabled,
                Style::new().opacity(Opacity::Number(0.5)).cursor(Cursor::NotAllowed),
            ),
    )
    .disabled(Form::saving())
    .text("Save");
# let html = Page::new("Form", Form { saving: false }).child(save).render();
# let at = |rule: &str| html.find(rule).unwrap_or_else(|| panic!("no {rule}"));
# assert!(at(":hover{background-color:#1d4ed8}") < at(":focus-visible{outline-style:solid}"));
# assert!(at(":focus-visible{outline-style:solid}") < at(":active{background-color:#1e3a8a}"));
# assert!(at(":active{background-color:#1e3a8a}") < at(":disabled{opacity:0.5}"));
