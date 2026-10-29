use hypersig::style::value::{Url, url};

// error[E0080]: evaluation panicked: a URL a page loads is a path or an `http` or
//     `https` URL, with no space or control character
const TRACKER: Url = url("javascript:alert(1)");
