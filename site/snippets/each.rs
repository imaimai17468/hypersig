use hypersig::html::{each, each_keyed, li, span, ul};
use hypersig::Signals;
use serde::{Deserialize, Serialize};

#[derive(Signals, Serialize, Deserialize)]
struct Track {
    id: i32,
    title: String,
}

#[derive(Signals, Serialize, Deserialize)]
struct Playlist {
    tracks: Vec<Track>,
    tags: Vec<String>,
}

// Matched to their elements by position.
let tags = ul::<Playlist>().child(each(Playlist::tags(), |tag| li().bind_text(tag)));

// Matched by id, so an item keeps its element when the list is reordered.
let tracks = ul::<Playlist>().child(each_keyed(
    Playlist::tracks(),
    |track| track.at(Track::id()),
    |track| {
        li().child(span().bind_text((track.index() + 1).to_text()))
            .child(span().bind_text(track.at(Track::title())))
    },
));
# let _ = (tags, tracks);
