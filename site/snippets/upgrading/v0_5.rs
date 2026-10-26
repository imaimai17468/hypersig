# use hypersig::Signals;
# use hypersig::html::{Id, Role, attr, div, el, input, label};
# use serde::{Deserialize, Serialize};
#
# #[derive(Signals, Serialize, Deserialize)]
# struct Search {
#     query: String,
# }
#
//-const QUERY: &str = "query";
const QUERY: Id = Id::new("query"); //+

let form = div::<Search>()
//-    .attr("role", "search")
    .role(Role::Search) //+
//-    .attr("aria-label", "Docs")
    .attr(attr::ARIA_LABEL, "Docs") //+
//-    .attr("data-lat", "35.68")
    .attr(hypersig::attr!("data-lat"), "35.68") //+
//-    .child(label().attr("for", QUERY).text("Search"))
    .child(label().label_for(&QUERY).text("Search")) //+
    .child(input().id(QUERY).bind(Search::query()))
//-    .child(el("sl-rating"));
    .child(el(hypersig::tag!("sl-rating"))); //+
# assert!(form.render().contains(r#"for="query""#));
