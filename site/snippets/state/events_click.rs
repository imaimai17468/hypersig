use hypersig::html::{button, div};
use hypersig::{Page, Signals};
use serde::{Deserialize, Serialize};

#[derive(Signals, Serialize, Deserialize)]
struct Photo {
    likes: i32,
    zoomed: bool,
}

let likes = Photo::likes();

let like = button::<Photo>().on_click(likes.set(likes + 1)).text("Like");
let picture = div::<Photo>().on_double_click(Photo::zoomed().toggle()).text("Double-click to zoom");
# let html = Page::new("Photo", Photo { likes: 0, zoomed: false }).child(like).child(picture).render();
# let _ = html;
