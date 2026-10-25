#![allow(unused)]
use hypersig::{FileInput, Patches, Signals, Upload, Uploaded};
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
hypersig::routes! {
    fn actions(Router<()>);
    upload AVATAR: Upload<Profile> = "/avatar" => |(), _: Uploaded<()>| async { Patches::<Profile>::new() };
}

fn main() {
    let _ = AVATAR.call(PICTURE);
}
