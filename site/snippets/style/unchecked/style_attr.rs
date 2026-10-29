use hypersig::html::div;

// error[E0080]: evaluation panicked: `El::css` and `El::bind_style` write the
// element's CSS
let card = div::<()>().attr(hypersig::attr!("style"), "margin: 0 auto");
