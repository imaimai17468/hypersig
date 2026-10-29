use hypersig::html::{Class, Stylesheet};

const LEGACY: Stylesheet = Stylesheet::new(".panel { padding: 1rem }");

// error[E0080]: evaluation panicked: the stylesheet has no selector for this class
const PANEL: Class = LEGACY.class("pannel");
