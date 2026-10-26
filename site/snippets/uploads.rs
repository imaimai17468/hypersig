use axum::Router;
use axum::http::StatusCode;
use hypersig::html::{button, div, attr};
use hypersig::{FileInput, Page, Patches, Signals, Upload, Uploaded};
use serde::{Deserialize, Serialize};

#[derive(Signals, Serialize, Deserialize)]
struct Profile {
    picture: String,
    uploading: bool,
    problem: String,
}

// Both constants carry the page's type, so a file input from another page does not compile.
const PICTURE: FileInput<Profile> = FileInput::new("picture");

hypersig::routes! {
    fn pages(Router<()>);
    upload AVATAR: Upload<Profile> = "/avatar" max_bytes 256 * 1024 => avatar;
}

let signals = Profile { picture: String::new(), uploading: false, problem: String::new() };
let page = Page::new("Profile", signals).child(
    div()
        .child(PICTURE.input().attr(attr::ACCEPT, "image/png"))
        .child(
            button()
                .on_click(
                    AVATAR
                        .call(PICTURE)
                        .indicator(Profile::uploading())
                        .error(Profile::problem())
                        .then(PICTURE.clear()),
                )
                .text("Upload"),
        ),
);

// The handler gets the arguments and the files, and answers as an action's does.
async fn avatar((): (), uploaded: Uploaded<()>) -> Result<Patches<Profile>, StatusCode> {
    let [file] = uploaded.files() else {
        return Err(StatusCode::UNPROCESSABLE_ENTITY);
    };
    if !file.bytes().starts_with(b"\x89PNG") {
        return Err(StatusCode::UNPROCESSABLE_ENTITY);
    }
    let name = file.file_name().unwrap_or("picture").to_owned();
    Ok(Profile::picture().patch(name).into())
}

let _app: Router = pages(Router::new());
# let _ = page.render();
