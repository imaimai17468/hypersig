use hypersig::{FileInput, Patches, Signals, Upload, Uploaded};
use serde::{Deserialize, Serialize};

#[derive(Signals, Serialize, Deserialize)]
struct Profile {}

#[derive(Signals, Serialize, Deserialize)]
struct Gallery {}

const PHOTO: FileInput<Gallery> = FileInput::new("photo");

hypersig::routes! {
    fn pages(Router<()>);
    upload AVATAR: Upload<Profile> = "/avatar" => avatar;
}
# async fn avatar((): (), _: Uploaded<()>) -> Patches<Profile> { Patches::new() }
# let _ = pages;

// error[E0308]: mismatched types
// expected `FileInput<Profile>`, found `FileInput<Gallery>`
let _upload = AVATAR.call(PHOTO);
