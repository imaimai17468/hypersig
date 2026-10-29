use hypersig::Signals;
use hypersig_ui::progress;
use serde::{Deserialize, Serialize};

#[derive(Signals, Serialize, Deserialize)]
struct Upload {
    done: f64,
}

// The bar takes a whole percentage, and `done` is a fraction.
let bar = progress::<Upload>("Uploading report.pdf", Upload::done());
