#![allow(unused)]
use hypersig::{FileInput, Signals, Upload};
use serde::{Deserialize, Serialize};

#[derive(Signals, Serialize, Deserialize)]
struct Profile {
    name: String,
}

#[derive(Signals, Serialize, Deserialize)]
struct Gallery {
    title: String,
}

const PICTURE: FileInput<Gallery> = FileInput::new("picture");
const AVATAR: Upload<Profile> = Upload::post("/avatar");

fn main() {
    let _ = AVATAR.call(PICTURE);
}
