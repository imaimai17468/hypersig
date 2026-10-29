#![allow(unused)]

use hypersig::style::Style;

fn main() {
    // `<grid-line>` has no line 0, so a bare integer is refused for every
    // grid line property, 1 as much as 0.
    let _ = Style::new().grid_row(0);
    let _ = Style::new().grid_row(1);
}
