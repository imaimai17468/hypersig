# use hypersig::style::value::{Url, url};
# use hypersig::style::{FontFace, Style};
#
//-let paper = url(format!("/{}", "paper.png"));
const PAPER: Url = url("/paper.png"); //+
const INTER: Url = url("/fonts/inter.woff2"); //+

//-let inter = FontFace::new("Inter", url("/fonts/inter.woff2".to_owned()));
let inter = FontFace::new("Inter", INTER); //+
//-let style = Style::new().background_image(paper);
let style = Style::new().background_image(PAPER); //+
# let _ = (inter, style);
