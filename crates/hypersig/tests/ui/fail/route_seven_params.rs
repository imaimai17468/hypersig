//! A route with more parameters than a tuple of `Segment`s can hold.

use hypersig::Route;

const DEEP: Route<(i32, i32, i32, i32, i32, i32, i32)> = Route::new("/{a}/{b}/{c}/{d}/{e}/{f}/{g}");

fn main() {
    let _ = DEEP.path();
}
