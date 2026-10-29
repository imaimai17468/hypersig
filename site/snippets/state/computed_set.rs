use hypersig::Signals;
use serde::{Deserialize, Serialize};

#[derive(Signals, Serialize, Deserialize)]
struct Cart {
    price: f64,
    #[signals(computed)]
    total: f64,
}

// error[E0599]: the method `set` exists for struct `Sig<Cart, f64, Computed>`, but its trait bounds were not satisfied
let reset = Cart::total().set(0.0);
