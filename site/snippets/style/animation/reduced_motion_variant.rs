use hypersig::style::value::{AnimationName, Opacity};
use hypersig::style::{Media, Style};

// error[E0599]: no variant, associated function, or constant named
// `PrefersReducedMotion` found for enum `hypersig::hypersig_style::Media`
let fade = Style::new()
    .opacity(Opacity::Number(1.0))
    .on(Media::PrefersReducedMotion, Style::new().animation_name(AnimationName::None));
