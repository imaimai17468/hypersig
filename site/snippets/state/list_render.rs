use hypersig::html::{each_keyed, li, ul};
use hypersig::{Page, Signals};
use serde::{Deserialize, Serialize};

#[derive(Signals, Serialize, Deserialize)]
struct Track {
    id: i32,
    title: String,
}

#[derive(Signals, Serialize, Deserialize)]
struct Playlist {
    tracks: Vec<Track>,
}

let tracks = vec![
    Track { id: 7, title: "Intro".into() },
    Track { id: 9, title: "Outro".into() },
];
let html = Page::new("Playlist", Playlist { tracks })
    .child(ul().child(each_keyed(
        Playlist::tracks(),
        |track| track.at(Track::id()),
        |track| li().bind_text(track.at(Track::title())),
    )))
    .render();
assert!(html.contains(">Intro</li>") && html.contains(">Outro</li>"));
