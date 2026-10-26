# use hypersig::{Asset, LanguageTag, Page, Signals};
# use serde::{Deserialize, Serialize};
#
# #[derive(Signals, Serialize, Deserialize)]
# struct Map {}
#
const MAP_JS: Asset = Asset::new("/map.js"); //+
const JAPANESE: LanguageTag = LanguageTag::new("ja"); //+

let page = Page::new("Map", Map {})
//-    .lang("ja")
    .lang(JAPANESE) //+
//-    .script("/map.js");
    .script(MAP_JS); //+
# assert!(page.render().contains(r#"src="/map.js""#));
