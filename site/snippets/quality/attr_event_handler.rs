use hypersig::html::img;

// error[E0080]: evaluation panicked: an `on*` attribute runs a script;
// attach a handler with the `on_*` methods
let _ = img::<()>().attr(hypersig::attr!("onError"), "alert(1)");
